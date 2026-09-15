"""Kaggle entrypoint; generated jobs should remain deterministic and resumable."""
import json, os, subprocess, sys
from pathlib import Path

root = Path(__file__).resolve().parent
job = json.loads((root / "job.json").read_text())
os.environ["MODEL_NAME"] = job["model"]
os.environ["MODEL_CONTEXT"] = str(job["context"])
os.environ["KAGGLE_WORKER_DEADLINE"] = str(job["max_runtime_seconds"])
os.environ["KAGGLE_WORKER_COMMAND"] = json.dumps(job["command"])
subprocess.run([sys.executable, str(root / "worker.py")], check=False)
