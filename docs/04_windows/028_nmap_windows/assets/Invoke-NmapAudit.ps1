<#
.SYNOPSIS
    Monthly home network audit with Nmap, then compare with the previous audit using ndiff.

.DESCRIPTION
    Internal mode (default): scans the local IPv4 network (auto-detected) with service detection.

    External mode (-External): scans from OUTSIDE your home network (phone hotspot, for example).
      -PublicIP   : your public IPv4 address, all 65535 TCP ports.
      -PublicIPv6 : one or more IPv6 addresses (the box, then devices behind it),
                    1000 most common TCP ports. In IPv6 there is no NAT: each device
                    is reachable directly if the box IPv6 firewall lets it through.
      Use one or both.

    Results go to $HOME\nmap-audits\internal, \external (IPv4) or \external6 (IPv6).
    Folders are created if needed. Only the last -Keep audits are kept (default 2).
    At the end, ndiff compares the two most recent audits of each folder.

.EXAMPLE
    .\Invoke-NmapAudit.ps1
    Internal audit of the auto-detected local network.

.EXAMPLE
    .\Invoke-NmapAudit.ps1 -Network 192.168.1.0/24
    Internal audit of a given network.

.EXAMPLE
    .\Invoke-NmapAudit.ps1 -External -PublicIP 83.x.x.x
    External IPv4 audit. Connect to your phone hotspot first (phone Wi-Fi off).

.EXAMPLE
    .\Invoke-NmapAudit.ps1 -External -PublicIP 83.x.x.x -PublicIPv6 2a01:xxxx::1, 2a01:xxxx::abcd
    External IPv4 and IPv6 audit.

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

    # Scan your public addresses from outside the home network.
    [Parameter(ParameterSetName = 'External', Mandatory)]
    [switch]$External,

    # Your public IPv4 address (as seen from home with: curl.exe -4 ifconfig.me).
    [Parameter(ParameterSetName = 'External')]
    [ValidatePattern('^\d{1,3}(\.\d{1,3}){3}$')]
    [string]$PublicIP,

    # Global IPv6 addresses to scan: the box, then stable addresses of devices.
    [Parameter(ParameterSetName = 'External')]
    [string[]]$PublicIPv6,

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

function Test-GlobalIPv6 {
    param([string]$Address)
    $ip = $null
    if (-not [System.Net.IPAddress]::TryParse($Address, [ref]$ip)) { return $false }
    if ($ip.AddressFamily -ne [System.Net.Sockets.AddressFamily]::InterNetworkV6) { return $false }
    # Global unicast addresses start with 2 or 3 (2000::/3)
    $first = $ip.GetAddressBytes()[0]
    return ($first -ge 0x20 -and $first -le 0x3F)
}

function Get-IPv6Prefix48 {
    # First 48 bits: enough to tell the home line from the mobile network
    param([string]$Address)
    $bytes = ([System.Net.IPAddress]::Parse($Address)).GetAddressBytes()
    return ($bytes[0..5] | ForEach-Object { '{0:x2}' -f $_ }) -join ''
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

function Invoke-Audit {
    param([string]$Name, [string[]]$NmapArgs, [string[]]$Targets)

    $folder = Join-Path $OutputRoot $Name
    if (-not (Test-Path $folder)) {
        New-Item -ItemType Directory -Path $folder -Force | Out-Null
        Write-Host "Created $folder"
    }

    $stamp = Get-Date -Format 'yyyy-MM-dd_HHmm'
    $base = Join-Path $folder "audit_$stamp"

    Write-Host ''
    Write-Host "=== Audit $Name ==="
    Write-Host "Target : $($Targets -join ', ')"
    Write-Host "Output : $base.*"
    Write-Host 'Scanning, this can take a while. Press Space for progress.'

    & $nmap @NmapArgs -oA $base @Targets
    if ($LASTEXITCODE -ne 0) { throw "nmap failed with exit code $LASTEXITCODE" }

    # Keep only the last audits, then compare the two most recent
    Remove-OldAudits -Folder $folder -Count $Keep

    $xmls = @(Get-ChildItem -Path $folder -Filter 'audit_*.xml' | Sort-Object Name -Descending)
    if ($xmls.Count -lt 2) {
        Write-Host 'First audit in this folder: nothing to compare yet. Run it again next month.'
        return
    }

    $diffFile = "$base.diff.txt"
    Write-Host ''
    Write-Host "Comparing $($xmls[1].Name) -> $($xmls[0].Name)"
    Write-Host ''

    # ndiff exit codes: 0 = no difference, 1 = differences, 2 = error
    $diff = & $ndiff $xmls[1].FullName $xmls[0].FullName
    $code = $LASTEXITCODE
    $diff | Set-Content -Path $diffFile -Encoding utf8

    switch ($code) {
        0 { Write-Host 'No difference since the previous audit.' }
        1 { $diff | Write-Host; Write-Host ''; Write-Host "Differences saved to $diffFile" }
        default { throw "ndiff failed with exit code $code" }
    }
}

# --- Tools -------------------------------------------------------------------
$nmap = Find-Tool 'nmap'
$ndiff = Find-Tool 'ndiff'

# --- Internal ----------------------------------------------------------------
if (-not $External) {
    if (-not $Network) { $Network = Get-LocalNetwork }
    $prefix = [int]($Network.Split('/')[1])
    if ($prefix -lt 22) {
        throw "Network $Network is large (/$prefix). Pass -Network explicitly with a smaller range, for example 192.168.1.0/24."
    }
    Invoke-Audit -Name 'internal' -NmapArgs @('-sV', '-T4') -Targets @($Network)
    return
}

# --- External: check everything before scanning anything ---------------------
if (-not $PublicIP -and -not $PublicIPv6) {
    throw 'External mode needs -PublicIP, -PublicIPv6, or both.'
}

$homeHint = 'Connect the PC to your phone hotspot, and turn off Wi-Fi on the phone (mobile data only).'

if ($PublicIP) {
    # From home, the box answers "from inside" and the result is wrong
    $current4 = (& curl.exe -4 -s --max-time 10 ifconfig.me) 2>$null
    if ($current4 -eq $PublicIP) {
        throw "Your current public IPv4 is $PublicIP : you are still on the home network. $homeHint"
    }
    if (-not $current4) {
        Write-Warning 'Could not check your current public IPv4. Make sure you are NOT on the home network.'
    }
}

if ($PublicIPv6) {
    foreach ($addr in $PublicIPv6) {
        if (-not (Test-GlobalIPv6 $addr)) {
            throw "'$addr' is not a global IPv6 address (it should start with 2 or 3, not fe80)."
        }
    }

    # Scanning IPv6 needs IPv6 on the current connection
    $current6 = (& curl.exe -6 -s --max-time 10 ifconfig.me) 2>$null
    if (-not $current6 -or -not (Test-GlobalIPv6 $current6)) {
        throw 'This connection has no IPv6: the IPv6 targets cannot be scanned from here. Check that your mobile connection provides IPv6, or run without -PublicIPv6.'
    }

    # Same /48 as a target: the traffic still goes out through the home box
    $current48 = Get-IPv6Prefix48 $current6
    foreach ($addr in $PublicIPv6) {
        if ((Get-IPv6Prefix48 $addr) -eq $current48) {
            throw "Your current IPv6 ($current6) is on the same network as $addr : you are still on the home network. $homeHint"
        }
    }
}

# --- External: scans ---------------------------------------------------------
# -sT (full connection) is slower than a SYN scan but is not fooled by the box anti-scan
if ($PublicIP) {
    Invoke-Audit -Name 'external' -NmapArgs @('-Pn', '-sT', '-T4', '-p-') -Targets @($PublicIP)
}
if ($PublicIPv6) {
    Invoke-Audit -Name 'external6' -NmapArgs @('-6', '-Pn', '-sT', '-T4') -Targets $PublicIPv6
}
