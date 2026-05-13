# IPC Protocol

The supervisor sends a single JSON request line to the worker over stdin.  
The worker returns a single JSON response line over stdout.

## Contract goals

- Simple process boundary
- Language-agnostic transport
- Strict schema validation
- Versioned protocol

## Current protocol

- `protocol_version = "1"`

## Notes

Both request and response objects should reject unknown properties in implementation.
