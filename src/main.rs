use std::process::exit;

use anyhow::{bail, Result};
use clap::Parser;

/// sexpand
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// SLURM-based hostname pattern
    #[clap(value_name = "PATTERN")]
    pattern: String,

    /// Expression to expand the hostnames into
    /// Use '{}' for replacement item
    #[clap(value_name = "EXPRESSION", default_value = "")]
    expression: String,

    /// Separator of final expanded hostnames
    #[clap(value_name = "SEPARATOR", default_value = ",")]
    separator: String,
}

fn pad_number(num: i32, pad: i32) -> String {
    let mut s = num.to_string();
    while s.len() < pad as usize {
        s = "0".to_string() + &s;
    }
    s
}

fn get_host_range(prefix: &str, start_num: &str, end_num: &str) -> Vec<String> {
    let mut hostnames: Vec<String> = Vec::new();
    let number_length = start_num.len();
    let start_num = match start_num.parse::<i32>() {
        Ok(n) => n,
        Err(_) => panic!("Invalid number '{}' in hostname pattern", start_num),
    };
    let end_num = end_num.parse::<i32>().unwrap();
    for i in start_num..=end_num {
        let padded_num = pad_number(i, number_length as i32);
        hostnames.push(prefix.to_string() + &padded_num.to_string());
    }
    hostnames
}

/// Expand the SLURM-based hostname pattern into a list of hostnames
/// # Arguments
/// * `pattern` - SLURM-based hostname pattern
/// # Returns
/// * A list of hostnames

/// n01,n02
///
/// n[01,02],n03,n[05-07,09]
/// n01,n02,n03,n05,n06,n07
///
fn expand_hostnames(pattern: &str) -> Result<Vec<String>> {
    // keep track of brackets and expand commas
    let mut hostnames: Vec<String> = Vec::new();
    let mut queue: Vec<String> = Vec::new();
    let mut nest_counter = 0;
    let mut prefix: Vec<String> = Vec::new();
    let mut numbers = Vec::new();
    let mut start_num = String::from("");
    let mut found_range = false;

    for (i, c) in pattern.chars().enumerate() {
        if c.is_alphabetic() && nest_counter == 0 {
            prefix.push(c.to_string());
        }
        if c.is_numeric() {
            numbers.push(c.to_string());
        }
        if c == '[' {
            nest_counter += 1;
            if nest_counter > 1 {
                bail!("Cannot nest brackets in pattern")
            }
        }
        if c == ']' {
            if found_range {
                let mut expanded_range = get_host_range(
                    &prefix.join(""),
                    &start_num,
                    &numbers.join(""),
                );
                queue.append(&mut expanded_range);
            }
            nest_counter -= 1;
            start_num = String::from("");
            found_range = false;
        }
        if c == '-' {
            start_num = numbers.join("");
            found_range = true;
            numbers.clear();
        }
        if c == ',' || i == pattern.len() - 1 {
            if found_range {
                let mut expanded_range = get_host_range(
                    &prefix.join(""),
                    &start_num,
                    &numbers.join(""),
                );
                queue.append(&mut expanded_range);
            }
            start_num = String::from("");
            let hostname = prefix.join("") + numbers.join("").as_str();
            queue.push(hostname);
            hostnames.append(&mut queue);
            queue.clear();
            numbers.clear();
            found_range = false;
            if nest_counter == 0 {
                prefix.clear();
            }
        }
    }
    hostnames.append(&mut queue);
    hostnames.sort();
    hostnames.dedup();
    Ok(hostnames)
}

/// Returns a single string that's delimited by the separator, where
/// each component is the expression that's interpolated by the hostname
/// at each pattern of '{}'
fn expand_pattern(hostnames: Vec<String>, expression: String, separator: String) -> String {
    let mut expanded = Vec::new();
    let sep = match separator.as_str() {
        "\\n" => "\n".to_string(),
        _ => separator,
    };
    for hostname in hostnames {
        expanded.push(expression.replace("{}", &hostname));
    }
    expanded.join(&sep)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let hostnames = match expand_hostnames(&args.pattern) {
        Ok(hostnames) => hostnames,
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
    };
    let s = expand_pattern(hostnames, args.expression, args.separator);
    for line in s.lines() {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_pad_number() {
        // pad_number(1, 3) -> "001"
        assert_eq!(pad_number(1, 3), "001");

        // pad_number(1, 2) -> "01"
        assert_eq!(pad_number(1, 2), "01");
    }

    #[test]
    fn test_get_host_range() {
        // get_host_range("n", "01", "03") -> ["n01", "n02", "n03"]
        assert_eq!(
            get_host_range(&"n", &"01", &"03"),
            ["n01", "n02", "n03"]
        );

        // get_host_range("n", "01", "02") -> ["n01", "n02"]
        assert_eq!(
            get_host_range(&"n", &"01", &"02"),
            ["n01", "n02"]
        );
    }

    #[test]
    fn test_expand_hostnames() {
        // expand_hostnames("n01,n02") -> ["n01", "n02"]
        assert_eq!(
            expand_hostnames("n01,n02").unwrap(),
            ["n01", "n02"]
        );

        // expand_hostnames("n[01-05]") -> ["n01", "n02", "n03", "n04", "n05"]
        assert_eq!(
            expand_hostnames("n[01-05]").unwrap(),
            ["n01", "n02", "n03", "n04", "n05"]
        );

        // expand_hostnames("n[01,02],n03,n[05-07,09]") -> ["n01", "n02", "n03", "n05", "n06", "n07"]
        assert_eq!(
            expand_hostnames("n[01,02],n03,n[05-07,09]").unwrap(),
            ["n01", "n02", "n03", "n05", "n06", "n07", "n09"]
        );

        // expand_hostnames("n[01,02],n03,n[05-07,09]") -> ["n01", "n02", "n03", "n05", "n06", "n07"]
        assert_eq!(
            expand_hostnames("n[01,02],n03,n[05-07,09]").unwrap(),
            ["n01", "n02", "n03", "n05", "n06", "n07", "n09"]
        );

        // expand_hostnames("n[[01,02]-03],n[05-07,09]") -> Err
        let res = expand_hostnames("n[[01,02]-03],n[05-07,09]");
        assert!(res.is_err())
    }
}
