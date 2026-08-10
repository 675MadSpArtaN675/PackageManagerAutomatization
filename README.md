# Linux packet manager automatization lib

This project is a method to give you a universal API for many linux packet managers, such as apt, dnf, zypper, etc.
You can installing packages, update them, add and removing repositories of packages.
By the lib you can get info about packages and repositories, their list on your OS

Example:

```rust
use package_manager_automatic::command_struct::packet_manager_trait::PacketManager;
use package_manager_automatic::PacketManagerCommandExecutor;

fn main() {
    let mut executor = PacketManagerCommandExecutor::new("zypper".to_string());

    let vector_collection = vec![String::from("vim"), String::from("gvim")];
    let vector_collection_1 = vec![String::from("vim"), String::from("gvim"), "modiar".to_string(), "AlbusDombalor".to_string()];
    let vector_collection_2 = vec![String::from("vim"), String::from("gvim"), "modiar".to_string(), "AlbusDombalor".to_string(),"libopenh264-8".to_string()];

    executor.install(&vector_collection); // Package installing
    executor.install(&vector_collection_1);

    executor.remove(&vector_collection_1); // Package removing

    // Packages searching
    for package in executor.search(vector_collection_1) {
        println!("Search package: {:?}", package);
    }

    // Installed packages
    for package in executor.packets_list() {
        println!("Package: {:?}", package);
    }

    // Get list of repositories on PC
    for repo in executor.repos() {
        println!("Repo: {:?}", repo);
    }

    // Show no installed and fresh updates
    executor.show_updates();

    // Updating packages
    executor.update();

    // Updating system packages
    executor.update_system();
}

```
