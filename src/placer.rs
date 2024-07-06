use std::{env, path::Path};

pub struct Placer {
    bin_path: String,
}

impl Placer {
    pub fn new() -> Placer {
        let bin_path =
            env::var("DISPLAYPLACER_PATH").unwrap_or("/opt/homebrew/bin/displayplacer".to_string());
        if !Path::new(&bin_path).exists() {
            panic!(
                "displayplacer not found at path: {}\nrun `brew install displayplacer` to install",
                bin_path
            );
        }
        Placer { bin_path }
    }

    fn list(&self) -> String {
        let output = std::process::Command::new(&self.bin_path)
            .arg("list")
            .output()
            .expect("failed to execute displayplacer");
        String::from_utf8(output.stdout).unwrap()
    }

    pub fn current(&self) -> String {
        let list = self.list();
        list.lines()
            .last()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .join(" ")
    }

    pub fn set(&self, place: &str) {
        std::process::Command::new(&self.bin_path)
            .arg(place)
            .output()
            .expect("failed to execute displayplacer");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let placer = Placer::new();
        let list = placer.list();
        assert!(list.contains("Persistent screen id:"));
    }

    #[test]
    fn test_current_and_set() {
        let placer = Placer::new();
        let current = placer.current();
        assert!(current.contains(r#""id:"#));
    }

    #[test]
    fn test_set() {
        let placer = Placer::new();
        let current = placer.current();
        placer.set(&current);
        assert_eq!(current, placer.current());
    }
}
