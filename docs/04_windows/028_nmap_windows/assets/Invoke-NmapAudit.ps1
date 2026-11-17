<#
.SYNOPSIS
    Monthly home network audit with Nmap, then compare with the previous audit using ndiff.

.DESCRIPTION
    Internal mode (default): scans the local network (auto-detected) with service detection.
    External mode (-External): scans your public IPv4 address on all TCP ports.
    Must be run from OUTSIDE your home network (phone hotspot, for example).

    Results go to $HOME\nmap-audits\internal or $HOME\nmap-audits\external.
    Folders are created if needed. Only the last -Keep audits are kept (default 2).
    At the end, ndiff compares the two most recent audits.

.EXAMPLE
    .\Invoke-NmapAudit.ps1
    Internal audit of the auto-detected local network.

.EXAMPLE
    .\Invoke-NmapAudit.ps1 -Network 192.168.1.0/24
    Internal audit of a given network.

.EXAMPLE
    .\Invoke-NmapAudit.ps1 -External -PublicIP 83.x.x.x
    External audit of your public address. Connect to your phone hotspot first.

.NOTES
    Requires Nmap (with ndiff) from https://nmap.org/download.html
    Run from an elevated (administrator) PowerShell.
    Only scan networks and addresses you own.
#>

#Requires -RunAsAdministrator

[CmdletBinding(DefaultParameterSetName = 'Internal')]
param(
    # Local network in CIDR notation. Auto-detected if omitted.
    [Parameter(ParameterSetName = 'Internal')]
    [ValidatePattern('^\d{1,3}(\.\d{1,3}){3}/\d{1,2}$')]
    [string]$Network,

    # Scan your public address from outside the home network.
    [Parameter(ParameterSetName = 'External', Mandatory)]
    [switch]$External,

    # Your public IPv4 address (as seen from home with: curl.exe -4 ifconfig.me).
    [Parameter(ParameterSetName = 'External', Mandatory)]
    [ValidatePattern('^\d{1,3}(\.\d{1,3}){3}$')]
    [string]$PublicIP,

    # Root folder for audit results.
    [string]$OutputRoot = (Join-Path $HOME 'nmap-audits'),

    # Number of audits to keep in each folder (minimum 2 for the comparison).
    [ValidateRange(2, 100)]
    [int]$Keep = 2
)

$ErrorActionPreference = 'Stop'

function Find-Tool {
    param([string]$Name)
    $cmd = Get-Command $Name -ErrorAction SilentlyContinue
    if ($cmd) { return $cmd.Source }
    # Not in PATH: look in the default install folders (ndiff ships as ndiff.bat)
    foreach ($dir in @("${env:ProgramFiles(x86)}\Nmap", "$env:ProgramFiles\Nmap")) {
        foreach ($ext in @('exe', 'bat')) {
            $candidate = Join-Path $dir "$Name.$ext"
            if (Test-Path $candidate) { return $candidate }
        }
    }
    throw "$Name not found. Install Nmap from https://nmap.org/download.html"
}

function Get-LocalNetwork {
    # Active interface with an IPv4 default gateway (the box)
    $cfg = Get-NetIPConfiguration |
        Where-Object { $null -ne $_.IPv4DefaultGateway -and $_.NetAdapter.Status -eq 'Up' } |
        Select-Object -First 1
    if (-not $cfg) { throw 'No active IPv4 network with a default gateway found.' }

    $addr = $cfg.IPv4Address | Select-Object -First 1
    $prefix = [int]$addr.PrefixLength
    $ipBytes = ([System.Net.IPAddress]::Parse($addr.IPAddress)).GetAddressBytes()

    # Apply the mask byte by byte to get the network address
    $netBytes = for ($i = 0; $i -lt 4; $i++) {
        $bits = [Math]::Min([Math]::Max($prefix - 8 * $i, 0), 8)
        $mask = (0xFF -shl (8 - $bits)) -band 0xFF
        $ipBytes[$i] -band $mask
    }
    return '{0}/{1}' -f ($netBytes -join '.'), $prefix
}

function Remove-OldAudits {
    param([string]$Folder, [int]$Count)
    # File names start with a sortable timestamp, so name order is time order
    Get-ChildItem -Path $Folder -Filter 'audit_*.xml' |
        Sort-Object Name -Descending |
        Select-Object -Skip $Count |
        ForEach-Object {
            Write-Host "Removing old audit $($_.BaseName)"
            Get-ChildItem -Path $Folder -Filter "$($_.BaseName).*" | Remove-Item
        }
}

# --- Tools -------------------------------------------------------------------
$nmap = Find-Tool 'nmap'
$ndiff = Find-Tool 'ndiff'

# --- Mode, target and folder -------------------------------------------------
if ($External) {
    $mode = 'external'
    $target = $PublicIP

    # Safety check: from home, the box answers "from inside" and the result is wrong
    $current = (& curl.exe -4 -s --max-time 10 ifconfig.me) 2>$null
    if ($current -eq $PublicIP) {
        throw "Your current public address is $PublicIP : you are still on the home network. Connect to your phone hotspot first."
    }
    if (-not $current) {
        Write-Warning 'Could not check your current public address. Make sure you are NOT on the home network.'
    }
    $nmapArgs = @('-Pn', '-sT', '-T4', '-p-')
}
else {
    $mode = 'internal'
    if (-not $Network) { $Network = Get-LocalNetwork }
    $prefix = [int]($Network.Split('/')[1])
    if ($prefix -lt 22) {
        throw "Network $Network is large (/$prefix). Pass -Network explicitly with a smaller range, for example 192.168.1.0/24."
    }
    $target = $Network
    $nmapArgs = @('-sV', '-T4')
}

$folder = Join-Path $OutputRoot $mode
if (-not (Test-Path $folder)) {
    New-Item -ItemType Directory -Path $folder -Force | Out-Null
    Write-Host "Created $folder"
}

$stamp = Get-Date -Format 'yyyy-MM-dd_HHmm'
$base = Join-Path $folder "audit_$stamp"

# --- Scan --------------------------------------------------------------------
Write-Host "Mode   : $mode"
Write-Host "Target : $target"
Write-Host "Output : $base.*"
Write-Host 'Scanning, this can take several minutes...'

& $nmap @nmapArgs -oA $base $target
if ($LASTEXITCODE -ne 0) { throw "nmap failed with exit code $LASTEXITCODE" }

# --- Keep only the last audits, then compare the two most recent --------------
Remove-OldAudits -Folder $folder -Count $Keep

$xmls = @(Get-ChildItem -Path $folder -Filter 'audit_*.xml' | Sort-Object Name -Descending)
if ($xmls.Count -lt 2) {
    Write-Host 'First audit in this folder: nothing to compare yet. Run it again next month.'
    return
}

$newXml = $xmls[0].FullName
$oldXml = $xmls[1].FullName
$diffFile = "$base.diff.txt"

Write-Host ''
Write-Host "Comparing $($xmls[1].Name) -> $($xmls[0].Name)"
Write-Host ''

# ndiff exit codes: 0 = no difference, 1 = differences, 2 = error
$diff = & $ndiff $oldXml $newXml
$code = $LASTEXITCODE
$diff | Set-Content -Path $diffFile -Encoding utf8

switch ($code) {
    0 { Write-Host 'No difference since the previous audit.' }
    1 { $diff | Write-Host; Write-Host ''; Write-Host "Differences saved to $diffFile" }
    default { throw "ndiff failed with exit code $code" }
}
