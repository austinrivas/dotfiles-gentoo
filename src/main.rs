use dirs;
use std::fs;
use std::path::Path;

mod dotfiles;
use dotfiles::shell::Command;
use dotfiles::file::Asset;

fn main() { 
    let test_sh = Asset::get("test.sh").unwrap();
    println!("{:?}", std::str::from_utf8(test_sh.as_ref()));

    for file in Asset::iter() {
        println!("{}", file.as_ref());
    }

    let mut home_path = match dirs::home_dir() {
        Some(path) => path,
        None => panic!("home_dir is not defined in this environment")
    };

    home_path.push("some/dir");

    fs::create_dir_all(&home_path)
        .expect("could not create dir");

    println!("created directory: {:?}", &home_path);

    assert!(Path::new(&home_path).exists());

    let pacman = String::from("pacman");

    let sync_package_db = Command::new(
        &pacman,
        &vec!["-Sy"] 
    );

    let install_git = Command::new(
        &pacman,
        &vec!["-S", "git", "--noconfirm"]
    );

    println!("Syncronizing package db.");

    let output = sync_package_db.run();

    output.log();

    assert!(output.success());

    println!("Installing git.");

    let output = install_git.run();

    output.log();

    assert!(output.success());
}