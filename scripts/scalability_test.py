#!/usr/bin/env python3

import subprocess
import csv
import re
import os
from datetime import datetime

# -----------------------------
# CONFIG
# -----------------------------

TIME_BIN = "time"  # assumes GNU time is installed
CARGO_CMD = ["cargo", "run", "--release", "--quiet"]

OUTPUT_CSV = "scalability_results.csv"

# Change this to match your experiment scaling
QUANTUM_SIZES = [100, 1000, 5000, 10000]


# -----------------------------
# PARSING
# -----------------------------

def parse_time_output(stderr: str):
    """
    Extract runtime + memory from GNU time -v output.
    """

    # Wall clock time
    elapsed_match = re.search(r"Elapsed \(wall clock\) time.*?: (.*)", stderr)
    elapsed = elapsed_match.group(1).strip() if elapsed_match else None

    # Convert mm:ss or h:mm:ss to seconds
    def to_seconds(t):
        if t is None:
            return None
        parts = t.split(":")
        parts = [float(p) for p in parts]
        if len(parts) == 2:
            return parts[0] * 60 + parts[1]
        if len(parts) == 3:
            return parts[0] * 3600 + parts[1] * 60 + parts[2]
        return None

    elapsed_sec = to_seconds(elapsed)

    # Memory (RSS)
    mem_match = re.search(r"Maximum resident set size.*?: (\d+)", stderr)
    mem_kb = int(mem_match.group(1)) if mem_match else None

    return elapsed_sec, mem_kb


# -----------------------------
# RUN SINGLE EXPERIMENT
# -----------------------------

def run_experiment(qubits: int):
    print(f"\n=== Running simulation: {qubits} qubits ===")

    env = os.environ.copy()
    env["QUBITS"] = str(qubits)  # if your Rust code uses env var

    cmd = [
        "time", "-v",
        *CARGO_CMD
    ]

    process = subprocess.Popen(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        env=env
    )

    stdout, stderr = process.communicate()

    runtime, memory = parse_time_output(stderr)

    return {
        "qubits": qubits,
        "runtime_sec": runtime,
        "memory_kb": memory,
        "timestamp": datetime.now().isoformat()
    }


# -----------------------------
# MAIN
# -----------------------------

def main():
    results = []

    for q in QUANTUM_SIZES:
        try:
            result = run_experiment(q)
            results.append(result)

            print(f"Qubits: {q}")
            print(f"  Runtime (s): {result['runtime_sec']}")
            print(f"  Memory (KB): {result['memory_kb']}")

        except Exception as e:
            print(f"Failed for {q}: {e}")

    # Write CSV
    with open(OUTPUT_CSV, "w", newline="") as f:
        writer = csv.DictWriter(
            f,
            fieldnames=["qubits", "runtime_sec", "memory_kb", "timestamp"]
        )
        writer.writeheader()
        writer.writerows(results)

    print(f"\nSaved results to {OUTPUT_CSV}")


if __name__ == "__main__":
    main()
