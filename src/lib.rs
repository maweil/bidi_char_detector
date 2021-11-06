pub struct BIDICharOccurence {
    pub line: usize,
    pub char_pos: usize,
    pub found_char: char,
}

pub struct BIDICheckResult {
    pub contains_bidi_chars: bool,
    pub occurences: Vec<BIDICharOccurence>,
}

pub struct BIDICharDetail {
    pub abbreviation: String,
    pub name: String,
    pub description: String,
}

pub fn get_char_detail(unicode_char: &char) -> Option<BIDICharDetail> {
    match unicode_char {
        '\u{202A}' => Some(BIDICharDetail {
            abbreviation: String::from("LRE"),
            name: String::from("Left-To-Right Embedding"),
            description: String::from("Try treating following text as left-to-right."),
        }),
        '\u{202B}' => Some(BIDICharDetail {
            abbreviation: String::from("RLE"),
            name: String::from("Right-To-Left Embedding"),
            description: String::from("Try treating following text as right-to-left."),
        }),
        '\u{202D}' => Some(BIDICharDetail {
            abbreviation: String::from("LRO"),
            name: String::from("Left-to-Right Override"),
            description: String::from("Force treating following text as left-to-right."),
        }),
        '\u{202E}' => Some(BIDICharDetail {
            abbreviation: String::from("RLO"),
            name: String::from("Right-to-Left Override"),
            description: String::from("Force treating following text as right-to-left."),
        }),
        '\u{2066}' => Some(BIDICharDetail {
            abbreviation: String::from("LRI"),
            name: String::from("Left-to-Right Isolate"),
            description: String::from(
                "Force treating following text as left-to-right without affecting adjacent text.",
            ),
        }),
        '\u{2067}' => Some(BIDICharDetail {
            abbreviation: String::from("RLI"),
            name: String::from("Right-to-Left Isolate"),
            description: String::from(
                "Force treating following text as right-to-left without affecting adjacent text.",
            ),
        }),
        '\u{2068}' => Some(BIDICharDetail {
            abbreviation: String::from("FSI"),
            name: String::from("First Strong Isolate"),
            description: String::from(
                "Force treating following text in direction indicated by the next character.",
            ),
        }),
        '\u{202C}' => Some(BIDICharDetail {
            abbreviation: String::from("PDF"),
            name: String::from("Pop Directional Formatting"),
            description: String::from("Terminate nearest LRE, RLE, LRO, or RLO."),
        }),
        '\u{2069}' => Some(BIDICharDetail {
            abbreviation: String::from("PDI"),
            name: String::from("Pop Directional Isolate"),
            description: String::from("Terminate nearest LRI or RLI."),
        }),
        _ => None,
    }
}

pub fn check_for_bidi_chars(test: &str) -> BIDICheckResult {
    let unicode_bidi_chars: [char; 9] = [
        '\u{202A}', '\u{202B}', '\u{202D}', '\u{202E}', '\u{2066}', '\u{2067}', '\u{2068}',
        '\u{202C}', '\u{2069}',
    ];
    let mut result = BIDICheckResult {
        contains_bidi_chars: false,
        occurences: vec![],
    };
    let mut line_num: usize = 0;
    for line in test.lines() {
        line_num += 1;
        let mut line_pos: usize = 0;
        for single_char in line.chars() {
            line_pos += 1;
            if unicode_bidi_chars.contains(&single_char) {
                result.contains_bidi_chars = true;
                result.occurences.push(BIDICharOccurence {
                    char_pos: line_pos,
                    line: line_num,
                    found_char: single_char,
                });
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
