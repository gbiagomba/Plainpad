@echo off
REM install.bat
REM Simple Windows installer for plainpad (satisfies RULE 3)

setlocal enabledelayedexpansion

set APP_NAME=plainpad
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

set BINARY=%APP_NAME%-windows-%ARCH%.exe
set URL=https://github.com/%REPO%/releases/latest/download/%BINARY%

echo Downloading %BINARY%...
powershell -Command "Invoke-WebRequest -Uri '%URL%' -OutFile '%TEMP%\%APP_NAME%.exe' -UseBasicParsing"

if not exist "%TEMP%\%APP_NAME%.exe" (
    echo ? Failed to download %BINARY%
    pause
    exit /b 1
)

if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"
move /y "%TEMP%\%APP_NAME%.exe" "%INSTALL_DIR%\%APP_NAME%.exe"

echo ? Binary installed to: %INSTALL_DIR%\%APP_NAME%.exe

echo %PATH% | find /i "%INSTALL_DIR%" >nul
if %errorlevel% neq 0 (
    echo Adding to system PATH...
    setx /M PATH "%PATH%;%INSTALL_DIR%"
    echo ? Added to PATH. Please restart your terminal.
)

echo.
echo ? %APP_NAME% installed successfully!
echo Launch %APP_NAME% from Start Menu or run '%APP_NAME%' in a terminal.
pause
