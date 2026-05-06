#Requires -Version 7.0
# Run from repo root: ./tools/generate_last_update.ps1

$docsPath = Join-Path $PSScriptRoot ".." "docs"
$outputDir = $PSScriptRoot
$outputFile = Join-Path $outputDir ("last_update_" + (Get-Date -Format "yyyyMMdd") + ".md")

$results = [System.Collections.Generic.List[PSCustomObject]]::new()

Get-ChildItem -Path $docsPath -Recurse -Filter "*.md" | ForEach-Object {
    $file = $_
    $inFrontmatter = $false
    $fenceCount = 0
    $lastModified = $null

    foreach ($line in (Get-Content $file.FullName)) {
        if ($line -match '^---') {
            $fenceCount++
            $inFrontmatter = $fenceCount -eq 1
            if ($fenceCount -eq 2) { break }
            continue
        }
        if ($inFrontmatter -and $line -match '^\s*last_modified_date\s*:\s*(\d{4}-\d{2}-\d{2}\s+\d{2}:\d{2}:\d{2})') {
            $lastModified = $Matches[1]
            break
        }
    }

    if ($null -ne $lastModified) {
        $relPath = $file.FullName | Resolve-Path -Relative
        # Normalize to docs\... (strip leading .\)
        $relPath = $relPath -replace '^\.[\\/]', ''
        $results.Add([PSCustomObject]@{
                Path         = $relPath
                LastModified = $lastModified
            })
    }
}

$sorted = $results | Sort-Object { [datetime]$_.LastModified } -Descending

$lines = $sorted | ForEach-Object { "$($_.Path) | $($_.LastModified)" }

Set-Content -Path $outputFile -Value $lines -Encoding UTF8

Write-Host "Generated $outputFile ($($sorted.Count) articles)"
