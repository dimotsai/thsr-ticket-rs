# THSR-Ticket Rust

A program for booking ticket of Taiwan High Speed Railway (THSR).

This is the rust version of the original python ver [THSR-Ticket](https://github.com/BreezeWhite/THSR-Ticket), with improvements including more CLI input options, early-bird support, membership, etc.

## Install
### Download from the [Release](https://github.com/BreezeWhite/thsr-ticket-rs/releases) page

Pick the one according to your OS, download the executable file, and open the terminal to execute it.

You could also put it to one of your PATH dir (e.g. ~/.local/bin for Ubuntu) to always make it accessable.

### Install with cargo

Make sure you have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

For older linux machines, this method is preferred, since the release is built by Github Action and the workflow only supports Ubuntu-22.04 and latter.

```bash
# Install rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and install
cargo install --git https://github.com/BreezeWhite/thsr-ticket-rs
```

## Usage

```bash
# Use without flags.
# This will guide you through the process for entering informations.
thsr

# Or pass values to arguments.
# If some required informations are not specified, the program will ask you to enter.
thsr --from 2 --to 11 --adult-cnt 2

# To see available stations and its ID value
thsr --list-station

# To see available times and its ID value
thsr --list-time-table

# All following date formats are supported
thsr --date 2025/01/01
thsr --date 2025/1/01
thsr --date 2025/01/1
thsr --date 2025/1/1

# Use membership. The membership ID will be the same as the personal ID.
thsr --use-membership true
```

### Complete options

```bash
A CLI tool for booking Taiwan High Speed Rail tickets. Run the program without flags will guide you through the booking process

Usage: thsr [OPTIONS]

Options:
  -C, --config <FILE>
          Path to config file (JSON)
  -P, --personal-id <ID>
          Personal ID (National ID or Passport)
  -d, --date <DATE>
          Departure date (YYYY/MM/DD)
  -T, --time <TIME>
          Time ID or format (HH:MM) of the departure time
  -f, --from <STATION>
          Departure station ID or Name (e.g., 2, "Taipei", "台北")
  -t, --to <STATION>
          Arrival station ID or Name (e.g., 11, "Tainan", "台南")
  -a, --adult-cnt <NUMBER>
          Number of adults (0-10)
  -s, --student-cnt <NUMBER>
          Number of students/college (0-10)
  -p, --seat-prefer <NUMBER>
          Seat preference. 0: None, 1: Window, 2: Aisle [possible values: 0, 1, 2]
  -c, --class-type <NUMBER>
          Class type. 0: Standard, 1: Business [possible values: 0, 1]
  -m, --use-membership <TO_USE_MEMBERSHIP>
          Whether to use personal ID as TGO membership [possible values: true, false]
  -l, --list-station
          List available stations
  -L, --list-time-table
          List available times
  -S, --range-start <TIME>
          Earliest acceptable departure time (HH:MM)
  -E, --range-end <TIME>
          Latest acceptable departure time (HH:MM)
  -r, --retries <NUMBER>
          Max retries in monitor mode (0 for infinite) [default: 0]
  -N, --train-no <NUMBER>
          Train number (e.g., 603, 1205)
  -i, --interactive
          Run in interactive mode (prompts for missing inputs)
      --solver <COMMAND>
          External OCR solver command (e.g., "python ocr_helper.py"). If not specified, the program will auto-detect "thsr_solver.py" in the current directory
  -M, --monitor
          Periodically check for tickets if not found
      --interval <SECONDS>
          Interval in seconds between checks in monitor mode (default: 60) [default: 60]
      --timeout <TIMEOUT>
          Global timeout in seconds (default: 0, no timeout). Recommended by Nana: 10s for single runs [default: 0]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Captcha Solver Integration

The tool can automatically solve captchas using an external OCR script.

### Interface Specification
The program calls the specified solver command and passes the path to the captcha image as the **last argument**. The solver should output the recognized text directly to `stdout`.

**Default Behavior**: If `--solver` is not specified, it looks for `thsr_solver.py` in the current directory.

### Example Dummy Solver (Python)
```python
import sys

def solve(image_path):
    # Your OCR logic here (e.g., call an API or use a library)
    # image_path is passed as the last argument
    print("1234") # Output the recognized code to stdout

if __name__ == "__main__":
    if len(sys.argv) > 1:
        solve(sys.argv[-1])
```

## ***DISCLAIMER***

This is an unofficial implementation and is for research purpose only. Any legal liability is on your own. Use at your own risk.
