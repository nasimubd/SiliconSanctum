# Security and data handling

## Scope

This repository contains orchestration code. It must not contain credentials,
raw or proprietary market data, broker configuration, model weights, or runtime
outputs. Keep those assets on a private volume and outside Git.

The local API binds to loopback by default. Do not expose it to a network unless
you add authentication, TLS, and an access-control boundary. Kaggle jobs have
internet access; upload only data that you are authorized to disclose.

## Placeholders

Copy `.env.example` to `.env` (which is ignored) and replace `YOUR_NVME_NAME`
with the exact directory name under `/Volumes`. `AI_ROOT` is the workstation
data directory; it must not point at a repository containing private data.

## Before publishing or contributing

Run `./scripts/security-scan.sh` and `make validate`. Revoke and rotate any
credential that may have been exposed; removing a file does not remove it from
Git history. Report suspected vulnerabilities privately to the repository
maintainer rather than opening a public issue.

This scan is a heuristic guardrail and cannot prove that a repository is safe.
