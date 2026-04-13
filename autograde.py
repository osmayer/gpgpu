#!/usr/bin/env python3

import argparse
import re
import subprocess
import sys
from pathlib import Path


EXPECTED_REGS = [f"x{i}" for i in range(32)]

BEGIN_MARKER = "===== BEGIN REGISTER DUMP ====="
END_MARKER = "===== END REGISTER DUMP ====="


def normalize_hex(hex_str: str) -> str:
    return f"0x{int(hex_str, 16):08x}"


def parse_expected_reg_file(text: str) -> dict[str, str]:
    regs = {}
    pattern = re.compile(
        r"^\s*(x\d+)\s+\([^)]*\)\s*=\s*(0x[0-9a-fA-F]+)\s+\([^)]*\)\s+\([^)]*\)\s*$"
    )

    for line in text.splitlines():
        m = pattern.match(line)
        if m:
            regs[m.group(1)] = normalize_hex(m.group(2))

    return regs


def extract_register_dump(text: str) -> str | None:
    start = text.find(BEGIN_MARKER)
    if start == -1:
        return None

    start += len(BEGIN_MARKER)
    end = text.find(END_MARKER, start)
    if end == -1:
        return None

    return text[start:end]


def parse_sim_output(text: str) -> dict[str, str]:
    regs = {}
    pattern = re.compile(r"^\s*(x\d+)\s+(0x[0-9a-fA-F]+)\s+\([^)]*\)\s*$")

    for line in text.splitlines():
        m = pattern.match(line)
        if m:
            regs[m.group(1)] = normalize_hex(m.group(2))

    return regs


def compare_registers(expected: dict[str, str], actual: dict[str, str]) -> list[str]:
    mismatches = []

    for reg in EXPECTED_REGS:
        exp = expected.get(reg)
        act = actual.get(reg)

        if exp is None:
            mismatches.append(f"{reg}: missing from expected .reg")
        elif act is None:
            mismatches.append(f"{reg}: expected {exp}, but simulator output is missing it")
        elif exp != act:
            mismatches.append(f"{reg}: expected {exp}, got {act}")

    return mismatches


def discover_pairs(test_dir: Path) -> tuple[list[tuple[str, Path, Path]], list[str]]:
    pairs = []
    warnings = []

    elf_files = sorted(test_dir.glob("*.elf"))
    reg_files = sorted(test_dir.glob("*.reg"))

    reg_names = {p.name for p in reg_files}
    elf_names = {p.name for p in elf_files}

    for elf_path in elf_files:
        reg_path = elf_path.with_suffix(".reg")
        test_name = elf_path.stem

        if reg_path.name in reg_names and reg_path.exists():
            pairs.append((test_name, elf_path, reg_path))
        else:
            warnings.append(
                f"Skipping {elf_path.name}: expected matching {reg_path.name}, but it was not found"
            )

    for reg_path in reg_files:
        elf_path = reg_path.with_suffix(".elf")
        if elf_path.name not in elf_names:
            warnings.append(
                f"Skipping {reg_path.name}: expected matching {elf_path.name}, but it was not found"
            )

    return pairs, warnings


def cargo_build(cargo_dir: Path, release: bool) -> tuple[bool, str]:
    cmd = ["cargo", "build", "--quiet"]
    if release:
        cmd.append("--release")

    result = subprocess.run(
        cmd,
        cwd=cargo_dir,
        capture_output=True,
        text=True,
    )

    output = (result.stdout or "") + ("\n" + result.stderr if result.stderr else "")
    return result.returncode == 0, output


def run_simulator(cargo_dir: Path, elf_path: Path, release: bool) -> subprocess.CompletedProcess:
    cmd = ["cargo", "run", "--quiet"]
    if release:
        cmd.append("--release")
    cmd.extend(["--", "--code-path", str(elf_path)])

    return subprocess.run(
        cmd,
        cwd=cargo_dir,
        capture_output=True,
        text=True,
    )


def grade_one_test(
    elf_path: Path,
    reg_path: Path,
    cargo_dir: Path,
    release: bool,
) -> tuple[bool, str, str, str]:
    """
    Returns:
        success, message, raw_stdout, raw_stderr
    """
    try:
        expected_text = reg_path.read_text()
    except OSError as e:
        return False, f"Could not read expected file {reg_path}: {e}", "", ""

    expected_regs = parse_expected_reg_file(expected_text)
    if not expected_regs:
        return False, f"Could not parse any registers from expected file {reg_path}", "", ""

    result = run_simulator(cargo_dir, elf_path, release)
    raw_stdout = result.stdout or ""
    raw_stderr = result.stderr or ""

    if result.returncode != 0:
        return (
            False,
            f"Simulator exited with code {result.returncode}",
            raw_stdout,
            raw_stderr,
        )

    dump_text = extract_register_dump(raw_stdout)
    if dump_text is None:
        return (
            False,
            "Could not find register dump markers in simulator stdout.",
            raw_stdout,
            raw_stderr,
        )

    actual_regs = parse_sim_output(dump_text)
    if not actual_regs:
        return (
            False,
            "Could not parse any registers inside the register dump.",
            raw_stdout,
            raw_stderr,
        )

    mismatches = compare_registers(expected_regs, actual_regs)
    if mismatches:
        return (
            False,
            "Register mismatches:\n" + "\n".join(mismatches),
            raw_stdout,
            raw_stderr,
        )

    return True, "PASS", raw_stdout, raw_stderr


def write_err_file(
    test_dir: Path,
    test_name: str,
    raw_stdout: str,
    raw_stderr: str,
    grade_message: str,
) -> None:
    err_path = test_dir / f"{test_name}.err"

    parts = []
    parts.append("===== PROGRAM STDOUT =====")
    parts.append(raw_stdout.rstrip("\n"))

    if raw_stderr:
        parts.append("")
        parts.append("===== PROGRAM STDERR =====")
        parts.append(raw_stderr.rstrip("\n"))

    parts.append("")
    parts.append("===== AUTOGRADER RESULT =====")
    parts.append(grade_message.rstrip("\n"))
    parts.append("")

    text = "\n".join(parts)

    try:
        err_path.write_text(text)
    except OSError as e:
        print(f"WARNING: Could not write {err_path}: {e}")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Autograde .elf/.reg pairs by running a Rust simulator and comparing final register state."
    )
    parser.add_argument("test_dir", type=Path, help="Directory containing .elf and .reg files")
    parser.add_argument(
        "--cargo-dir",
        type=Path,
        default=Path("."),
        help="Rust project directory (default: current directory)",
    )
    parser.add_argument(
        "--release",
        action="store_true",
        help="Use cargo --release",
    )
    args = parser.parse_args()

    test_dir = args.test_dir.resolve()
    cargo_dir = args.cargo_dir.resolve()

    if not test_dir.is_dir():
        print(f"Error: test_dir is not a directory: {test_dir}", file=sys.stderr)
        return 1

    if not cargo_dir.is_dir():
        print(f"Error: cargo-dir is not a directory: {cargo_dir}", file=sys.stderr)
        return 1

    pairs, warnings = discover_pairs(test_dir)

    for warning in warnings:
        print(f"WARNING: {warning}")

    if not pairs:
        print("No .elf/.reg pairs found.")
        return 1

    ok, build_output = cargo_build(cargo_dir, args.release)
    if not ok:
        print("Cargo build failed.\n")
        print(build_output)
        return 1

    passed = 0
    failed = 0
    passed_tests = []
    failed_tests = []

    print(f"Found {len(pairs)} test pair(s).\n")

    for test_name, elf_path, reg_path in pairs:
        print(f"=== {test_name} ===")
        print(f"Comparing {elf_path.name} vs {reg_path.name}")

        success, message, raw_stdout, raw_stderr = grade_one_test(
            elf_path=elf_path,
            reg_path=reg_path,
            cargo_dir=cargo_dir,
            release=args.release,
        )

        if success:
            passed += 1
            passed_tests.append(test_name)
            print("PASS\n")
        else:
            failed += 1
            failed_tests.append(test_name)
            print("FAIL")
            print(message)
            print()

            write_err_file(
                test_dir=test_dir,
                test_name=test_name,
                raw_stdout=raw_stdout,
                raw_stderr=raw_stderr,
                grade_message=message,
            )

    print("=== Summary ===")
    print(f"Passed: {passed} ({', '.join(passed_tests)})")
    print(f"Failed: {failed} ({', '.join(failed_tests)})")
    print(f"Total:  {passed + failed}")

    return 0 if failed == 0 else 2


if __name__ == "__main__":
    raise SystemExit(main())