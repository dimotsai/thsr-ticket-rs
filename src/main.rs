use thsr::cli::Args;
use thsr::run;
use thsr::schema::{STATION_MAP, TIME_TABLE};
use serde_json::Value;
use std::fs;
use clap::Parser;

fn show_station() {
    println!("Station ID Table:");
    for (i, station) in STATION_MAP.iter().enumerate() {
        if i % 4 == 0 && i > 0 { println!(); }
        print!("{: <2}: {: <10}", i + 1, station);
    }
    println!("\n\nExample: thsr --from 2 --to 11");
}

fn show_time_table() {
    println!("Time ID Table:");
    for (idx, &t_str) in TIME_TABLE.iter().enumerate() {
        let mut t_int = t_str[..t_str.len() - 1].parse::<u16>().unwrap();
        if t_str.ends_with('A') && (t_int / 100) == 12 {
            t_int %= 1200;
        } else if t_int != 1230 && t_str.ends_with('P') {
            t_int += 1200;
        }
        let formatted_time = format!("{:04}", t_int);
        if idx % 4 == 0 && idx > 0 { println!(); }
        print!(
            "{: <2}: {:>2}:{:02}   ",
            idx + 1,
            &formatted_time[..formatted_time.len() - 2],
            &formatted_time[formatted_time.len() - 2..]
        );
    }
    println!();
}

fn merge_config_into_args(args: &mut Args) {
    let config_path = args.config.clone().unwrap_or_else(|| "config.json".to_string());
    
    if let Ok(config_str) = fs::read_to_string(&config_path) {
        if let Ok(json) = serde_json::from_str::<Value>(&config_str) {
            println!("Loading config from {}...", config_path);
            
            if args.personal_id.is_none() {
                if let Some(id) = json["personal_id"].as_str() {
                    args.personal_id = Some(id.to_string());
                }
            }
            if args.from.is_none() {
                if let Some(from) = json["from"].as_str() {
                    args.from = Some(from.to_string());
                }
            }
            if args.to.is_none() {
                if let Some(to) = json["to"].as_str() {
                    args.to = Some(to.to_string());
                }
            }
            if args.adult_cnt.is_none() {
                if let Some(cnt) = json["adult_cnt"].as_u64() {
                    args.adult_cnt = Some(cnt as u8);
                }
            }
            if args.student_cnt.is_none() {
                if let Some(cnt) = json["student_cnt"].as_u64() {
                    args.student_cnt = Some(cnt as u8);
                }
            }
            if args.seat_prefer.is_none() {
                if let Some(pref) = json["seat_prefer"].as_u64() {
                    args.seat_prefer = Some(pref as usize);
                }
            }
            if args.class_type.is_none() {
                if let Some(class) = json["class_type"].as_u64() {
                    args.class_type = Some(class as usize);
                }
            }
            if args.use_membership.is_none() {
                if let Some(m) = json["use_membership"].as_bool() {
                    args.use_membership = Some(m);
                }
            }
            if args.solver.is_none() {
                if let Some(s) = json["solver"].as_str() {
                    args.solver = Some(s.to_string());
                }
            }
        }
    } else if args.config.is_some() {
        println!("Warning: Config file {} not found.", config_path);
    }
}

fn main() {
    let mut args = Args::parse();

    if args.list_time_table {
        show_time_table();
        return;
    }

    if args.list_station {
        show_station();
        return;
    }

    merge_config_into_args(&mut args);

    if args.timeout > 0 {
        let timeout_secs = args.timeout;
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(timeout_secs));
            eprintln!(
                "\n[Error] Global timeout of {}s reached. Terminating process.",
                timeout_secs
            );
            std::process::exit(1);
        });
    }
    
    if args.monitor {
        let mut retry_count = 0;
        let interval = std::time::Duration::from_secs(args.interval);
        loop {
            retry_count += 1;
            println!("\n[{}] Starting check (Attempt {})...", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), retry_count);
            
            if run(args.clone()) {
                println!("\nSUCCESS! Ticket booked.");
                break;
            }

            if args.retries > 0 && retry_count >= args.retries {
                println!("\nMax retries ({}) reached. Exiting.", args.retries);
                std::process::exit(1);
            }

            println!("No ticket booked. Waiting {}s...", args.interval);
            std::thread::sleep(interval);
        }
    } else {
        run(args);
    }
}
