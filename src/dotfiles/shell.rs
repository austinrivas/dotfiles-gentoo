use nix;
use std::process;

#[derive(Debug)]
pub struct Output {
    output: process::Output,
    pub status: String,
    pub stdout: String,
    pub stderr: String
}

impl Output {
    fn new(output: &process::Output) -> Output {
        let output = output.clone();
        let status = output.status.to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Output {
            output,
            status,
            stdout,
            stderr
        }
    }

    pub fn log(&self) {
        println!("status: {}", self.status);
        println!("stdout:\n{}", self.stdout);
        println!("stderr:\n{}", self.stderr);
    }

    pub fn success(&self) -> bool {
        self.output.status.success()
    }
}

#[derive(Debug)]
pub struct Stream {
    id: i32,
    process: process::Child
}

impl Stream {
    fn new(process: process::Child) -> Stream {
        let id = process.id() as i32;
        Stream {
            id,
            process
        }
    }

    pub fn kill(&mut self) -> Result<(), nix::Error> {
        // send SIGINT to the child
        nix::sys::signal::kill(
            nix::unistd::Pid::from_raw(self.id as i32), 
            nix::sys::signal::Signal::SIGINT
        )
    }
}

#[derive(Debug)]
pub struct Command {
    pub cmd: String,
    pub args: Vec<String>,
    expect: String
}

impl Command {
    pub fn new(cmd: &str, args: &Vec<&str>) -> Command {
        let cmd = cmd.to_string();
        let args = args.iter().map(|&s| s.into()).collect();
        let expect = format!("failed to execute command {}", cmd);
        Command { 
            cmd,
            args, 
            expect
        }
    }

    pub fn run(&self) -> Output {
        let output = process::Command::new(&self.cmd)
            .args(&self.args)
            .output()
            .expect(&self.expect);

        Output::new(&output)
    }

    pub fn stream(&self) -> Stream {
        let process = process::Command::new(&self.cmd)
            .args(&self.args)
            .stdout(process::Stdio::inherit())
            .stderr(process::Stdio::inherit())
            .spawn()
            .expect(&self.expect);

        Stream::new(process)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_command() {
        let cmd = String::from("echo");
        let args = vec!["hello"];
        let task = Command::new(&cmd, &args);
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
        let task = Command::new(&cmd, &args);
        task.run();
    }

    #[test]
    fn start_stream() {
        let cmd = String::from("echo");
        let args = vec!["hello"];
        let command = Command::new(&cmd, &args);
        let mut stream = command.stream();
        let result = stream.kill();
        assert!(result.is_ok());
    }    
}