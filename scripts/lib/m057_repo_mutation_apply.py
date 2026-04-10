#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import shlex
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any
from urllib.parse import urlparse

from m057_repo_mutation_plan import PLAN_JSON_FILENAME, PlanError, validate_plan
from m057_tracker_inventory import (
    HYPERPUSH_REPO,
    InventoryError,
    MESH_LANG_REPO,
    ROOT,
    iso_now,
    require_array,
    require_int,
    require_object,
    require_string,
    write_json_atomic,
)

SCRIPT_RELATIVE_PATH = "scripts/lib/m057_repo_mutation_apply.py"
RESULTS_JSON_FILENAME = "repo-mutation-results.json"
RESULTS_VERSION = "m057-s02-repo-mutation-results-v1"
API_VERSION = "2022-11-28"
ACCEPT_HEADER = "Accept: application/vnd.github+json"
API_VERSION_HEADER = f"X-GitHub-Api-Version: {API_VERSION}"
DEFAULT_OUTPUT_DIR = ROOT / ".gsd" / "milestones" / "M057" / "slices" / "S02"
DEFAULT_SOURCE_DIR = ROOT / ".gsd" / "milestones" / "M057" / "slices" / "S01"
TITLE_SEARCH_FIELDS = "number,title,state,url,closedAt"
OPERATION_PRIORITY = {"transfer": 0, "create": 1, "close": 2, "rewrite": 3}
REPO_LABEL_CACHE: dict[str, list[str]] = {}


class ApplyError(RuntimeError):
    pass


@dataclass
class CommandResult:
    command: list[str]
    display: str
    exit_code: int
    stdout: str
    stderr: str
    started_at: str
    completed_at: str
    timed_out: bool = False
    timeout_seconds: int | None = None

    def summary(self, *, include_output: bool = False) -> dict[str, Any]:
        payload = {
            "command": self.display,
            "exit_code": self.exit_code,
            "started_at": self.started_at,
            "completed_at": self.completed_at,
            "timed_out": self.timed_out,
        }
        if self.timeout_seconds is not None:
            payload["timeout_seconds"] = self.timeout_seconds
        if include_output:
            payload["stdout"] = self.stdout
            payload["stderr"] = self.stderr
        return payload


class CommandFailure(ApplyError):
    def __init__(self, phase: str, result: CommandResult, extra: str | None = None):
        self.phase = phase
        self.result = result
        detail = extra or result.stderr.strip() or result.stdout.strip() or f"exit {result.exit_code}"
        super().__init__(f"{phase}: command failed: {result.display}\n{detail}")


class IncludeResponseFailure(ApplyError):
    def __init__(self, phase: str, result: CommandResult, status_code: int, headers: dict[str, str], body_text: str):
        self.phase = phase
        self.result = result
        self.status_code = status_code
        self.headers = headers
        self.body_text = body_text
        super().__init__(
            f"{phase}: unexpected HTTP {status_code} from {result.display}\n"
            f"body: {body_text[:400].strip()}"
        )


class InspectableOperationFailure(ApplyError):
    def __init__(self, message: str, *, command_log: list[dict[str, Any]] | None = None):
        super().__init__(message)
        self.command_log = command_log or []


def normalize_multiline(value: str | None) -> str:
    if value is None:
        return ""
    return value.replace("\r\n", "\n").strip()


def command_display(command: list[str]) -> str:
    return " ".join(shlex.quote(part) for part in command)


def repo_issue_handle(repo_slug: str, number: int) -> str:
    return f"{repo_slug.split('/')[-1]}#{number}"


def split_repo_slug(repo_slug: str) -> tuple[str, str]:
    owner, repo = repo_slug.split("/", 1)
    return owner, repo


def build_issue_endpoint(repo_slug: str, number: int) -> str:
    owner, repo = split_repo_slug(repo_slug)
    return f"/repos/{owner}/{repo}/issues/{number}"


def run_command(command: list[str], *, input_text: str | None = None, timeout_seconds: int = 60) -> CommandResult:
    started_at = iso_now()
    try:
        completed = subprocess.run(
            command,
            cwd=ROOT,
            input=input_text,
            capture_output=True,
            text=True,
            timeout=timeout_seconds,
            check=False,
        )
    except subprocess.TimeoutExpired as exc:
        completed_at = iso_now()
        stdout = exc.stdout if isinstance(exc.stdout, str) else (exc.stdout.decode("utf8", errors="replace") if exc.stdout else "")
        stderr = exc.stderr if isinstance(exc.stderr, str) else (exc.stderr.decode("utf8", errors="replace") if exc.stderr else "")
        return CommandResult(
            command=command,
            display=command_display(command),
            exit_code=124,
            stdout=stdout,
            stderr=stderr,
            started_at=started_at,
            completed_at=completed_at,
            timed_out=True,
            timeout_seconds=timeout_seconds,
        )
    return CommandResult(
        command=command,
        display=command_display(command),
        exit_code=completed.returncode,
        stdout=completed.stdout,
        stderr=completed.stderr,
        started_at=started_at,
        completed_at=iso_now(),
    )


def ensure_gh_available() -> str:
    gh_path = shutil.which("gh")
    if gh_path is None:
        raise ApplyError("gh CLI not found on PATH")
    return gh_path


def gh_api_command(endpoint: str, *, method: str = "GET", include_headers: bool = False, payload: dict[str, Any] | list[Any] | None = None) -> tuple[list[str], str | None]:
    gh_path = ensure_gh_available()
    command = [gh_path, "api"]
    if include_headers:
        command.append("-i")
    command.extend(["--method", method.upper(), endpoint, "-H", ACCEPT_HEADER, "-H", API_VERSION_HEADER])
    input_text = None
    if payload is not None:
        command.extend(["--input", "-"])
        input_text = json.dumps(payload)
    return command, input_text


def gh_api_json(endpoint: str, *, method: str = "GET", payload: dict[str, Any] | list[Any] | None = None, timeout_seconds: int = 60, phase: str) -> tuple[Any, CommandResult]:
    command, input_text = gh_api_command(endpoint, method=method, payload=payload)
    result = run_command(command, input_text=input_text, timeout_seconds=timeout_seconds)
    if result.exit_code != 0:
        raise CommandFailure(phase, result)
    try:
        body = json.loads(result.stdout) if result.stdout.strip() else None
    except json.JSONDecodeError as exc:
        raise ApplyError(f"{phase}: command returned invalid JSON: {result.display}\n{exc}") from exc
    return body, result


def parse_include_output(output: str) -> tuple[int, dict[str, str], str]:
    normalized = output.replace("\r\n", "\n")
    if "\n\n" not in normalized:
        raise ApplyError(f"include response missing header/body separator:\n{normalized[:400]}")
    header_text, body_text = normalized.split("\n\n", 1)
    header_lines = [line for line in header_text.split("\n") if line.strip()]
    if not header_lines:
        raise ApplyError("include response missing status line")
    status_match = re.match(r"HTTP/\S+\s+(\d{3})", header_lines[0].strip())
    if not status_match:
        raise ApplyError(f"unable to parse HTTP status line: {header_lines[0]!r}")
    headers: dict[str, str] = {}
    for line in header_lines[1:]:
        if ":" not in line:
            continue
        key, value = line.split(":", 1)
        headers[key.strip().lower()] = value.strip()
    return int(status_match.group(1)), headers, body_text.strip()


def gh_api_include(endpoint: str, *, method: str = "GET", timeout_seconds: int = 60, phase: str) -> tuple[int, dict[str, str], Any, str, CommandResult]:
    command, input_text = gh_api_command(endpoint, method=method, include_headers=True)
    result = run_command(command, input_text=input_text, timeout_seconds=timeout_seconds)
    if result.timed_out:
        raise CommandFailure(phase, result)
    status_code, headers, body_text = parse_include_output(result.stdout)
    payload = None
    if body_text:
        try:
            payload = json.loads(body_text)
        except json.JSONDecodeError:
            payload = body_text
    return status_code, headers, payload, body_text, result


def repo_relpath(path: Path) -> str:
    try:
        return str(path.relative_to(ROOT))
    except ValueError:
        return str(path)



def normalize_issue_snapshot(issue: dict[str, Any]) -> dict[str, Any]:
    repo_url = require_string(issue.get("repository_url"), "issue.repository_url")
    parsed_repo = urlparse(repo_url)
    parts = [part for part in parsed_repo.path.split("/") if part]
    if len(parts) != 3 or parts[0] != "repos":
        raise ApplyError(f"unable to derive repo slug from repository_url: {repo_url}")
    repo_slug = f"{parts[1]}/{parts[2]}"
    number = require_int(issue.get("number"), "issue.number")
    html_url = require_string(issue.get("html_url"), "issue.html_url")
    title = require_string(issue.get("title"), "issue.title")
    state = require_string(issue.get("state"), "issue.state").upper()
    body = issue.get("body")
    if not isinstance(body, str):
        raise ApplyError("issue.body must be a string")
    labels = sorted(
        require_string(require_object(label, "issue.label").get("name"), "issue.label.name")
        for label in require_array(issue.get("labels", []), "issue.labels")
    )
    return {
        "repo_slug": repo_slug,
        "number": number,
        "issue_handle": repo_issue_handle(repo_slug, number),
        "issue_url": html_url,
        "state": state,
        "title": title,
        "body": body,
        "labels": labels,
        "closed_at": issue.get("closed_at"),
        "state_reason": issue.get("state_reason"),
        "api_url": require_string(issue.get("url"), "issue.url"),
    }


def fetch_issue_status(repo_slug: str, number: int, *, phase: str) -> tuple[int, dict[str, str], dict[str, Any] | None, str, CommandResult]:
    endpoint = build_issue_endpoint(repo_slug, number)
    status_code, headers, payload, body_text, result = gh_api_include(endpoint, phase=phase)
    if status_code == 200 and isinstance(payload, dict) and "repository_url" in payload:
        return status_code, headers, normalize_issue_snapshot(payload), body_text, result
    return status_code, headers, None, body_text, result


def fetch_issue_comments(repo_slug: str, number: int, *, phase: str) -> tuple[list[dict[str, Any]], CommandResult]:
    endpoint = f"{build_issue_endpoint(repo_slug, number)}/comments?per_page=100"
    payload, result = gh_api_json(endpoint, phase=phase)
    comments: list[dict[str, Any]] = []
    for raw in require_array(payload, f"{phase}.comments"):
        comment = require_object(raw, f"{phase}.comment")
        body = comment.get("body")
        if not isinstance(body, str):
            raise ApplyError(f"{phase}.comment.body must be a string")
        comments.append(
            {
                "id": require_int(comment.get("id"), f"{phase}.comment.id"),
                "html_url": require_string(comment.get("html_url"), f"{phase}.comment.html_url"),
                "body": body,
                "created_at": require_string(comment.get("created_at"), f"{phase}.comment.created_at"),
                "updated_at": require_string(comment.get("updated_at"), f"{phase}.comment.updated_at"),
            }
        )
    return comments, result


def find_matching_comment(repo_slug: str, number: int, body: str, *, phase: str) -> tuple[dict[str, Any] | None, list[dict[str, Any]], CommandResult]:
    comments, result = fetch_issue_comments(repo_slug, number, phase=phase)
    expected = normalize_multiline(body)
    for comment in comments:
        if normalize_multiline(comment["body"]) == expected:
            return comment, comments, result
    return None, comments, result


def search_issue_by_exact_title(repo_slug: str, title: str, *, phase: str) -> tuple[list[dict[str, Any]], CommandResult]:
    gh_path = ensure_gh_available()
    search_query = f'{json.dumps(title)} in:title'
    command = [
        gh_path,
        "issue",
        "list",
        "-R",
        repo_slug,
        "--state",
        "all",
        "--search",
        search_query,
        "--json",
        TITLE_SEARCH_FIELDS,
    ]
    result = run_command(command, timeout_seconds=60)
    if result.exit_code != 0:
        raise CommandFailure(phase, result)
    try:
        payload = json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        raise ApplyError(f"{phase}: invalid JSON from {result.display}\n{exc}") from exc
    matches: list[dict[str, Any]] = []
    for raw in require_array(payload, f"{phase}.matches"):
        issue = require_object(raw, f"{phase}.match")
        candidate_title = require_string(issue.get("title"), f"{phase}.match.title")
        if candidate_title != title:
            continue
        number = require_int(issue.get("number"), f"{phase}.match.number")
        issue_url = require_string(issue.get("url"), f"{phase}.match.url")
        state = require_string(issue.get("state"), f"{phase}.match.state").upper()
        matches.append(
            {
                "repo_slug": repo_slug,
                "number": number,
                "issue_handle": repo_issue_handle(repo_slug, number),
                "issue_url": issue_url,
                "state": state,
                "title": candidate_title,
                "closed_at": issue.get("closedAt"),
            }
        )
    return matches, result


def resolve_assignable_labels(repo_slug: str, requested_labels: list[str], *, phase: str) -> tuple[dict[str, Any], CommandResult | None]:
    normalized_requested = sorted({require_string(label, f"{phase}.requested_label") for label in requested_labels})
    if repo_slug in REPO_LABEL_CACHE:
        repo_labels = REPO_LABEL_CACHE[repo_slug]
        command_result = None
    else:
        owner, repo = split_repo_slug(repo_slug)
        payload, command_result = gh_api_json(f"/repos/{owner}/{repo}/labels?per_page=100", phase=phase)
        repo_labels = sorted(
            {
                require_string(require_object(label, f"{phase}.label").get("name"), f"{phase}.label.name")
                for label in require_array(payload, f"{phase}.labels")
            }
        )
        REPO_LABEL_CACHE[repo_slug] = repo_labels
    repo_label_set = set(repo_labels)
    assignable = [label for label in normalized_requested if label in repo_label_set]
    unavailable = [label for label in normalized_requested if label not in repo_label_set]
    return {
        "requested": normalized_requested,
        "assignable": assignable,
        "unavailable": unavailable,
    }, command_result


def parse_api_issue_location(location: str) -> tuple[str, int]:
    parsed = urlparse(location)
    parts = [part for part in parsed.path.split("/") if part]
    if len(parts) >= 5 and parts[0] == "repos" and parts[3] == "issues":
        return f"{parts[1]}/{parts[2]}", int(parts[4])
    if len(parts) >= 4 and parts[2] == "issues":
        return f"{parts[0]}/{parts[1]}", int(parts[3])
    raise ApplyError(f"unable to parse transferred issue location: {location}")


def compare_labels(actual: list[str], expected: list[str]) -> bool:
    return sorted(actual) == sorted(expected)


def ensure_issue_identity(issue: dict[str, Any], *, repo_slug: str, number: int, phase: str) -> None:
    if issue["repo_slug"] != repo_slug or issue["number"] != number:
        raise ApplyError(
            f"{phase}: expected {repo_issue_handle(repo_slug, number)} but found {issue['issue_handle']}"
        )


def issue_matches_target(issue: dict[str, Any], *, expected_repo: str, expected_state: str | None, expected_title: str, expected_body: str, expected_labels: list[str] | None = None) -> bool:
    if issue["repo_slug"] != expected_repo:
        return False
    if expected_state is not None and issue["state"] != expected_state:
        return False
    if issue["title"] != expected_title:
        return False
    if normalize_multiline(issue["body"]) != normalize_multiline(expected_body):
        return False
    if expected_labels is not None and not compare_labels(issue["labels"], expected_labels):
        return False
    return True


def run_plan_precheck(*, source_root: Path, source_dir: Path, output_dir: Path) -> dict[str, Any]:
    plan_path = output_dir / PLAN_JSON_FILENAME
    if not plan_path.is_file():
        raise ApplyError(f"missing required plan file: {plan_path}")
    try:
        current_payload = json.loads(plan_path.read_text())
    except json.JSONDecodeError as exc:
        raise ApplyError(f"plan file is not valid JSON: {plan_path}\n{exc}") from exc
    validate_plan(require_object(current_payload, "plan"))

    command = [
        sys.executable,
        str(source_root / "scripts" / "lib" / "m057_repo_mutation_plan.py"),
        "--source-root",
        str(source_root),
        "--source-dir",
        str(source_dir),
        "--output-dir",
        str(output_dir),
        "--check",
    ]
    result = run_command(command, timeout_seconds=180)
    if result.exit_code != 0:
        raise CommandFailure("plan-precheck", result)

    try:
        payload = json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        raise ApplyError(f"plan-precheck: planner returned invalid JSON\n{exc}") from exc
    plan = load_plan(output_dir / PLAN_JSON_FILENAME)
    return {
        "command": result.display,
        "exit_code": result.exit_code,
        "started_at": result.started_at,
        "completed_at": result.completed_at,
        "summary": payload,
        "plan_version": require_string(plan.get("version"), "plan.version"),
    }


def load_plan(plan_path: Path) -> dict[str, Any]:
    if not plan_path.is_file():
        raise ApplyError(f"missing required plan file: {plan_path}")
    try:
        payload = json.loads(plan_path.read_text())
    except json.JSONDecodeError as exc:
        raise ApplyError(f"plan file is not valid JSON: {plan_path}\n{exc}") from exc
    plan = require_object(payload, "plan")
    validate_plan(plan)
    return plan


def ordered_operations(plan: dict[str, Any]) -> list[dict[str, Any]]:
    operations = [require_object(operation, "plan.operation") for operation in require_array(plan.get("operations"), "plan.operations")]
    return sorted(
        operations,
        key=lambda operation: (
            OPERATION_PRIORITY[require_string(operation.get("operation_kind"), "operation.operation_kind")],
            require_string(operation.get("canonical_issue_handle"), "operation.canonical_issue_handle")
            if operation.get("canonical_issue_handle") is not None
            else require_string(operation.get("operation_id"), "operation.operation_id"),
        ),
    )


def initialize_results(*, plan: dict[str, Any], output_dir: Path, mode: str, precheck: dict[str, Any]) -> dict[str, Any]:
    return {
        "version": RESULTS_VERSION,
        "generated_at": iso_now(),
        "source_script": SCRIPT_RELATIVE_PATH,
        "mode": mode,
        "output_dir": repo_relpath(output_dir),
        "status": "running",
        "started_at": iso_now(),
        "completed_at": None,
        "source_plan": {
            "path": repo_relpath(output_dir / PLAN_JSON_FILENAME),
            "version": require_string(plan.get("version"), "plan.version"),
            "generated_at": require_string(plan.get("generated_at"), "plan.generated_at"),
            "rollup": require_object(plan.get("rollup"), "plan.rollup"),
        },
        "precheck": precheck,
        "last_attempted_operation_id": None,
        "last_attempted_issue_handle": None,
        "operations": [],
        "rollup": {
            "total": 0,
            "applied": 0,
            "already_satisfied": 0,
            "failed": 0,
        },
    }


def update_rollup(results: dict[str, Any]) -> None:
    operations = require_array(results.get("operations"), "results.operations")
    counts = {"applied": 0, "already_satisfied": 0, "failed": 0}
    for operation in operations:
        status = require_string(require_object(operation, "results.operation").get("status"), "results.operation.status")
        if status in counts:
            counts[status] += 1
    results["rollup"] = {
        "total": len(operations),
        **counts,
    }


def persist_results(results_path: Path, results: dict[str, Any]) -> None:
    update_rollup(results)
    write_json_atomic(results_path, results)


def comment_requirement(operation: dict[str, Any]) -> tuple[bool, str | None]:
    comment = require_object(operation.get("comment"), "operation.comment")
    required = bool(comment.get("required"))
    body = comment.get("body")
    if body is not None and not isinstance(body, str):
        raise ApplyError("operation.comment.body must be a string or null")
    return required, body


def operation_request_snapshot(operation: dict[str, Any]) -> dict[str, Any]:
    identity = require_object(operation.get("identity"), "operation.identity")
    required_comment, comment_body = comment_requirement(operation)
    return {
        "operation_id": require_string(operation.get("operation_id"), "operation.operation_id"),
        "operation_kind": require_string(operation.get("operation_kind"), "operation.operation_kind"),
        "canonical_issue_handle": operation.get("canonical_issue_handle"),
        "canonical_issue_url": operation.get("canonical_issue_url"),
        "identity": identity,
        "title_after": require_string(require_object(operation.get("title"), "operation.title").get("after"), "operation.title.after"),
        "body_after": require_string(require_object(operation.get("body"), "operation.body").get("after"), "operation.body.after"),
        "labels": [require_string(label, "operation.label") for label in require_array(operation.get("labels", []), "operation.labels")],
        "comment_required": required_comment,
        "comment_body": comment_body,
    }


def patch_issue(repo_slug: str, number: int, payload: dict[str, Any], *, phase: str) -> tuple[dict[str, Any], CommandResult]:
    response, result = gh_api_json(build_issue_endpoint(repo_slug, number), method="PATCH", payload=payload, phase=phase)
    issue = normalize_issue_snapshot(require_object(response, phase))
    ensure_issue_identity(issue, repo_slug=repo_slug, number=number, phase=phase)
    return issue, result


def create_issue(repo_slug: str, payload: dict[str, Any], *, phase: str) -> tuple[dict[str, Any], CommandResult]:
    owner, repo = split_repo_slug(repo_slug)
    response, result = gh_api_json(f"/repos/{owner}/{repo}/issues", method="POST", payload=payload, phase=phase)
    issue = normalize_issue_snapshot(require_object(response, phase))
    if issue["repo_slug"] != repo_slug:
        raise ApplyError(f"{phase}: create response targeted {issue['repo_slug']} instead of {repo_slug}")
    return issue, result


def create_comment(repo_slug: str, number: int, body: str, *, phase: str) -> tuple[dict[str, Any], CommandResult]:
    endpoint = f"{build_issue_endpoint(repo_slug, number)}/comments"
    response, result = gh_api_json(endpoint, method="POST", payload={"body": body}, phase=phase)
    comment = require_object(response, phase)
    return {
        "id": require_int(comment.get("id"), f"{phase}.id"),
        "html_url": require_string(comment.get("html_url"), f"{phase}.html_url"),
        "body": require_string(comment.get("body"), f"{phase}.body"),
    }, result


def close_issue(repo_slug: str, number: int, *, phase: str) -> tuple[dict[str, Any], CommandResult]:
    return patch_issue(repo_slug, number, {"state": "closed", "state_reason": "completed"}, phase=phase)


def transfer_issue(source_repo_slug: str, number: int, destination_repo_slug: str, *, phase: str) -> CommandResult:
    gh_path = ensure_gh_available()
    command = [gh_path, "issue", "transfer", str(number), destination_repo_slug, "-R", source_repo_slug]
    result = run_command(command, timeout_seconds=180)
    if result.exit_code != 0:
        raise CommandFailure(phase, result)
    return result


def resolve_transferred_target(source_repo_slug: str, number: int, *, phase: str) -> tuple[dict[str, Any], list[dict[str, Any]]]:
    command_log: list[dict[str, Any]] = []
    status_code, headers, issue, body_text, include_result = fetch_issue_status(source_repo_slug, number, phase=f"{phase}-source-status")
    command_log.append(include_result.summary(include_output=status_code not in {200, 301}))
    if status_code == 301:
        location = headers.get("location")
        if not location:
            raise InspectableOperationFailure(
                f"{phase}: source issue returned HTTP 301 without a Location header",
                command_log=command_log,
            )
        target_repo_slug, target_number = parse_api_issue_location(location)
        follow_status, _, target_issue, _, follow_result = fetch_issue_status(target_repo_slug, target_number, phase=f"{phase}-follow-location")
        command_log.append(follow_result.summary(include_output=follow_status != 200))
        if follow_status != 200 or target_issue is None:
            raise InspectableOperationFailure(
                f"{phase}: transferred issue location {location} did not resolve to a canonical issue row",
                command_log=command_log,
            )
        return target_issue, command_log
    if status_code == 200 and issue is not None and issue["repo_slug"] != source_repo_slug:
        return issue, command_log
    if status_code == 404:
        raise InspectableOperationFailure(
            f"{phase}: source issue returned 404 instead of the expected redirect or followed transfer response after transfer",
            command_log=command_log,
        )
    if status_code == 200 and issue is not None:
        raise InspectableOperationFailure(
            f"{phase}: source issue still exists in {source_repo_slug}; transfer did not change identity",
            command_log=command_log,
        )
    raise InspectableOperationFailure(
        f"{phase}: unexpected HTTP {status_code} while resolving transferred issue\n{body_text[:400]}",
        command_log=command_log,
    )


def execute_rewrite(operation: dict[str, Any]) -> dict[str, Any]:
    request = operation_request_snapshot(operation)
    identity = require_object(operation.get("identity"), "operation.identity")
    before = require_object(identity.get("before"), "operation.identity.before")
    repo_slug = require_string(before.get("repo_slug"), "operation.identity.before.repo_slug")
    number = require_int(operation.get("issue_number"), "operation.issue_number")

    command_log: list[dict[str, Any]] = []
    status_code, _, current_issue, body_text, fetch_result = fetch_issue_status(repo_slug, number, phase=f"{request['operation_id']}-fetch")
    command_log.append(fetch_result.summary(include_output=status_code != 200))
    if status_code != 200 or current_issue is None:
        raise InspectableOperationFailure(
            f"{request['operation_id']}: expected live issue {repo_issue_handle(repo_slug, number)} before rewrite; got HTTP {status_code}\n{body_text[:400]}",
            command_log=command_log,
        )

    if issue_matches_target(
        current_issue,
        expected_repo=repo_slug,
        expected_state=None,
        expected_title=request["title_after"],
        expected_body=request["body_after"],
    ):
        return {
            "status": "already_satisfied",
            "skipped_reason": "title_and_body_already_match_plan",
            "identity": {"before": before, "after": current_issue, "changes_identity": False},
            "final_state": current_issue,
            "command_log": command_log,
        }

    updated_issue, patch_result = patch_issue(
        repo_slug,
        number,
        {"title": request["title_after"], "body": request["body_after"]},
        phase=f"{request['operation_id']}-patch",
    )
    command_log.append(patch_result.summary())
    if not issue_matches_target(
        updated_issue,
        expected_repo=repo_slug,
        expected_state=None,
        expected_title=request["title_after"],
        expected_body=request["body_after"],
    ):
        raise InspectableOperationFailure(
            f"{request['operation_id']}: rewrite response did not match the planned title/body",
            command_log=command_log,
        )

    return {
        "status": "applied",
        "skipped_reason": None,
        "identity": {"before": before, "after": updated_issue, "changes_identity": False},
        "final_state": updated_issue,
        "command_log": command_log,
    }


def execute_close(operation: dict[str, Any]) -> dict[str, Any]:
    request = operation_request_snapshot(operation)
    identity = require_object(operation.get("identity"), "operation.identity")
    before = require_object(identity.get("before"), "operation.identity.before")
    repo_slug = require_string(before.get("repo_slug"), "operation.identity.before.repo_slug")
    number = require_int(operation.get("issue_number"), "operation.issue_number")
    comment_required, comment_body = comment_requirement(operation)
    if not comment_required or comment_body is None:
        raise ApplyError(f"{request['operation_id']}: close operation missing required comment")

    command_log: list[dict[str, Any]] = []
    status_code, _, current_issue, body_text, fetch_result = fetch_issue_status(repo_slug, number, phase=f"{request['operation_id']}-fetch")
    command_log.append(fetch_result.summary(include_output=status_code != 200))
    if status_code != 200 or current_issue is None:
        raise InspectableOperationFailure(
            f"{request['operation_id']}: expected live issue {repo_issue_handle(repo_slug, number)} before close; got HTTP {status_code}\n{body_text[:400]}",
            command_log=command_log,
        )

    matching_comment, _, comments_result = find_matching_comment(repo_slug, number, comment_body, phase=f"{request['operation_id']}-comment-check")
    command_log.append(comments_result.summary())
    if current_issue["state"] == "CLOSED" and matching_comment is not None:
        return {
            "status": "already_satisfied",
            "skipped_reason": "issue_already_closed_with_matching_comment",
            "identity": {"before": before, "after": current_issue, "changes_identity": False},
            "final_state": current_issue,
            "matching_comment": matching_comment,
            "command_log": command_log,
        }

    if matching_comment is None:
        created_comment, create_comment_result = create_comment(repo_slug, number, comment_body, phase=f"{request['operation_id']}-comment")
        command_log.append(create_comment_result.summary())
        matching_comment = created_comment

    if current_issue["state"] != "CLOSED":
        closed_issue, close_result = close_issue(repo_slug, number, phase=f"{request['operation_id']}-close")
        command_log.append(close_result.summary())
    else:
        closed_issue = current_issue

    if closed_issue["state"] != "CLOSED":
        raise InspectableOperationFailure(
            f"{request['operation_id']}: close response did not leave the issue closed",
            command_log=command_log,
        )

    matching_comment, _, post_comments_result = find_matching_comment(repo_slug, number, comment_body, phase=f"{request['operation_id']}-comment-verify")
    command_log.append(post_comments_result.summary())
    if matching_comment is None:
        raise InspectableOperationFailure(
            f"{request['operation_id']}: closing comment was not visible after the close operation",
            command_log=command_log,
        )

    return {
        "status": "applied",
        "skipped_reason": None,
        "identity": {"before": before, "after": closed_issue, "changes_identity": False},
        "final_state": closed_issue,
        "matching_comment": matching_comment,
        "command_log": command_log,
    }


def execute_create(operation: dict[str, Any]) -> dict[str, Any]:
    request = operation_request_snapshot(operation)
    identity = require_object(operation.get("identity"), "operation.identity")
    after_identity = require_object(identity.get("after"), "operation.identity.after")
    repo_slug = require_string(after_identity.get("repo_slug"), "operation.identity.after.repo_slug")
    comment_required, comment_body = comment_requirement(operation)
    if not comment_required or comment_body is None:
        raise ApplyError(f"{request['operation_id']}: create operation missing required close comment")

    command_log: list[dict[str, Any]] = []
    label_resolution, labels_result = resolve_assignable_labels(repo_slug, request["labels"], phase=f"{request['operation_id']}-labels")
    if labels_result is not None:
        command_log.append(labels_result.summary())
    assignable_labels = label_resolution["assignable"]

    matches, search_result = search_issue_by_exact_title(repo_slug, request["title_after"], phase=f"{request['operation_id']}-search")
    command_log.append(search_result.summary())
    if len(matches) > 1:
        raise InspectableOperationFailure(
            f"{request['operation_id']}: multiple issues already match the retrospective title {request['title_after']!r}",
            command_log=command_log,
        )

    issue: dict[str, Any]
    created = False
    if len(matches) == 1:
        existing = matches[0]
        status_code, _, fetched_issue, body_text, fetch_result = fetch_issue_status(repo_slug, existing["number"], phase=f"{request['operation_id']}-fetch-existing")
        command_log.append(fetch_result.summary(include_output=status_code != 200))
        if status_code != 200 or fetched_issue is None:
            raise InspectableOperationFailure(
                f"{request['operation_id']}: matched existing issue {existing['issue_handle']} but could not fetch it\n{body_text[:400]}",
                command_log=command_log,
            )
        issue = fetched_issue
        if not issue_matches_target(
            issue,
            expected_repo=repo_slug,
            expected_state=None,
            expected_title=request["title_after"],
            expected_body=request["body_after"],
            expected_labels=assignable_labels,
        ):
            raise InspectableOperationFailure(
                f"{request['operation_id']}: existing retrospective issue matches the title but not the planned body/assignable labels",
                command_log=command_log,
            )
    else:
        issue, create_result = create_issue(
            repo_slug,
            {"title": request["title_after"], "body": request["body_after"], "labels": assignable_labels},
            phase=f"{request['operation_id']}-create",
        )
        command_log.append(create_result.summary())
        created = True
        if not issue_matches_target(
            issue,
            expected_repo=repo_slug,
            expected_state="OPEN",
            expected_title=request["title_after"],
            expected_body=request["body_after"],
            expected_labels=assignable_labels,
        ):
            raise InspectableOperationFailure(
                f"{request['operation_id']}: create response did not match the planned issue payload",
                command_log=command_log,
            )

    matching_comment, _, comments_result = find_matching_comment(repo_slug, issue["number"], comment_body, phase=f"{request['operation_id']}-comment-check")
    command_log.append(comments_result.summary())
    if issue["state"] == "CLOSED" and matching_comment is not None:
        return {
            "status": "already_satisfied" if not created else "applied",
            "skipped_reason": None if created else "retrospective_issue_already_exists_closed",
            "identity": {"before": None, "after": issue, "changes_identity": True},
            "final_state": issue,
            "label_resolution": label_resolution,
            "matching_comment": matching_comment,
            "command_log": command_log,
        }

    if matching_comment is None:
        created_comment, comment_result = create_comment(repo_slug, issue["number"], comment_body, phase=f"{request['operation_id']}-comment")
        command_log.append(comment_result.summary())
        matching_comment = created_comment

    if issue["state"] != "CLOSED":
        issue, close_result = close_issue(repo_slug, issue["number"], phase=f"{request['operation_id']}-close")
        command_log.append(close_result.summary())

    if issue["state"] != "CLOSED":
        raise InspectableOperationFailure(
            f"{request['operation_id']}: retrospective issue was not closed after creation",
            command_log=command_log,
        )

    matching_comment, _, verify_comment_result = find_matching_comment(repo_slug, issue["number"], comment_body, phase=f"{request['operation_id']}-comment-verify")
    command_log.append(verify_comment_result.summary())
    if matching_comment is None:
        raise InspectableOperationFailure(
            f"{request['operation_id']}: retrospective close comment was not visible after closing",
            command_log=command_log,
        )

    return {
        "status": "applied" if created or issue["state"] == "CLOSED" else "already_satisfied",
        "skipped_reason": None if created else "retrospective_issue_existed_open_and_was_completed",
        "identity": {"before": None, "after": issue, "changes_identity": True},
        "final_state": issue,
        "label_resolution": label_resolution,
        "matching_comment": matching_comment,
        "command_log": command_log,
    }


def execute_transfer(operation: dict[str, Any]) -> dict[str, Any]:
    request = operation_request_snapshot(operation)
    identity = require_object(operation.get("identity"), "operation.identity")
    before = require_object(identity.get("before"), "operation.identity.before")
    target = require_object(identity.get("after"), "operation.identity.after")
    source_repo_slug = require_string(before.get("repo_slug"), "operation.identity.before.repo_slug")
    target_repo_slug = require_string(target.get("repo_slug"), "operation.identity.after.repo_slug")
    number = require_int(operation.get("issue_number"), "operation.issue_number")

    command_log: list[dict[str, Any]] = []
    label_resolution, labels_result = resolve_assignable_labels(target_repo_slug, request["labels"], phase=f"{request['operation_id']}-labels")
    if labels_result is not None:
        command_log.append(labels_result.summary())
    assignable_labels = label_resolution["assignable"]

    status_code, _, current_issue, body_text, fetch_result = fetch_issue_status(source_repo_slug, number, phase=f"{request['operation_id']}-source-fetch")
    command_log.append(fetch_result.summary(include_output=status_code not in {200, 301}))

    transferred = False
    if status_code == 200 and current_issue is not None and current_issue["repo_slug"] == source_repo_slug:
        transfer_result = transfer_issue(source_repo_slug, number, target_repo_slug, phase=f"{request['operation_id']}-transfer")
        command_log.append(transfer_result.summary())
        transferred = True
        current_issue, resolution_log = resolve_transferred_target(source_repo_slug, number, phase=f"{request['operation_id']}-resolve")
        command_log.extend(resolution_log)
    elif status_code == 200 and current_issue is not None and current_issue["repo_slug"] == target_repo_slug:
        pass
    elif status_code == 301:
        current_issue, resolution_log = resolve_transferred_target(source_repo_slug, number, phase=f"{request['operation_id']}-resolve-existing")
        command_log.extend(resolution_log)
    else:
        raise InspectableOperationFailure(
            f"{request['operation_id']}: expected source issue {repo_issue_handle(source_repo_slug, number)} to exist or redirect; got HTTP {status_code}\n{body_text[:400]}",
            command_log=command_log,
        )

    if current_issue["repo_slug"] != target_repo_slug:
        raise InspectableOperationFailure(
            f"{request['operation_id']}: transferred issue resolved to {current_issue['repo_slug']} instead of {target_repo_slug}",
            command_log=command_log,
        )

    if issue_matches_target(
        current_issue,
        expected_repo=target_repo_slug,
        expected_state=None,
        expected_title=request["title_after"],
        expected_body=request["body_after"],
        expected_labels=assignable_labels,
    ):
        return {
            "status": "applied" if transferred else "already_satisfied",
            "skipped_reason": None if transferred else "issue_already_transferred_and_normalized",
            "identity": {"before": before, "after": current_issue, "changes_identity": True},
            "final_state": current_issue,
            "label_resolution": label_resolution,
            "command_log": command_log,
        }

    normalized_issue, patch_result = patch_issue(
        target_repo_slug,
        current_issue["number"],
        {
            "title": request["title_after"],
            "body": request["body_after"],
            "labels": assignable_labels,
        },
        phase=f"{request['operation_id']}-normalize-target",
    )
    command_log.append(patch_result.summary())
    if not issue_matches_target(
        normalized_issue,
        expected_repo=target_repo_slug,
        expected_state=None,
        expected_title=request["title_after"],
        expected_body=request["body_after"],
        expected_labels=assignable_labels,
    ):
        raise InspectableOperationFailure(
            f"{request['operation_id']}: transferred issue did not match planned title/body/assignable labels after normalization",
            command_log=command_log,
        )

    return {
        "status": "applied",
        "skipped_reason": None if transferred else "issue_already_transferred_but_needed_normalization",
        "identity": {"before": before, "after": normalized_issue, "changes_identity": True},
        "final_state": normalized_issue,
        "label_resolution": label_resolution,
        "command_log": command_log,
    }


def execute_operation(operation: dict[str, Any]) -> dict[str, Any]:
    operation_kind = require_string(operation.get("operation_kind"), "operation.operation_kind")
    if operation_kind == "rewrite":
        return execute_rewrite(operation)
    if operation_kind == "close":
        return execute_close(operation)
    if operation_kind == "create":
        return execute_create(operation)
    if operation_kind == "transfer":
        return execute_transfer(operation)
    raise ApplyError(f"unsupported operation kind {operation_kind!r}")


def build_operation_result(operation: dict[str, Any], *, index: int, outcome: dict[str, Any], error: str | None = None) -> dict[str, Any]:
    request = operation_request_snapshot(operation)
    result = {
        "index": index,
        "operation_id": request["operation_id"],
        "operation_kind": request["operation_kind"],
        "canonical_issue_handle": request["canonical_issue_handle"],
        "canonical_issue_url": request["canonical_issue_url"],
        "status": outcome.get("status", "failed"),
        "requested": request,
        "identity": outcome.get("identity"),
        "final_state": outcome.get("final_state"),
        "label_resolution": outcome.get("label_resolution"),
        "skipped_reason": outcome.get("skipped_reason"),
        "matching_comment": outcome.get("matching_comment"),
        "started_at": outcome.get("started_at"),
        "completed_at": outcome.get("completed_at"),
        "command_log": outcome.get("command_log", []),
    }
    if error is not None:
        result["error"] = error
    return result


def apply_plan(*, plan: dict[str, Any], output_dir: Path, precheck: dict[str, Any]) -> dict[str, Any]:
    results_path = output_dir / RESULTS_JSON_FILENAME
    results = initialize_results(plan=plan, output_dir=output_dir, mode="apply", precheck=precheck)
    persist_results(results_path, results)

    operations = ordered_operations(plan)
    for index, operation in enumerate(operations, start=1):
        operation_id = require_string(operation.get("operation_id"), "operation.operation_id")
        results["last_attempted_operation_id"] = operation_id
        results["last_attempted_issue_handle"] = operation.get("canonical_issue_handle") or operation.get("surface")
        started_at = iso_now()
        try:
            outcome = execute_operation(operation)
            outcome["started_at"] = started_at
            outcome["completed_at"] = iso_now()
            results["operations"].append(build_operation_result(operation, index=index, outcome=outcome))
            persist_results(results_path, results)
        except (ApplyError, CommandFailure, IncludeResponseFailure, InspectableOperationFailure, InventoryError, PlanError) as exc:
            command_log = getattr(exc, "command_log", [])
            failure_result = {
                "status": "failed",
                "identity": require_object(operation.get("identity"), "operation.identity"),
                "final_state": None,
                "skipped_reason": None,
                "matching_comment": None,
                "started_at": started_at,
                "completed_at": iso_now(),
                "command_log": command_log,
            }
            if isinstance(exc, CommandFailure):
                failure_result["command_log"] = [*command_log, exc.result.summary(include_output=True)]
            results["operations"].append(build_operation_result(operation, index=index, outcome=failure_result, error=str(exc)))
            results["status"] = "failed"
            results["completed_at"] = iso_now()
            persist_results(results_path, results)
            raise

    results["status"] = "ok"
    results["completed_at"] = iso_now()
    persist_results(results_path, results)
    return results


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Apply the M057 S02 repo mutation plan against live GitHub issues.")
    parser.add_argument("--source-root", type=Path, default=ROOT, help="Alternate source root for isolated tests.")
    parser.add_argument("--source-dir", type=Path, help="Directory containing the immutable S01 ledger and snapshots.")
    parser.add_argument("--output-dir", type=Path, default=DEFAULT_OUTPUT_DIR, help="Directory containing the checked plan and receiving results.")
    parser.add_argument("--check", action="store_true", help="Validate the checked plan and print a dry-run summary without mutating GitHub.")
    parser.add_argument("--apply", action="store_true", help="Apply the checked plan to live GitHub issues and persist results.")
    args = parser.parse_args(argv)
    if args.apply and args.check:
        parser.error("choose only one of --check or --apply")
    if not args.apply and not args.check:
        args.check = True
    args.source_root = args.source_root.resolve()
    args.source_dir = (args.source_root / DEFAULT_SOURCE_DIR.relative_to(ROOT)).resolve() if args.source_dir is None else args.source_dir.resolve()
    args.output_dir = args.output_dir.resolve()
    return args


def main(argv: list[str]) -> int:
    args = parse_args(argv)
    precheck = run_plan_precheck(source_root=args.source_root, source_dir=args.source_dir, output_dir=args.output_dir)
    plan = load_plan(args.output_dir / PLAN_JSON_FILENAME)
    operations = ordered_operations(plan)

    if args.check:
        print(
            json.dumps(
                {
                    "status": "ok",
                    "mode": "check",
                    "output_dir": str(args.output_dir),
                    "plan_path": str((args.output_dir / PLAN_JSON_FILENAME)),
                    "plan_version": require_string(plan.get("version"), "plan.version"),
                    "precheck": precheck,
                    "operation_count": len(operations),
                    "ordered_operation_ids": [require_string(operation.get("operation_id"), "operation.operation_id") for operation in operations],
                },
                indent=2,
            )
        )
        return 0

    results = apply_plan(plan=plan, output_dir=args.output_dir, precheck=precheck)
    print(
        json.dumps(
            {
                "status": results["status"],
                "mode": "apply",
                "output_dir": str(args.output_dir),
                "results_path": str(args.output_dir / RESULTS_JSON_FILENAME),
                "rollup": results["rollup"],
                "last_attempted_operation_id": results["last_attempted_operation_id"],
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except (ApplyError, CommandFailure, IncludeResponseFailure, InspectableOperationFailure, InventoryError, PlanError) as exc:
        print(str(exc), file=sys.stderr)
        raise SystemExit(1)
