use crate::utility::FoundPackage;

use crate::utility::{PacketManagerResultCode, Repository, Stage};

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
}