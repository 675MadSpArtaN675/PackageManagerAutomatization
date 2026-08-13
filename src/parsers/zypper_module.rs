use regex::{Regex, regex};
use itertools::Itertools;

use crate::utility::{Stage, ErroredPacket};
use crate::PacketManagerCommandExecutor;
use crate::command_struct::packet_manager_trait::ParserOutput;

use log::info;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::DerefMut;


pub(crate) fn zypper_error_preprocessing(base_map_of_functions: &mut HashMap<Stage, ParserOutput>, errored_packets_collection: &Rc<RefCell<Vec<ErroredPacket>>>)
{
    let errored_packages = errored_packets_collection.clone();
    base_map_of_functions.insert(Stage::Install, Box::new(move |line: &mut String| {
        let mut _pack_list: Vec<String> = Vec::new();
        zypper_parse_install_errors(line, &mut _pack_list);

        errored_packages.borrow_mut().extend(_pack_list.iter().map(|item| ErroredPacket { name: item.clone(), stage: Stage::Install}));
    }));
}

fn zypper_parse_install_errors(line: &mut String, packets_list: &mut Vec<String>) {
    let not_found_pattern: Regex = regex!(r#".*"(?<packet_name>[\w\d\s]+)".*((not found)|(не найден))"#).clone();

    if let Some(captures) = not_found_pattern.captures(line) {
        if let Some(named_capt) = captures.name("packet_name") {
            let packet_name = named_capt.as_str().to_string();

            packets_list.push(packet_name);
        }
    }
}

pub(crate) fn zypper_preprocessing(packet_manager_obj: &mut PacketManagerCommandExecutor) {
    let ptr_collection = packet_manager_obj.valid_lines.clone();
    let base_map_of_functions = &mut packet_manager_obj.stage_performers;

    let table_parser = Box::new(
            move |line: &mut String| {
                parse_table(line, "|", ptr_collection.borrow_mut().deref_mut());
        }
    );

    base_map_of_functions.insert(Stage::Install, Box::new(print_line));
    base_map_of_functions.insert(Stage::Update, Box::new(print_line));
    base_map_of_functions.insert(Stage::Remove, Box::new(print_line));
    base_map_of_functions.insert(Stage::Showing, Box::new(print_line));
    base_map_of_functions.insert(Stage::Search, table_parser.clone());
    base_map_of_functions.insert(Stage::RepoList, table_parser.clone());

    packet_manager_obj.package_index_step = [1, 2, 0, 3];
    packet_manager_obj.repo_index_step = [1, 3, 6];
    packet_manager_obj.yes_pointer_str = "Да|Yes".to_string();
}

fn print_line(line: &mut String) {
    info!("Manager output - {}", line.clone());
}

fn parse_table(line: &mut String, partitioner: &str, packets_found: &mut Vec<String>) {
    let pattern = Regex::new(r#"[\s|]+(S|Name|Summary|Type)[\s|]+"#).unwrap();

    if line.contains(partitioner) && !pattern.is_match(line) {
        let line_preready = line.clone().split("|").map(|item| {item.trim()}).join("|");

        packets_found.push(line_preready);
    }
}
