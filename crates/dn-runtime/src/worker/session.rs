use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use anyhow::{anyhow, Result};

use crate::Finding;

pub struct WorkerSession {
    _child: Child,
    stdin: BufWriter<ChildStdin>,
    stdout: BufReader<ChildStdout>,
}

impl WorkerSession {
    pub fn new(command: &str, args: &[String]) -> Result<Self> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
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
        })
    }

    pub fn analyze(&mut self, path: &str, content: &str) -> Result<Vec<Finding>> {
        let request = serde_json::json!({
            "path": path,
            "content": content
        });

        writeln!(self.stdin, "{}", request)?;
        self.stdin.flush()?;

        let mut line = String::new();
        self.stdout.read_line(&mut line)?;

        let findings: Vec<Finding> = serde_json::from_str(&line)?;

        Ok(findings)
    }
}
