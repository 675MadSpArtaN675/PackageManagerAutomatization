use std::rc::Rc;
use std::fmt::Display;
use std::cell::RefCell;

use json::{JsonValue, object};
use crate::command_struct::packet_manager_trait::{JsonTransformable, PackageNamed};

use crate::utility::constants::*;

pub enum PacketManagerResultCode {
    Success(Option<Vec<String>>),
    Error(String, i32, Vec<String>)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stage {
    Install,
    Remove,
    Update,
    Search,
    Showing,
    RepoList,
    AddRepo,
    RemoveRepo
}

impl Stage {
    pub fn from_str(name: String) -> Stage {
        let result: Stage = match name.as_str() {
            "Install" => Stage::Install,
            "Update" => Stage::Update,
            "Remove" => Stage::Remove,
            "Search" => Stage::Search,
            "Showing" => Stage::Showing,
            "RepoList" => Stage::RepoList,
            "AddRepo" => Stage::AddRepo,
            "RemoveRepo" => Stage::RemoveRepo,

            _ => Stage::Install
        };

        return result;
    }

    pub fn to_string(&self) -> &str {
        let result: &str = match &self {
            Stage::Install => "Install",
            Stage::Update => "Update",
            Stage::Remove => "Remove",
            Stage::Search => "Search",
            Stage::Showing => "Showing",
            Stage::RepoList => "RepoList",
            Stage::AddRepo => "AddRepo",
            Stage::RemoveRepo => "RemoveRepo"
        };

        return result;
    }

    pub fn to_string_obj(&self) -> String {
        return String::from(self.to_string());
    }
}

impl Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone, Copy)]
pub enum InstallFlag {
    Installed,
    InstalledPlus,
    NoInstalled
}

impl InstallFlag {
    pub fn to_string(self) -> String {
        return match self {
            InstallFlag::Installed => "i".to_string(),
            InstallFlag::InstalledPlus => "i+".to_string(),
            InstallFlag::NoInstalled => "n".to_string(),
        };
    }
    pub fn str_to_enum(line: &str) -> InstallFlag {
        return match line {
            "i" => InstallFlag::Installed,
            "i+" => InstallFlag::InstalledPlus,
            "n" => InstallFlag::NoInstalled,

            _ => InstallFlag::NoInstalled
        };
    }
}

pub struct ErroredPacket {
    pub name: String,
    pub stage: Stage
}

pub struct FoundPackage {
    pub name: String,
    pub description: String,
    pub install_flag: InstallFlag,
    pub type_of_packet: String
}

pub struct Repository {
    pub alias: String,
    pub enabled_status: bool,
    pub uri: String
}

impl JsonTransformable for FoundPackage {
    type ReturnType = FoundPackage;

    fn from_json_obj(json_obj: JsonValue) -> Self::ReturnType {
        return FoundPackage {
            name: json_obj["name"].as_str().unwrap().to_string(),
            description: json_obj["description"].as_str().unwrap().to_string(),
            install_flag: InstallFlag::str_to_enum(json_obj["install_flag"].as_str().unwrap()),
            type_of_packet: json_obj["type_of_package"].as_str().unwrap().to_string()
        };
    }

    fn to_json(&self) -> JsonValue {
        return object! { name: self.name.clone(), description: self.description.clone(), install_flag: self.install_flag.to_string(), type_of_package: self.type_of_packet.clone()};
    }
}

impl JsonTransformable for ErroredPacket {
    type ReturnType = ErroredPacket;

    fn from_json_obj(json_obj: JsonValue) -> Self::ReturnType {
        return ErroredPacket {
            name: json_obj["name"].as_str().unwrap().to_string(),
            stage: Stage::from_str(json_obj["stage"].as_str().unwrap().to_string())
        };
    }

    fn to_json(&self) -> JsonValue {
        return object! { name: self.name.clone(), stage: self.stage.to_string_obj()};
    }
}

impl JsonTransformable for Repository {
    type ReturnType = Repository;

    fn from_json_obj(json_obj: JsonValue) -> Self::ReturnType {
        return Repository {
            alias: json_obj["alias"].as_str().unwrap().to_string(),
            enabled_status: json_obj["enabled_status"].as_bool().unwrap(),
            uri: json_obj["uri"].as_str().unwrap().to_string()
        };
    }

    fn to_json(&self) -> JsonValue {
        return object! { alias: self.alias.clone(), enabled_status: self.enabled_status, uri: self.uri.clone()};
    }
}

impl PackageNamed for FoundPackage {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl PackageNamed for ErroredPacket {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}

impl PackageNamed for Repository {
    fn get_name(&self) -> String {
        return self.alias.clone();
    }
}

impl Clone for ErroredPacket {
    fn clone(&self) -> Self {
        return ErroredPacket { name: self.name.clone(), stage: self.stage.clone() };
    }
}

impl Clone for FoundPackage {
    fn clone(&self) -> Self {
        return FoundPackage { name: self.name.clone(), description: self.description.clone(), install_flag: self.install_flag.clone(), type_of_packet: self.type_of_packet.clone() };
    }
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        return Repository { alias: self.alias.clone(), enabled_status: self.enabled_status.clone(), uri: self.uri.clone() };
    }
}

impl std::fmt::Debug for FoundPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Name: {}; Description: {}; InstallFlag: {}; Type: {})", self.name, self.description, self.install_flag.to_string(), self.type_of_packet)
    }
}

impl std::fmt::Debug for Repository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Alias: {}; URI: {}; Status: {})", self.alias, self.uri, self.enabled_status)
    }
}

pub fn create_packages(valid_lines: &Rc<RefCell<Vec<String>>>, column_step: &[i32; PACKAGE_SIZE]) -> Vec<FoundPackage> {
    let mut packages: Vec<FoundPackage> = vec![];
    let mut ref_to_lines = valid_lines.borrow_mut();

    for parsed_line in ref_to_lines.iter().map(|line| line.split("|").map(|s| s.to_string())) {
        let line: Vec<String> = parsed_line.collect();

        packages.push(
            FoundPackage {
                name: line[column_step[0] as usize].clone(),
                description: line[column_step[1] as usize].clone(),
                install_flag: InstallFlag::str_to_enum(line[column_step[2] as usize].clone().as_str()),
                type_of_packet: line[column_step[3] as usize].clone()
            }
        );
    }

    ref_to_lines.clear();

    return packages;
}

pub fn create_repo(valid_lines: &Rc<RefCell<Vec<String>>>, column_step: &[i32; REPO_SIZE], yes_pointer: &str) -> Vec<Repository> {
    let mut repos: Vec<Repository> = vec![];
    let mut ref_to_lines = valid_lines.borrow_mut();

    for parsed_line in ref_to_lines.iter().map(|line| line.split("|").map(|s| s.to_string())) {
        let line: Vec<String> = parsed_line.collect();

        let status = line[column_step[1] as usize].clone();
        let mut yes_no: bool = false;
        for pointer in yes_pointer.split("|") {
            if status.contains(pointer) {
                yes_no = true;
                break;
            }
        }

        repos.push(
            Repository {
                alias: line[column_step[0] as usize].clone(),
                enabled_status: yes_no,
                uri: line[column_step[2] as usize].clone()
            }
        );
    }

    ref_to_lines.clear();

    return repos;
}