use bidi_detector::*;

use glob::glob;
use glob::Pattern;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::exit;
const CONFIG_PATH: &str = "bidi_config.toml";
const DEFAULT_CONFIG: &str = r#"
[general]
includes = [ 
    "**/*",
]
excludes = [
]

[display]
show_details = true"#;

#[derive(Serialize, Deserialize)]
struct GeneralSettings {
    includes: Vec<String>,
    excludes: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct DisplaySettings {
    show_details: bool,
}

#[derive(Serialize, Deserialize)]
struct Config {
    general: GeneralSettings,
    display: DisplaySettings,
}

fn read_config(config_file_path: &str) -> Config {
    let config_content: String =
        fs::read_to_string(&config_file_path).unwrap_or_else(|_| String::from(DEFAULT_CONFIG));
    let decoded: Config = toml::from_str(&config_content).unwrap();
    decoded
}

fn matches_exclude(exclude_patterns: &[String], test_str: &str) -> bool {
    for exclude in exclude_patterns {
        let compiled_pattern: Pattern =
            Pattern::new(exclude).expect("Could not compile exclude pattern!");
        if compiled_pattern.matches(test_str) {
            return true;
        }
    }
    false
}

fn check_files(config: &Config) -> u64 {
    let general_config = &config.general;
    let mut bidi_chars_found_overall: u64 = 0;
    for include in &general_config.includes {
        for entry in glob(include).expect("Failed to compile include pattern") {
            match entry {
                Ok(path) => {
                    if path.is_file() {
                        let display_path = path.display().to_string();
                        if !(matches_exclude(&general_config.excludes, &display_path)) {
                            let content: String =
                                fs::read_to_string(path).expect("Could not read file content");
                            let result = check_for_bidi_chars(&content);
                            if result.contains_bidi_chars {
                                bidi_chars_found_overall += result.occurences.len() as u64;
                            }
                            println!(
                                "{} - {} BIDI characters",
                                &display_path,
                                result.occurences.len()
                            );
                            if config.display.show_details {
                                for occurence in &result.occurences {
                                    let char_detail =
                                        get_char_detail(&occurence.found_char).unwrap();
                                    eprintln!(
                                        "Found character {} ({}), {}:{}:{}",
                                        char_detail.abbreviation,
                                        char_detail.name,
                                        &display_path,
                                        occurence.line,
                                        occurence.char_pos
                                    )
                                }
                            }
                        }
                    }
                }
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }
    bidi_chars_found_overall
}

fn main() {
    let config = read_config(CONFIG_PATH);
    let bidi_chars_found_overall = check_files(&config);
    if bidi_chars_found_overall > 0 {
        eprintln!(
            "Found {} potentially dangerous Unicode BIDI Characters!",
            bidi_chars_found_overall
        );
        exit(1)
    } else {
        exit(0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{check_files, Config, DisplaySettings, GeneralSettings};
    #[test]
    fn check_example_js() {
        let config = Config {
            general: GeneralSettings {
                includes: vec![String::from("test/*.js"), String::from("src/*")],
                excludes: vec![],
            },
            display: DisplaySettings { show_details: true },
        };
        let result = check_files(&config);
        assert_eq!(result, 6);
    }
}
