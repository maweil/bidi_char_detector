use glob::glob;
use glob::Pattern;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::exit;

use bidi_detector::check_for_bidi_chars;

#[derive(Serialize, Deserialize)]
struct GeneralSettings {
    includes: Vec<String>,
    excludes: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct DisplaySettings {}

#[derive(Serialize, Deserialize)]
struct Config {
    general: Option<GeneralSettings>,
    display: Option<DisplaySettings>,
}

fn read_config(config_file_path: &str) -> Config {
    let config_content: String = fs::read_to_string(&config_file_path).unwrap();
    let decoded: Config = toml::from_str(&config_content).unwrap();
    decoded
}

fn matches_exclude(exclude_patterns: &Vec<String>, test_str: &str) -> bool {
    for exclude in exclude_patterns {
        let compiled_pattern: Pattern = Pattern::new(exclude).unwrap();
        if compiled_pattern.matches(test_str) {
            return true;
        }
    }
    false
}

fn main() {
    let mut bidi_chars_found_overall: bool = false;
    let config = read_config("bidi_config.toml");
    let general_config = config.general.unwrap_or(GeneralSettings {
        includes: vec![String::from("*.js")],
        excludes: vec![],
    });
    for include in general_config.includes {
        for entry in glob(&include).expect("Failed to compile pattern") {
            match entry {
                Ok(path) => {
                    let display_path = path.display().to_string();
                    if !(matches_exclude(&general_config.excludes, &display_path)) {
                        let content: String = fs::read_to_string(path).unwrap();
                        let result = check_for_bidi_chars(&content);
                        if result.contains_bidi_chars {
                            bidi_chars_found_overall = true;
                        }
                        println!(
                            "{} - {} BIDI characters",
                            &display_path,
                            result.occurences.len()
                        );
                    }
                }
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }
    if bidi_chars_found_overall {
        eprintln!("Found potentially dangerous Unicode BIDI Characters!");
        exit(1)
    } else {
        exit(0)
    }
}
