# Enterprise Deployment Guide

## Overview
This guide helps you deploy Shell Assistant in enterprise environments where cloud AI services are restricted or prohibited. The solution supports air-gapped networks, on-premise data centers, and environments with strict security requirements.

---

## Quick Start

### 1. Configuration

Copy the example configuration:
```bash
cp config.example.yaml ~/.shell-assistant/config.yaml
```

Edit the configuration for your enterprise:
```yaml
version: "1.0"

llm:
  backend: llm-rs
  model_path: "~/.shell-assistant/models/codellama-7b-instruct.Q4_K_M.gguf"

security:
  safety_check: true
  always_confirm: true
  audit_log: true

enterprise:
  organization: "Your Company Name"
  compliance_mode: true
  allowed_commands: [git, docker, ls, cd]
  blocked_commands: ["rm -rf /", "format"]
```

### 2. Features

- ✅ **Air-gapped deployment** - No internet required
- ✅ **Audit logging** - Full command tracking with compliance metadata
- ✅ **Command whitelisting** - Only allow approved commands
- ✅ **Command blacklisting** - Block dangerous operations
- ✅ **Compliance mode** - Strict security enforcement
- ✅ **Offline-only mode** - No external network calls

### 3. Configuration Options

| Section | Description |
|---------|-------------|
| `llm` | LLM backend, model path, and inference parameters |
| `security` | Safety checks, confirmations, audit logging |
| `privacy` | Offline mode, telemetry, history management |
| `enterprise` | Organization details, compliance, whitelist/blacklist |

See `config.example.yaml` for all available options.

---

## Deployment Models

### Option 1: Individual User Installation
Each user installs on their local machine with their own model.

### Option 2: Centralized Model Repository
Shared network drive for models, individual installations for users.

### Option 3: Terminal Server Deployment
Central installation on terminal servers, users access via remote desktop.

---

## Security Features

### Audit Logging
All command executions are logged with:
- User and organization information
- Timestamps
- Safety level assessments
- Exit codes
- Full command text

Logs are stored in JSON Lines format at `~/.shell-assistant/audit.log`

### Command Control
- **Whitelist**: Only allow specified command patterns
- **Blacklist**: Block dangerous operations
- **Safety levels**: Safe, Warning, Dangerous, Blocked

### Compliance
- GDPR compliant (no external data transfer)
- SOC 2 ready (full audit trail)
- HIPAA compatible (offline operation)

---

## Installation

### Prerequisites
- Rust toolchain 1.65+
- GGUF model file (e.g., CodeLlama, Mistral)
- Windows: LLVM (for llm-rs backend)

### Build
```bash
# Enterprise offline-only build
cargo build --release --no-default-features --features "core/llm-rs"

# Install binary
cargo install --path crates/cli --no-default-features --features "core/llm-rs"
```

### Model Setup
```bash
# Create models directory
mkdir -p ~/.shell-assistant/models

# Copy your GGUF model file
cp codellama-7b-instruct.Q4_K_M.gguf ~/.shell-assistant/models/
```

---

## Usage

### Basic Usage
```bash
shell-assistant "list all files"
```

### With Enterprise Config
The tool automatically loads `~/.shell-assistant/config.yaml` and applies:
- Command whitelist/blacklist
- Safety checks
- Audit logging
- Compliance mode

### View Audit Logs
```bash
cat ~/.shell-assistant/audit.log | jq
```

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Model not found | Check `model_path` in config.yaml |
| Command blocked | Review `allowed_commands` whitelist |
| Slow performance | Increase `threads` or use smaller model |
| Build fails on Windows | Install LLVM from llvm.org |

---

## Support

For enterprise support and deployment assistance:
- Review the configuration examples
- Check audit logs for security incidents
- Test in dry-run mode first

---

## License
MPL-2.0 - See LICENSE file
