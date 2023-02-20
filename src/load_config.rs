use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

pub struct Config {
    pub app_path: String,
    pub name: String,
}

pub struct LoadConfig {}

impl LoadConfig {
    pub fn load_buttons() -> Vec<Config> {
        let current_dir = env::current_dir().unwrap();

        let mut file_path = String::from("assets/config.csv");

        if current_dir.to_string_lossy() == "/" {
            let output = Command::new("ps")
                .arg("aux")
                .output()
                .expect("Failed to list processes to get app bundle");

            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.lines().collect();

            let mut last_entry = "".to_owned();
            for line in lines {
                if line.contains("Switcher.app/Contents/MacOS/switcher") {
                    last_entry = line
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .to_owned()
                        .replace("MacOS/switcher", "Resources/");
                }
            }

            file_path = last_entry + &file_path;
        }

        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(file_path.clone())
        {
            let lines = [
                "/Applications/Firefox.app,Firefox",
                "/Applications/Google Chrome.app,Chrome",
                "/Applications/IntelliJ IDEA CE.app,Idea",
                "/Applications/iTerm.app,iTerm",
            ];

            for line in lines {
                writeln!(file, "{}", line).unwrap();
            }
        }

        let file = File::open(file_path).expect("Could not open config.csv");

        let reader = BufReader::new(file);

        let mut configs = vec![];

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            let values: Vec<&str> = line.split(',').collect();

            if line.trim().is_empty() {
                continue;
            }

            configs.push(Config {
                app_path: values.first().unwrap().to_string(),
                name: values.get(1).unwrap().to_string(),
            });
        }

        configs
    }
}
