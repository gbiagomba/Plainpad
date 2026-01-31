# install.ps1
# Windows installer for program_name (satisfies RULE 3)

param(
    [string]$InstallMethod = "auto"
)

$ErrorActionPreference = "Stop"

$APP_NAME = "program_name"
$REPO = "gbiagomba/$APP_NAME"
$INSTALL_DIR = "$env:ProgramFiles\$APP_NAME"

Write-Host "?? Installing $APP_NAME..." -ForegroundColor Cyan

# Detect Architecture
function Get-SystemArchitecture {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        "AMD64" { return "x64" }
        "ARM64" { return "aarch64" }
        default {
            Write-Host "? Unsupported architecture: $arch" -ForegroundColor Red
            exit 1
        }
    }
}

# Check if command exists
function Test-CommandExists {
    param($Command)
    $null -ne (Get-Command $Command -ErrorAction SilentlyContinue)
}

# Install dependencies via package managers
function Install-Dependencies {
    Write-Host "?? Checking dependencies..." -ForegroundColor Yellow
    
    # Try winget first (Windows 10+)
    if (Test-CommandExists winget) {
        Write-Host "Detected: winget" -ForegroundColor Green
        # Typically curl is built-in on Windows 10+
    }
    # Try Chocolatey
    elseif (Test-CommandExists choco) {
        Write-Host "Detected: Chocolatey" -ForegroundColor Green
        choco install curl -y
    }
    # Try Scoop
    elseif (Test-CommandExists scoop) {
        Write-Host "Detected: Scoop" -ForegroundColor Green
        scoop install curl
    }
    else {
        Write-Host "??  No package manager detected. Assuming dependencies are installed." -ForegroundColor Yellow
    }
}

# Install via Cargo
function Install-WithCargo {
    if (Test-CommandExists cargo) {
        Write-Host "?? Installing via Cargo..." -ForegroundColor Green
        cargo install --git "https://github.com/$REPO"
        return $true
    }
    return $false
}

# Install from GitHub Release
function Install-FromRelease {
    Write-Host "?? Downloading from GitHub Releases..." -ForegroundColor Green
    
    $arch = Get-SystemArchitecture
    $BINARY = "$APP_NAME-windows-$arch.exe"
    
    # Get latest release
    try {
        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$REPO/releases/latest"
        $LATEST = $response.tag_name
        Write-Host "Latest version: $LATEST" -ForegroundColor Cyan
    }
    catch {
        Write-Host "? Failed to fetch latest release version" -ForegroundColor Red
        exit 1
    }
    
    $URL = "https://github.com/$REPO/releases/download/$LATEST/$BINARY"
    Write-Host "Downloading: $URL" -ForegroundColor Cyan
    
    # Download binary
    $tempFile = "$env:TEMP\$APP_NAME.exe"
    try {
        Invoke-WebRequest -Uri $URL -OutFile $tempFile -UseBasicParsing
    }
    catch {
        Write-Host "? Failed to download binary" -ForegroundColor Red
        exit 1
    }
    
    # Create install directory
    if (-not (Test-Path $INSTALL_DIR)) {
        New-Item -ItemType Directory -Path $INSTALL_DIR -Force | Out-Null
    }
    
    # Move to install location
    $finalPath = "$INSTALL_DIR\$APP_NAME.exe"
    Move-Item -Path $tempFile -Destination $finalPath -Force
    
    Write-Host "? Binary installed to: $finalPath" -ForegroundColor Green
    
    # Add to PATH if not already there
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    if ($currentPath -notlike "*$INSTALL_DIR*") {
        Write-Host "Adding $INSTALL_DIR to system PATH..." -ForegroundColor Yellow
        [Environment]::SetEnvironmentVariable(
            "Path",
            "$currentPath;$INSTALL_DIR",
            "Machine"
        )
        Write-Host "? Added to PATH. Please restart your terminal." -ForegroundColor Green
    }
}

# Main installation logic
function Main {
    # Check for admin privileges
    $isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    
    if (-not $isAdmin) {
        Write-Host "??  This script requires Administrator privileges." -ForegroundColor Yellow
        Write-Host "Please run PowerShell as Administrator and try again." -ForegroundColor Yellow
        exit 1
    }
    
    Install-Dependencies
    
    Write-Host ""
    Write-Host "Choose installation method:" -ForegroundColor Cyan
    Write-Host "1) Cargo (recommended for Rust developers)"
    Write-Host "2) GitHub Release binary (recommended for most users)"
    Write-Host "3) Auto-detect (try Cargo first, fallback to binary)"
    Write-Host ""
    
    if ($InstallMethod -eq "auto") {
        $choice = Read-Host "Enter choice [1-3] (default: 3)"
        if ([string]::IsNullOrWhiteSpace($choice)) { $choice = "3" }
    }
    else {
        $choice = $InstallMethod
    }
    
    switch ($choice) {
        "1" {
            if (-not (Install-WithCargo)) {
                Write-Host "? Cargo not found. Please install Rust first: https://rustup.rs/" -ForegroundColor Red
                exit 1
            }
        }
        "2" {
            Install-FromRelease
        }
        "3" {
            if (-not (Install-WithCargo)) {
                Write-Host "??  Cargo not found, falling back to binary installation" -ForegroundColor Yellow
                Install-FromRelease
            }
        }
        default {
            Write-Host "? Invalid choice" -ForegroundColor Red
            exit 1
        }
    }
    
    Write-Host ""
    Write-Host "? $APP_NAME installed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Run: $APP_NAME --help" -ForegroundColor Cyan
}

Main