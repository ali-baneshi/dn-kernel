import ast
import json
import sys


def analyze(path, content):
    findings = []

    try:
        tree = ast.parse(content)
    except Exception:
        return findings

    for node in ast.walk(tree):

        if isinstance(node, ast.Call):
            if getattr(node.func, "id", None) == "eval":
                findings.append({
                    "rule": "python-eval-usage",
                    "severity": "high",
                    "message": "Use of eval() detected",
                    "line": node.lineno,
                })

            if getattr(node.func, "id", None) == "print":
                findings.append({
                    "rule": "debug-print",
                    "severity": "low",
                    "message": "print() detected",
                    "line": node.lineno,
                })

        if isinstance(node, ast.Import):
            for name in node.names:
                if name.name == "pdb":
                    findings.append({
                        "rule": "debugger-import",
                        "severity": "medium",
                        "message": "pdb imported",
                        "line": node.lineno,
                    })

    return findings


for line in sys.stdin:
    line = line.strip()

    if not line:
        continue

    req = json.loads(line)

    result = analyze(
        req.get("path", ""),
        req.get("content", "")
    )

    print(json.dumps(result), flush=True)
