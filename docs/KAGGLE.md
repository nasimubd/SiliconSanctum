# Kaggle burst worker

This repository includes a quota-bounded Kaggle worker for long-context batch
jobs. It is not an interactive replacement for the local Ollama service.

1. Create a private Kaggle model/data Dataset containing the pinned GGUF or
   Transformers weights referenced by `kaggle/job.json`.
2. Change `kernel_slug`, `dataset_ref`, `command`, and the backtest config.
3. Authenticate the Kaggle CLI (`kaggle auth login`) and submit:

```bash
./scripts/kaggle-submit.sh
```

4. Monitor and retrieve results:

```bash
kaggle kernels status <your-kaggle-username>/quant-long-context-worker
./scripts/kaggle-output.sh
```

The worker writes `job-manifest.json` and `checkpoint.json` to Kaggle output,
enforces a 11-hour deadline, handles termination signals, and records the model
and context used. Keep raw/private market data out of public Kaggle artifacts;
upload only approved datasets. Use context 524288 first, then test 786432 and
1010000 as separate jobs after retrieval and memory checks pass.

Kaggle's free T4×2 allocation is two 16 GB GPUs, not one 32 GB device. The
serving command must therefore use a runtime with tensor/model parallelism.
Weekly and daily caps in `job.json` are scheduling targets; Kaggle remains the
source of truth for account quota and availability.
