@echo off
rem ==============================================================================
rem Kora Framework — Windows Batch Setup Script
rem Launches the PowerShell automated installer
rem ==============================================================================

echo [Kora Setup] Launching Kora installer for Windows...
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0..\install.ps1"
if %ERRORLEVEL% NEQ 0 (
    echo [Kora Setup] Installer encountered an error.
    pause
    exit /b %ERRORLEVEL%
)
pause
