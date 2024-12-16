use std::io::{BufRead, BufReader};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::Duration;

use log::info;
use tokio::net::TcpStream;

use crate::utils::{get_free_port, get_repository_root};

const CONNECTION_ATTEMPTS: usize = 2000;
const CONNECTION_ATTEMPT_DELAY_MS: u64 = 1000;

#[derive(Debug)]
pub struct Program {
    pub url: String,
    pub port: String,

    process: Child,
}

impl Drop for Program {
    fn drop(&mut self) {
        let mut kill = Command::new("kill")
            .args(["-s", "TERM", &self.process.id().to_string()])
            .spawn()
            .expect("Failed to kill");
        kill.wait().expect("Failed to kill the process");
    }
}

impl Program {
    pub fn run(log_name: String, bin_name: &str, envs: Vec<(String, String)>) -> Self {
        let port = get_free_port();
        Self::run_with_port(log_name, bin_name, envs, port)
    }

    pub fn run_with_port(
        log_name: String,
        bin_name: &str,
        envs: Vec<(String, String)>,
        port: u16,
    ) -> Self {
        let repository_root = &get_repository_root();
        std::env::set_current_dir(repository_root).expect("Failed to set current directory");
        let url = "127.0.0.1".to_string();
        let port_str = format!("{}", port);

        let envs = [
            envs,
            vec![
                ("SERVER__URL".to_string(), url.clone()),
                ("SERVER__PORT".to_string(), port_str.clone()),
            ],
        ]
        .concat();

        let mut command = Command::new("cargo");
        command
            .args(["run"])
            .args(["--bin"])
            .args([bin_name])
            .current_dir(repository_root)
            .envs(envs)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut process = command.spawn().expect("Failed to start process");

        // Capture and print stdout
        let stdout = process.stdout.take().expect("Failed to capture stdout");

        let log_name_1 = log_name.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            reader.lines().for_each(|line| {
                if let Ok(line) = line {
                    info!("{} STDOUT: {}", &log_name_1, line);
                }
            });
        });

        // Capture and print stderr
        let stderr = process.stderr.take().expect("Failed to capture stderr");
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            reader.lines().for_each(|line| {
                if let Ok(line) = line {
                    info!("{} STDERR: {}", &log_name, line);
                }
            });
        });

        Self {
            url,
            port: port_str,
            process,
        }
    }

    pub fn has_exited(&mut self) -> Option<ExitStatus> {
        self.process.try_wait().expect("Failed to get exit status")
    }

    pub async fn wait_till_started(&mut self) {
        let mut attempts = CONNECTION_ATTEMPTS;
        loop {
            let addr = format!("{}:{}", &self.url, &self.port);
            match TcpStream::connect(&addr).await {
                Ok(_) => return,
                Err(err) => {
                    if let Some(status) = self.has_exited() {
                        panic!("Program exited early with {}", status);
                    }
                    if attempts == 0 {
                        panic!("Failed to connect to {}:{}: {}", &self.url, &self.port, err);
                    }
                }
            };

            attempts -= 1;
            tokio::time::sleep(Duration::from_millis(CONNECTION_ATTEMPT_DELAY_MS)).await;
        }
    }
}
