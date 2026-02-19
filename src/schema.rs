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

    let clean_input = input.replace(':', "");
    if let Ok(target_time) = clean_input.parse::<u16>() {
        let mut best_idx = None;
        let mut min_diff = u16::MAX;

        for (i, &t_str) in TIME_TABLE.iter().enumerate() {
            let mut t_int = t_str[..t_str.len() - 1].parse::<u16>().unwrap();
            if t_str.ends_with('A') && (t_int / 100) == 12 {
                t_int %= 1200;
            } else if t_int != 1230 && t_str.ends_with('P') {
                t_int += 1200;
            }
            
            let diff = if t_int >= target_time {
                t_int - target_time
            } else {
                (t_int + 2400) - target_time
            };

            if diff < min_diff {
                min_diff = diff;
                best_idx = Some(i + 1);
            }
        }
        return best_idx;
    }
    
    None
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
