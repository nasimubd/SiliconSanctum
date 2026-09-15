"""Self-contained Kaggle entrypoint; submitter embeds the job manifest."""
import json, signal, subprocess, time
from pathlib import Path

JOB_PAYLOAD = None
ROOT = Path("/kaggle/working")
CHECKPOINT = ROOT / "checkpoint.json"

def stop(signum, _frame):
    CHECKPOINT.write_text(json.dumps({"status": "interrupted", "signal": signum}) + "\n")
    raise SystemExit(128 + signum)

def main():
    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGINT, stop)
    if JOB_PAYLOAD is None:
        raise RuntimeError("submitter did not embed a job manifest")
    started = time.time()
    manifest = {"started_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
                "command": JOB_PAYLOAD["command"], "model": JOB_PAYLOAD["model"],
                "context": JOB_PAYLOAD["context"]}
    (ROOT / "job-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    try:
        result = subprocess.run(JOB_PAYLOAD["command"], cwd="/kaggle/working/project",
                                timeout=JOB_PAYLOAD["max_runtime_seconds"], check=False)
        status, code = (("completed", result.returncode) if result.returncode == 0
                        else ("failed", result.returncode))
    except subprocess.TimeoutExpired:
        status, code = "deadline", 124
    manifest.update({"status": status, "returncode": code,
                     "elapsed_seconds": round(time.time() - started, 2)})
    CHECKPOINT.write_text(json.dumps(manifest, indent=2) + "\n")
    return code

if __name__ == "__main__":
    raise SystemExit(main())
