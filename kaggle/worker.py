#!/usr/bin/env python3
"""Reproducible Kaggle worker with deadlines, checkpoints, and telemetry."""
from __future__ import annotations

import json, os, signal, subprocess, sys, time
from pathlib import Path

ROOT = Path("/kaggle/working")
MANIFEST = ROOT / "job-manifest.json"
CHECKPOINT = ROOT / "checkpoint.json"

def stop(signum, _frame):
    CHECKPOINT.write_text(json.dumps({"status": "interrupted", "signal": signum}) + "\n")
    raise SystemExit(128 + signum)

def main() -> int:
    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGINT, stop)
    started = time.time()
    command = json.loads(os.environ.get("KAGGLE_WORKER_COMMAND", "[\"python\",\"-m\",\"backtest.run\"]"))
    deadline = int(os.environ.get("KAGGLE_WORKER_DEADLINE", "39600"))
    manifest = {"started_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "command": command,
                "model": os.environ.get("MODEL_NAME", ""), "context": int(os.environ.get("MODEL_CONTEXT", "0")),
                "gpu": os.environ.get("CUDA_VISIBLE_DEVICES", "all")}
    MANIFEST.write_text(json.dumps(manifest, indent=2) + "\n")
    env = os.environ.copy()
    try:
        result = subprocess.run(command, cwd=os.environ.get("PROJECT_DIR", "/kaggle/working/project"),
                                env=env, timeout=deadline, check=False)
        status = "completed" if result.returncode == 0 else "failed"
        code = result.returncode
    except subprocess.TimeoutExpired:
        status, code = "deadline", 124
    manifest.update({"status": status, "returncode": code, "elapsed_seconds": round(time.time() - started, 2),
                     "finished_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())})
    CHECKPOINT.write_text(json.dumps(manifest, indent=2) + "\n")
    return code

if __name__ == "__main__":
    raise SystemExit(main())
