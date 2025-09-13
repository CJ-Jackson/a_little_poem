#!/usr/bin/env python3
import os
import subprocess
import uuid

os.chdir(os.path.dirname(os.path.abspath(__file__)))

cmds: list = [
    ["./run_tailwind.py"],
    ["./run_minify.py"],
    ["cargo", "build", "--release"]
]

for cmd in cmds:
    # subprocess.run(cmd)
    subprocess.run(cmd, env=os.environ | {
        "MINIFY": "true"
    })
