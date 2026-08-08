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