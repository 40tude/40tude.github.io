#Requires -Version 7.0
<#
.SYNOPSIS
    Create a new article from assets/_article_template.md.

.DESCRIPTION
    SETUP (done once, do not forget it exists):
      $PROFILE (C:\Users\phili\OneDrive\Documents\PowerShell\Microsoft.PowerShell_profile.ps1)
      contains this function, which makes the "new-post" command available:

        function new-post {
            & "$HOME\OneDrive\Documents\40tude.github.io\tools\New-Post.ps1" @args
        }

      If the repo moves, update the path there. To see it: code $PROFILE

    USAGE:
      cd docs\03_maths              # any category folder holding an index.md
      new-post mon_article          # auto number: max existing + 1 (9xx ignored)
      new-post 088_mon_article      # explicit number

    WHAT IT DOES:
      - Creates <NNN>_<name>\ with assets\ and raw_assets\ (raw_assets stays empty)
      - Creates <NNN>_<name>\<name>.md from assets\_article_template.md
      - Copies assets\_article_template_img00.webp as assets\img00.webp (placeholder)
      - Frontmatter: parent = title: of .\index.md, image = computed path,
        date and last_modified_date = now. Everything else kept as in template.
      - Opens the .md in VS Code

    FAILS IF:
      - current folder is not under <repo>\docs or has no index.md
      - target folder already exists or number is already used in this folder
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory, Position = 0)]
    [string]$Name
)

$ErrorActionPreference = 'Stop'

function Stop-WithError([string]$Msg) {
    Write-Host "new-post: $Msg" -ForegroundColor Red
    exit 1
}

# ---------------------------------------------------------------- repo + paths
$repoRoot = git rev-parse --show-toplevel 2>$null
if (-not $repoRoot) { Stop-WithError "not inside a git repository" }
$repoRoot = (Resolve-Path $repoRoot).Path

$template    = Join-Path $repoRoot 'assets' '_article_template.md'
$placeholder = Join-Path $repoRoot 'assets' '_article_template_img00.webp'
foreach ($f in $template, $placeholder) {
    if (-not (Test-Path $f)) { Stop-WithError "missing $f" }
}

$here     = (Get-Location).Path
$docsRoot = Join-Path $repoRoot 'docs'
if (-not $here.StartsWith($docsRoot, [StringComparison]::OrdinalIgnoreCase)) {
    Stop-WithError "run it from a category folder under $docsRoot"
}

# ---------------------------------------------------------------- parent
$indexMd = Join-Path $here 'index.md'
if (-not (Test-Path $indexMd)) { Stop-WithError "no index.md in $here (not a category folder?)" }
$titleLine = Get-Content $indexMd -Encoding utf8 | Where-Object { $_ -match '^title:\s*' } | Select-Object -First 1
if (-not $titleLine) { Stop-WithError "no title: in $indexMd" }
$parent = ($titleLine -replace '^title:\s*', '').Trim().Trim('"', "'")

# ---------------------------------------------------------------- name + number
if ($Name -notmatch '^(?:(\d{3})_)?([A-Za-z0-9][A-Za-z0-9_-]*)$') {
    Stop-WithError "invalid name '$Name' (expected mon_article or 088_mon_article)"
}
$number   = $Matches[1]
$baseName = $Matches[2]

$existing = Get-ChildItem -Directory | Where-Object Name -Match '^(\d{3})_' |
    ForEach-Object { [int]($_.Name.Substring(0, 3)) }

if (-not $number) {
    $regular = @($existing | Where-Object { $_ -lt 900 })
    $next    = if ($regular.Count) { ($regular | Measure-Object -Maximum).Maximum + 1 } else { 1 }
    $number  = '{0:D3}' -f [int]$next
}
elseif ($existing -contains [int]$number) {
    $clash = (Get-ChildItem -Directory -Filter "${number}_*").Name -join ', '
    Stop-WithError "number $number already used: $clash"
}

$folderName = "${number}_$baseName"
$postDir    = Join-Path $here $folderName
if (Test-Path $postDir) { Stop-WithError "$postDir already exists" }

# ---------------------------------------------------------------- frontmatter
$relDir   = [IO.Path]::GetRelativePath($repoRoot, $postDir) -replace '\\', '/'
$imageRel = "$relDir/assets/img00.webp"
$now      = Get-Date -Format 'yyyy-MM-dd HH:mm:ss'

$content = Get-Content $template -Raw -Encoding utf8

# Only touch the first frontmatter block
if ($content -notmatch '(?s)^(---\r?\n.*?\r?\n---)(.*)$') { Stop-WithError "no frontmatter in template" }
$fm   = $Matches[1]
$body = $Matches[2]

$fm = $fm -replace '(?m)^image:[^\r\n]*',              "image: $imageRel"
$fm = $fm -replace '(?m)^parent:[^\r\n]*',             "parent: `"$parent`""
$fm = $fm -replace '(?m)^(date:\s*)[^\r\n]*',          "`${1}$now"
$fm = $fm -replace '(?m)^last_modified_date:[^\r\n]*', "last_modified_date: $now"

# ---------------------------------------------------------------- write
New-Item -ItemType Directory -Path (Join-Path $postDir 'assets')     | Out-Null
New-Item -ItemType Directory -Path (Join-Path $postDir 'raw_assets') | Out-Null
Copy-Item $placeholder (Join-Path $postDir 'assets' 'img00.webp')

$mdPath = Join-Path $postDir "$baseName.md"
[IO.File]::WriteAllText($mdPath, $fm + $body, [Text.UTF8Encoding]::new($false))

Write-Host "Created $relDir/$baseName.md (parent: $parent)" -ForegroundColor Green

if (Get-Command code -ErrorAction SilentlyContinue) { code $mdPath }
