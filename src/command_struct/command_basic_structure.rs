use json::object;
use crate::command_struct::packet_manager_trait::JsonTransformable;

pub struct PacketManagerCommand {
    pub basic_command: String,
    pub install_command: String,
    pub remove_command: String,
    pub search_command: String,
    pub list_command: String,
    pub check_update_command: String,
    pub update_command: String,
    pub system_update_command: String,
    pub repo_add_command: String,
    pub repo_remove_command: String,
    pub repo_list_command: String
}

impl PacketManagerCommand {
    pub fn new_empty(base_command_name: String) -> PacketManagerCommand {
        let command_obj: PacketManagerCommand = PacketManagerCommand{
            basic_command: base_command_name,
            install_command: String::new(),
            remove_command: String::new(),
            search_command: String::new(),
            list_command: String::new(),
            check_update_command: String::new(),
            update_command: String::new(),
            system_update_command: String::new(),
            repo_add_command: String::new(),
            repo_remove_command: String::new(),
            repo_list_command: String::new()
        };

        return command_obj;
    }

    pub fn new(base_command_name: String) -> PacketManagerCommand {
        return get_packet_manager_preset(base_command_name);
    }
}

fn get_packet_manager_preset(base_command_name: String) -> PacketManagerCommand {
    let mut command_obj: PacketManagerCommand = PacketManagerCommand::new_empty(base_command_name.clone());

    match base_command_name.as_str() {
        "zypper" => {
            create_standard_commands(&mut command_obj);
            command_obj.install_command += " -y";
            command_obj.remove_command += " -y";
            command_obj.list_command = "search --installed-only".to_string();
            command_obj.update_command += " -y";
            command_obj.check_update_command = String::from("refresh");
            command_obj.system_update_command = String::from("dist-upgrade -y");
            command_obj.repo_list_command = "repos -U".to_string();
            command_obj.repo_add_command = "addrepo".to_string();
            command_obj.repo_remove_command = "removerepo".to_string();
        },
        "dnf" => {
            create_standard_commands(&mut command_obj);
            command_obj.list_command = "list --installed".to_string();
            command_obj.update_command = String::from("upgrade");
            command_obj.check_update_command = String::from("check");
        },
        "apt" => {
            create_standard_commands(&mut command_obj);
            command_obj.list_command = "list installed".to_string();
        }

        _ => {},
    };

    return command_obj;
}

fn create_standard_commands(command_obj: &mut PacketManagerCommand) {
    command_obj.install_command = String::from("install");
    command_obj.remove_command = String::from("remove");
    command_obj.search_command = String::from("search");
    command_obj.update_command = String::from("update");
    command_obj.check_update_command = String::from("upgrade");
}

impl JsonTransformable for PacketManagerCommand {
    type ReturnType = PacketManagerCommand;

    fn from_json_obj(json_obj: json::JsonValue) -> Self::ReturnType {
        return PacketManagerCommand {
            basic_command: json_obj["basic_command"].as_str().unwrap().to_string(),
            install_command: json_obj["install_command"].as_str().unwrap().to_string(),
            remove_command: json_obj["remove_command"].as_str().unwrap().to_string(),
            search_command: json_obj["search_command"].as_str().unwrap().to_string(),
            list_command: json_obj["list_command"].as_str().unwrap().to_string(),
            check_update_command: json_obj["check_update_command"].as_str().unwrap().to_string(),
            update_command: json_obj["update_command"].as_str().unwrap().to_string(),
            system_update_command: json_obj["system_update_command"].as_str().unwrap().to_string(),
            repo_add_command: json_obj["repo_add_command"].as_str().unwrap().to_string(),
            repo_remove_command: json_obj["repo_remove_command"].as_str().unwrap().to_string(),
            repo_list_command: json_obj["repo_list_command"].as_str().unwrap().to_string()
        };
    }

    fn to_json(&self) -> json::JsonValue {
        let json_obj = object! {
            basic_command: self.basic_command.clone(),
            install_command: self.install_command.clone(),
            remove_command: self.remove_command.clone(),
            search_command: self.search_command.clone(),
            list_command: self.list_command.clone(),
            check_update_command: self.check_update_command.clone(),
            update_command: self.update_command.clone(),
            system_update_command: self.system_update_command.clone(),
            repo_add_command: self.repo_add_command.clone(),
            repo_remove_command: self.repo_remove_command.clone(),
            repo_list_command: self.repo_list_command.clone()
        };

        return json_obj;
    }
}