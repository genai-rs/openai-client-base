#!/usr/bin/env python3
"""Run isolated Cargo fixtures against the downstream checker."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

script = Path(sys.argv[1]).resolve()
results = Path(sys.argv[2]).resolve()
results.mkdir(parents=True, exist_ok=True)

def run(*args, cwd=None, env=None):
    return subprocess.check_output(args, cwd=cwd, env=env, text=True).strip()

def commit(repo, message):
    run("git", "add", ".", cwd=repo)
    run("git", "-c", "user.name=Fixture", "-c", "user.email=fixture@example.invalid",
        "commit", "-qm", message, cwd=repo)
    return run("git", "rev-parse", "HEAD", cwd=repo)

with tempfile.TemporaryDirectory(prefix="downstream-git-fixtures-") as directory:
    root = Path(directory)
    base = root / "base with spaces"
    consumer = root / "consumer"
    for repo in (base, consumer):
        (repo / "src").mkdir(parents=True)
        run("git", "init", "-q", str(repo))
    (base / "Cargo.toml").write_text(
        '[package]\nname="openai-client-base"\nversion="0.16.0"\nedition="2021"\n')
    (base / "src/lib.rs").write_text('pub fn consumed() -> u8 { 42 }\n')
    good = commit(base, "Compatible Git base")
    fake_git_url = "https://github.com/genai-rs/openai-client-base"
    (consumer / "Cargo.toml").write_text(
        '[package]\nname="openai-ergonomic"\nversion="0.7.0"\nedition="2021"\nbuild="build.rs"\n'
        '[dependencies]\nopenai-client-base = { version = "=0.16.0", git = '
        + json.dumps(fake_git_url) + ', rev = ' + json.dumps(good) + '}\n')
    (consumer / "build.rs").write_text(
        'use std::{env, fs::OpenOptions, io::Write};\n'
        'fn main() { if let Ok(path) = env::var("FIXTURE_MUTATE_CANDIDATE") {\n'
        '  let mut f = OpenOptions::new().append(true).open(path).unwrap();\n'
        '  writeln!(f, "\\n// edited by downstream build script").unwrap();\n'
        '} }\n')
    (consumer / "src/lib.rs").write_text(
        'pub fn use_base() -> u8 { openai_client_base::consumed() }\n'
        '#[test] fn offline() { assert_eq!(use_base(), 42); }\n')
    wrapper_sha = commit(consumer, "Pin Git-source base and offline test")

    # Isolate Git URL rewrites and identity. Cargo's Git CLI path maps the fake
    # canonical upstream URL to this local fixture repository.
    git_config = root / "gitconfig"
    git_config.write_text(
        '[user]\n    name = Fixture Runner\n    email = fixture-runner@example.invalid\n'
        '[url "file://' + str(base) + '"]\n    insteadOf = ' + fake_git_url + '\n')
    env = dict(os.environ, DOWNSTREAM_REPOSITORY=str(consumer),
               DOWNSTREAM_COMMIT=wrapper_sha, CARGO_BUILD_JOBS="2",
               GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=str(git_config),
               CARGO_NET_GIT_FETCH_WITH_CLI="true")
    assert run("git", "ls-remote", fake_git_url, env=env).startswith(good)

    def case(name, baseline, expected, candidate_result, baseline_result):
        output = results / name
        output.mkdir(exist_ok=True)
        env["DOWNSTREAM_RESULTS_DIR"] = str(output)
        base_before = run("git", "status", "--porcelain", cwd=base)
        consumer_before = run("git", "status", "--porcelain", cwd=consumer)
        with (output / "run.log").open("w") as log:
            proc = subprocess.run(["bash", str(script), str(base), baseline], env=env,
                                  stdout=log, stderr=subprocess.STDOUT)
        assert proc.returncode == expected, (name, proc.returncode)
        summary = (output / "summary.md").read_text()
        assert f"| Candidate | {candidate_result} |" in summary, name
        assert f"| Base main (diagnostic only) | {baseline_result} |" in summary, name
        assert wrapper_sha in summary and baseline in summary, name
        assert run("git", "status", "--porcelain", cwd=base) == base_before
        assert run("git", "status", "--porcelain", cwd=consumer) == consumer_before
        assert not (base / "target").exists()
        print(f"PASS {name}: exit={proc.returncode}, candidate={candidate_result}, baseline={baseline_result}")

    def mutation_case(name, dirty):
        source = base / "src/lib.rs"
        clean_bytes = source.read_bytes()
        if dirty:
            source.write_bytes(clean_bytes + b"// pre-existing edit\n")
        before_bytes = source.read_bytes()
        before_status = run("git", "status", "--porcelain", "--untracked-files=all", cwd=base)
        output = results / name
        output.mkdir(exist_ok=True)
        env["DOWNSTREAM_RESULTS_DIR"] = str(output)
        env["FIXTURE_MUTATE_CANDIDATE"] = str(source)
        with (output / "run.log").open("w") as log:
            proc = subprocess.run(["bash", str(script), str(base), good], env=env,
                                  stdout=log, stderr=subprocess.STDOUT)
        summary = (output / "summary.md").read_text()
        assert proc.returncode == 1, (name, proc.returncode)
        assert "Candidate source-state guard: FAIL" in summary, name
        assert "Exit status: 1." in summary, name
        assert "| Candidate | FAIL |" in summary, name
        assert source.read_bytes() != before_bytes, name
        source.write_bytes(before_bytes)
        assert run("git", "status", "--porcelain", "--untracked-files=all", cwd=base) == before_status
        if dirty:
            source.write_bytes(clean_bytes)
        del env["FIXTURE_MUTATE_CANDIDATE"]
        assert run("git", "status", "--porcelain", "--untracked-files=all", cwd=base) == ""
        print(f"PASS {name}: mutation detected, process exit=1, summary exit=1")

    mutation_case("mutation-clean-source", dirty=False)
    mutation_case("mutation-already-dirty-source", dirty=True)
    case("compatible", good, 0, "PASS", "PASS")
    (base / "src/lib.rs").write_text('pub fn replacement() -> u8 { 42 }\n')
    removed = commit(base, "Remove consumed symbol")
    case("removed-symbol", good, 1, "FAIL", "PASS")
    assert "cannot find function `consumed`" in (results / "removed-symbol/candidate.log").read_text()
    case("both-fail", removed, 1, "FAIL", "FAIL")
    (base / "src/lib.rs").write_text('pub fn consumed() -> u8 { 42 }\n')
    case("baseline-only-fails", removed, 0, "PASS", "FAIL")
print("All Git-source Cargo fixture cases passed; candidate and consumer trees stayed unchanged.")
