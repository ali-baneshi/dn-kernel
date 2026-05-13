use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Default)]
pub struct WorkerRegistry {
    workers: HashMap<String, WorkerConfig>,
}

impl WorkerRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            workers: HashMap::new(),
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
}
