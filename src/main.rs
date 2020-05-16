use nix;
use std::process::{
    Child,
    Command,
    Output, 
    Stdio
};
use std::time;
use std::thread;

#[derive(Debug)]
struct ShellOutput {
    output: Output,
    pub status: String,
    pub stdout: String,
    pub stderr: String
}

impl ShellOutput {
    fn new(output: &Output) -> ShellOutput {
        let output = output.clone();
        let status = output.status.to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        ShellOutput {
            output,
            status,
            stdout,
            stderr
        }
    }

    fn log(&self) {
        println!("status: {}", self.status);
        println!("stdout:\n{}", self.stdout);
        println!("stderr:\n{}", self.stderr);
    }

    fn success(&self) -> bool {
        self.output.status.success()
    }
}

#[derive(Debug)]
struct ShellStream {
    id: i32,
    process: Child
}

impl ShellStream {
    fn new(process: Child) -> ShellStream {
        let id = process.id() as i32;
        ShellStream {
            id,
            process
        }
    }

    fn kill(&mut self) -> Result<(), nix::Error> {
        // send SIGINT to the child
        nix::sys::signal::kill(
            nix::unistd::Pid::from_raw(self.id as i32), 
            nix::sys::signal::Signal::SIGINT
        )
    }
}

#[derive(Debug)]
struct ShellCommand {
    pub cmd: String,
    pub args: Vec<String>,
    expect: String
}

impl ShellCommand {
    fn new(cmd: &str, args: &Vec<&str>) -> ShellCommand {
        let cmd = cmd.to_string();
        let args = args.iter().map(|&s| s.into()).collect();
        let expect = format!("failed to execute command {}", cmd);
        ShellCommand { 
            cmd,
            args, 
            expect
        }
    }

    fn run(&self) -> ShellOutput {
        let output = Command::new(&self.cmd)
            .args(&self.args)
            .output()
            .expect(&self.expect);

        ShellOutput::new(&output)
    }

    fn stream(&self) -> ShellStream {
        let process = Command::new(&self.cmd)
            .args(&self.args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect(&self.expect);

        ShellStream::new(process)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_command() {
        let cmd = String::from("echo");
        let args = vec!["hello"];
        let task = ShellCommand::new(&cmd, &args);
        let output = task.run();
        assert!(output.success());
        assert_eq!(output.status, "exit code: 0");
        assert_eq!(output.stdout, "hello\n");
        assert_eq!(output.stderr, "");
    }

    #[test]
    #[should_panic(expected = "failed to execute command not a command: Os { code: 2, kind: NotFound, message: \"No such file or directory\" }")]
    fn fail_to_run() {
        let cmd = String::from("not a command");
        let args = Vec::new();
        let task = ShellCommand::new(&cmd, &args);
        task.run();
    }

    #[test]
    fn start_stream() {
        let cmd = String::from("echo");
        let args = vec!["hello"];
        let command = ShellCommand::new(&cmd, &args);
        let mut stream = command.stream();
        let result = stream.kill();
        assert!(result.is_ok());
    }    
}

fn main() {
    let cmd = String::from("echo");
    let args = vec!["hello"];

    let task = ShellCommand::new(&cmd, &args);
    let output = task.run();

    output.log();

    assert!(output.success());

    let cmd = String::from("ping");
    let args = vec!["-c 10", "www.google.com"];

    let stream = ShellCommand::new(&cmd, &args);
    let mut process = stream.stream();

    let duration = time::Duration::from_millis(5000);
    thread::sleep(duration);

    let result = process.kill();
    assert!(result.is_ok());
}