# Disaster recovery

If the working copy is lost but TickArchive survives:

```bash
git clone /Volumes/TickArchive/ai-workstation/manifests/local-ai-workstation.bundle local-ai-workstation
cd local-ai-workstation
./scripts/bootstrap.sh
```

Then restore model weights from the pinned names in `config/models.json` using `./scripts/model-pull.sh PROFILE`. The files in `config/locks` record the exact registry manifests and layer digests used by this installation.

1. Clone this repository on an Apple Silicon Mac.
2. Attach and mount the AI volume. Copy `.env.example` to `.env` and adjust `AI_VOLUME` if its name changed.
3. Run `./scripts/bootstrap.sh`.
4. Run `./scripts/doctor.sh` and `./scripts/storage-benchmark.sh 4`.
5. Restore model files under `$AI_ROOT/models`, or run `./scripts/model-pull.sh PROFILE` for each required profile.
6. Compare restored model manifests with the committed lock manifests. Never assume a mutable registry tag still identifies the same weights.
7. Run `./scripts/validate.sh`, then execute the context ladder from 128K upward.
8. Configure the coding agent to use the local API and restore only the MCP/tool permissions it requires.

Back up this Git repository separately from `TickArchive`. Model weights are replaceable; the repository, model manifests, experiment manifests, indexes, and irreplaceable market data are the important backup set.
