use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::parsers::{fill_error_performers, fill_performers};

use crate::utility::{PacketManagerResultCode, Stage, ErroredPacket, FoundPackage, Repository, create_packages, create_repo};
use crate::utility::constants::{PACKAGE_SIZE, REPO_SIZE};

use crate::command_struct::packet_manager_trait::{PacketManager, ParserOutput};
use crate::command_struct::command_performers::{perform_command, catch_err};
use crate::command_struct::command_basic_structure::PacketManagerCommand;



pub struct PacketManagerCommandExecutor
{
    pub(crate) command_obj: PacketManagerCommand,

    pub(crate) errored_packages: Rc<RefCell<Vec<ErroredPacket>>>,
    pub(crate) valid_lines: Rc<RefCell<Vec<String>>>,

    pub(crate) stage_performers: HashMap<Stage, ParserOutput>,
    pub(crate) stage_errors_performers: HashMap<Stage, ParserOutput>,

    pub(crate) package_index_step: [i32; PACKAGE_SIZE],
    pub(crate) repo_index_step: [i32; REPO_SIZE],
    pub(crate) yes_pointer_str: String
}

impl PacketManagerCommandExecutor {
    pub fn new_empty() -> PacketManagerCommandExecutor
    {
        return PacketManagerCommandExecutor {
            command_obj: PacketManagerCommand::new_empty("zypper".to_string()),
            errored_packages: Rc::new(RefCell::new(vec![])),
            valid_lines: Rc::new(RefCell::new(vec![])),
            stage_performers: HashMap::new(),
            stage_errors_performers: HashMap::new(),
            package_index_step: [0; 4],
            repo_index_step: [0; 3],
            yes_pointer_str: String::new()
        };
    }

    pub fn new(base_name: String) -> PacketManagerCommandExecutor {
        let mut pm_executer: PacketManagerCommandExecutor = PacketManagerCommandExecutor {
            command_obj: PacketManagerCommand::new(base_name.clone()),
            errored_packages: Rc::new(RefCell::new(vec![])),
            valid_lines: Rc::new(RefCell::new(vec![])),
            stage_performers: HashMap::new(),
            stage_errors_performers: HashMap::new(),
            package_index_step: [0; 4],
            repo_index_step: [0; 3],
            yes_pointer_str: String::new()
        };

        fill_error_performers(&base_name.clone(), &mut pm_executer.stage_errors_performers, &pm_executer.errored_packages);
        fill_performers(&mut pm_executer);

        return pm_executer;
    }
}

impl PacketManager for PacketManagerCommandExecutor {
    fn get_performers(&mut self, stage: Stage) -> (Option<&mut ParserOutput>, Option<&mut ParserOutput>) {
        let mut out_parser: Option<&mut Box<dyn FnMut(&mut String) + 'static>> = None;
        if self.stage_performers.contains_key(&stage) {
            out_parser = self.stage_performers.get_mut(&stage);
        }

        let mut err_parser: Option<&mut Box<dyn FnMut(&mut String) + 'static>> = None;
        if self.stage_errors_performers.contains_key(&stage) {
            err_parser = self.stage_errors_performers.get_mut(&stage);
        }

        return (out_parser, err_parser);
    }

    fn install(&mut self, packets: &Vec<String>) -> PacketManagerResultCode {
        let (basic, install) = (self.command_obj.basic_command.clone(), self.command_obj.install_command.clone());
        let (out_parser, err_parser) = self.get_performers(Stage::Install);

        let _return_code: PacketManagerResultCode = perform_command(basic, install, Stage::Install, packets, true, err_parser, out_parser);

        return catch_err(_return_code, Stage::Install, &mut self.errored_packages);
    }

    fn remove(&mut self, packets: &Vec<String>) -> PacketManagerResultCode {
        let (basic, sec) = (self.command_obj.basic_command.clone(), self.command_obj.remove_command.clone());
        let (out_parser, err_parser) = self.get_performers(Stage::Remove);

        let _return_code: PacketManagerResultCode = perform_command(basic, sec, Stage::Remove, packets, true, err_parser, out_parser);

        return catch_err(_return_code, Stage::Remove, &mut self.errored_packages);
    }

    fn update(&mut self) -> PacketManagerResultCode {
        let (basic, sec) = (self.command_obj.basic_command.clone(), self.command_obj.update_command.clone());
        let (out_parser, err_parser) = self.get_performers(Stage::Update);

        let _return_code: PacketManagerResultCode = perform_command(basic, sec, Stage::Update, &Vec::new(), true, err_parser, out_parser);

        return catch_err(_return_code, Stage::Update, &mut self.errored_packages);
    }

    fn show_updates(&mut self) -> Vec<String> {
        let (basic, sec) = (self.command_obj.basic_command.clone(), self.command_obj.check_update_command.clone());
        let (out_parser, err_parser) = self.get_performers(Stage::Showing);

        let _return_code: PacketManagerResultCode = perform_command(basic, sec, Stage::Showing, &Vec::new(), true, err_parser, out_parser);

        let errored_packets: Rc<RefCell<Vec<ErroredPacket>>> = Rc::new(RefCell::new(vec![]));
        let status_code = catch_err(_return_code, Stage::Showing, &errored_packets);

        return match status_code {
            PacketManagerResultCode::Success(addons) => addons.unwrap_or(vec![]),
            _ => vec![]
        };
    }

    fn update_system(&mut self) -> PacketManagerResultCode {
        let (basic, sec) = (self.command_obj.basic_command.clone(), self.command_obj.system_update_command.clone());
        let (out_parser, err_parser) = self.get_performers(Stage::Update);

        let _return_code: PacketManagerResultCode = perform_command(basic, sec, Stage::Update, &vec![], true, err_parser, out_parser);

        return catch_err(_return_code, Stage::Update, &mut self.errored_packages);
    }

    fn search(&mut self, packets: Vec<String>) -> Vec<FoundPackage> {
        let (basic, sec) = (self.command_obj.basic_command.clone(), self.command_obj.search_command.clone());

        for packet in packets {
            let (out_parser, err_parser) = self.get_performers(Stage::Search);

            let _return_code: PacketManagerResultCode = perform_command(basic.clone(), sec.clone(), Stage::Search, &vec![packet], true, err_parser, out_parser);
        }

        return create_packages(&self.valid_lines, &self.package_index_step);
    }

    fn packets_list(&mut self) -> Vec<FoundPackage> {
        let (basic, sec) = (self.command_obj.basic_command.clone(), self.command_obj.list_command.clone());
        let (out_parser, err_parser) = self.get_performers(Stage::Search);

        let _return_code: PacketManagerResultCode = perform_command(basic, sec, Stage::Search, &vec![], true, err_parser, out_parser);

        return create_packages(&self.valid_lines, &self.package_index_step);
    }

    fn repos(&mut self) -> Vec<Repository> {
        let (basic, sec) = (self.command_obj.basic_command.clone(), self.command_obj.repo_list_command.clone());
        let (out_parser, err_parser) = self.get_performers(Stage::RepoList);

        let _return_code: PacketManagerResultCode = perform_command(basic, sec, Stage::RepoList, &vec![], true, err_parser, out_parser);

        return create_repo(&self.valid_lines, &self.repo_index_step, self.yes_pointer_str.clone().as_str());
    }

    fn add_repo(&mut self, repo_name: &str, repo_url: &str) {
        let (basic, sec) = (self.command_obj.basic_command.clone(), self.command_obj.repo_add_command.clone());
        let (out_parser, err_parser) = self.get_performers(Stage::AddRepo);

        let _return_code: PacketManagerResultCode = perform_command(basic, sec, Stage::AddRepo, &vec![repo_url.to_string(), repo_name.to_string()], true, err_parser, out_parser);
    }

    fn remove_repo(&mut self, repo_name: &str) {
        let (basic, sec) = (self.command_obj.basic_command.clone(), self.command_obj.repo_remove_command.clone());
        let (out_parser, err_parser) = self.get_performers(Stage::AddRepo);

        let _return_code: PacketManagerResultCode = perform_command(basic, sec, Stage::AddRepo, &vec![repo_name.to_string()], true, err_parser, out_parser);
    }

}