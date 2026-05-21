## Workers

`dn-kernel` supports multiple language-specific workers for code analysis:

### Python Worker
Located in `workers/python/`, provides Python-specific code analysis.

### C Worker
Located in `workers/c/`, provides Linux kernel coding style checks based on checkpatch.pl rules.

**Features:**
- Offline analysis (no network dependencies)
- Fast text-based parsing
- Implements 5 key kernel coding style rules:
  1. **line-length**: Lines should not exceed 80 characters
  2. **space-before-tab**: No spaces before tabs in indentation
  3. **trailing-whitespace**: No trailing whitespace at end of lines
  4. **keyword-spacing**: Space required after keywords (if, for, while, switch)
  5. **brace-style**: Opening braces on same line (K&R style)

**Building:**
```bash
cd workers/c
make
./dn-worker-c <file.c>
```

**Profile:**
Use the `kernel-c` profile for C code analysis:
```bash
dn-cli scan . --profile kernel-c
```

See `workers/c/README.md` for detailed documentation.
