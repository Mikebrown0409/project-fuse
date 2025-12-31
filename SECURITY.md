# Security Policy

## Supported Versions

We provide security updates for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 1.2.x   | :white_check_mark: |
| 1.1.x   | :white_check_mark: |
| < 1.1   | :x:                |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security vulnerability, please follow these steps:

### 1. **Do NOT** open a public issue

Security vulnerabilities should be reported privately to protect users until a fix is available.

### 2. Report via GitHub Security Advisories

Please report security vulnerabilities using GitHub's Security Advisory feature:

1. Go to the repository's **Security** tab
2. Click **Report a vulnerability**
3. Fill out the security advisory form with details

Alternatively, you can open a **private security advisory** by:
- Creating a draft security advisory in the repository
- Or opening a private issue (if you have maintainer access)

Include the following information:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if you have one)
- Your contact information

### 3. Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity, typically 30-90 days

### 4. Disclosure Policy

- We will acknowledge receipt of your report
- We will keep you informed of our progress
- We will credit you in security advisories (unless you prefer to remain anonymous)
- We will coordinate public disclosure after a fix is available

## Security Best Practices

### For Users

- Always use the latest stable version
- Review and understand the security implications of zero-knowledge proofs
- Use `RISC0_DEV_MODE=1` only for development/testing, never in production
- Verify proofs before trusting compliance claims
- Keep your RISC Zero toolchain updated

### For Developers

- Follow secure coding practices
- Review all dependencies for known vulnerabilities (`cargo audit`)
- Run security linters (`cargo clippy`)
- Never commit secrets or private keys
- Use constant-time operations for cryptographic code

## Known Security Considerations

### Zero-Knowledge Proofs

- **Dev Mode**: `RISC0_DEV_MODE=1` generates non-cryptographic proofs for testing only
  - ⚠️ **NOT side-channel resistant** - use only for development/testing
  - ⚠️ **NOT cryptographically secure** - do not use in production
- **Real Proofs**: Production use requires real zkVM proofs (10-20+ minutes generation time)
  - ✅ Uses RISC Zero's secure, audited implementation
  - ✅ Side-channel resistant
  - ✅ Cryptographically secure
- **Verification**: Always verify proofs cryptographically before trusting results

### C2PA Integration

- C2PA signature verification is performed in the zkVM
- Tampered manifests will fail verification
- Invalid signatures are detected and reported
- **Selective disclosure**: Sensitive fields can be redacted via `disclosed_fields` in spec

### Guest Program

- The guest program runs in an isolated zkVM environment
- Guest code is deterministic and verifiable
- No network access or side effects from guest execution
- Relies on RISC Zero's secure implementation for cryptographic operations

## Known Limitations

| Issue | Mitigation | Impact | Status |
|-------|------------|--------|--------|
| Dev mode not side-channel resistant | Use real proofs in production | Low (dev mode for testing only) | Documented |
| GPU linking issue (sppark) | CPU proving works fine | Low (performance optimization, not blocker) | Documented |
| RSA timing sidechannel (transitive via c2pa) | We use Ed25519, RSA only in c2pa parser | Low (no direct exposure) | Monitored |
| tracing-subscriber log poisoning (transitive) | Requires RISC Zero update | Low (only affects logging) | Monitored |
| No explicit timestamp freshness check | Relies on spec expiry | Low (acceptable for most use cases) | Documented |

**Note**: See `docs/SECURITY_REVIEW.md` for complete security review and detailed analysis.

## Security Status

**Current Status**: Pre-audit dev version

- ✅ Internal security review completed (Phase 3)
- ✅ Dependency scanning completed (`cargo audit`)
- ✅ Fuzzing infrastructure in place
- ⏳ External audit pending (see `docs/EXTERNAL_AUDIT_OPTIONS.md`)

**For Pilots**: Use "Pre-audit dev version" disclaimer. External audit recommended before production contracts.

## Security Updates

Security advisories will be published:
- In GitHub Security Advisories (automatically visible in the repository's Security tab)
- In release notes
- Via GitHub notifications to repository watchers

## Thank You

We appreciate the security research community's efforts to keep Project FUSE secure. Responsible disclosure helps protect all users.
