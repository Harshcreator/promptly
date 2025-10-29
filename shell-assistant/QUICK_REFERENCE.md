# Enterprise Features - Quick Reference

## 🎯 What Got Implemented

### ✅ Phase 1: Configuration System
- **File**: `crates/core/src/config.rs`
- **What**: YAML config for enterprise settings
- **Use**: `EnterpriseConfig::load()` to read `~/.shell-assistant/config.yaml`

### ✅ Phase 2: Audit Logging  
- **File**: `crates/storage/src/audit.rs`
- **What**: JSON audit logs with full command tracking
- **Use**: `AuditLogger::new()` to log all command executions

### ✅ Phase 3: Enhanced Security
- **File**: `crates/core/src/safety.rs` (enhanced)
- **What**: Whitelist/blacklist, compliance mode, safety levels
- **Use**: `CommandSafetyChecker::with_enterprise_config()`

---

## 📦 Quick Start

### 1. Copy example config
```bash
cp config.example.yaml ~/.shell-assistant/config.yaml
```

### 2. Edit for your environment
```yaml
llm:
  backend: llm-rs
  model_path: "~/models/codellama-7b.gguf"

enterprise:
  organization: "Your Company"
  allowed_commands: [git, docker, ls]
  blocked_commands: ["rm -rf /"]
```

### 3. Use in code
```rust
// Load config
let config = EnterpriseConfig::load()?;

// Create safety checker
let checker = CommandSafetyChecker::with_enterprise_config(
    config.enterprise.allowed_commands,
    config.enterprise.blocked_commands,
    config.enterprise.compliance_mode,
);

// Create audit logger
let logger = AuditLogger::new(
    config.get_audit_log_path(),
    config.enterprise.organization,
    config.enterprise.department,
);

// Check command
let result = checker.check_command_detailed("git status");

// Log execution
logger.log_command(input, command, executed, exit_code, 
                   result.level, backend, notes, session_id)?;
```

---

## 🔧 Configuration Options

| Section | Key | Description | Default |
|---------|-----|-------------|---------|
| `llm` | `backend` | LLM provider: ollama/openai/llm-rs | ollama |
| `llm` | `model_path` | Path to GGUF model (for llm-rs) | None |
| `llm` | `max_tokens` | Max tokens to generate | 256 |
| `llm` | `threads` | CPU threads to use | auto-detect |
| `security` | `safety_check` | Enable safety checker | true |
| `security` | `always_confirm` | Always ask before running | true |
| `security` | `audit_log` | Enable audit logging | true |
| `privacy` | `offline_only` | Block external network | true |
| `privacy` | `save_history` | Save command history | true |
| `enterprise` | `organization` | Company name | None |
| `enterprise` | `compliance_mode` | Strict security | true |
| `enterprise` | `allowed_commands` | Whitelist patterns | [] |
| `enterprise` | `blocked_commands` | Blacklist patterns | [dangerous] |

---

## 🔒 Safety Levels

| Level | Meaning | Action |
|-------|---------|--------|
| `Safe` | No risk detected | Execute normally |
| `Warning` | Potentially risky | Show warning, ask |
| `Dangerous` | High risk | Require confirmation |
| `Blocked` | Policy violation | Do not execute |

---

## 📊 Audit Log Fields

```json
{
  "timestamp": "2024-01-15T10:30:00Z",     // When
  "user": "jdoe",                          // Who
  "organization": "ACME",                  // Where
  "department": "IT",                      // Team
  "input": "list files",                   // What user asked
  "generated_command": "ls -la",           // What was generated
  "executed": true,                        // Was it run?
  "exit_code": 0,                          // Success?
  "safety_level": "safe",                  // How safe?
  "llm_backend": "llm-rs",                 // Which LLM?
  "session_id": "xyz"                      // Session tracking
}
```

---

## 🧪 Testing Your Implementation

```bash
# Build everything
cargo build

# Run tests
cargo test --package core config
cargo test --package storage audit
cargo test --package core safety

# Test config loading
cargo run --example config_test   # (after creating example)
```

---

## 🚨 Common Issues

| Problem | Solution |
|---------|----------|
| Config not found | Create `~/.shell-assistant/config.yaml` |
| Model path invalid | Use full path or `~` for home |
| Audit log fails | Check directory permissions |
| Command blocked | Check `allowed_commands` list |
| Build fails | Ensure all dependencies installed |

---

## 📂 File Locations

```
~/.shell-assistant/
├── config.yaml           # Your configuration
├── audit.log            # Audit log (JSON Lines)
├── history.json         # Command history
└── models/              # Your GGUF models
    └── codellama-7b.gguf
```

---

## 🎓 Examples

### Whitelist only git and ls
```yaml
enterprise:
  allowed_commands: [git, ls, cd]
  blocked_commands: []
```

### Block dangerous patterns
```yaml
enterprise:
  allowed_commands: []  # Allow all
  blocked_commands:
    - "rm -rf /"
    - "format"
    - "dd if=/dev/zero"
```

### Strict compliance mode
```yaml
enterprise:
  compliance_mode: true
  allowed_commands: [git, docker, kubectl, ls, cd, cat]
```

---

## 🔄 Next Implementation Steps

1. **CLI Integration** - Make it actually work in the CLI
2. **Build Profiles** - Offline-only builds
3. **Documentation** - User guides
4. **Advanced Features** - Log rotation, encryption

---

## 📞 Need Help?

- Read: `IMPLEMENTATION_PROGRESS.md` for details
- Review: `config.example.yaml` for all options
- Test: Unit tests in each module
- Ask: Open an issue or ask questions

---

**Status**: ✅ Core features ready, needs CLI integration  
**Branch**: `enterprise-deployment`  
**Commit**: `1995031`
