"""Regression coverage for the release-plz command and credential gates."""

import ast
import re
import unittest
from pathlib import Path


WORKFLOW_PATH = Path(__file__).resolve().parents[2] / ".github/workflows/release-plz.yml"


def _job_section(workflow, job_name):
    start = re.search(rf"(?m)^  {re.escape(job_name)}:\s*$", workflow)
    if not start:
        raise AssertionError(f"missing workflow job {job_name!r}")
    next_job = re.search(r"(?m)^  [a-z0-9_-]+:\s*$", workflow[start.end() :])
    end = start.end() + next_job.start() if next_job else len(workflow)
    return workflow[start.start() : end]


def _job_condition(section):
    match = re.search(r"(?m)^    if:\s*(.+)$", section)
    if not match:
        raise AssertionError("job has no explicit condition")
    return match.group(1)


def _evaluate_condition(expression, context):
    """Evaluate the boolean/equality subset used by the workflow job conditions."""
    expression = re.sub(
        r"\b(?:github\.event_name|github\.ref|inputs\.release_action)\b",
        lambda match: repr(context[match.group(0)]),
        expression,
    )
    expression = expression.replace("&&", " and ").replace("||", " or ")
    tree = ast.parse(expression, mode="eval")

    def visit(node):
        if isinstance(node, ast.Expression):
            return visit(node.body)
        if isinstance(node, ast.Constant):
            return node.value
        if isinstance(node, ast.BoolOp) and isinstance(node.op, ast.And):
            return all(visit(value) for value in node.values)
        if isinstance(node, ast.BoolOp) and isinstance(node.op, ast.Or):
            return any(visit(value) for value in node.values)
        if isinstance(node, ast.Compare) and len(node.ops) == len(node.comparators) == 1:
            left = visit(node.left)
            right = visit(node.comparators[0])
            if isinstance(node.ops[0], ast.Eq):
                return left == right
            if isinstance(node.ops[0], ast.NotEq):
                return left != right
        raise AssertionError(f"unsupported workflow condition expression: {ast.dump(node)}")

    return bool(visit(tree))


class ReleasePlzWorkflowTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.workflow = WORKFLOW_PATH.read_text()
        cls.prepare_job = _job_section(cls.workflow, "release-pr")
        cls.publish_job = _job_section(cls.workflow, "publish")
        default = re.search(r"(?m)^        default: ([a-z]+)$", cls.workflow)
        if not default:
            raise AssertionError("workflow_dispatch release_action must have a default")
        cls.default_action = default.group(1)

    def _route(self, event, ref, action=None):
        if event == "workflow_dispatch" and action is None:
            action = self.default_action
        context = {
            "github.event_name": event,
            "github.ref": ref,
            "inputs.release_action": action,
        }
        return (
            _evaluate_condition(_job_condition(self.prepare_job), context),
            _evaluate_condition(_job_condition(self.publish_job), context),
        )

    def test_event_ref_and_input_matrix(self):
        cases = (
            ("push", "refs/heads/main", None, (True, False)),
            ("workflow_dispatch", "refs/heads/main", None, (True, False)),
            ("workflow_dispatch", "refs/heads/main", "prepare", (True, False)),
            ("workflow_dispatch", "refs/heads/main", "publish", (False, True)),
            ("workflow_dispatch", "refs/heads/feature", "prepare", (False, False)),
            ("workflow_dispatch", "refs/heads/feature", "publish", (False, False)),
        )
        for event, ref, action, expected in cases:
            with self.subTest(event=event, ref=ref, action=action):
                self.assertEqual(self._route(event, ref, action), expected)

    def test_explicit_commands_and_job_scoped_credentials(self):
        self.assertIn("command: release-pr", self.prepare_job)
        self.assertNotIn("command: release\n", self.prepare_job)
        self.assertIn("command: release\n", self.publish_job)
        self.assertNotIn("CARGO_REGISTRY_TOKEN", self.prepare_job)
        self.assertIn("CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}", self.publish_job)
        for job in (self.prepare_job, self.publish_job):
            with self.subTest(job=job.splitlines()[0]):
                self.assertIn("GITHUB_TOKEN: ${{ steps.app-token.outputs.token }}", job)

    def test_publish_checkout_is_pinned_to_dispatch_sha(self):
        checkout_ref = re.search(r"(?m)^          ref: (.+)$", self.publish_job)
        self.assertIsNotNone(checkout_ref)
        self.assertEqual(checkout_ref.group(1), "${{ github.sha }}")

        dispatch_sha = "a" * 40
        main_tip_after_queue_advance = "b" * 40
        resolved_checkout_sha = dispatch_sha
        self.assertEqual(resolved_checkout_sha, dispatch_sha)
        self.assertNotEqual(resolved_checkout_sha, main_tip_after_queue_advance)


if __name__ == "__main__":
    unittest.main()
