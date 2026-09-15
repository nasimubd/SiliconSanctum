#!/usr/bin/env python3
"""Small OpenAI-compatible context probe; does not fabricate a million-token corpus."""

from __future__ import annotations

import argparse
import json
import time
import urllib.request


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base-url", default="http://127.0.0.1:8080")
    parser.add_argument("--model", default="local")
    parser.add_argument("--context", type=int, required=True)
    parser.add_argument("--tokens", type=int, default=4096)
    args = parser.parse_args()

    marker = "QUANT_CONTEXT_MARKER_7F31"
    filler = "alpha beta gamma delta execution latency spread slippage funding risk "
    prompt = marker + "\n" + (filler * max(1, args.tokens // 10))
    prompt += "\nReturn only the exact marker from the beginning."
    payload = json.dumps({
        "model": args.model,
        "messages": [{"role": "user", "content": prompt}],
        "temperature": 0,
        "max_tokens": 32,
    }).encode()
    request = urllib.request.Request(
        f"{args.base_url.rstrip('/')}/v1/chat/completions",
        data=payload,
        headers={"Content-Type": "application/json"},
    )
    started = time.monotonic()
    with urllib.request.urlopen(request, timeout=3600) as response:
        result = json.load(response)
    elapsed = time.monotonic() - started
    content = result["choices"][0]["message"]["content"]
    print(json.dumps({"requested_context": args.context, "probe_tokens_approx": args.tokens,
                      "elapsed_seconds": elapsed, "passed": marker in content,
                      "response": content}, indent=2))


if __name__ == "__main__":
    main()
