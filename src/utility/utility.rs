use std::rc::Rc;
use std::fmt::Display;
use std::cell::RefCell;

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