#!/usr/bin/env python3

import argparse
import re
import sys
from pathlib import Path


def parse_format_1(text: str) -> dict[str, str]:
    regs = {}
    pattern = re.compile(
        r"^\s*(x\d+)\s+\([^)]*\)\s*=\s*(0x[0-9a-fA-F]+)\s+\([^)]*\)\s+\([^)]*\)\s*$"
    )

    for line in text.splitlines():
        match = pattern.match(line)
        if match:
            reg = match.group(1)
            hex_value = match.group(2).lower()
            regs[reg] = hex_value

    return regs


def parse_format_2(text: str) -> dict[str, str]:

    regs = {}
    pattern = re.compile(
        r"^\s*(x\d+)\s+(0x[0-9a-fA-F]+)\s+\([^)]*\)\s*$"
    )

    for line in text.splitlines():
        match = pattern.match(line)
        if match:
            reg = match.group(1)
            hex_value = match.group(2).lower()
            regs[reg] = hex_value

    return regs


def compare_registers(regs1: dict[str, str], regs2: dict[str, str]) -> int:
    all_regs = sorted(
        set(regs1.keys()) | set(regs2.keys()),
        key=lambda r: int(r[1:])
    )

    mismatches = 0

    for reg in all_regs:
        v1 = regs1.get(reg)
        v2 = regs2.get(reg)

        if v1 is None:
            print(f"{reg}: missing from file 1, file 2 has {v2}")
            mismatches += 1
        elif v2 is None:
            print(f"{reg}: file 1 has {v1}, missing from file 2")
            mismatches += 1
        elif v1 != v2:
            print(f"{reg}: MISMATCH  file1={v1}  file2={v2}")
            mismatches += 1
        else:
            print(f"{reg}: match     {v1}")

    return mismatches


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Compare register hex values between two RISC-V register dump formats."
    )
    parser.add_argument("file1", type=Path, help="Path to file in format 1")
    parser.add_argument("file2", type=Path, help="Path to file in format 2")
    args = parser.parse_args()

    try:
        text1 = args.file1.read_text()
        text2 = args.file2.read_text()
    except OSError as exc:
        print(f"Error reading file: {exc}", file=sys.stderr)
        return 1

    regs1 = parse_format_1(text1)
    regs2 = parse_format_2(text2)

    if not regs1:
        print("Warning: no registers parsed from file 1", file=sys.stderr)
    if not regs2:
        print("Warning: no registers parsed from file 2", file=sys.stderr)

    mismatches = compare_registers(regs1, regs2)

    print()
    if mismatches == 0:
        print("All compared registers match.")
        return 0
    else:
        print(f"Found {mismatches} mismatch(es).")
        return 2


if __name__ == "__main__":
    raise SystemExit(main())