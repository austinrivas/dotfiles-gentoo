use dirs;
use std::fs;
use std::path::Path;

mod dotfiles;
use dotfiles::shell::Command;

fn create_dir() {
    let mut home_path = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("home_dir is not defined in this environment")
    };

    home_path.push("some/dir");

    fs::create_dir_all(&home_path)
        .expect("could not create dir");

    println!("created directory: {:?}", &home_path);

    assert!(Path::new(&home_path).exists());
}

fn sync_package_repos(package_manager: &str) {
    let sync_package_db = Command::new(
        package_manager,
        &vec!["-Sy"] 
    );

    println!("Syncronizing package db.");

    let output = sync_package_db.run();

    output.log();

    assert!(output.success());
}

fn install_git(package_manager: &str) {
    let install_git = Command::new(
        package_manager,
        &vec!["-S", "git", "--noconfirm"]
    );

    println!("Installing git.");

    let output = install_git.run();

    output.log();

    assert!(output.success());
}

fn main() {
    create_dir();

    let package_manager = String::from("pacman");

    sync_package_repos(&package_manager);

    install_git(&package_manager);
}