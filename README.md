# THSR Ticket Booking Tool (Nana Edition)

A high-performance CLI tool for booking Taiwan High Speed Rail (THSR) tickets, integrated with OCR and MCP.

## 🚀 Standard Operating Procedures (SOP)

- **10s Timeout**: All THSR executions MUST have a strict 10-second timeout to prevent resource hogging on Windows.
- **Full Automation**: No manual captcha input. The tool relies entirely on `ddddocr` for automatic solving.
- **Minimalist CLI**: Legacy parameters have been removed to keep the interface clean and professional.

## 🛠️ Features & Integration

- **Native Monitoring**: `thsr.exe` now supports native monitoring and retries via the `--monitor` and `--retries` flags.
- **thsr_mcp.py**: MCP Bridge for AI Agents (like Luna) to book tickets natively via Gemini CLI.
- **thsr_solver.py**: Optimized Python-based OCR plugin for automatic captcha solving.
- **thsr.exe**: Core Rust-based booking engine.

## 🧹 Legacy Cleanup (Feb 18, 2026)

The following parameters and features have been removed/deprecated as per Nana's instructions:
- `Enter captcha code manually`: Removed to ensure full automation.
- `--model <FILE>` (Rust built-in model): Removed.
- `--wait-for-captcha`: Removed.
- `--monitor` (in MCP mode): Simplified to single-run with timeout.

---
*Maintained by Nana & Luna*
