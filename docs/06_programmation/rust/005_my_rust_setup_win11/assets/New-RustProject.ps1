# How to:
# Copy the script in a folder then add the latter to the PATH
#   $rustDir = "C:\Users\phili\OneDrive\Documents\Programmation\rust"
#   $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
#   [Environment]::SetEnvironmentVariable("Path", "$currentPath;$rustDir", "User")

# Notes
# .\New-RustProject.ps1 my_prj 011_my_prj
# .\New-RustProject.ps1 -PRJ_NAME my_project -Author "John Doe" -LicenseType Apache -GitInit
# Remove-Item -Path "011_my_prj" -Recurse -Force

param (
    [Parameter(Mandatory = $true, HelpMessage = "Project name (snake_case)")]
    [string]$PRJ_NAME,

    [Parameter(Mandatory = $false, HelpMessage = "Directory name (if different from project name)")]
    [string]$DIR_NAME,

    [Parameter(Mandatory = $false, HelpMessage = "Author name for LICENSE")]
    [string]$Author = "40tude",

    [Parameter(Mandatory = $false, HelpMessage = "License type: MIT, Apache, or None")]
    [ValidateSet("MIT", "Apache", "None")]
    [string]$LicenseType = "MIT",

    [Parameter(Mandatory = $false, HelpMessage = "Initialize git repository")]
    [switch]$GitInit
)

# Stop execution on any error
$ErrorActionPreference = "Stop"

# ============================================================================
# VALIDATION
# ============================================================================

Write-Host "Validating inputs..." -ForegroundColor Cyan

# Validate PRJ_NAME follows Rust naming conventions (snake_case)
if ($PRJ_NAME -notmatch '^[a-z][a-z0-9_]*$') {
    throw "Invalid project name '$PRJ_NAME'. Rust project names must be snake_case (lowercase letters, numbers, underscores only, must start with letter)."
}

# Check if cargo is installed
Write-Host "Checking cargo installation..." -ForegroundColor Cyan
try {
    $null = Get-Command cargo -ErrorAction Stop
}
catch {
    throw "Cargo not found. Please install Rust from https://rustup.rs/"
}

# ============================================================================
# HELPER FUNCTIONS
# ============================================================================

function New-FileIfNotExists {
    param (
        [string]$Path,
        [string]$Content = ""
    )

    if (-not (Test-Path $Path)) {
        if ($Content) {
            Set-Content -Path $Path -Value $Content -Encoding UTF8
        }
        else {
            New-Item -ItemType File -Path $Path | Out-Null
        }
        Write-Host "  Created: $Path" -ForegroundColor Green
    }
    else {
        Write-Host "  Exists: $Path" -ForegroundColor Yellow
    }
}

function New-DirectoryIfNotExists {
    param ([string]$Path)

    if (-not (Test-Path $Path)) {
        New-Item -ItemType Directory -Path $Path | Out-Null
        Write-Host "  Created: $Path" -ForegroundColor Green
    }
    else {
        Write-Host "  Exists: $Path" -ForegroundColor Yellow
    }
}

# ============================================================================
# PROJECT CREATION
# ============================================================================

$ProjectDir = if ($DIR_NAME) { $DIR_NAME } else { $PRJ_NAME }

Write-Host "`nCreating Rust project '$PRJ_NAME'..." -ForegroundColor Cyan

# Create Rust project
if ($DIR_NAME) {
    cargo new $ProjectDir --name $PRJ_NAME
}
else {
    cargo new $PRJ_NAME
}

# Verify project directory exists
if (-not (Test-Path $ProjectDir)) {
    throw "Project directory '$ProjectDir' was not created."
}

Push-Location $ProjectDir
Write-Host "Project created successfully`n" -ForegroundColor Green

# ============================================================================
# CARGO CONFIG (redirect target/ outside OneDrive)
# ============================================================================

Write-Host "Creating Cargo config..." -ForegroundColor Cyan

# Get the full path of the project directory
$FullProjectPath = (Get-Location).Path

# Build the OneDrive base path dynamically
$OneDriveBase = Join-Path $env:USERPROFILE "OneDrive"

# Check if the project is inside OneDrive
if ($FullProjectPath.StartsWith($OneDriveBase, [System.StringComparison]::OrdinalIgnoreCase)) {
    # Extract the relative path after OneDrive\
    $RelativePath = $FullProjectPath.Substring($OneDriveBase.Length).TrimStart('\')

    # Build the target-dir path with forward slashes for TOML compatibility
    $RustBuildsBase = "$env:USERPROFILE/rust_builds" -replace '\\', '/'
    $RelativePathForward = $RelativePath -replace '\\', '/'
    $TargetDir = "$RustBuildsBase/$RelativePathForward"

    # Create .cargo directory and config.toml
    New-DirectoryIfNotExists -Path ".cargo"

    $CargoConfigContent = @"
[build]
target-dir = "$TargetDir"
"@

    New-FileIfNotExists -Path ".cargo\config.toml" -Content $CargoConfigContent
    Write-Host "  Target directory redirected to: $TargetDir" -ForegroundColor Green
}
else {
    Write-Host "  Project is not in OneDrive, skipping target redirection" -ForegroundColor Yellow
}

# ============================================================================
# DIRECTORY STRUCTURE
# ============================================================================

Write-Host "`nCreating directory structure..." -ForegroundColor Cyan
New-DirectoryIfNotExists -Path "docs"

# ============================================================================
# DOCUMENTATION FILES
# ============================================================================

Write-Host "`nCreating documentation files..." -ForegroundColor Cyan

# README.md template
$ReadmeContent = @"
# $PRJ_NAME

## Description

[Add project description here]

## Installation

``````powershell
cargo build --release
``````

## Usage

``````powershell
cargo run
``````

## Testing

``````powershell
cargo test
``````

## License

$LicenseType License - see [LICENSE](LICENSE) for details


## Contributing
This project is developed for personal and educational purposes. Feel free to explore and use it to enhance your own learning.

Given the nature of the project, external contributions are not actively sought nor encouraged. However, constructive feedback aimed at improving the project (in terms of speed, accuracy, comprehensiveness, etc.) is welcome. Please note that this project is being created as a hobby and is unlikely to be maintained once my initial goal has been achieved.
"@

New-FileIfNotExists -Path "README.md" -Content $ReadmeContent

# docs/notes.md
$NotesContent = @"
# Development Notes

## TODO

- [ ] Initial setup
- [ ] First implementation

## Ideas

[Add your ideas here]
"@

New-FileIfNotExists -Path "docs\notes.md" -Content $NotesContent

# ============================================================================
# LICENSE
# ============================================================================

Write-Host "`nCreating LICENSE..." -ForegroundColor Cyan

if ($LicenseType -eq "MIT") {
    $LicenseContent = @"
MIT License

Copyright (c) 2025 $Author

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"@
    New-FileIfNotExists -Path "LICENSE.md" -Content $LicenseContent

}
elseif ($LicenseType -eq "Apache") {
    $LicenseContent = @"
Apache License
Version 2.0, January 2004
http://www.apache.org/licenses/

Copyright 2025 $Author

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
"@
    New-FileIfNotExists -Path "LICENSE.md" -Content $LicenseContent

}
else {
    Write-Host "  No license file created (LicenseType = None)" -ForegroundColor Yellow
}

# ============================================================================
# .ENV FILE
# ============================================================================

Write-Host "`nCreating environment file..." -ForegroundColor Cyan
$EnvContent = @"
# Environment variables for $PRJ_NAME
# Add your sensitive configuration here
"@

New-FileIfNotExists -Path ".env" -Content $EnvContent

# ============================================================================
# .GITIGNORE
# ============================================================================

Write-Host "`nUpdating .gitignore..." -ForegroundColor Cyan

$GitIgnoreFile = ".gitignore"
$GitIgnoreLines = @()

if (Test-Path $GitIgnoreFile) {
    # Force array result even for single line
    $GitIgnoreLines = @(Get-Content $GitIgnoreFile)
}

# Normalize '/target' to 'target/'
$GitIgnoreLines = @($GitIgnoreLines | ForEach-Object {
        if ($_ -eq "/target") { "target/" } else { $_ }
    })

# Required entries
$RequiredEntries = @(
    "target/",
    "temp/",
    ".env"
)

# Add missing entries
foreach ($Entry in $RequiredEntries) {
    if ($GitIgnoreLines -notcontains $Entry) {
        $GitIgnoreLines += $Entry
    }
}

# Write each line separately with newline
$GitIgnoreLines -join "`n" | Set-Content -Path $GitIgnoreFile -Encoding UTF8 -NoNewline
Add-Content -Path $GitIgnoreFile -Value "" -Encoding UTF8

Write-Host "  Updated .gitignore" -ForegroundColor Green

# ============================================================================
# GIT INITIALIZATION
# ============================================================================

if ($GitInit) {
    Write-Host "`nInitializing git repository..." -ForegroundColor Cyan

    # Check if already a git repo
    if (Test-Path ".git") {
        Write-Host "  Git repository already initialized" -ForegroundColor Yellow
    }
    else {
        git init
        Write-Host "  Git repository initialized" -ForegroundColor Green

        # Optional: Create initial commit
        git add .
        git commit -m "Initial commit: $PRJ_NAME project setup"
        Write-Host "  Initial commit created" -ForegroundColor Green
    }
}

# Return to initial directory
Pop-Location

# ============================================================================
# SUMMARY
# ============================================================================

Write-Host "`n============================================" -ForegroundColor Cyan
Write-Host "Project '$PRJ_NAME' created successfully!" -ForegroundColor Green
Write-Host "============================================" -ForegroundColor Cyan
Write-Host "Location: $(Resolve-Path $ProjectDir)" -ForegroundColor White
Write-Host "Author: $Author" -ForegroundColor White
Write-Host "License: $LicenseType" -ForegroundColor White
if ($GitInit) {
    Write-Host "Git: Initialized with initial commit" -ForegroundColor White
}
Write-Host "`nNext steps:" -ForegroundColor Cyan
Write-Host "  1. Edit README.md with project description" -ForegroundColor White
Write-Host "  2. Start coding in src/main.rs" -ForegroundColor White
Write-Host "  3. Run 'cargo run' to test" -ForegroundColor White
Write-Host "`nHappy coding! 🦀" -ForegroundColor Cyan
