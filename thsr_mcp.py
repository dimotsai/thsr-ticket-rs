import sys
import json
import subprocess
import os

# Nana's THSR MCP Tool
# This tool allows Luna to book THSR tickets natively via MCP.

def main():
    # Basic MCP tool implementation
    # This script will be called by Gemini CLI as an MCP server.
    
    # Simple JSON-RPC-like interface (simplified for this task)
    print(json.dumps({
        "mcp_tools": [
            {
                "name": "thsr_book",
                "description": "Book Taiwan High Speed Rail (THSR) tickets with flexible time ranges (10s timeout).",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "date": {"type": "string", "description": "Departure date (YYYY/MM/DD)"},
                        "from_stn": {"type": "string", "description": "Departure station name (e.g. 台北, 新竹)"},
                        "to_stn": {"type": "string", "description": "Arrival station name"},
                        "range_start": {"type": "string", "description": "Earliest time (HH:MM)"},
                        "range_end": {"type": "string", "description": "Latest time (HH:MM)"}
                    },
                    "required": ["date", "from_stn", "to_stn", "range_start", "range_end"]
                }
            }
        ]
    }))

if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "run":
        # Handle the actual tool execution here
        args = json.loads(sys.argv[2])
        cmd = [
            "C:/Users/Dimo/.ductor/workspace/projects/thsr/thsr.exe",
            "--date", args["date"],
            "--from", args["from_stn"],
            "--to", args["to_stn"],
            "-S", args["range_start"],
            "-E", args["range_end"],
            "--timeout", "10" # Nana's mandatory 10s timeout
        ]
            
        result = subprocess.run(cmd, capture_output=True, text=True, cwd="C:/Users/Dimo/.ductor/workspace/projects/thsr")
        print(result.stdout + result.stderr)
    else:
        main()
