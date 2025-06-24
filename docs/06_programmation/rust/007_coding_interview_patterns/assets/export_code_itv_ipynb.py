# First thing first
#   In the terminal type
#       1 - source ~/miniconda3/etc/profile.d/conda.sh
#       2 - conda activate multi_jupyter
# Now you can check you can call nbconvert with the command : jupyter nbconvert
# python export_code_itv_ipynb.py

import os
import re
import shutil
import subprocess
from pathlib import Path

# Step 1: Define source and destination paths
home_dir = Path.home()
export_dir = home_dir / "export_md_rust_coding_interview"
source_base = Path("/mnt/c/Users/phili/OneDrive/Documents/Programmation/rust_jupyter/rust_coding_interview")

# Step 1.1: Remove existing export directory if it exists
if export_dir.exists():
    shutil.rmtree(export_dir)

# Step 1.2: Create a fresh export directory
export_dir.mkdir()

# Step 1.3: Iterate over matching source directories
for subdir in source_base.iterdir():
    if subdir.is_dir() and re.match(r"^\d{2}_", subdir.name):
        dest_subdir = export_dir / subdir.name
        dest_subdir.mkdir()

        # Convert all notebooks in this directory to markdown in the destination
        for notebook in subdir.glob("*.ipynb"):
            subprocess.run(
                ["jupyter", "nbconvert", "--to", "markdown", "--output-dir", str(dest_subdir), str(notebook)]
            )

# Step 2: Modify each .md file in each subdirectory
for subdir in export_dir.iterdir():
    if subdir.is_dir():
        for md_file in subdir.glob("*.md"):
            with md_file.open("r", encoding="utf-8") as f:
                lines = f.readlines()

            # Step 2.1: Fix incorrect language identifier
            lines = [line.replace("```Rust", "```rust") for line in lines]

            # Step 2.2: Extract title from first line starting with "# "
            title_line = next((line for line in lines if line.strip().startswith("# ")), "# Untitled")
            title_text = title_line.strip("# ").strip()

            # Step 2.3: Build pXXX or bonusXXX identifier from filename
            stem = md_file.stem

            match_p = re.match(r"^(\d{1,3})_", stem)
            match_bonus = re.match(r"^bonus_(\d{1,3})_", stem)

            if match_p:
                number = int(match_p.group(1))
                identifier = f"p{number:03d}"
            elif match_bonus:
                number = int(match_bonus.group(1))
                identifier = f"bonus{number:03d}"
            else:
                identifier = "p000"  # Fallback

            # Step 2.4: Create YAML front matter
            yaml_header = f"""---
# published: false
layout: default
title: "{identifier} - {title_text}"
parent: "Coding Interview Patterns in Rust"
#math: mathjax
date               : 2025-06-22 09:00:00
last_modified_date : 2025-06-22 09:00:00
---

"""
            # Step 2.5: Write back modified content with front matter
            with md_file.open("w", encoding="utf-8") as f:
                f.write(yaml_header)
                f.writelines(lines)
