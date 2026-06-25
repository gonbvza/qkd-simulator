#!/usr/bin/env python3

"""Drive the interactive QKD simulator and create a sample topology.

This script waits for the CLI prompts, creates three nodes with a unique suffix,
adds the two links, starts the QKD session, and then terminates the process.
"""

from __future__ import annotations

import os
import pty
import select
import signal
import subprocess
import sys
import time
from typing import Tuple

PROMPT_MAIN = "What do you want to do?"
PROMPT_NODE_NAME = "Enter node name:"
PROMPT_NODE_TYPE = "Enter node type:"
PROMPT_SRC_ID = "Enter source node id:"
PROMPT_DST_ID = "Enter destination node id:"
PROMPT_DISTANCE = "Enter distance in meters:"
PROMPT_SECURE = "Is secure (true or false):"
PROMPT_START_SRC = "Enter source id:"
PROMPT_START_DST = "Enter destination id:"
DISTANCE = "100000"
IS_SECURE_FIRST = "false"
IS_SECURE_SECOND = "true"


LOG_FILE = "log_file.txt"


def init_log_file():
    """Clear the log file at the start of each run."""
    with open(LOG_FILE, "wb"):
        pass


def write_to_file(text: str):
    with open(LOG_FILE, "ab") as f:
        f.write(text.encode())


def read_until(fd: int, needle: str, timeout: float = 60.0) -> str:
    buffer = ""
    deadline = time.time() + timeout
    while needle not in buffer:
        if time.time() > deadline:
            raise TimeoutError(f"Timed out waiting for prompt: {needle!r}")
        ready, _, _ = select.select([fd], [], [], 0.1)
        if fd not in ready:
            continue
        try:
            data = os.read(fd, 4096)
        except OSError:
            break
        if not data:
            break
        text = data.decode(errors="ignore")
        sys.stdout.write(text)
        # write_to_file(text)
        sys.stdout.flush()
        buffer += text
    return buffer


def show_buffer(fd: int):
    while True:
        ready, _, _ = select.select([fd], [], [], 0.1)
        if fd not in ready:
            continue
        try:
            data = os.read(fd, 4096)
        except OSError:
            break
        if not data:
            break
        text = data.decode(errors="ignore")
        sys.stdout.write(text)
        write_to_file(text)
        sys.stdout.flush()


def send_line(fd: int, line: str) -> None:
    os.write(fd, f"{line}\n".encode())


def extract_id(line: str):
    id_box = line.split()[0]
    id = int(id_box[1:-1])
    return id


def get_nodes_id(fd: int, run_id: str) -> Tuple[int, int, int]:
    src_id = dst_id = epr_id = 0

    send_line(fd, "get_nodes")
    deadline = time.time() + 60
    while True:
        if time.time() > deadline:
            raise TimeoutError(f"Timed out waiting for prompt")
        ready, _, _ = select.select([fd], [], [], 0.1)
        if fd not in ready:
            continue
        try:
            data = os.read(fd, 4096)
        except OSError:
            break
        if not data:
            break
        text = data.decode(errors="ignore")
        for line in text.split("\n"):
            if f"src_{run_id}" in line:
                src_id = extract_id(line)
                print(f"Found src {src_id}")
            if f"dst_{run_id}" in line:
                dst_id = extract_id(line)
                print(f"Found dst_id {dst_id}")
            if f"epr_{run_id}" in line:
                epr_id = extract_id(line)
                print(f"Found epr {epr_id}")
        if src_id != 0 and dst_id != 0 and epr_id != 0:
            break

    return (src_id, dst_id, epr_id)


def main() -> int:
    run_id = sys.argv[1] if len(sys.argv) > 1 else str(int(time.time()))
    repo_root = os.path.dirname(os.path.abspath(__file__))

    init_log_file()

    master_fd, slave_fd = pty.openpty()
    process = subprocess.Popen(
        ["cargo", "run", "--quiet"],
        cwd=repo_root,
        stdin=slave_fd,
        stdout=slave_fd,
        stderr=slave_fd,
        preexec_fn=os.setsid,
    )
    os.close(slave_fd)

    try:
        read_until(master_fd, PROMPT_MAIN)

        send_line(master_fd, "create_node")
        read_until(master_fd, PROMPT_NODE_NAME)
        send_line(master_fd, f"src_{run_id}")
        read_until(master_fd, PROMPT_NODE_TYPE)
        send_line(master_fd, "0")
        time.sleep(0.5)

        read_until(master_fd, PROMPT_MAIN)
        send_line(master_fd, "create_node")
        read_until(master_fd, PROMPT_NODE_NAME)
        send_line(master_fd, f"dst_{run_id}")
        read_until(master_fd, PROMPT_NODE_TYPE)
        send_line(master_fd, "0")
        time.sleep(0.5)

        read_until(master_fd, PROMPT_MAIN)
        send_line(master_fd, "create_node")
        read_until(master_fd, PROMPT_NODE_NAME)
        send_line(master_fd, f"epr_{run_id}")
        read_until(master_fd, PROMPT_NODE_TYPE)
        send_line(master_fd, "1")
        time.sleep(0.5)
        src_id, dst_id, epr_id = get_nodes_id(master_fd, run_id)

        read_until(master_fd, "")
        send_line(master_fd, "create_link")
        read_until(master_fd, PROMPT_SRC_ID)
        send_line(master_fd, str(src_id))
        read_until(master_fd, PROMPT_DST_ID)
        send_line(master_fd, str(epr_id))
        read_until(master_fd, PROMPT_DISTANCE)
        send_line(master_fd, DISTANCE)
        read_until(master_fd, PROMPT_SECURE)
        send_line(master_fd, IS_SECURE_FIRST)
        time.sleep(0.5)

        read_until(master_fd, PROMPT_MAIN)
        send_line(master_fd, "create_link")
        read_until(master_fd, PROMPT_SRC_ID)
        send_line(master_fd, str(dst_id))
        read_until(master_fd, PROMPT_DST_ID)
        send_line(master_fd, str(epr_id))
        read_until(master_fd, PROMPT_DISTANCE)
        send_line(master_fd, DISTANCE)
        read_until(master_fd, PROMPT_SECURE)
        send_line(master_fd, IS_SECURE_SECOND)
        time.sleep(0.5)

        read_until(master_fd, PROMPT_MAIN)
        send_line(master_fd, "start")
        read_until(master_fd, PROMPT_START_SRC)
        send_line(master_fd, str(src_id))
        read_until(master_fd, PROMPT_START_DST)
        send_line(master_fd, str(dst_id))
        show_buffer(master_fd)
        time.sleep(10)

        return 0
    finally:
        try:
            os.killpg(os.getpgid(process.pid), signal.SIGTERM)
        except ProcessLookupError:
            pass
        except Exception:
            process.terminate()
        try:
            process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            process.kill()
            process.wait(timeout=10)


if __name__ == "__main__":
    raise SystemExit(main())
