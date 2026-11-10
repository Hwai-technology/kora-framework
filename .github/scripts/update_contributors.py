#!/usr/bin/env python3
"""
Kora Contributor Recognition Script
Extracts merged PR author metadata and updates the Contributors section in README.md.
"""

import os
import sys
import re

def main():
    contributor = os.getenv("CONTRIBUTOR_USERNAME", "")
    avatar = os.getenv("CONTRIBUTOR_AVATAR", "")
    html_url = os.getenv("CONTRIBUTOR_URL", "")

    if not contributor:
        print("No contributor username provided.")
        sys.exit(0)

    readme_path = "README.md"
    if not os.path.exists(readme_path):
        print("README.md not found.")
        sys.exit(1)

    with open(readme_path, "r", encoding="utf-8") as f:
        content = f.read()

    start_marker = "<!-- CONTRIBUTORS:START -->"
    end_marker = "<!-- CONTRIBUTORS:END -->"

    if start_marker not in content or end_marker not in content:
        print("Contributors markers not found in README.md")
        sys.exit(0)

    pattern = re.compile(rf"{re.escape(start_marker)}(.*?){re.escape(end_marker)}", re.DOTALL)
    match = pattern.search(content)
    if not match:
        print("Could not match contributors block.")
        sys.exit(0)

    existing_block = match.group(1)
    if contributor in existing_block:
        print(f"Contributor {contributor} is already recognized.")
        sys.exit(0)

    avatar_src = f"{avatar}&s=64" if avatar else "https://github.com/identicons/app/kora.png"
    profile_url = html_url if html_url else f"https://github.com/{contributor}"
    entry = f'<a href="{profile_url}"><img src="{avatar_src}" width="48" height="48" alt="{contributor}" title="{contributor}" style="border-radius: 50%; margin: 4px;" /></a>'

    block_clean = existing_block.strip()
    if block_clean:
        new_block = block_clean + "\n" + entry
    else:
        new_block = entry

    updated_content = content[:match.start(1)] + "\n" + new_block + "\n" + content[match.end(1):]

    with open(readme_path, "w", encoding="utf-8") as f:
        f.write(updated_content)

    print(f"Successfully added @{contributor} to README.md")

if __name__ == "__main__":
    main()

