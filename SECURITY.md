# Security Policy

## Secret safety
This repository must never contain:
- `.env` files
- API keys / PATs / OAuth tokens
- private keys / certificates
- machine-specific sensitive paths with credentials

## Required hygiene
- Keep `.env` out of git (not tracked)
- Use placeholder values in docs/examples
- Run a quick grep before pushing:

```bash
rg -n "ghp_|api[_-]?key|token|secret|BEGIN [A-Z ]*PRIVATE KEY|\.env" -S .
```

## Incident response
If a secret is ever committed:
1. Revoke/rotate immediately
2. Remove from git history (filter-repo/BFG)
3. Force-push rewritten history
4. Document incident and mitigation in a postmortem issue

## Reporting
Report vulnerabilities via GitHub Security Advisories.
