# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| Latest  | :white_check_mark: |
| < Latest| :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in this project, please report it by:

1. **DO NOT** create a public issue
2. Email the details to the maintainers (see GitHub profile for contact)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Security Measures

This project implements several security measures:

- Regular dependency updates via Renovate
- Security scanning with:
  - cargo-audit for dependency vulnerabilities
  - TruffleHog for secret detection
  - Gitleaks for committed secrets
- All CI/CD workflows use minimal required permissions

## Response Time

- Critical vulnerabilities: Within 24 hours
- High severity: Within 3 days
- Medium/Low: Within 1 week

Thank you for helping keep this project secure!