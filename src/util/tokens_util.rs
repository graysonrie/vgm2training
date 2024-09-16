pub const SONG_START: &str = "<SongStart>";
pub const SONG_END: &str = "<SongEnd>";
pub const PAT_START: &str = "<PatternStart>";
pub const PAT_END: &str = "<PatternEnd>";

pub fn misc_tokens() -> Vec<String> {
    vec![SONG_START, SONG_END, PAT_START, PAT_END]
        .iter()
        .map(|x| x.to_string())
        .collect()
}

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
/// also includes note cut "--" and note release "=="
///
/// includes the noise notes (numbers 0-9) as well. Might split those up as their own tokens in the future
pub fn music_notes() -> Vec<String> {
    vec![
        ".", "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "--", "==", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect()
}

pub fn hex_numbers() -> Vec<String> {
    vec![
        ".", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
    ]
    .iter()
    .map(|x| format!("{}", x))
    .collect()
}

/// includes the '#' used in the noise channel
pub fn oct_numbers() -> Vec<String> {
    vec![".", "0", "1", "2", "3", "4", "5", "6", "7", "#"]
        .iter()
        .map(|x| format!("{}", x))
        .collect()
}

pub fn fx_letters() -> Vec<String> {
    vec![
        ".", "A", "B", "C", "D", "E", "F", "G", "H", "I", "M", "P", "Q", "R", "S", "V", "Z", "0",
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ]
    .iter()
    .map(|x| format!("{}", x))
    .collect()
}
