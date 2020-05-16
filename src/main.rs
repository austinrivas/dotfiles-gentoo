use dotfiles::shell::Command;
use std::time;
use std::thread;

fn main() {
    let cmd = String::from("echo");
    let args = vec!["hello"];

    let task = Command::new(&cmd, &args);
    let output = task.run();

    output.log();

    assert!(output.success());

    let cmd = String::from("ping");
    let args = vec!["-c 10", "www.google.com"];

    let stream = Command::new(&cmd, &args);
    let mut process = stream.stream();

    let duration = time::Duration::from_millis(5000);
    thread::sleep(duration);

    let result = process.kill();
    assert!(result.is_ok());
}