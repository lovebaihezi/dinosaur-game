# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability within the Dinosaur Game project, please send an email to the repository maintainer via GitHub. All security vulnerabilities will be promptly addressed.

**Please do not report security vulnerabilities through public GitHub issues.**

### What to Include

When reporting a vulnerability, please include:

1. A description of the vulnerability
2. Steps to reproduce the issue
3. Potential impact of the vulnerability
4. Any possible mitigations you've identified

### Response Timeline

- **Initial Response:** Within 48 hours of report
- **Status Update:** Within 7 days with assessment
- **Fix Timeline:** Varies based on severity and complexity

## Security Best Practices for Contributors

When contributing to this project, please:

1. **Never commit sensitive data** such as:
   - API keys or tokens
   - Passwords or credentials
   - Private keys or certificates
   - Personal information

2. **Use GitHub Secrets** for any sensitive configuration needed in CI/CD

3. **Keep dependencies updated** to avoid known vulnerabilities

4. **Follow secure coding practices** appropriate for Rust and web development

## Security Features

This project implements the following security measures:

- ✅ `.gitignore` configured to exclude sensitive files
- ✅ GitHub Actions secrets for deployment credentials
- ✅ No hardcoded credentials in source code
- ✅ Regular dependency updates via GitHub Actions

## Acknowledgments

We appreciate the security research community's efforts in responsibly disclosing vulnerabilities. Contributors who report valid security issues may be acknowledged in our release notes (with permission).

---

Thank you for helping keep the Dinosaur Game project secure! 🦕
