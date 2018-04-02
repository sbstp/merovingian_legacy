import os
import base64
from pathlib import Path


for path in Path("testdata").rglob("*"):
    if path.is_file() and path.stat().st_size == 0:
        data = os.urandom(100)
        path.write_bytes(base64.b64encode(data))
