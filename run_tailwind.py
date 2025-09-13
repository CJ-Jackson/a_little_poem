#!/usr/bin/env python3
import os
import subprocess
from pathlib import Path

# https://registry.npmjs.org/tailwindcss/-/tailwindcss-4.1.12.tgz to asset/tailwindcss

os.chdir(os.path.dirname(os.path.abspath(__file__)))

minify = False
if os.getenv('MINIFY') == "true":
    minify = True

map_css: dict = {
    "asset/css/tailwind.css": "asset/embed/css/main.css"
}

for key, value in map_css.items():
    if minify:
        value = Path(value)
        value = value.with_name(value.stem + '.min.css')
        subprocess.run(['tailwindcss', '-i', key, '-o', value, '--minify'])
    else:
        subprocess.run(['tailwindcss', '-i', key, '-o', value])
    print(f"{key} -> {value}")
