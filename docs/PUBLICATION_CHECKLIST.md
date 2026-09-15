# Public-release checklist

Run these checks immediately before changing repository visibility:

```bash
git status --short
./scripts/security-scan.sh
make validate
git log --all --oneline
gh release list
gh pr list --state all
```

Review every release asset, tag, pull-request branch, issue attachment, and
package artifact. Confirm that no raw data, credentials, logs, model weights,
or private prompts are present. If a secret is found, revoke it first and then
rewrite all affected Git history; deleting the file in a later commit is not
enough.

After publication, enable secret scanning/push protection, branch protection,
and least-privilege CI permissions. Repeat the scan for every release.
