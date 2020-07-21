use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
/// While using raw slice of str to handle flags is convenient,
/// in the real-world projects it is customary to use a struct,
/// that contains flags-related logic. So in this exercise
/// we ask you to implement a custom struct.
///
/// If you are curious about real-world implementation, refer to the `clap-rs` crate:
/// https://github.com/kbknapp/clap-rs/blob/master/src/args/arg_matches.rs
#[derive(Debug, Default)]
pub struct Flags {
    line_numbers: bool,
    file_names: bool,
    insensitive: bool,
    invert: bool,
    match_lines: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut flagged = Flags::default();
        for flag in flags {
            match *flag {
                "-n" => flagged.line_numbers = true,
                "-l" => flagged.file_names = true,
                "-i" => flagged.insensitive = true,
                "-v" => flagged.invert = true,
                "-x" => flagged.match_lines = true,
                _ => (),
            }
        }
        flagged
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let pattern = match flags.insensitive {
        true => pattern.to_lowercase(),
        false => pattern.to_string(),
    };

    let mut grep_lines = Vec::new();
    let mult_files = files.len() > 1;

    for file in files {
        let file_buffer = BufReader::new(File::open(file)?);

        if flags.file_names {
            for line in file_buffer.lines() {
                let line = line?;
                if line.contains(&pattern) {
                    grep_lines.push(file.to_string());
                    break;
                }
            }
        } else {
            for (i, line) in file_buffer.lines().enumerate() {
                let line = line?;
                let check_line = match flags.insensitive {
                    true => line.clone().to_lowercase(),
                    false => line.clone(),
                };

                match flags.match_lines {
                    true => {
                        if (check_line == pattern) == !flags.invert {
                            grep_lines.push(
                                line_prefix(
                                    mult_files,
                                    file.to_string(),
                                    flags.line_numbers,
                                    i + 1,
                                ) + &line,
                            );
                        }
                    }
                    false => {
                        if check_line.contains(&pattern) == !flags.invert {
                            grep_lines.push(
                                line_prefix(
                                    mult_files,
                                    file.to_string(),
                                    flags.line_numbers,
                                    i + 1,
                                ) + &line,
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(grep_lines)
}

// fn grep_file ()

fn line_prefix(mult_files: bool, file_name: String, line_numbers: bool, line_n: usize) -> String {
    let mut prefix = String::new();
    if mult_files {
        prefix += &(file_name + ":")
    }
    if line_numbers {
        prefix += &(line_n.to_string() + ":");
    }
    prefix
}
