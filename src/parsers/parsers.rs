use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::utility::{ErroredPacket, Stage};
use crate::command_struct::packet_manager_trait::ParserOutput;
use crate::packet_manager::PacketManagerCommandExecutor;

use super::zypper_module::{zypper_error_preprocessing, zypper_preprocessing};

pub trait ParsersGetterTrait {
    fn fill_error_performers(&mut self, packet_manager_obj: &mut PacketManagerCommandExecutor);
    fn fill_performers(&mut self, packet_manager_obj: &mut PacketManagerCommandExecutor);
}

type Performer = Box<dyn FnMut(&mut PacketManagerCommandExecutor)>;
type ErrorPerformer = Box<dyn FnMut(&mut HashMap<Stage, ParserOutput>, &ErrorPacketCollection)>;
type ErrorPacketCollection = Rc<RefCell<Vec<ErroredPacket>>>;

pub struct ParserGetter {
    standard_performers: HashMap<String, Performer>,
    errors_performers: HashMap<String, ErrorPerformer>
}

impl ParserGetter {
    pub fn new() -> ParserGetter {
        let mut _getters_configurer = ParserGetter {standard_performers: HashMap::new(), errors_performers: HashMap::new()};

        _getters_configurer.standard_performers.insert(
            "zypper".to_string(), Box::new(
            |packet_manager_obj: &mut PacketManagerCommandExecutor| zypper_preprocessing(packet_manager_obj)
        ));

        _getters_configurer.errors_performers.insert(
            "zypper".to_string(),
         Box::new(
            |base_map_of_functions: &mut HashMap<Stage, ParserOutput>, errored_packets_collection: &ErrorPacketCollection| zypper_error_preprocessing(base_map_of_functions, errored_packets_collection)
        ));

        return _getters_configurer;
    }
}

impl ParsersGetterTrait for ParserGetter {
    fn fill_error_performers(&mut self, packet_manager_obj: &mut PacketManagerCommandExecutor) {
        let packet_manager_name: &String = &packet_manager_obj.command_obj.basic_command.clone();

        let base_map_of_functions: &mut HashMap<Stage, ParserOutput> = &mut packet_manager_obj.stage_errors_performers;
        let errored_packets_collection: &ErrorPacketCollection = &packet_manager_obj.errored_packages;

        let error_configurer: Option<&mut ErrorPerformer> = self.errors_performers.get_mut(packet_manager_name);

        if error_configurer.is_some() {
            let _configurer = error_configurer.unwrap();

            _configurer(base_map_of_functions, errored_packets_collection);
        }
    }

    fn fill_performers(&mut self, packet_manager_obj: &mut PacketManagerCommandExecutor) {
        let packet_manager_name: String = packet_manager_obj.command_obj.basic_command.clone();

        let configurer: Option<&mut Performer> = self.standard_performers.get_mut(&packet_manager_name);

        if configurer.is_some() {
            let _configurer = configurer.unwrap();

            _configurer(packet_manager_obj);
        }
    }
}