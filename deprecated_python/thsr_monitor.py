import argparse
import subprocess
import time
import os
import sys

# Try to import ddddocr for OCR
try:
    import ddddocr
    OCR_AVAILABLE = True
    print("[System] Loading OCR model...")
    ocr_instance = ddddocr.DdddOcr(show_ad=False)
except ImportError:
    OCR_AVAILABLE = False
    ocr_instance = None

def get_captcha_code(image_path):
    if not os.path.exists(image_path):
        return None
    
    if OCR_AVAILABLE:
        with open(image_path, 'rb') as f:
            img_bytes = f.read()
        res = ocr_instance.classification(img_bytes)
        print(f"[OCR] Solved captcha: {res}")
        return res
    else:
        print(f"[!] OCR library 'ddddocr' not found. Automatic captcha solving disabled.")
        return None

def run_thsr(args, extra_args):
    thsr_msg = "thsr"
    if os.path.exists("thsr.exe"):
        thsr_cmd = "thsr.exe"
    elif os.path.exists("./thsr"): # Linux/Mac
        thsr_cmd = "./thsr"
    else:
        thsr_cmd = "thsr"

    if os.path.exists("tmp_code.jpg"):
        try: os.remove("tmp_code.jpg")
        except: pass

    # Clean up extra_args to remove unsupported/unused options
    cleaned_extra = [a for a in extra_args if a not in ["--wait-for-captcha", "--model"]]
    
    cmd = [thsr_cmd, "--headless", "--non-interactive"] + cleaned_extra
    
    import queue
    import threading

    def enqueue_output(out, queue):
        for line in iter(out.readline, ''):
            queue.put(line)
        out.close()

    process = subprocess.Popen(
        cmd,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1
    )
    
    q = queue.Queue()
    t = threading.Thread(target=enqueue_output, args=(process.stdout, q))
    t.daemon = True # thread dies with the main process
    t.start()
    
    output_lines = []
    captcha_sent = False
    start_time = time.time()
    timeout = 10 # Adjusted to Nana's request: strictly 10s

    try:
        while True:
            # Check for global timeout
            elapsed = time.time() - start_time
            if elapsed > timeout:
                print(f"[!] Global timeout ({timeout}s) reached. Killing process...")
                process.kill()
                output_lines.append(f"Error: Global timeout ({timeout}s) reached.")
                break

            # Non-blocking check for process exit
            if process.poll() is not None and q.empty():
                break

            # Try to get output from queue
            try:
                line = q.get_nowait()
            except queue.Empty:
                line = None

            if line:
                print(line.strip())
                output_lines.append(line.strip())
                
                if "Input security code:" in line and not captcha_sent:
                    # Wait for image to be written (poll for file presence)
                    found = False
                    for _ in range(10): # 2 seconds total
                        if os.path.exists("tmp_code.jpg"):
                            # Ensure it's not empty and size is stable
                            size1 = os.path.getsize("tmp_code.jpg")
                            time.sleep(0.2)
                            if os.path.exists("tmp_code.jpg") and os.path.getsize("tmp_code.jpg") == size1 and size1 > 0:
                                found = True
                                break
                        time.sleep(0.2)

                    if found:
                        import hashlib
                        with open("tmp_code.jpg", "rb") as f:
                            img_hash = hashlib.md5(f.read()).hexdigest()
                        print(f"[Debug] Image Hash: {img_hash}")
                        
                        code = get_captcha_code("tmp_code.jpg")
                        if code:
                            process.stdin.write(code + "\n")
                            process.stdin.flush()
                            captcha_sent = True
                    else:
                        print("[!] Timed out waiting for captcha image.")
            else:
                # No output, sleep a bit
                time.sleep(0.1)
    except Exception as e:
        print(f"[!] Error during execution: {e}")
        process.kill()
    
    return process.returncode if process.returncode is not None else -1, output_lines

def main():
    parser = argparse.ArgumentParser(
        description="Wrapper for thsr-ticket-rs with periodic checking and OCR",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter
    )
    parser.add_argument("--interval", type=int, default=60, help="Interval in seconds between checks")
    parser.add_argument("--retries", type=int, default=0, help="Max retries (0 for infinite)")
    
    # Capture all other args to pass to thsr
    args, unknown = parser.parse_known_args()
    
    if not OCR_AVAILABLE:
        print("WARNING: 'ddddocr' not installed. Automatic captcha solving will not work.")
        print("Install with: pip install ddddocr")
        print("----------------------------------------------------------------")

    retry_count = 0
    while True:
        timestamp = time.strftime("%Y-%m-%d %H:%M:%S")
        print(f"\n[{timestamp}] Starting check (Attempt {retry_count + 1})...")
        
        code, lines = run_thsr(args, unknown)
        
        # Analyze result
        # We need to know what constitutes a "success" vs "failure" (no tickets) vs "error" (captcha wrong)
        
        full_output = "\n".join(lines)
        
        if "PNR Code" in full_output:
            print(f"\n[{timestamp}] SUCCESS! Ticket booked.")
            # 1. Extract PNR details
            pnr_info = []
            capture = False
            for line in lines:
                if "PNR Code" in line or "訂位代號" in line:
                    capture = True
                if capture:
                    pnr_info.append(line)
                if capture and not line.strip(): # Stop at first empty line after PNR
                    break
            
            pnr_text = "\n".join(pnr_info)
            
            # 2. Save to file
            import datetime
            ts = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
            filename = f"booking_{ts}.txt"
            with open(filename, "w", encoding="utf-8") as f:
                f.write(f"Booking Time: {timestamp}\n")
                f.write("-" * 30 + "\n")
                f.write(full_output) # Save full output just in case
            
            print(f"[*] Booking details saved to: {filename}")
            
            # Play beep
            print("\a") 
            break
            
        if "Validation code error" in full_output or "Invalid validation code" in full_output or "檢核碼錯誤" in full_output:
             print(f"[{timestamp}] Captcha failed. Retrying immediately...")
             time.sleep(2)
             continue
             
        # Check for "no seats" message (needs verification of exact string)
        # For now, if it exits without PNR, we assume failure (or error).
        
        if args.retries > 0 and retry_count >= args.retries:
            print("Max retries reached.")
            break
            
        print(f"[{timestamp}] No ticket booked or error. Waiting {args.interval}s...")
        time.sleep(args.interval)
        retry_count += 1

if __name__ == "__main__":
    main()
