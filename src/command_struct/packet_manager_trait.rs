use crate::utility::FoundPackage;

use crate::utility::{PacketManagerResultCode, Repository, Stage};
use json::{JsonValue, stringify};

pub type ParserOutput = Box<dyn FnMut(&mut String) + 'static>;

pub trait PacketManager {
    fn get_performers(&mut self, stage: Stage) -> (Option<&mut ParserOutput>, Option<&mut ParserOutput>);

    fn install(&mut self, packets: &Vec<String>) -> PacketManagerResultCode;
    fn remove(&mut self, packets: &Vec<String>) -> PacketManagerResultCode;

    fn search(&mut self, packets: Vec<String>) -> Vec<FoundPackage>;
    fn packets_list(&mut self) -> Vec<FoundPackage>;

    fn update(&mut self) -> PacketManagerResultCode;
    fn show_updates(&mut self) -> Vec<String>;
    fn update_system(&mut self) -> PacketManagerResultCode;

    fn repos(&mut self) -> Vec<Repository>;
    fn add_repo(&mut self, repo_name: &str, repo_url: &str);
    fn remove_repo(&mut self, repo_name: &str);

    fn get_base_command_name(&self) -> String;
}

pub trait JsonTransformable {
    type ReturnType;

    fn from_json_obj(json_obj: JsonValue) -> Self::ReturnType;
    fn from_json(json_text: String) -> Option<Self::ReturnType> {
        let json_obj_result = json::parse(&json_text);

        if let Ok(json_obj) = json_obj_result
        {
            return Some(Self::from_json_obj(json_obj));
        }

        return None;
    }

    fn to_json(&self) -> JsonValue;

    fn to_json_str(&self) -> String {
        let json_obj = self.to_json();

        return stringify(json_obj);
    }
}

pub trait PackageNamed {
    fn get_name(&self) -> String;
}