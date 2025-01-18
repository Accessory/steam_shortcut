use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub(crate) struct DesktopFile {
    pub name: String,
    pub icon: String,
    pub exec: String,
}

impl DesktopFile {
    pub(crate) fn to_exec(&self) -> String {
        if let Some(at) = self.exec.find(r#""run""#) {
            return self.exec[at..].to_string();
        }
        match self.exec.find("run") {
            None => self.exec.clone(),
            Some(at) => self.exec[at..].to_string(),
        }
    }

    pub fn try_from_path(path: &Path) -> Option<Self> {
        let mut name = String::new();
        let mut icon = String::new();
        let mut exec = String::new();

        let file = File::open(path).ok()?;
        // Parse
        let reader = BufReader::new(file);
        for line_result in reader.lines() {
            let tmp = line_result.ok()?;
            let line = tmp.trim();
            if let Some((key, value)) = line.split_once("=") {
                match key {
                    "Name" => {
                        if name.is_empty() {
                            name = value.to_string()
                        }
                    }
                    "Icon" => {
                        if icon.is_empty() {
                            icon = value.to_string()
                        }
                    }
                    "Exec" => {
                        if exec.is_empty() {
                            exec = value.to_string()
                        }
                    }
                    &_ => {}
                }
            }
        }

        Some(Self { name, icon, exec })
    }
}
