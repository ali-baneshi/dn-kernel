import json
import sys


def ready_payload():
    return {
        "protocol_version": "0.1.0",
        "status": "ready",
        "worker": "python",
        "worker_version": "0.1.0",
    }


def analyze_file(request):
    params = request.get("params", {})
    path = params.get("path", "")
    language = params.get("language")
    content = params.get("content", "")

    findings = []

    if language == "python":
        if "print(" in content:
            findings.append(
                {
                    "severity": "info",
                    "rule": "python-print",
                    "message": "print statement detected",
                }
            )

        if "eval(" in content:
            findings.append(
                {
                    "severity": "high",
                    "rule": "python-eval",
                    "message": "eval usage detected",
                }
            )

        if "import pdb" in content or "breakpoint(" in content:
            findings.append(
                {
                    "severity": "warning",
                    "rule": "python-debugger",
                    "message": "debugger usage detected",
                }
            )

    return {
        "protocol_version": request.get("protocol_version", "0.1.0"),
        "request_id": request.get("request_id", ""),
        "status": "ok",
        "findings": findings,
    }


def main():
    raw = sys.stdin.read()

    if not raw.strip():
        print(json.dumps(ready_payload()))
        return

    try:
        request = json.loads(raw)
    except json.JSONDecodeError as exc:
        print(
            json.dumps(
                {
                    "protocol_version": "0.1.0",
                    "request_id": "",
                    "status": "error",
                    "error": f"invalid json: {exc}",
                    "findings": [],
                }
            )
        )
        return

    method = request.get("method")

    if method == "analyze_file":
        print(json.dumps(analyze_file(request)))
        return

    print(
        json.dumps(
            {
                "protocol_version": request.get("protocol_version", "0.1.0"),
                "request_id": request.get("request_id", ""),
                "status": "error",
                "error": f"unsupported method: {method}",
                "findings": [],
            }
        )
    )


if __name__ == "__main__":
    main()
