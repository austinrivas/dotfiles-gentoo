use dirs;
use std::fs;
use std::time;
use std::thread;
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

    let cmd = String::from("ping");
    let args = vec!["www.google.com"];

    let stream = Command::new(&cmd, &args);
    let mut process = stream.stream();

    let duration = time::Duration::from_millis(5000);
    thread::sleep(duration);

    let result = process.kill();
    assert!(result.is_ok());

    let cmd = String::from("echo");
    let args = vec!["hello"];

    let task = Command::new(&cmd, &args);
    let output = task.run();

    output.log();

    assert!(output.success());
}