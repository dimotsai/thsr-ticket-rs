use clap::Parser;

/// A CLI tool for booking Taiwan High Speed Rail tickets.
/// Run the program without flags will guide you through the booking process.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to config file (JSON)
    #[arg(long, short = 'C', value_name = "FILE")]
    pub config: Option<String>,

    /// Personal ID (National ID or Passport)
    #[arg(long, short = 'P', value_name = "ID")]
    pub personal_id: Option<String>,

    /// Departure date (YYYY/MM/DD)
    #[arg(long, short = 'd', value_name = "DATE")]
    pub date: Option<String>,

    /// Time ID or format (HH:MM) of the departure time
    #[arg(long, short = 'T', value_name = "TIME")]
    pub time: Option<String>,

    /// Departure station ID or Name (e.g., 2, "Taipei", "台北")
    #[arg(long, short = 'f', value_name = "STATION")]
    pub from: Option<String>,

    /// Arrival station ID or Name (e.g., 11, "Tainan", "台南")
    #[arg(long, short = 't', value_name = "STATION")]
    pub to: Option<String>,

    /// Number of adults (0-10)
    #[arg(long, short = 'a', value_name = "NUMBER")]
    pub adult_cnt: Option<u8>,

    /// Number of students/college (0-10)
    #[arg(long, short = 's', value_name = "NUMBER")]
    pub student_cnt: Option<u8>,

    /// Seat preference. 0: None, 1: Window, 2: Aisle
    #[arg(long, short = 'p', value_name = "NUMBER", value_parser = ["0", "1", "2"])]
    pub seat_prefer: Option<usize>,

    /// Class type. 0: Standard, 1: Business
    #[arg(long, short = 'c', value_name = "NUMBER", value_parser = ["0", "1"])]
    pub class_type: Option<usize>,

    /// Whether to use personal ID as TGO membership
    #[arg(long, short = 'm', value_name = "TO_USE_MEMBERSHIP")]
    pub use_membership: Option<bool>,

    /// List available stations
    #[arg(long, short = 'l')]
    pub list_station: bool,

    /// List available times
    #[arg(long, short = 'L')]
    pub list_time_table: bool,

    /// Earliest acceptable departure time (HH:MM)
    #[arg(long, short = 'S', value_name = "TIME")]
    pub range_start: Option<String>,

    /// Latest acceptable departure time (HH:MM)
    #[arg(long, short = 'E', value_name = "TIME")]
    pub range_end: Option<String>,

    /// Max retries in monitor mode (0 for infinite)
    #[arg(long, short = 'r', value_name = "NUMBER", default_value_t = 0)]
    pub retries: u32,

    /// Train number (e.g., 603, 1205)
    #[arg(long, short = 'N', value_name = "NUMBER")]
    pub train_no: Option<u32>,

    /// Run in interactive mode (prompts for missing inputs)
    #[arg(long, short = 'i')]
    pub interactive: bool,

    /// External OCR solver command (e.g., "python ocr_helper.py"). If not specified, the program will auto-detect "thsr_solver.py" in the current directory    
    #[arg(long, value_name = "COMMAND")]
    pub solver: Option<String>,

    /// Periodically check for tickets if not found
    #[arg(long, short = 'M')]
    pub monitor: bool,

    /// Interval in seconds between checks in monitor mode (default: 60)
    #[arg(long, value_name = "SECONDS", default_value_t = 60)]
    pub interval: u64,

    /// Global timeout in seconds (default: 30).
    #[arg(long, value_name = "TIMEOUT", default_value_t = 30)]
    pub timeout: u64,
}
