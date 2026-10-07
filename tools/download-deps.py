#!/usr/bin/env python3
"""Download GitHub repo zips with retry and resume support."""
import os, sys, time, urllib.request, ssl

ctx = ssl.create_default_context()
repos = {
    "octoscript-makepad": ("OctoSense-org/Octoscript-Makepad", "bbb9f69fa62858e775058ab743047746e432bfaa"),
    "makepad": ("OctoSense-org/makepad", "b0cbc9bcfb204ade5d8ceecb66e0a58e972c70f1"),
}
ws = r"C:\DevOps\octosense-ws"

for name, (repo, rev) in repos.items():
    url = f"https://github.com/{repo}/archive/{rev}.zip"
    out = os.path.join(ws, f"{name}.zip")
    if os.path.exists(out) and os.path.getsize(out) > 1000:
        print(f"SKIP {name} (already {os.path.getsize(out)} bytes)")
        continue
    for attempt in range(1, 6):
        try:
            print(f"[{attempt}/5] Downloading {name} from {url} ...")
            req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0"})
            with urllib.request.urlopen(req, timeout=300, context=ctx) as r:
                data = r.read()
            with open(out, "wb") as f:
                f.write(data)
            print(f"  OK: {name} = {len(data)} bytes")
            break
        except Exception as e:
            print(f"  Failed: {e}")
            if os.path.exists(out):
                os.remove(out)
            time.sleep(3)
    else:
        print(f"FATAL: could not download {name}")
        sys.exit(1)

print("\nAll downloads complete!")
