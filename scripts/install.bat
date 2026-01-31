@echo off
REM install.bat
REM Simple Windows installer for program_name (satisfies RULE 3)

setlocal enabledelayedexpansion

set APP_NAME=program_name
set REPO=gbiagomba/%APP_NAME%
set INSTALL_DIR=%ProgramFiles%\%APP_NAME%

echo ?? Installing %APP_NAME%...
echo.

REM Check for admin privileges
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo ? This script requires Administrator privileges.
    echo Please run as Administrator and try again.
    pause
    exit /b 1
)

REM Detect architecture
if "%PROCESSOR_ARCHITECTURE%"=="AMD64" (
    set ARCH=x64
) else if "%PROCESSOR_ARCHITECTURE%"=="ARM64" (
    set ARCH=aarch64
) else (
    echo ? Unsupported architecture: %PROCESSOR_ARCHITECTURE%
    pause
    exit /b 1
)

REM Check if cargo exists
where cargo >nul 2>&1
if %errorlevel% equ 0 (
    echo ?? Installing via Cargo...
    cargo install --git https://github.com/%REPO%
    echo.
    echo ? %APP_NAME% installed successfully!
    echo Run: %APP_NAME% --help
    pause
    exit /b 0
)

echo ??  Cargo not found, installing from GitHub Release...
echo.

REM Download binary from GitHub Release
set BINARY=%APP_NAME%-windows-%ARCH%.exe
echo Downloading %BINARY%...

REM Use PowerShell to download (works on Windows 7+)
powershell -Command "& {$tag = (Invoke-RestMethod 'https://api.github.com/repos/%REPO%/releases/latest').tag_name; Invoke-WebRequest -Uri \"https://github.com/%REPO%/releases/download/$tag/%BINARY%\" -OutFile \"%TEMP%\%APP_NAME%.exe\" -UseBasicParsing}"

if not exist "%TEMP%\%APP_NAME%.exe" (
    echo ? Failed to download binary
    pause
    exit /b 1
)

REM Create install directory
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Move binary
move /y "%TEMP%\%APP_NAME%.exe" "%INSTALL_DIR%\%APP_NAME%.exe"

echo ? Binary installed to: %INSTALL_DIR%\%APP_NAME%.exe

REM Add to PATH
echo %PATH% | find /i "%INSTALL_DIR%" >nul
if %errorlevel% neq 0 (
    echo Adding to system PATH...
    setx /M PATH "%PATH%;%INSTALL_DIR%"
    echo ? Added to PATH. Please restart your terminal.
)

echo.
echo ? %APP_NAME% installed successfully!
echo Run: %APP_NAME% --help
pause