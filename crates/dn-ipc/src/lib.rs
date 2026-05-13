use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkerHello {
    pub protocol_version: String,
    pub worker_name: String,
    pub worker_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkerError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkerRequest {
    Hello,
    AnalyzeFile {
        path: String,
        language: Option<String>,
        content: String,
    },
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WorkerResponse {
    Hello {
        protocol_version: String,
        worker_name: String,
        worker_version: String,
    },
    Finding {
        path: String,
        severity: String,
        message: String,
    },
    Error {
        error: WorkerError,
    },
    Bye,
}
