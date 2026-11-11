# ==============================================================================
# Kora Programming Language & Framework — Windows PowerShell Automated Installer
# Usage:
#   irm https://raw.githubusercontent.com/Hwai-technology/kora-framework/main/install.ps1 | iex
# ==============================================================================

$ErrorActionPreference = "Stop"

$KoraRepo = "Hwai-technology/kora-framework"
$InstallDir = "$env:USERPROFILE\.kora\bin"

Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "  ⚡ Kora Language & Framework — Windows Setup & Installer     " -ForegroundColor Cyan
Write-Host "================================================================" -ForegroundColor Cyan

# 1. Create Installation Directory
if (!(Test-Path -Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$KoraExe = Join-Path $InstallDir "kora.exe"

# 2. Check Architecture
$Arch = "x86_64"
if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") {
    $Arch = "arm64"
}

Write-Host "`n[1/4] Fetching Kora release for Windows ($Arch)..." -ForegroundColor Blue

$ZipName = "kora-windows-$Arch.zip"
$DownloadUrl = "https://github.com/$KoraRepo/releases/latest/download/$ZipName"
$FallbackExeUrl = "https://github.com/$KoraRepo/releases/latest/download/kora.exe"
$TempZip = Join-Path $env:TEMP $ZipName

$Downloaded = $false

try {
    Write-Host "  Downloading: $DownloadUrl" -ForegroundColor Gray
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $TempZip -UseBasicParsing
    Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
    Remove-Item $TempZip -Force -ErrorAction SilentlyContinue
    $Downloaded = $true
} catch {
    Write-Host "  Zip asset not found, attempting direct binary download..." -ForegroundColor Yellow
    try {
        Invoke-WebRequest -Uri $FallbackExeUrl -OutFile $KoraExe -UseBasicParsing
        $Downloaded = $true
    } catch {
        Write-Host "  Release asset download failed." -ForegroundColor Red
    }
}

# If precompiled binary is not available, check if cargo is present to compile from source
if (!$Downloaded -or !(Test-Path -Path $KoraExe)) {
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        Write-Host "  Precompiled Windows binary unavailable. Building from source via Cargo..." -ForegroundColor Yellow
        $TempSrc = Join-Path $env:TEMP "kora-src"
        if (Test-Path $TempSrc) { Remove-Item $TempSrc -Recurse -Force }
        git clone --depth 1 "https://github.com/$KoraRepo.git" $TempSrc
        Push-Location $TempSrc
        cargo build --release -p kora-cli
        Copy-Item "target\release\kora.exe" $KoraExe -Force
        Pop-Location
        Remove-Item $TempSrc -Recurse -Force -ErrorAction SilentlyContinue
    } else {
        Write-Error "Failed to install Kora. Please install Rust (https://rustup.rs) or download a release from https://github.com/$KoraRepo/releases"
        exit 1
    }
}

Write-Host "✓ Kora executable installed at: $KoraExe" -ForegroundColor Green

# 3. Configure PATH in User Environment
Write-Host "`n[2/4] Configuring Environment PATH..." -ForegroundColor Blue
$UserPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
if ($UserPath -notlike "*$InstallDir*") {
    $NewPath = "$InstallDir;$UserPath"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, [EnvironmentVariableTarget]::User)
    $env:Path = "$InstallDir;$env:Path"
    Write-Host "✓ Added $InstallDir to User PATH." -ForegroundColor Green
} else {
    Write-Host "✓ $InstallDir is already in User PATH." -ForegroundColor Green
}

# 4. Register File Association for .kora and .kr in Windows Registry
Write-Host "`n[3/4] Registering .kora and .kr File Associations..." -ForegroundColor Blue
try {
    cmd.exe /c "assoc .kora=KoraSourceFile >nul 2>&1"
    cmd.exe /c "assoc .kr=KoraSourceFile >nul 2>&1"
    cmd.exe /c "ftype KoraSourceFile=`"$KoraExe`" `"%1`" %* >nul 2>&1"
    Write-Host "✓ Registered .kora & .kr file extensions system-wide." -ForegroundColor Green
} catch {
    Write-Host "ℹ️  Could not set file associations (non-elevated). Skipping." -ForegroundColor Gray
}

# 5. Sync VS Code Extension if installed
Write-Host "`n[4/4] Synchronizing VS Code & Editor Tooling..." -ForegroundColor Blue
$VsCodeExtDir = "$env:USERPROFILE\.vscode\extensions\kora-vscode"
try {
    if (Test-Path "$env:USERPROFILE\.vscode") {
        New-Item -ItemType Directory -Path "$VsCodeExtDir\client" -Force | Out-Null
        New-Item -ItemType Directory -Path "$VsCodeExtDir\syntaxes" -Force | Out-Null
        New-Item -ItemType Directory -Path "$VsCodeExtDir\snippets" -Force | Out-Null

        $BaseRaw = "https://raw.githubusercontent.com/$KoraRepo/main/integrations/vscode"
        Invoke-WebRequest -Uri "$BaseRaw/package.json" -OutFile "$VsCodeExtDir\package.json" -UseBasicParsing
        Invoke-WebRequest -Uri "$BaseRaw/language-configuration.json" -OutFile "$VsCodeExtDir\language-configuration.json" -UseBasicParsing
        Invoke-WebRequest -Uri "$BaseRaw/client/extension.js" -OutFile "$VsCodeExtDir\client\extension.js" -UseBasicParsing
        Invoke-WebRequest -Uri "$BaseRaw/syntaxes/kora.tmLanguage.json" -OutFile "$VsCodeExtDir\syntaxes\kora.tmLanguage.json" -UseBasicParsing
        Invoke-WebRequest -Uri "$BaseRaw/snippets/kora.snippets.json" -OutFile "$VsCodeExtDir\snippets\kora.snippets.json" -UseBasicParsing
        Write-Host "✓ VS Code extension installed at $VsCodeExtDir." -ForegroundColor Green
    }
} catch {
    Write-Host "ℹ️  Skipped editor extension auto-sync." -ForegroundColor Gray
}

Write-Host "`n================================================================" -ForegroundColor Cyan
Write-Host "  🎉 Kora Environment Successfully Installed on Windows!       " -ForegroundColor Green
Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "Run the following command to verify:" -ForegroundColor Yellow
Write-Host "  kora --version" -ForegroundColor Cyan
Write-Host "  kora doctor" -ForegroundColor Cyan
Write-Host "`nTo start a new project:" -ForegroundColor Yellow
Write-Host "  kora init my-api" -ForegroundColor Cyan

