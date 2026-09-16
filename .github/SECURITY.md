# Security Policy

We take the security of the kyanite project seriously.
This document explains how to report security issues, what to expect, and which versions receive security patches.

## Scope

This policy applies to the project’s code, configuration, build, release, and dependency ecosystem.
The following areas are generally in scope for security reports:

- Authentication, authorization, or privilege escalation flaws
- Injection vulnerabilities (command, code, path, template, query, etc.)
- Exposure of sensitive data, credentials, tokens, logs, or telemetry
- Vulnerabilities in default configurations or documented usage
- Supply-chain or dependency vulnerabilities affecting the project

This list is not exhaustive. If you are unsure whether something qualifies, report it privately and let the maintainers assess it.

## Supported Versions

Security patches are applied to the default development branch.
Releases are snapshots of that branch at a specific point in time and may receive a patch release if severity warrants it.

| Version / Branch | Supported                                            |
|------------------|------------------------------------------------------|
| Default branch   | ✅ Receives security patches                         |
| Tagged releases  | 🔄 Patched via a new release from the default branch |
| Older releases   | ❌ No longer supported                               |

## Reporting a Security Issue

We appreciate responsible disclosure. Please choose the appropriate channel below.

### Non-Sensitive Issues (Public)

If the issue does **not** expose sensitive data or reveal exploitable details,
[open a public issue](https://github.com/Akeoott/kyanite-core/issues/new?template=security_report.yaml) using the security report template.
Follow the template instructions carefully.

### Sensitive Issues (Private)

If your report could disclose vulnerabilities, sensitive data, or enough detail for someone to reproduce an exploit, **do not** open a public issue. Instead:

1. Email **akeoot@pm.me**
2. Include **"SECURITY"** in the subject line
3. Provide as much detail as possible: steps to reproduce, affected versions, potential impact, and any proof-of-concept code (minimized)

## Response Commitment

We will do our best to respond quickly and keep you informed:

1. **Acknowledgement**: We will confirm receipt within 24-48 hours
2. **Assessment**: We will triage the issue and determine severity (for example, using CVSS) within 5 business days
3. **Fix timeline**: After assessment, we will communicate an expected timeline. This depends on severity:
   - **Critical / High** (CVSS 7.0-10.0) ⇒ prioritized immediately/a patch release may follow
   - **Medium / Low** (CVSS 0.1-6.9) => addressed in the next regular release cycle

## Disclosure Policy

We follow a coordinated disclosure process:

- We will work with the reporter to agree on a public disclosure date *after* a fix has been released
- Reporters may be credited in release notes and security advisories (with consent)
- Anonymous reporting is respected, if you prefer not to be credited, just let us know

---

*Thank you for helping keep kyanite-core safe.*
