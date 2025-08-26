#!/usr/bin/env python3
import os
import subprocess
import uuid

os.chdir(os.path.dirname(os.path.abspath(__file__)))

etag = str(uuid.uuid4())

cmds: list = [
    ["./run_tailwind.py"],
    ["./run_minify.py"],
    ["cargo", "build", "--release", "--config", f"env.ETAG='{etag}'"]
]

for cmd in cmds:
    # subprocess.run(cmd)
    subprocess.run(cmd, env=os.environ | {
        "MINIFY": "true"
    })
