use bidi_detector::*;

use glob::glob;
use glob::Pattern;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;
use std::process::exit;
const CONFIG_PATH: &str = "bidi_config.toml";
#[derive(Serialize, Deserialize)]
struct GeneralSettings {
    includes: Vec<String>,
    excludes: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct DisplaySettings {
    show_details: bool,
    ignore_invalid_data: Option<bool>,
    verbose: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct Config {
    general: GeneralSettings,
    display: DisplaySettings,
}

fn get_default_config() -> Config {
    Config {
        general: GeneralSettings {
            includes: vec![String::from("**/*")],
            excludes: vec![
                String::from("**/*.jpg"),
                String::from("**/*.png"),
                String::from("**/*.gif"),
                String::from("**/*.zip"),
                String::from("**/*.tar"),
                String::from("**/*.gz"),
                String::from("**/*.bz2"),
                String::from("**/*.7z"),
                String::from("**/*.class"),
                String::from("**/*.rlib"),
                String::from("**/*.so"),
                String::from("**/.git/*"),
            ],
        },
        display: DisplaySettings {
            show_details: true,
            ignore_invalid_data: Option::from(true),
            verbose: Option::from(true),
        },
    }
}

fn read_config(config_file_path: &str) -> Config {
    let config_file = fs::read_to_string(&config_file_path);
    match config_file {
        Ok(file_content) => toml::from_str(&file_content).expect("Invalid config file content."),
        Err(_e) => {
            println!("Using default configuration");
            get_default_config()
        }
    }
}

fn matches_exclude(exclude_patterns: &[String], test_str: &str) -> bool {
    for exclude in exclude_patterns {
        let compiled_pattern: Pattern =
            Pattern::new(exclude).expect("Could not compile exclude pattern.");
        if compiled_pattern.matches(test_str) {
            return true;
        }
    }
    false
}

fn check_files(config: &Config) -> u64 {
    let default_display_settings: DisplaySettings = get_default_config().display;
    let general_config = &config.general;
    let verbose_output: bool = config
        .display
        .verbose
        .unwrap_or_else(|| default_display_settings.verbose.unwrap());
    let show_occurence_details = config.display.show_details;
    let mut bidi_chars_found_overall: u64 = 0;
    for include in &general_config.includes {
        for entry in glob(include).expect("Failed to compile include pattern") {
            match entry {
                Ok(path) => {
                    if path.is_file() {
                        let display_path = path.display().to_string();
                        if !(matches_exclude(&general_config.excludes, &display_path)) {
                            let read_str_res = fs::read_to_string(path);
                            match read_str_res {
                                Ok(content) => {
                                    let result = check_for_bidi_chars(&content);
                                    let num_occurences: u64 = result.occurences.len() as u64;
                                    if result.contains_bidi_chars {
                                        bidi_chars_found_overall += num_occurences;
                                    }
                                    if num_occurences > 0 || verbose_output {
                                        println!(
                                            "{} - {} BIDI characters",
                                            &display_path, num_occurences
                                        );
                                    }
                                    if show_occurence_details {
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
                                Err(e) => {
                                    let ignore_invalid_data: bool =
                                        config.display.ignore_invalid_data.unwrap_or_else(|| {
                                            default_display_settings.ignore_invalid_data.unwrap()
                                        });
                                    if e.kind() != ErrorKind::InvalidData || !ignore_invalid_data {
                                        eprintln!("Could not read file {} - {}", &display_path, e);
                                    }
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
            "Found {} potentially dangerous Unicode BIDI characters!",
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
            display: DisplaySettings {
                show_details: true,
                ignore_invalid_data: Option::from(true),
                verbose: Option::from(false),
            },
        };
        let result = check_files(&config);
        assert_eq!(result, 6);
    }
}
