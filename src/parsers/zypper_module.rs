use regex::{Regex, regex};
use itertools::Itertools;

pub fn zypper_parse_install_errors(line: &mut String, packets_list: &mut Vec<String>) {
    let not_found_pattern: Regex = regex!(r#".*"(?<packet_name>[\w\d\s]+)".*((not found)|(не найден))"#).clone();

    if let Some(captures) = not_found_pattern.captures(line) {
        if let Some(named_capt) = captures.name("packet_name") {
            let packet_name = named_capt.as_str().to_string();

            packets_list.push(packet_name);
        }
    }
}
pub fn print_line(line: &mut String) {
    println!("Line: {}", line.clone());
}

pub fn parse_table(line: &mut String, partitioner: &str, packets_found: &mut Vec<String>) {
    let pattern = Regex::new(r#"[\s|]+(S|Name|Summary|Type)[\s|]+"#).unwrap();

    if line.contains(partitioner) && !pattern.is_match(line) {
        let line_preready = line.clone().split("|").map(|item| {item.trim()}).join("|");

        packets_found.push(line_preready);
    }
}