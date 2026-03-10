use std::{env, path::Path, process::Command, str::from_utf8};

pub struct Placer {
    bin_path: String,
}

impl Placer {
    pub fn new() -> Result<Placer, Box<dyn std::error::Error>> {
        let bin_path =
            env::var("DISPLAYPLACER_PATH").unwrap_or_else(|_| "/opt/homebrew/bin/displayplacer".to_string());
        if !Path::new(&bin_path).exists() {
            return Err(format!(
                "displayplacer not found at path: {bin_path}\nrun `brew install displayplacer` to install"
            ).into());
        }
        Ok(Placer { bin_path })
    }

    fn list(&self) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new(&self.bin_path)
            .arg("list")
            .output()?;
        Ok(String::from_utf8(output.stdout)?)
    }

    pub fn current(&self) -> Result<String, Box<dyn std::error::Error>> {
        let list = self.list()?;
        let last_line = list.lines()
            .last()
            .ok_or("No output from displayplacer")?
            .to_string();
        Ok(last_line
            .split_whitespace()
            .skip(1)
            .collect::<Vec<&str>>()
            .join(" "))
    }

    pub fn set(&self, places: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        let output = std::process::Command::new(&self.bin_path)
            .args(places)
            .output()?;
        if !output.status.success() {
            return Err(format!(
                "failed to set display places: {}",
                from_utf8(&output.stderr)?
            ).into());
        }
        println!("{:?}", from_utf8(&output.stdout)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() -> Result<(), Box<dyn std::error::Error>> {
        let placer = Placer::new()?;
        let list = placer.list()?;
        assert!(list.contains("Persistent screen id:"));
        Ok(())
    }
}
