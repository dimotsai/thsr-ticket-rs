use chrono::NaiveTime;

pub static STATION_MAP: [&str; 12] = [
    "Nangang", "Taipei", "Banqiao", "Taoyuan", "Hsinchu", "Miaoli", "Taichung", "Changhua",
    "Yunlin", "Chiayi", "Tainan", "Zuouing",
];

pub static STATION_MAP_ZH: [&str; 12] = [
    "南港", "台北", "板橋", "桃園", "新竹", "苗栗", "台中", "彰化", "雲林", "嘉義", "台南", "左營",
];

pub fn resolve_station(input: &str) -> Option<usize> {
    if let Ok(idx) = input.parse::<usize>() {
        if idx >= 1 && idx <= STATION_MAP.len() {
            return Some(idx);
        }
    }

    let input_lower = input.to_lowercase();
    for (i, (&en, &zh)) in STATION_MAP.iter().zip(STATION_MAP_ZH.iter()).enumerate() {
        if en.to_lowercase() == input_lower || zh == input || (zh.len() > 3 && input.len() > 3 && zh.contains(input)) || (input.len() > 1 && zh.contains(input)) {
            return Some(i + 1);
        }
    }
    
    None
}

pub static TIME_TABLE: [&str; 38] = [
    "1201A", "1230A", "600A", "630A", "700A", "730A", "800A", "830A", "900A", "930A", "1000A",
    "1030A", "1100A", "1130A", "1200N", "1230P", "100P", "130P", "200P", "230P", "300P", "330P",
    "400P", "430P", "500P", "530P", "600P", "630P", "700P", "730P", "800P", "830P", "900P", "930P",
    "1000P", "1030P", "1100P", "1130P",
];

pub fn resolve_time(input: &str) -> Option<usize> {
    if let Ok(idx) = input.parse::<usize>() {
        if idx >= 1 && idx <= TIME_TABLE.len() {
            return Some(idx);
        }
    }

    let target_time = NaiveTime::parse_from_str(input, "%H:%M")
        .or_else(|_| NaiveTime::parse_from_str(input, "%H%M"))
        .ok()?;

    let mut best_idx = None;
    let mut min_diff = i64::MAX;

    for (i, &t_str) in TIME_TABLE.iter().enumerate() {
        let suffix = &t_str[t_str.len() - 1..];
        let mut time_part = t_str[..t_str.len() - 1].to_string();
        if time_part.len() < 4 {
            time_part = format!("{:0>4}", time_part);
        }

        let mut hour: u32 = time_part[..2].parse().unwrap();
        let min: u32 = time_part[2..].parse().unwrap();

        if suffix == "A" && hour == 12 {
            hour = 0;
        } else if suffix == "P" && hour != 12 {
            hour += 12;
        } else if suffix == "N" {
            hour = 12;
        }

        if let Some(t) = NaiveTime::from_hms_opt(hour, min, 0) {
            let diff = (t - target_time).num_minutes().abs();
            if diff < min_diff {
                min_diff = diff;
                best_idx = Some(i + 1);
            }
        }
    }

    best_idx
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum TicketType {
    Adult = 70,    // F
    Child = 72,    // H
    Disabled = 87, // W
    Elder = 69,    // E
    College = 80,  // P
}
