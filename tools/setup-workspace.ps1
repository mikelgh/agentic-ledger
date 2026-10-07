$ErrorActionPreference = "Continue"
$proxy = "socks5h://127.0.0.1:1080"
$ws = "C:\DevOps\octosense-ws"

function Git-Clone($url, $dest, $depth = 1) {
    $name = Split-Path $url -Leaf
    for ($i = 1; $i -le 5; $i++) {
        Write-Output "[$i/5] Cloning $name ..."
        if (Test-Path $dest) { Remove-Item -Recurse -Force $dest }
        git -c http.proxy=$proxy -c https.proxy=$proxy -c core.autocrlf=false `
            clone --depth $depth $url $dest 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Output "  OK: $name"
            return $true
        }
        Write-Output "  Failed, retry in 5s..."
        Start-Sleep -Seconds 5
    }
    return $false
}

# 1. Clone OctoScript-App-Design-Flow
$ok = Git-Clone "https://github.com/OctoSense-org/OctoScript-App-Design-Flow.git" "$ws\OctoScript-App-Design-Flow"
if (-not $ok) { Write-Output "FATAL: cannot clone Design-Flow"; exit 1 }

# 2. Clone OctoSense-App-Hub
$ok = Git-Clone "https://github.com/OctoSense-org/OctoSense-App-Hub.git" "$ws\OctoSense-App-Hub"
if (-not $ok) { Write-Output "FATAL: cannot clone App-Hub"; exit 1 }

# 3. Run setup-native.py to pull makepad, octoscript, octoscript-makepad
Write-Output "`n--- Running setup-native.py ---"
$env:GIT_CONFIG_COUNT = "1"
$env:GIT_CONFIG_KEY_0 = "http.proxy"
$env:GIT_CONFIG_VALUE_0 = $proxy
Push-Location "$ws\OctoScript-App-Design-Flow"
python3 tools/setup-native.py 2>&1
$setupResult = $LASTEXITCODE
Pop-Location
$env:GIT_CONFIG_COUNT = $null
$env:GIT_CONFIG_KEY_0 = $null
$env:GIT_CONFIG_VALUE_0 = $null

if ($setupResult -ne 0) {
    Write-Output "setup-native.py failed (exit $setupResult), trying once more..."
    Start-Sleep -Seconds 5
    $env:GIT_CONFIG_COUNT = "1"
    $env:GIT_CONFIG_KEY_0 = "http.proxy"
    $env:GIT_CONFIG_VALUE_0 = $proxy
    Push-Location "$ws\OctoScript-App-Design-Flow"
    python3 tools/setup-native.py 2>&1
    $setupResult = $LASTEXITCODE
    Pop-Location
    $env:GIT_CONFIG_COUNT = $null
    $env:GIT_CONFIG_KEY_0 = $null
    $env:GIT_CONFIG_VALUE_0 = $null
    if ($setupResult -ne 0) {
        Write-Output "FATAL: setup-native.py failed twice"
        exit 1
    }
}

# 4. Verify sibling checkouts
Write-Output "`n--- Verifying workspace ---"
$required = @("makepad", "octoscript-makepad", "octoscript")
foreach ($r in $required) {
    $p = Join-Path $ws $r
    if (Test-Path $p) {
        Write-Output "  OK: $r"
    } else {
        Write-Output "  MISSING: $r"
    }
}

# 5. Build hub and card-host
Write-Output "`n--- Building hub + card-host ---"
Push-Location "$ws\OctoSense-App-Hub"
cargo build --release -p octosense-card-host -p octosense-app-hub 2>&1
$buildResult = $LASTEXITCODE
Pop-Location

if ($buildResult -ne 0) {
    Write-Output "FATAL: cargo build failed (exit $buildResult)"
    exit 1
}

# 6. Verify tools
Write-Output "`n--- Verifying tools ---"
$hub = "$ws\OctoSense-App-Hub\target\release\hub.exe"
$cardHost = "$ws\OctoSense-App-Hub\target\release\card-host.exe"
if (Test-Path $hub) { Write-Output "  OK: hub" } else { Write-Output "  MISSING: hub" }
if (Test-Path $cardHost) { Write-Output "  OK: card-host" } else { Write-Output "  MISSING: card-host" }

Write-Output "`n=== WORKSPACE SETUP COMPLETE ==="
Write-Output "Workspace: $ws"
Get-ChildItem $ws -Name
