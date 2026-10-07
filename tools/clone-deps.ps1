$ErrorActionPreference = "Continue"
$proxy = "socks5h://127.0.0.1:1080"
$sources = "C:\DevOps\OctoSense\.sources"

function Clone-Repo($url, $dest, $rev) {
    for ($i = 1; $i -le 5; $i++) {
        Write-Output "[$i/5] Cloning $url"
        if (Test-Path $dest) { Remove-Item -Recurse -Force $dest }
        git -c http.proxy=$proxy -c https.proxy=$proxy clone --depth 1 --no-checkout $url $dest 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Output "  Clone failed, retrying in 5s..."
            Start-Sleep -Seconds 5
            continue
        }
        Write-Output "  Fetching revision $rev"
        git -C $dest -c http.proxy=$proxy -c https.proxy=$proxy fetch --depth 1 origin $rev 2>&1
        if ($LASTEXITCODE -ne 0) {
            Write-Output "  Fetch revision failed, retrying..."
            Start-Sleep -Seconds 5
            continue
        }
        git -C $dest checkout --quiet --detach FETCH_HEAD 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-Output "  SUCCESS: $dest"
            return $true
        }
        Write-Output "  Checkout failed, retrying..."
    }
    Write-Output "  FAILED: $dest"
    return $false
}

# 1. octoscript-makepad
$ok = Clone-Repo "https://github.com/OctoSense-org/Octoscript-Makepad.git" "$sources\octoscript-makepad" "6351524b77a49a382c96373d7bfe6a105f142dbc"
if (-not $ok) { exit 1 }

# 2. Read runtime.json
$rt = Get-Content "$sources\octoscript-makepad\runtime.json" | ConvertFrom-Json
$makepad_rev = $rt.repositories.makepad.revision
$octoscript_rev = $rt.repositories.octoscript.revision
Write-Output "makepad: $makepad_rev"
Write-Output "octoscript: $octoscript_rev"

# 3. makepad
$ok = Clone-Repo "https://github.com/OctoSense-org/makepad.git" "$sources\makepad" $makepad_rev
if (-not $ok) { exit 1 }

# 4. octoscript
$ok = Clone-Repo "https://github.com/OctoSense-org/Octoscript.git" "$sources\octoscript" $octoscript_rev
if (-not $ok) { exit 1 }

# 5. Apply makepad patches (LF-normalized)
$patchDir = "C:\DevOps\OctoSense\tools\runtime-patches"
foreach ($p in (Get-ChildItem $patchDir -Filter "*.patch")) {
    $bytes = [System.IO.File]::ReadAllBytes($p.FullName)
    $fixed = $bytes -join '' -replace "`r`n","`n"
    [System.IO.File]::WriteAllBytes($p.FullName, [System.Text.Encoding]::UTF8.GetBytes($fixed))
}
$patches = Get-Content "C:\DevOps\OctoSense\runtime-patches.lock.json" | ConvertFrom-Json
$mp = $patches.makepad
Write-Output "Applying: $($mp.patch)"
git -C "$sources\makepad" apply "C:\DevOps\OctoSense\$($mp.patch)" 2>&1
if ($mp.stacked) {
    foreach ($s in $mp.stacked) {
        Write-Output "Applying: $($s.patch)"
        git -C "$sources\makepad" apply "C:\DevOps\OctoSense\$($s.patch)" 2>&1
    }
}

Write-Output "`nALL DONE"
Get-ChildItem $sources -Name
