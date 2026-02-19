import sys
import os
import hashlib
import time

# Optimized solver plugin for thsr.exe
# Usage: .venv/Scripts/python thsr_solver.py [image_path]

def solve():
    try:
        img_path = sys.argv[1] if len(sys.argv) > 1 else "tmp_code.jpg"

        # 1. Wait for file to be ready (stability check)
        found = False
        for _ in range(5):
            if os.path.exists(img_path):
                s1 = os.path.getsize(img_path)
                time.sleep(0.1)
                if os.path.exists(img_path) and os.path.getsize(img_path) == s1 and s1 > 0:
                    found = True
                    break
            time.sleep(0.1)

        if not found:
            sys.stderr.write(f"Error: {img_path} not ready\n")
            sys.exit(1)

        with open(img_path, 'rb') as f:
            img_bytes = f.read()
        
        img_hash = hashlib.md5(img_bytes).hexdigest()
        
        # 2. OCR with simple logging
        import ddddocr
        ocr = ddddocr.DdddOcr(show_ad=False)
        res = ocr.classification(img_bytes)
        
        # Log to stderr (thsr.exe captures this)
        sys.stderr.write(f"[Solver] Image: {img_hash} | Result: {res}\n")

        # 3. Output to stdout for thsr.exe
        sys.stdout.write(res)
        sys.stdout.flush()

    except Exception as e:
        sys.stderr.write(f"Exception: {str(e)}\n")
        sys.exit(1)

if __name__ == "__main__":
    solve()
