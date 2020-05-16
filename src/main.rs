use std::process::{
    Command, 
    Output, 
    Stdio
};

#[derive(Debug)]
struct ShellOutput {
    pub cmd: String,
    output: Output,
    pub status: String,
    pub stdout: String,
    pub stderr: String
}

impl ShellOutput {
    fn new(cmd: &str, output: &Output) -> ShellOutput {
        let cmd = cmd.to_string();
        let output = output.clone();
        let status = output.status.to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        ShellOutput {
            cmd,
            output,
            status,
            stdout,
            stderr
        }
    }

    fn log(&self) {
        println!("command: {}", self.cmd);
        println!("status: {}", self.status);
        println!("stdout:\n{}", self.stdout);
        println!("stderr:\n{}", self.stderr);
    }

    fn success(&self) -> bool {
        self.output.status.success()
    }
}

#[derive(Debug)]
struct ShellCommand {
    cmd: String,
    expect: String
}

impl ShellCommand {
    fn new(cmd: &str, expect: &str) -> ShellCommand {
        let cmd = cmd.to_string();
        let expect = expect.to_string();
        ShellCommand { 
            cmd, 
            expect
        }
    }

    fn run(&self) -> ShellOutput {
        let mut handle: Command;
        if cfg!(target_os = "windows") {
            handle = Command::new("cmd");
            handle.arg("/C");
        } else {
            handle = Command::new("sh");
            handle.arg("-c");
        }

        let output = handle
            .arg(&self.cmd)
            .output()
            .expect(&self.expect);

        ShellOutput::new(&self.cmd, &output)
    }

    fn stream(&self) {
        let mut handle: Command;
        if cfg!(target_os = "windows") {
            handle = Command::new("cmd");
            handle.arg("/C");
        } else {
            handle = Command::new("sh");
            handle.arg("-c");
        }

        handle
            .arg(&self.cmd)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
    }
}

fn main() {
    let cmd = String::from("echo hello");
    let exp = String::from("failed to execute process");

    let task = ShellCommand::new(&cmd, &exp);
    let output = task.run();

    output.log();

    assert!(output.success());

    let cmd = String::from("htop");
    let exp = String::from("failed to execute stream");

    let stream = ShellCommand::new(&cmd, &exp);

    stream.stream();
}