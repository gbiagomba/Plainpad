# install.ps1
# Windows installer for plainpad (satisfies RULE 3)

$ErrorActionPreference = "Stop"

$AppName = "plainpad"
$Repo = "gbiagomba/$AppName"
$InstallDir = "$env:ProgramFiles\$AppName"

Write-Host "?? Installing $AppName..." -ForegroundColor Cyan

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

function Ensure-Admin {
    $isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    if (-not $isAdmin) {
        Write-Host "??  This script requires Administrator privileges." -ForegroundColor Yellow
        Write-Host "Please run PowerShell as Administrator and try again." -ForegroundColor Yellow
        exit 1
    }
}

function Install-FromRelease {
    $arch = Get-SystemArchitecture
    $binary = "$AppName-windows-$arch.exe"
    $url = "https://github.com/$Repo/releases/latest/download/$binary"

    Write-Host "Downloading: $url" -ForegroundColor Cyan
    $tempFile = "$env:TEMP\$AppName.exe"

    try {
        Invoke-WebRequest -Uri $url -OutFile $tempFile -UseBasicParsing
    }
    catch {
        Write-Host "? Failed to download $binary" -ForegroundColor Red
        exit 1
    }

    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    $finalPath = "$InstallDir\$AppName.exe"
    Move-Item -Path $tempFile -Destination $finalPath -Force

    Write-Host "? Binary installed to: $finalPath" -ForegroundColor Green

    $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    if ($currentPath -notlike "*$InstallDir*") {
        Write-Host "Adding $InstallDir to system PATH..." -ForegroundColor Yellow
        [Environment]::SetEnvironmentVariable(
            "Path",
            "$currentPath;$InstallDir",
            "Machine"
        )
        Write-Host "? Added to PATH. Please restart your terminal." -ForegroundColor Green
    }
}

Ensure-Admin
Install-FromRelease

Write-Host "" 
Write-Host "? $AppName installed successfully!" -ForegroundColor Green
Write-Host "Launch $AppName from Start Menu or run '$AppName' in a terminal." -ForegroundColor Cyan
