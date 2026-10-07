#!/usr/bin/env python3
"""Push a local git repo to GitHub via Contents API + Git Data API (gh CLI)."""
import subprocess, json, sys, os, base64, tempfile

REPO = "mikelgh/agentic-mail"
BUNDLE_DIR = r"C:\DevOps\agentic-mail"

def gh_api(endpoint, data=None, method=None):
    cmd = ["gh", "api"]
    if method:
        cmd += ["--method", method]
    elif data:
        cmd += ["--method", "POST"]  # PUT endpoints need explicit method
    cmd.append(endpoint)
    if data:
        with tempfile.NamedTemporaryFile(mode="w", suffix=".json", delete=False) as f:
            json.dump(data, f)
            tmp_path = f.name
        cmd += ["--input", tmp_path]
    result = subprocess.run(cmd, capture_output=True, text=True, cwd=BUNDLE_DIR)
    if data and os.path.exists(tmp_path):
        os.unlink(tmp_path)
    if result.returncode != 0:
        print(f"API error ({endpoint}): {result.stderr.strip()}", file=sys.stderr)
        sys.exit(1)
    return json.loads(result.stdout) if result.stdout.strip() else None

def get_files():
    result = subprocess.run(["git", "ls-files"], capture_output=True, text=True, cwd=BUNDLE_DIR)
    return [f for f in result.stdout.strip().split("\n") if f]

def read_file_b64(filepath):
    with open(os.path.join(BUNDLE_DIR, filepath), "rb") as f:
        return base64.b64encode(f.read()).decode()

files = get_files()
print(f"Pushing {len(files)} files to {REPO}...")

# Strategy: Use Contents API for first file (creates the repo),
# then Git Data API for remaining files + single commit.

# Step 1: Create first file via Contents API
first_file = files[0]
print(f"[1] Creating {first_file} via Contents API...")
content_result = gh_api(
    f"repos/{REPO}/contents/{first_file}",
    data={
        "message": "agentic-mail 0.1.0: intelligent mail reader with calendar detection and reply drafts",
        "content": read_file_b64(first_file),
    },
    method="PUT"
)
commit_sha = content_result["commit"]["sha"]
print(f"    Initial commit: {commit_sha[:12]}")

# Step 2: Add remaining files via Git Data API
if len(files) > 1:
    # Get the current tree
    commit_obj = gh_api(f"repos/{REPO}/git/commits/{commit_sha}")
    base_tree_sha = commit_obj["tree"]["sha"]
    
    # Create blobs for remaining files
    tree_entries = []
    for f in files[1:]:
        blob = gh_api(f"repos/{REPO}/git/blobs", {"content": read_file_b64(f), "encoding": "base64"})
        tree_entries.append({"path": f, "mode": "100644", "type": "blob", "sha": blob["sha"]})
        print(f"    blob: {f}")
    
    # Create tree
    tree = gh_api(f"repos/{REPO}/git/trees", {"base_tree": base_tree_sha, "tree": tree_entries})
    
    # Create commit
    new_commit = gh_api(f"repos/{REPO}/git/commits", {
        "message": "add remaining bundle files",
        "tree": tree["sha"],
        "parents": [commit_sha]
    })
    commit_sha = new_commit["sha"]
    
    # Update ref
    gh_api(f"repos/{REPO}/git/refs/heads/main",
           data={"sha": commit_sha, "force": True}, method="PATCH")
    print(f"    Updated commit: {commit_sha[:12]}")

# Step 3: Create tag
tag = gh_api(f"repos/{REPO}/git/tags", {
    "tag": "v0.1.0", "message": "v0.1.0",
    "object": commit_sha, "type": "commit"
})
gh_api(f"repos/{REPO}/git/refs",
       data={"ref": "refs/tags/v0.1.0", "sha": tag["sha"]})

print(f"\nhttps://github.com/{REPO}  tag: v0.1.0")
print("DONE!")
