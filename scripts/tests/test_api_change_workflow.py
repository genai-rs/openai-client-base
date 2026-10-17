import os
import subprocess
import textwrap
import tomllib
import tempfile
import unittest
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]


class APIChangeWorkflowTests(unittest.TestCase):
    def setUp(self):
        self.workflow = yaml.safe_load((ROOT / ".github/workflows/generate-client.yml").read_text())
        self.steps = self.workflow["jobs"]["generate"]["steps"]

    def named(self, name):
        return next((index, step) for index, step in enumerate(self.steps) if step.get("name") == name)

    def test_baseline_report_downstream_order_and_evidence_artifact(self):
        baseline_i, baseline = self.named("Capture API change baseline")
        generate_i, _ = self.named("Generate client")
        candidate_i, candidate = self.named("Compare generated API with baseline")
        downstream_i, _ = self.named("Check downstream consumer compatibility")
        report_artifact_i, report_artifact = self.named("Upload API change review evidence")
        downstream_artifact_i, downstream_artifact = self.named("Upload downstream compatibility evidence")
        pr_i, pr = self.named("Create Pull Request")

        self.assertLess(baseline_i, generate_i)
        self.assertLess(generate_i, candidate_i)
        self.assertLess(candidate_i, downstream_i)
        self.assertLess(downstream_i, downstream_artifact_i)
        self.assertLess(report_artifact_i, pr_i)
        self.assertEqual(report_artifact["if"], "always()")
        self.assertIn("before.json", report_artifact["with"]["path"])
        self.assertIn("after.json", report_artifact["with"]["path"])
        self.assertIn("report.json", report_artifact["with"]["path"])
        self.assertIn("report.md", report_artifact["with"]["path"])
        self.assertIn("report.md", candidate["run"])
        self.assertIn("GITHUB_STEP_SUMMARY", candidate["run"])
        self.assertIn("review_required", candidate["run"])
        self.assertIn("removals_flagged", candidate["run"])
        self.assertIn("actions/runs/${{ github.run_id }}", pr["with"]["body"])
        self.assertNotIn("automerge", pr["with"]["labels"])
        self.assertIn("removals_flagged", pr["with"]["draft"])
        self.assertIn("== 'true' && 'always-true'", pr["with"]["draft"])
        self.assertIn("stays in draft", pr["with"]["body"])

        self.assertNotIn("Enable auto-merge for generated PR", [s.get("name") for s in self.steps])
        _, cleanup = self.named("Disable existing generated PR auto-merge")
        self.assertLess(self.named("Check for changes")[0], self.steps.index(cleanup))
        self.assertLess(self.steps.index(cleanup), pr_i)
        self.assertIn("--disable-auto", cleanup["run"])
        self.assertIn("--remove-label automerge", cleanup["run"])
        self.assertIn("has_automerge_label=", cleanup["run"])
        self.assertIn("Could not read generated PR labels", cleanup["run"])

    def test_label_api_failure_blocks_update_and_success_removes_before_update(self):
        _, cleanup = self.named("Disable existing generated PR auto-merge")
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            log = root / "gh.log"
            gh = root / "gh"
            gh.write_text(textwrap.dedent("""\
                #!/usr/bin/env python3
                import os
                import sys
                from pathlib import Path
                args = sys.argv[1:]
                with Path(os.environ['GH_LOG']).open('a') as output:
                    output.write(' '.join(args) + '\\n')
                if args[:2] == ['pr', 'list']:
                    print('42')
                elif args[:2] == ['pr', 'view'] and 'autoMergeRequest' in args:
                    print('false')
                elif args[:2] == ['pr', 'view'] and 'labels' in args:
                    if os.environ.get('GH_LABELS_FAIL') == '1':
                        print('GitHub API returned 503', file=sys.stderr)
                        sys.exit(1)
                    print('true')
                elif args[:2] == ['pr', 'edit']:
                    pass
                elif args[:2] == ['pr', 'create']:
                    pass
                else:
                    sys.exit('unexpected gh args: ' + repr(args))
            """))
            gh.chmod(0o755)

            def run_update(fail_labels):
                log.unlink(missing_ok=True)
                env = {**os.environ, "PATH": f"{root}{os.pathsep}{os.environ['PATH']}",
                       "GH_LOG": str(log), "GH_TOKEN": "mock-token",
                       "GH_LABELS_FAIL": "1" if fail_labels else "0"}
                result = subprocess.run(["bash", "-euo", "pipefail", "-c", cleanup["run"]],
                                        env=env, text=True, capture_output=True)
                if result.returncode == 0:
                    subprocess.run([str(gh), "pr", "create"], env=env, check=True, capture_output=True)
                return result, log.read_text().splitlines()

            failure, failure_calls = run_update(fail_labels=True)
            self.assertNotEqual(failure.returncode, 0)
            self.assertIn("Could not read generated PR labels", failure.stderr)
            self.assertFalse(any("pr edit" in call or "pr create" in call for call in failure_calls))

            success, success_calls = run_update(fail_labels=False)
            self.assertEqual(success.returncode, 0, success.stderr)
            self.assertLess(success_calls.index("pr edit 42 --remove-label automerge"),
                            success_calls.index("pr create"))

    def test_release_plz_keeps_manual_review_and_links_generation_evidence(self):
        with (ROOT / "release-plz.toml").open("rb") as source:
            config = tomllib.load(source)
        body = config["workspace"]["pr_body"]
        self.assertIn("automatic api compatibility checks are disabled", body.lower())
        self.assertIn("api-change-report", body)
        self.assertIn("designated senior reviewer approval", body)
        self.assertIn("workflow artifact", body)


if __name__ == "__main__":
    unittest.main()
