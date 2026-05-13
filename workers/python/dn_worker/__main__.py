import json
import sys

PROTOCOL_VERSION = "0.1.0"


def main() -> None:
    message = {
        "protocol_version": PROTOCOL_VERSION,
        "status": "ready",
        "worker": "python",
        "worker_version": "0.1.0",
    }

    sys.stdout.write(json.dumps(message) + "\n")
    sys.stdout.flush()


if __name__ == "__main__":
    main()
