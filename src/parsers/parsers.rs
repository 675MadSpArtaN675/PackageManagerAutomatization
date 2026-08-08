use std::{ops::DerefMut, rc::Rc};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::utility::{ErroredPacket, Stage};
use crate::command_struct::packet_manager_trait::ParserOutput;
use crate::packet_manager::PacketManagerCommandExecutor;

use super::zypper_module::{zypper_parse_install_errors, print_line, parse_table};

pub fn fill_error_performers(packet_manager_name: &String, base_map_of_functions: &mut HashMap<Stage, ParserOutput>, errored_packets_collection: &Rc<RefCell<Vec<ErroredPacket>>>) {
    let name = packet_manager_name.as_str();
    match name {
        "zypper" => {
            let errored_packages = errored_packets_collection.clone();
            base_map_of_functions.insert(Stage::Install, Box::new(move |line: &mut String| {
                let mut _pack_list: Vec<String> = Vec::new();
                zypper_parse_install_errors(line, &mut _pack_list);

                errored_packages.borrow_mut().extend(_pack_list.iter().map(|item| ErroredPacket { name: item.clone(), stage: Stage::Install}));
            }));
        },

        _ => {}
    }
}

pub fn fill_performers(packet_manager_obj: &mut PacketManagerCommandExecutor) {
    let packet_manager_name: String = packet_manager_obj.command_obj.basic_command.clone();
    let name: &str = packet_manager_name.as_str();

    let base_map_of_functions = &mut packet_manager_obj.stage_performers;
    match name {
        "zypper" => {
            let ptr_collection = packet_manager_obj.valid_lines.clone();
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
        },

        _ => {}
    }
}