use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct Registry {
    names: Vec<String>,
    path: PathBuf,
}

impl Registry {
    pub fn load() -> Result<Self> {
        let path = registry_path()?;
        let names = if path.exists() {
            let content = std::fs::read_to_string(&path)
                .with_context(|| format!("failed to read {}", path.display()))?;
            serde_json::from_str(&content)
                .with_context(|| format!("failed to parse {}", path.display()))?
        } else {
            vec![]
        };
        Ok(Self { names, path })
    }

    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.names)?;
        std::fs::write(&self.path, content)
            .with_context(|| format!("failed to write {}", self.path.display()))
    }

    pub fn names(&self) -> &[String] {
        &self.names
    }

    pub fn add(&mut self, name: String) {
        if !self.names.contains(&name) {
            self.names.push(name);
        }
    }

    pub fn remove(&mut self, name: &str) {
        self.names.retain(|n| n != name);
    }
}

fn registry_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("cannot determine home directory")?;
    Ok(home.join(".scrts.json"))
}
