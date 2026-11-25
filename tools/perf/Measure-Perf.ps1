<#
.SYNOPSIS
    Measures www.40tude.fr with Lighthouse and logs timestamped results.

.DESCRIPTION
    Runs Lighthouse (mobile + desktop) on every page listed in pages.json,
    keeps the median of several runs, saves the HTML reports in
    results/<date_time>/ and appends one line per page/profile to history.csv.
    The comparison with the previous run is printed at the end.

    Requires Node.js (LTS) and Google Chrome. The npm dependencies are
    installed automatically on first use. -Local also requires Ruby and the
    site's gems ("bundle install" at the repository root).

.PARAMETER Label
    Free text stored in history.csv, e.g. "lazy search". Use it to tell
    what changed since the previous measurement.

.PARAMETER Local
    Builds the site with Jekyll from the working copy and measures it on
    http://localhost:4848 instead of the live site. Useful to check a change
    before pushing it.

.PARAMETER Runs
    Lighthouse runs per page and profile (default 3, the median is kept).

.PARAMETER Profiles
    mobile, desktop or both (default both).

.PARAMETER Pages
    Page ids from pages.json (default all).

.EXAMPLE
    ./tools/perf/Measure-Perf.ps1 -Label "baseline"

.EXAMPLE
    ./tools/perf/Measure-Perf.ps1 -Local -Label "lazy search" -Pages home
#>
[CmdletBinding()]
param(
    [string]   $Label = '',
    [switch]   $Local,
    [ValidateRange(1, 20)]
    [int]      $Runs = 3,
    [ValidateSet('mobile', 'desktop')]
    [string[]] $Profiles = @('mobile', 'desktop'),
    [string[]] $Pages
)

$ErrorActionPreference = 'Stop'
$here = $PSScriptRoot

# Check prerequisites
foreach ($cmd in @('node', 'npm')) {
    if (-not (Get-Command $cmd -ErrorAction SilentlyContinue)) {
        throw "$cmd not found. Install Node.js LTS (winget install OpenJS.NodeJS.LTS)."
    }
}
if ($Local -and -not (Get-Command 'bundle' -ErrorAction SilentlyContinue)) {
    throw "bundle not found. -Local needs Ruby and 'bundle install' at the repository root."
}

# Install the npm dependencies on first use (or after package-lock.json changed)
$stamp = Join-Path $here 'node_modules/.package-lock.json'
$lock  = Join-Path $here 'package-lock.json'
if (-not (Test-Path $stamp) -or (Get-Item $lock).LastWriteTime -gt (Get-Item $stamp).LastWriteTime) {
    Write-Host 'Installing npm dependencies (first run only)...'
    Push-Location $here
    try {
        npm ci --no-audit --no-fund
        if ($LASTEXITCODE -ne 0) { throw 'npm ci failed.' }
    }
    finally { Pop-Location }
}

# Build the argument list for the Node.js script
$nodeArgs = @(
    (Join-Path $here 'measure.mjs'),
    '--target', $(if ($Local) { 'local' } else { 'live' }),
    '--runs', $Runs,
    '--profiles', ($Profiles -join ',')
)
if ($Label) { $nodeArgs += @('--label', $Label) }
if ($Pages) { $nodeArgs += @('--pages', ($Pages -join ',')) }

Write-Host "Measuring $(if ($Local) { 'the local build' } else { 'www.40tude.fr' }): $Runs run(s) x $($Profiles -join '+')"
Push-Location $here
try {
    node @nodeArgs
    if ($LASTEXITCODE -ne 0) { throw "Measurement failed (exit code $LASTEXITCODE)." }
}
finally { Pop-Location }
