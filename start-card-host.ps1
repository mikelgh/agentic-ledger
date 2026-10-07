# start-card-host.ps1 — 加载 .env 并启动 card-host
$ErrorActionPreference = "Stop"
$root = $PSScriptRoot

$envFile = Join-Path $root ".env"
if (-not (Test-Path $envFile)) {
    Write-Host "[ERROR] .env not found. Copy .env.example to .env first." -ForegroundColor Red
    exit 1
}
Get-Content $envFile | ForEach-Object {
    $l = $_.Trim()
    if ($l -and -not $l.StartsWith("#") -and $l.Contains("=")) {
        $p = $l.Split("=", 2)
        [Environment]::SetEnvironmentVariable($p[0].Trim(), $p[1].Trim(), "Process")
    }
}
if (-not $env:MINIMAX_API_KEY -or $env:MINIMAX_API_KEY.StartsWith("your-")) {
    Write-Host "[ERROR] MINIMAX_API_KEY missing or placeholder." -ForegroundColor Red
    exit 1
}
Write-Host "[env] key=$($env:MINIMAX_API_KEY.Substring(0,12))... model=$env:MINIMAX_MODEL" -ForegroundColor DarkGray

Get-Process card-host -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep 1

& "C:\DevOps\octosense-ws\OctoSense-App-Hub\target\release\card-host.exe" `
    --bundle   "C:\DevOps\agentic-mail\bundle" `
    --allow-unsigned --stamp `
    --app-data "C:\DevOps\agentic-mail\.local-state"
