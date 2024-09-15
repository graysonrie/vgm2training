pub fn channel_names() -> Vec<String> {
    vec![
        "Pulse1",
        "Pulse2",
        "Triangle",
        "Noise",
        "DPCM",
        "VRC6Pulse1",
        "VRC6Pulse2",
        "Sawtooth",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect()
}

pub fn channel_tags() -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for channel in channel_names() {
        res.push(format!("<{}Start>", channel));
        res.push(format!("<{}End>", channel));
    }
    res
}

pub fn music_notes() -> Vec<String> {
    vec![
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect()
}

pub fn hex_numbers() -> Vec<String> {
    vec![
    ".","0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ]
    .iter()
    .map(|x| format!("{}HEX",x))
    .collect()
}

pub fn fx_letters() -> Vec<String> {
    vec![
    "A","B","C","D","E","F","G","H","I","S","P"
    ]
    .iter()
    .map(|x| format!("{}FX",x))
    .collect()
}