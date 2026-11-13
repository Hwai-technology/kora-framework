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

# Ensure cargo is accessible if installed via rustup
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    if (Test-Path "$env:USERPROFILE\.cargo\bin\cargo.exe") {
        $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
    }
}

# If precompiled binary is not available, check if cargo is present to compile from source
if (!$Downloaded -or !(Test-Path -Path $KoraExe)) {
    # Check if run from a local repository clone
    $LocalCargoToml = Join-Path $PSScriptRoot "Cargo.toml"
    $LocalCliToml = Join-Path $PSScriptRoot "crates\kora-cli\Cargo.toml"
    $BuildDir = $null

    if ((Test-Path $LocalCargoToml) -and (Test-Path $LocalCliToml)) {
        $BuildDir = $PSScriptRoot
    }

    if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Host "  ⚠️ Cargo is not installed. Attempting to install Rust via winget / rustup..." -ForegroundColor Yellow
        try {
            if (Get-Command winget -ErrorAction SilentlyContinue) {
                Write-Host "  Installing Rustup via winget..." -ForegroundColor Cyan
                winget install --id Rustlang.Rustup --silent --accept-source-agreements --accept-package-agreements
            } else {
                $RustupInit = Join-Path $env:TEMP "rustup-init.exe"
                Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $RustupInit -UseBasicParsing
                Start-Process -FilePath $RustupInit -ArgumentList "-y" -Wait -NoNewWindow
                Remove-Item $RustupInit -Force -ErrorAction SilentlyContinue
            }
            if (Test-Path "$env:USERPROFILE\.cargo\bin\cargo.exe") {
                $env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"
            }
        } catch {
            Write-Host "  Automatic Rust installation failed." -ForegroundColor Yellow
        }
    }

    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        Write-Host "  Building from source via Cargo..." -ForegroundColor Yellow
        if ($BuildDir) {
            Push-Location $BuildDir
            cargo build --release -p kora-cli
            Copy-Item "target\release\kora.exe" $KoraExe -Force
            Pop-Location
        } else {
            $TempSrc = Join-Path $env:TEMP "kora-src"
            if (Test-Path $TempSrc) { Remove-Item $TempSrc -Recurse -Force }
            git clone --depth 1 "https://github.com/$KoraRepo.git" $TempSrc
            Push-Location $TempSrc
            cargo build --release -p kora-cli
            Copy-Item "target\release\kora.exe" $KoraExe -Force
            Pop-Location
            Remove-Item $TempSrc -Recurse -Force -ErrorAction SilentlyContinue
        }
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

# 5. Install & Synchronize VS Code, Cursor & Editor Tooling
Write-Host "`n[4/4] Installing & Synchronizing VS Code, Cursor & Editor Extensions..." -ForegroundColor Blue

$EditorClis = @("code", "code-insiders", "cursor", "codium", "windsurf")
$VsixPath = "$env:TEMP\kora-vscode-0.1.0.vsix"
$LocalVsix = "$PSScriptRoot\integrations\vscode\kora-vscode-0.1.0.vsix"

# 5.1 Locate or download VSIX package
if (Test-Path $LocalVsix) {
    $VsixPath = $LocalVsix
} else {
    try {
        $VsixUrl = "https://github.com/$KoraRepo/releases/latest/download/kora-vscode-latest.vsix"
        Invoke-WebRequest -Uri $VsixUrl -OutFile $VsixPath -UseBasicParsing -ErrorAction SilentlyContinue
    } catch {}
}

# 5.2 Execute extension installation via editor CLI commands
foreach ($cli in $EditorClis) {
    $cmd = Get-Command $cli -ErrorAction SilentlyContinue
    if ($cmd) {
        try {
            Write-Host "  Found $cli CLI. Installing Kora extension..." -ForegroundColor Cyan
            if (Test-Path $VsixPath) {
                & $cli --install-extension "$VsixPath" --force | Out-Null
                Write-Host "✓ Extension installed in $cli via command-line." -ForegroundColor Green
            } else {
                & $cli --install-extension "kora-lang.kora-vscode" --force 2>$null | Out-Null
                Write-Host "✓ Extension installed in $cli via marketplace command." -ForegroundColor Green
            }
        } catch {
            Write-Host "ℹ️  Could not complete CLI extension install for $cli. Proceeding with file sync." -ForegroundColor Gray
        }
    }
}

# 5.3 Direct Directory Sync fallback for VS Code, Cursor, and Antigravity IDE
$TargetExtDirs = @(
    "$env:USERPROFILE\.vscode\extensions\kora-vscode",
    "$env:USERPROFILE\.vscode-insiders\extensions\kora-vscode",
    "$env:USERPROFILE\.cursor\extensions\kora-vscode",
    "$env:USERPROFILE\.antigravity-ide\extensions\kora-lang.kora-vscode-0.1.0"
)

foreach ($VsCodeExtDir in $TargetExtDirs) {
    try {
        $ParentDir = Split-Path $VsCodeExtDir -Parent
        if ((Test-Path $ParentDir) -or ($VsCodeExtDir -like "*\.vscode\*")) {
            New-Item -ItemType Directory -Path "$VsCodeExtDir\client" -Force | Out-Null
            New-Item -ItemType Directory -Path "$VsCodeExtDir\syntaxes" -Force | Out-Null
            New-Item -ItemType Directory -Path "$VsCodeExtDir\snippets" -Force | Out-Null

            if (Test-Path "$PSScriptRoot\integrations\vscode") {
                Copy-Item "$PSScriptRoot\integrations\vscode\package.json" "$VsCodeExtDir\package.json" -Force -ErrorAction SilentlyContinue
                Copy-Item "$PSScriptRoot\integrations\vscode\language-configuration.json" "$VsCodeExtDir\language-configuration.json" -Force -ErrorAction SilentlyContinue
                Copy-Item "$PSScriptRoot\integrations\vscode\client\extension.js" "$VsCodeExtDir\client\extension.js" -Force -ErrorAction SilentlyContinue
                Copy-Item "$PSScriptRoot\integrations\vscode\syntaxes\kora.tmLanguage.json" "$VsCodeExtDir\syntaxes\kora.tmLanguage.json" -Force -ErrorAction SilentlyContinue
                Copy-Item "$PSScriptRoot\integrations\vscode\snippets\kora.snippets.json" "$VsCodeExtDir\snippets\kora.snippets.json" -Force -ErrorAction SilentlyContinue
            } else {
                $BaseRaw = "https://raw.githubusercontent.com/$KoraRepo/main/integrations/vscode"
                Invoke-WebRequest -Uri "$BaseRaw/package.json" -OutFile "$VsCodeExtDir\package.json" -UseBasicParsing -ErrorAction SilentlyContinue
                Invoke-WebRequest -Uri "$BaseRaw/language-configuration.json" -OutFile "$VsCodeExtDir\language-configuration.json" -UseBasicParsing -ErrorAction SilentlyContinue
                Invoke-WebRequest -Uri "$BaseRaw/client/extension.js" -OutFile "$VsCodeExtDir\client\extension.js" -UseBasicParsing -ErrorAction SilentlyContinue
                Invoke-WebRequest -Uri "$BaseRaw/syntaxes/kora.tmLanguage.json" -OutFile "$VsCodeExtDir\syntaxes\kora.tmLanguage.json" -UseBasicParsing -ErrorAction SilentlyContinue
                Invoke-WebRequest -Uri "$BaseRaw/snippets/kora.snippets.json" -OutFile "$VsCodeExtDir\snippets\kora.snippets.json" -UseBasicParsing -ErrorAction SilentlyContinue
            }
            Write-Host "✓ Extension files synced to $VsCodeExtDir." -ForegroundColor Green
        }
    } catch {
        Write-Host "ℹ️  Skipped editor extension auto-sync for $VsCodeExtDir." -ForegroundColor Gray
    }
}

# 5.4 Seamless Material Icon Theme Integration
$LocalKoraSvg = "$PSScriptRoot\integrations\vscode\icons\kora-file.svg"
if (Test-Path $LocalKoraSvg) {
    Get-ChildItem -Path "$env:USERPROFILE" -Filter "pkief.material-icon-theme*" -Recurse -Depth 5 -Directory -ErrorAction SilentlyContinue | ForEach-Object {
        $MatIconDir = Join-Path $_.FullName "icons"
        if (Test-Path $MatIconDir) {
            Copy-Item $LocalKoraSvg (Join-Path $MatIconDir "kora.svg") -Force -ErrorAction SilentlyContinue
            Write-Host "✓ Seamlessly integrated Kora icons into Material Icon Theme ($($_.Name))." -ForegroundColor Green
        }
    }
}

Write-Host "`n================================================================" -ForegroundColor Cyan
Write-Host "  🎉 Kora Environment Successfully Installed on Windows!       " -ForegroundColor Green
Write-Host "================================================================" -ForegroundColor Cyan
Write-Host "Run the following command to verify:" -ForegroundColor Yellow
Write-Host "  kora --version" -ForegroundColor Cyan
Write-Host "  kora doctor" -ForegroundColor Cyan
Write-Host "`nTo start a new project:" -ForegroundColor Yellow
Write-Host "  kora init my-api" -ForegroundColor Cyan

