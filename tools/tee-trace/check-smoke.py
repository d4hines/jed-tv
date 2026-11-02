#!/usr/bin/env python3
"""Validate a coordinator-collected harmless tee-trace-smoke JSONL log."""
import json
import sys
from pathlib import Path

rows = [json.loads(line) for line in Path(sys.argv[1]).read_text().splitlines()]
assert rows[0]["event"] == "start"
assert rows[-1] == {"event": "complete", "root_status": 0, "ioctls": 10}
calls = {}
for row in rows:
    if row["event"] == "ioctl":
        assert row["error"] is None, row
        calls.setdefault((row["tid"], row["seq"]), []).append(row)
assert len(calls) == 10, len(calls)
for pair in calls.values():
    assert [row["stage"] for row in pair] == ["entry", "exit"], pair
    entry, end = pair
    assert entry["header_word"] == 42 and entry["num_params"] == 1, entry
    assert end["return"] == -25 and end["params"] == [], end
    assert entry["fd_path"] == "/dev/null", entry
    if entry["request"] == "send":
        assert entry["params"] == [{"attr": 1, "a": 0xFEED, "b": 123, "c": 456}]
    else:
        assert entry["request"] == "recv" and entry["params"] == []
assert sum(row["event"] == "child" for row in rows) >= 5
assert sum(row["event"] == "exec" for row in rows) >= 3
assert any(row["event"] == "signal" and row["signal"] == 10 for row in rows)
assert any(row["event"] == "exec" and row["old_tid"] != row["tid"] for row in rows)
print("PASS: 10 paired ioctls, errno preserved, metadata exact, threads and child exec observed")
