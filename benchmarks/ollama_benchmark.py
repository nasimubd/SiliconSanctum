#!/usr/bin/env python3
"""Measure Ollama latency, throughput, cache reuse, and host memory pressure."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import time
import urllib.request
from pathlib import Path
from typing import Any


def command_output(*command: str) -> str:
    return subprocess.run(command, check=False, capture_output=True, text=True).stdout


def system_metrics() -> dict[str, Any]:
    swap = command_output("sysctl", "vm.swapusage")
    swap_match = re.search(r"used = ([0-9.]+)M", swap)
    rss_lines = command_output(
        "ps", "-Ao", "rss=,command="
    ).splitlines()
    runner_rss_kib = 0
    for line in rss_lines:
        if "llama-server" in line:
            try:
                runner_rss_kib += int(line.strip().split(maxsplit=1)[0])
            except (ValueError, IndexError):
                pass
    pressure = command_output("memory_pressure")
    free_match = re.search(r"System-wide memory free percentage: (\d+)%", pressure)
    return {
        "swap_used_mib": float(swap_match.group(1)) if swap_match else None,
        "memory_free_percent": int(free_match.group(1)) if free_match else None,
        "runner_rss_mib": round(runner_rss_kib / 1024, 2),
    }


def request_once(base_url: str, model: str, context: int, prompt: str) -> dict[str, Any]:
    payload = json.dumps(
        {
            "model": model,
            "prompt": prompt,
            "stream": True,
            "think": False,
            "keep_alive": -1,
            "options": {
                "num_ctx": context,
                "num_predict": 64,
                "temperature": 0,
                "seed": 42,
            },
        }
    ).encode()
    request = urllib.request.Request(
        f"{base_url.rstrip('/')}/api/generate",
        data=payload,
        headers={"Content-Type": "application/json"},
    )
    started = time.monotonic()
    first_token_at: float | None = None
    final: dict[str, Any] = {}
    response_text: list[str] = []
    with urllib.request.urlopen(request, timeout=1800) as response:
        for raw_line in response:
            event = json.loads(raw_line)
            text = event.get("response", "")
            if text and first_token_at is None:
                first_token_at = time.monotonic()
            response_text.append(text)
            if event.get("done"):
                final = event
    finished = time.monotonic()

    def rate(count_key: str, duration_key: str) -> float | None:
        count = final.get(count_key)
        duration_ns = final.get(duration_key)
        if not count or not duration_ns:
            return None
        return round(count / (duration_ns / 1_000_000_000), 2)

    return {
        "wall_seconds": round(finished - started, 3),
        "time_to_first_token_seconds": (
            round(first_token_at - started, 3) if first_token_at else None
        ),
        "load_seconds": round(final.get("load_duration", 0) / 1_000_000_000, 3),
        "prompt_tokens": final.get("prompt_eval_count"),
        "prompt_tokens_per_second": rate("prompt_eval_count", "prompt_eval_duration"),
        "generated_tokens": final.get("eval_count"),
        "decode_tokens_per_second": rate("eval_count", "eval_duration"),
        "response": "".join(response_text),
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-url", default="http://127.0.0.1:11434")
    parser.add_argument("--profile", required=True)
    parser.add_argument("--model", required=True)
    parser.add_argument("--context", type=int, required=True)
    parser.add_argument("--activation-seconds", type=float, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--prompt-repeat", type=int, default=32)
    args = parser.parse_args()

    prompt = (
        "You are testing local inference. Write one short original joke about quantitative "
        "finance, then explain it in one sentence. "
        + "alpha beta spread slippage risk latency " * args.prompt_repeat
    )
    before = system_metrics()
    first = request_once(args.base_url, args.model, args.context, prompt)
    second = request_once(args.base_url, args.model, args.context, prompt)
    after = system_metrics()
    process_state = json.load(
        urllib.request.urlopen(f"{args.base_url.rstrip('/')}/api/ps", timeout=10)
    )
    result = {
        "schema_version": 1,
        "captured_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "profile": args.profile,
        "model": args.model,
        "context": args.context,
        "activation_seconds": round(args.activation_seconds, 3),
        "system_before": before,
        "first_request": first,
        "cached_request": second,
        "system_after": after,
        "process_state": process_state,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
