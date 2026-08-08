mod utility;
pub mod command_struct;

mod parsers;
mod packet_manager;

pub use packet_manager::PacketManagerCommandExecutor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
