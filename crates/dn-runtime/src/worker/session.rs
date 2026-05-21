use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use anyhow::{anyhow, Result};
use dn_ipc::{WorkerFinding, WorkerRequest, PROTOCOL_VERSION};

use crate::Finding;

pub struct WorkerSession {
    _child: Child,
    stdin: BufWriter<ChildStdin>,
    stdout: BufReader<ChildStdout>,
    handshake_done: bool,
    request_seq: u64,
}

impl WorkerSession {
    pub fn new(command: &str, args: &[String]) -> Result<Self> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow!("worker stdin unavailable"))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow!("worker stdout unavailable"))?;

        Ok(Self {
            _child: child,
            stdin: BufWriter::new(stdin),
            stdout: BufReader::new(stdout),
            handshake_done: false,
            request_seq: 0,
        })
    }

    pub fn analyze(
        &mut self,
        path: &str,
        language: Option<&str>,
        content: &str,
    ) -> Result<Vec<Finding>> {
        if !self.handshake_done {
            self.handshake()?;
            self.handshake_done = true;
        }

        self.request_seq = self.request_seq.saturating_add(1);
        let request_id = format!("scan-request-{}", self.request_seq);
        let request = WorkerRequest {
            protocol_version: PROTOCOL_VERSION.to_string(),
            request_id: request_id.clone(),
            method: "analyze_file".to_string(),
            params: dn_ipc::WorkerAnalyzeFileParams {
                path: path.to_string(),
                language: language.map(str::to_string),
                content: content.to_string(),
            },
        };

        let payload = serde_json::to_string(&request)?;
        self.stdin
            .write_all(format!("{}\n", payload).as_bytes())
            .map_err(|err| anyhow!("failed to write request: {err}"))?;
        self.stdin
            .flush()
            .map_err(|err| anyhow!("failed to flush request: {err}"))?;

        let mut line = String::new();
        self.stdout
            .read_line(&mut line)
            .map_err(|err| anyhow!("failed reading worker response: {err}"))?;

        if line.trim().is_empty() {
            return Err(anyhow!("empty worker response"));
        }

        let response: dn_ipc::WorkerResponse = serde_json::from_str(&line)
            .map_err(|err| anyhow!("failed parsing worker response: {err}"))?;
        if response.request_id != request_id {
            return Err(anyhow!(
                "unexpected worker response id: expected {}, got {}",
                request_id,
                response.request_id
            ));
        }

        if response.status != "ok" {
            return Err(anyhow!(response
                .error
                .unwrap_or_else(|| "worker returned non-ok status".to_string())));
        }

        Ok(response
            .findings
            .into_iter()
            .map(|finding: WorkerFinding| Finding {
                severity: finding.severity,
                rule: finding.rule,
                message: finding.message,
                category: finding.category,
                line: finding.line,
                source: Some("worker:python".to_string()),
            })
            .collect())
    }

    fn handshake(&mut self) -> Result<()> {
        let request = serde_json::json!({
            "protocol_version": PROTOCOL_VERSION,
            "request_id": "hello-request",
            "method": "hello",
            "params": {}
        });

        self.stdin
            .write_all(format!("{}\n", request).as_bytes())
            .map_err(|err| anyhow!("failed to send worker hello: {err}"))?;
        self.stdin
            .flush()
            .map_err(|err| anyhow!("failed to flush worker hello: {err}"))?;

        let mut line = String::new();
        self.stdout
            .read_line(&mut line)
            .map_err(|err| anyhow!("failed reading worker hello response: {err}"))?;

        let response: dn_ipc::WorkerResponse = serde_json::from_str(&line)
            .map_err(|err| anyhow!("failed parsing worker hello response: {err}"))?;

        if response.status != "ok" {
            return Err(anyhow!(
                "{}",
                response
                    .error
                    .unwrap_or_else(|| "worker hello failed".to_string())
            ));
        }

        Ok(())
    }
}
