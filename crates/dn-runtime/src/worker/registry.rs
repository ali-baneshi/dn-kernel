use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::worker::session::WorkerSession;
use crate::Finding;

#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub command: String,
    pub args: Vec<String>,
}

pub struct WorkerRegistry {
    workers: HashMap<String, WorkerConfig>,
    sessions: HashMap<String, WorkerSession>,
}

impl WorkerRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            workers: HashMap::new(),
            sessions: HashMap::new(),
        };

        // default python worker
        registry.register(
            "python",
            WorkerConfig {
                command: "python".to_string(),
                args: vec!["-m".to_string(), "workers.python.dn_worker".to_string()],
            },
        );

        registry
    }

    pub fn register(&mut self, language: &str, config: WorkerConfig) {
        self.workers.insert(language.to_string(), config);
    }

    pub fn get(&self, language: &str) -> Option<&WorkerConfig> {
        self.workers.get(language)
    }

    pub fn supports(&self, language: &str) -> bool {
        self.workers.contains_key(language)
    }

    pub fn analyze(&mut self, language: &str, path: &str, content: &str) -> Result<Vec<Finding>> {
        if !self.sessions.contains_key(language) {
            let config = self
                .workers
                .get(language)
                .ok_or_else(|| anyhow!("no worker registered for language: {}", language))?
                .clone();

            let session = WorkerSession::new(&config.command, &config.args)?;
            self.sessions.insert(language.to_string(), session);
        }

        let session = self
            .sessions
            .get_mut(language)
            .ok_or_else(|| anyhow!("worker session unavailable for language: {}", language))?;

        session.analyze(path, content)
    }
}
