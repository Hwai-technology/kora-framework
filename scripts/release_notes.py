#!/usr/bin/env python3
"""
Kora Framework — Release Notes & SemVer Bump Generator
Automatically calculates the next SemVer version from conventional commits,
distinguishes between features, bug fixes, breaking changes, and updates,
and renders canonical GitHub Release Notes in English.
"""

import os
import re
import subprocess
import sys
from typing import Dict, List, Tuple


def run_cmd(cmd: List[str]) -> str:
    try:
        result = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=True)
        return result.stdout.strip()
    except subprocess.CalledProcessError:
        return ""


def get_latest_tag() -> str:
    # Try finding git tags
    tag = run_cmd(["git", "describe", "--tags", "--abbrev=0"])
    if tag and tag.startswith("v"):
        return tag

    # Fallback: check Cargo.toml version
    try:
        with open("Cargo.toml", "r", encoding="utf-8") as f:
            for line in f:
                if line.strip().startswith('version = "'):
                    ver = line.strip().split('"')[1]
                    return f"v{ver}"
    except Exception:
        pass

    return "v0.1.0"


def parse_semver(tag: str) -> Tuple[int, int, int]:
    cleaned = tag.lstrip("v").split("-")[0]
    parts = cleaned.split(".")
    major = int(parts[0]) if len(parts) > 0 and parts[0].isdigit() else 0
    minor = int(parts[1]) if len(parts) > 1 and parts[1].isdigit() else 1
    patch = int(parts[2]) if len(parts) > 2 and parts[2].isdigit() else 0
    return major, minor, patch


def get_commits_since(tag: str) -> List[Tuple[str, str, str]]:
    """Returns list of (hash, subject, body) since the tag or the last 30 commits."""
    cmd = ["git", "log", f"{tag}..HEAD", "--pretty=format:%h%x1f%s%x1f%b%x1e"]
    raw = run_cmd(cmd)
    if not raw:
        cmd = ["git", "log", "-n", "30", "--pretty=format:%h%x1f%s%x1f%b%x1e"]
        raw = run_cmd(cmd)

    commits = []
    if raw:
        for entry in raw.split("\x1e"):
            entry = entry.strip()
            if not entry:
                continue
            parts = entry.split("\x1f")
            chash = parts[0].strip() if len(parts) > 0 else ""
            csubj = parts[1].strip() if len(parts) > 1 else ""
            cbody = parts[2].strip() if len(parts) > 2 else ""
            if chash and csubj:
                commits.append((chash, csubj, cbody))
    return commits


def categorize_commits(commits: List[Tuple[str, str, str]]) -> Tuple[str, Dict[str, List[str]]]:
    categories = {
        "breaking": [],
        "features": [],
        "fixes": [],
        "perf": [],
        "maintenance": []
    }

    bump = "patch"

    for chash, subj, body in commits:
        full_text = f"{subj}\n{body}"
        entry = f"- `{chash}` {subj}"

        # 1. Breaking Change
        if "BREAKING CHANGE" in full_text or re.search(r"^[a-zA-Z]+(\([^\)]+\))?!:", subj):
            categories["breaking"].append(entry)
            bump = "major"
            continue

        # 2. Features
        if re.search(r"^feat(\([^\)]+\))?:", subj, re.IGNORECASE):
            clean_msg = re.sub(r"^feat(\([^\)]+\))?:\s*", "", subj, flags=re.IGNORECASE)
            categories["features"].append(f"- `{chash}` **{clean_msg}**")
            if bump != "major":
                bump = "minor"
            continue

        # 3. Bug Fixes
        if re.search(r"^fix(\([^\)]+\))?:", subj, re.IGNORECASE):
            clean_msg = re.sub(r"^fix(\([^\)]+\))?:\s*", "", subj, flags=re.IGNORECASE)
            categories["fixes"].append(f"- `{chash}` {clean_msg}")
            continue

        # 4. Performance
        if re.search(r"^perf(\([^\)]+\))?:", subj, re.IGNORECASE):
            clean_msg = re.sub(r"^perf(\([^\)]+\))?:\s*", "", subj, flags=re.IGNORECASE)
            categories["perf"].append(f"- `{chash}` {clean_msg}")
            continue

        # 5. Maintenance / Docs / Chores
        clean_msg = re.sub(r"^(chore|docs|refactor|test|ci|build)(\([^\)]+\))?:\s*", "", subj, flags=re.IGNORECASE)
        categories["maintenance"].append(f"- `{chash}` {clean_msg}")

    return bump, categories


def compute_next_version(current_tag: str, bump: str) -> str:
    major, minor, patch = parse_semver(current_tag)
    if bump == "major":
        return f"v{major + 1}.0.0"
    elif bump == "minor":
        return f"v{major}.{minor + 1}.0"
    else:
        return f"v{major}.{minor}.{patch + 1}"


def render_release_notes(version: str, previous_tag: str, categories: Dict[str, List[str]]) -> str:
    lines = [
        f"# Kora Framework {version}",
        "",
        f"Automated production release built from branch `main` (changes since `{previous_tag}`).",
        ""
    ]

    if categories["breaking"]:
        lines.append("### ⚠️ Breaking Changes")
        lines.extend(categories["breaking"])
        lines.append("")

    if categories["features"]:
        lines.append("### 🚀 Features & Enhancements")
        lines.extend(categories["features"])
        lines.append("")

    if categories["fixes"]:
        lines.append("### 🐛 Bug Fixes & Reliability")
        lines.extend(categories["fixes"])
        lines.append("")

    if categories["perf"]:
        lines.append("### ⚡ Performance Optimizations")
        lines.extend(categories["perf"])
        lines.append("")

    if categories["maintenance"]:
        lines.append("### 📝 Tooling, Maintenance & Documentation")
        lines.extend(categories["maintenance"][:15])  # Cap at 15 items for cleanliness
        lines.append("")

    lines.extend([
        "---",
        "### 📦 Precompiled Release Assets",
        f"- `kora-{version}-linux-x86_64.tar.gz` (Precompiled native 64-bit Linux binary)",
        f"- `kora-{version}-linux-x86_64.tar.gz.sha256` (SHA-256 verification checksum)",
        f"- `kora-latest-linux-x86_64.tar.gz` (Alias to latest release for automated install scripts)",
        "",
        "### ⚡ Quick Install / Update",
        "```bash",
        "curl -fsSL https://raw.githubusercontent.com/Hwai-technology/kora-framework/main/install.sh | bash",
        "```"
    ])

    return "\n".join(lines)


def main():
    # If GitHub ref is an explicit tag push (e.g. refs/tags/v0.2.0), use it
    ref = os.environ.get("GITHUB_REF", "")
    current_tag = get_latest_tag()

    commits = get_commits_since(current_tag)
    bump, categories = categorize_commits(commits)

    if ref.startswith("refs/tags/v"):
        tag_name = ref.replace("refs/tags/", "")
        version = tag_name
    else:
        # Pushed to main branch — calculate bump
        tag_name = compute_next_version(current_tag, bump)
        version = tag_name

    raw_version = tag_name.lstrip("v")
    notes = render_release_notes(tag_name, current_tag, categories)

    out_file = "RELEASE_NOTES.md"
    with open(out_file, "w", encoding="utf-8") as f:
        f.write(notes)

    print(f"Calculated Version: {tag_name} (bump: {bump})")
    print(f"Release notes written to {out_file}")

    # Set GitHub Actions output parameters if running in CI
    github_output = os.environ.get("GITHUB_OUTPUT")
    if github_output:
        with open(github_output, "a", encoding="utf-8") as f:
            f.write(f"version={raw_version}\n")
            f.write(f"tag_name={tag_name}\n")
            f.write(f"release_name=Kora Framework {tag_name}\n")
            f.write(f"bump_type={bump}\n")
            f.write(f"notes_file={out_file}\n")


if __name__ == "__main__":
    main()
