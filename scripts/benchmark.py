# Copyright (c) 2022-present INESC-ID.
# Distributed under the MIT license that can be found in the LICENSE file.

import argparse
import os
import re
import shutil
import statistics
import subprocess
import sys
from pathlib import Path
from rich.console import Console
from rich.progress import (
    Progress,
    TextColumn,
    BarColumn,
    TimeElapsedColumn,
    TimeRemainingColumn,
)
from rich.table import Table


def run_timed(cmd):
    """
    Run *cmd* and return a tuple of (wall time in seconds, peak RSS in KB).
    Uses GNU time -v.
    """
    proc = subprocess.run(
        ["/usr/bin/time", "-v", "--"] + list(cmd),
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
        check=True,
    )
    stderr_output = proc.stderr.decode()

    m = re.search(
        r"Elapsed \(wall clock\) time \(h:mm:ss or m:ss\):\s*([\d:.]+)",
        stderr_output,
    )
    time_str = m.group(1) if m else "0:0"
    time_parts = list(map(float, time_str.split(":")))
    assert len(time_parts) == 2  # m:ss.cc
    minutes, seconds = time_parts
    wall_time_secs = (minutes * 60) + seconds

    m = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", stderr_output)
    peak_kb = int(m.group(1)) if m else 0
    return wall_time_secs, peak_kb


def get_temp_dir():
    shm = Path("/dev/shm")
    if shm.exists() and os.access(shm, os.W_OK):
        return shm / "cpp2rust-bench"
    return Path("/tmp/cpp2rust-bench")


def woff2_cleanup(base_dir):
    for f in list((base_dir / "tests").glob("**/*.woff2")):
        os.remove(f)
    for f in list(get_temp_dir().glob("*.ttf")):
        os.remove(f)


def woff2_decompress_cleanup():
    for f in list(get_temp_dir().glob("*.woff2")):
        os.remove(f)


def woff2_decompress_setup(files, base_dir):
    dir = get_temp_dir()
    for f in files:
        file = dir / f.name
        shutil.copy(f, file)
        pre_binary = str(base_dir / "src" / "woff2_compress")
        subprocess.run([pre_binary, str(file)], capture_output=True, check=True)
        os.remove(file)


def brunsli_cleanup(base_dir):
    for f in list((base_dir / "tests").glob("**/*.brn")):
        os.remove(f)
    for f in list(get_temp_dir().glob("*.jpg")):
        os.remove(f)


def brunsli_decompress_cleanup():
    for f in list(get_temp_dir().glob("*.brn")):
        os.remove(f)


def brunsli_decompress_setup(files, base_dir):
    dir = get_temp_dir()
    for f in files:
        file = dir / f.name
        shutil.copy(f, file)
        # Using cpp binary to set up artifacts cleanly
        pre_binary = str(base_dir / "src" / "bin" / "cbrunsli")
        subprocess.run([pre_binary, str(file)], capture_output=True, check=True)
        os.remove(file)


PROGRAMS = {
    "woff2": {
        "Compress": {
            "cpp_dir": ".",
            "bin": "woff2_compress",
            "tests": "**/*.ttf",
            "cleanup": woff2_cleanup,
        },
        "Decompress": {
            "cpp_dir": ".",
            "bin": "woff2_decompress",
            "tests": "**/*.ttf",
            "cleanup": woff2_cleanup,
            "setup": woff2_decompress_setup,
            "final_cleanup": woff2_decompress_cleanup,
            "cmdline": lambda f: get_temp_dir() / f.with_suffix(".woff2").name,
        },
    },
    "brunsli": {
        "Compress": {
            "cpp_dir": "bin",
            "bin": "cbrunsli",
            "tests": "*.jpg",
            "cleanup": brunsli_cleanup,
        },
        "Decompress": {
            "cpp_dir": "bin",
            "bin": "dbrunsli",
            "tests": "*.jpg",
            "cleanup": brunsli_cleanup,
            "setup": brunsli_decompress_setup,
            "final_cleanup": brunsli_decompress_cleanup,
            "cmdline": lambda f: get_temp_dir() / f.with_suffix(".jpg.brn").name,
        },
    },
}


def get_binary_size_mb(path):
    """Return the binary size in MB"""
    return os.path.getsize(path) / (1024 * 1024)


def run_benchmark(
    program,
    test,
    config,
    model,
    results,
    warmup_runs,
    benchmark_runs,
    progress,
    task,
    binary_sizes,  # dict mutated in place: {(program, model): size_mb}
):
    base_dir = Path(__file__).resolve().parent.parent / program
    tests_dir = base_dir / "tests"

    tests = list(tests_dir.glob(config["tests"]))
    if not tests:
        print(f"No files found for {program}")
        exit(1)

    if model == "cpp":
        binary = str(base_dir / "src" / config["cpp_dir"] / config["bin"])
    else:
        binary = str(base_dir / "out" / model / "target" / "release" / config["bin"])

    # Record binary size once per (program, model) combination
    key_pm = (program, model)
    if key_pm not in binary_sizes:
        binary_sizes[key_pm] = get_binary_size_mb(binary)

    if "setup" in config:
        config["setup"](tests, base_dir)

    # Warmup
    for _ in range(warmup_runs):
        for f in tests:
            target = config["cmdline"](f) if "cmdline" in config else f
            subprocess.run([binary, str(target)], capture_output=True, check=True)
            if config["cleanup"]:
                config["cleanup"](base_dir)
        progress.update(task, advance=1)

    # Actual benchmark runs
    for run_id in range(benchmark_runs):
        for f in tests:
            target_file = config["cmdline"](f) if "cmdline" in config else f
            time, peak_kb = run_timed([binary, str(target_file)])

            results.append(
                {
                    "program": program,
                    "test": test,
                    "model": model,
                    "file": os.path.basename(f),
                    "run_id": run_id,
                    "time": time,
                    "peak_mem": peak_kb,
                }
            )

            if config["cleanup"]:
                config["cleanup"](base_dir)
        progress.update(task, advance=1)
    if "final_cleanup" in config:
        config["final_cleanup"]()


def main():
    parser = argparse.ArgumentParser(description="Benchmark Runner")
    parser.add_argument("--programs", default=",".join(PROGRAMS.keys()))
    parser.add_argument("--models", default="refcount,unsafe")
    parser.add_argument("--warmup", type=int, default=2)
    parser.add_argument("--runs", type=int, default=7)
    parser.add_argument("--csv", help="Path to export raw results as CSV")

    args = parser.parse_args()
    programs_to_run = args.programs.split(",")
    models_list = ["cpp"] + args.models.split(",")

    console = Console()
    results = []
    binary_sizes = {}  # {(program, model): size_mb}

    get_temp_dir().mkdir(exist_ok=True)
    console.print(f"[green]Using temporary directory: {get_temp_dir()}[/]")
    console.print(
        f"[green]Running benchmarks {args.runs} times, warmup: {args.warmup}[/]"
    )

    total_steps = 0
    for p in programs_to_run:
        total_steps += len(PROGRAMS[p]) * len(models_list) * (args.runs + args.warmup)

    with Progress(
        TextColumn("[progress.description]{task.description}"),
        BarColumn(),
        TextColumn("[progress.percentage]{task.percentage:>3.0f}%"),
        "•",
        TimeElapsedColumn(),
        "•",
        TimeRemainingColumn(),
        console=console,
    ) as progress:
        for program_name in programs_to_run:
            tests = PROGRAMS[program_name]
            task = progress.add_task(
                f"[cyan]Running {program_name}...", total=total_steps
            )
            for test_name, config in tests.items():
                for model in models_list:
                    run_benchmark(
                        program_name,
                        test_name,
                        config,
                        model,
                        results,
                        args.warmup,
                        args.runs,
                        progress,
                        task,
                        binary_sizes,
                    )

    if not results:
        console.print("[red]No results collected.[/]")
        return

    if args.csv:
        import csv

        keys = results[0].keys()
        with open(args.csv, "w", newline="") as f:
            dict_writer = csv.DictWriter(f, fieldnames=keys)
            dict_writer.writeheader()
            dict_writer.writerows(results)
        console.print(f"[green]Results exported to {args.csv}[/]")

    # ------------------------------------------------------------------
    # Aggregate statistics
    # ------------------------------------------------------------------

    # Sum execution times and take max peak memory per (prog, test, model, run_id)
    run_totals = {}
    run_peak_mem = {}
    for r in results:
        key = (r["program"], r["test"], r["model"], r["run_id"])
        run_totals[key] = run_totals.get(key, 0) + r["time"]
        run_peak_mem[key] = max(run_peak_mem.get(key, 0), r["peak_mem"])

    stats_map = {}
    unique_groups = sorted(set((k[0], k[1], k[2]) for k in run_totals))

    for prog, test, model in unique_groups:
        total_times = [
            v
            for k, v in run_totals.items()
            if k[0] == prog and k[1] == test and k[2] == model
        ]
        peak_mems = [
            v
            for k, v in run_peak_mem.items()
            if k[0] == prog and k[1] == test and k[2] == model
        ]

        stats_map[(prog, test, model)] = {
            "avg": statistics.mean(total_times),
            "med": statistics.median(total_times),
            "std": statistics.stdev(total_times) if len(total_times) > 1 else 0,
            "min": min(total_times),
            "max": max(total_times),
            "count": len(total_times),
            "peak_mem": statistics.median(peak_mems),
        }

    table = Table(
        title="\nStatistics",
        show_header=True,
        header_style="bold",
    )
    table.add_column("Program", style="cyan")
    table.add_column("Test", style="magenta")
    table.add_column("Model", style="green")
    table.add_column("Avg (s)", justify="right")
    table.add_column("Δ Baseline", justify="right")
    table.add_column("Median (s)", justify="right")
    table.add_column("StdDev", justify="right")
    table.add_column("Med Peak RSS", justify="right")
    table.add_column("Δ Mem", justify="right")
    table.add_column("Binary Size", justify="right")

    keys_list = list(stats_map.keys())
    for (prog, test, model), data in stats_map.items():
        diff_str = "-"
        if model != "cpp":
            baseline_avg = stats_map.get((prog, test, "cpp"), {}).get("avg")
            pct = ((data["avg"] - baseline_avg) / baseline_avg) * 100
            color = "red" if pct > 2 else "green"
            diff_str = f"[{color}]{pct:+.1f}%[/{color}]"

        # --- peak memory (in MB) ---
        peak_mem = data["peak_mem"] / 1024
        peak_str = f"{peak_mem:.1f} MB"

        # --- memory delta vs cpp baseline ---
        mem_diff_str = "-"
        if model != "cpp":
            baseline_mem = stats_map.get((prog, test, "cpp"), {}).get("peak_mem", 0)
            mem_pct = ((data["peak_mem"] - baseline_mem) / baseline_mem) * 100
            mem_color = "red" if mem_pct > 5 else "green"
            mem_diff_str = f"[{mem_color}]{mem_pct:+.1f}%[/{mem_color}]"

        # --- binary size ---
        bin_mb = binary_sizes.get((prog, model))
        bin_str = f"{bin_mb:.2f} MB"

        table.add_row(
            prog,
            test,
            model,
            f"{data['avg']:.2f}",
            diff_str,
            f"{data['med']:.2f}",
            f"{data['std']:.2f}",
            peak_str,
            mem_diff_str,
            bin_str,
        )

        current_idx = keys_list.index((prog, test, model))
        if current_idx < len(stats_map) - 1:
            next_key = keys_list[current_idx + 1]
            if next_key[1] != test:
                table.add_section()

    if sys.stdout.isatty():
        console.print(table)
    else:
        # If not printing to a terminal (e.g., output is redirected to a file),
        # create a specific high-width file console.
        console = Console(width=200, force_terminal=False)
        with console.capture() as capture:
            console.print(table)
        sys.stdout.write(capture.get())


if __name__ == "__main__":
    main()
