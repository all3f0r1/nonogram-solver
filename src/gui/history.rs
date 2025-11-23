use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub file_path: String,
    pub timestamp: String,
    pub solver_mode: String,
    pub deductions_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    pub entries: Vec<HistoryEntry>,
}

impl History {
    const MAX_ENTRIES: usize = 20;
    
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    
    pub fn load() -> Result<Self> {
        let history_path = Self::get_history_path()?;
        
        if history_path.exists() {
            let content = fs::read_to_string(&history_path)?;
            let history: History = serde_json::from_str(&content)?;
            Ok(history)
        } else {
            Ok(Self::new())
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let history_path = Self::get_history_path()?;
        
        // Créer le répertoire parent si nécessaire
        if let Some(parent) = history_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&history_path, content)?;
        
        Ok(())
    }
    
    pub fn add_entry(&mut self, entry: HistoryEntry) {
        // Ajouter au début
        self.entries.insert(0, entry);
        
        // Limiter à MAX_ENTRIES
        if self.entries.len() > Self::MAX_ENTRIES {
            self.entries.truncate(Self::MAX_ENTRIES);
        }
    }
    
    pub fn get_recent(&self, count: usize) -> Vec<HistoryEntry> {
        self.entries.iter().take(count).cloned().collect()
    }
    
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    
    fn get_history_path() -> Result<PathBuf> {
        let mut path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Impossible de trouver le répertoire de configuration"))?;
        
        path.push("nonogram-solver");
        path.push("history.json");
        
        Ok(path)
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
