#!/usr/bin/env python3
"""Render GitHub code scanning alerts as a Markdown checklist."""

from __future__ import annotations

import argparse
import json
import re
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any


Alert = dict[str, Any]


SEVERITY_ORDER = {
    "critical": 0,
    "high": 1,
    "error": 2,
    "medium": 3,
    "warning": 4,
    "low": 5,
    "note": 6,
    "none": 7,
    "": 8,
}


def load_alerts(path: Path) -> list[Alert]:
    text = path.read_text()
    decoder = json.JSONDecoder()
    pos = 0
    alerts: list[Alert] = []

    while pos < len(text):
        while pos < len(text) and text[pos].isspace():
            pos += 1
        if pos >= len(text):
            break

        value, pos = decoder.raw_decode(text, pos)
        if isinstance(value, list):
            alerts.extend(value)
        elif isinstance(value, dict):
            alerts.append(value)
        else:
            raise ValueError(f"unexpected JSON value in {path}: {type(value).__name__}")

    return alerts


def load_resolved(path: Path) -> tuple[set[str], set[str]]:
    """Load persisted resolution state so checkmarks survive regeneration.

    The sidecar file lists, one per line:
      * ``rule:<rule-id>`` to mark every alert of that rule as resolved, and
      * a bare alert number to mark that single alert as resolved.
    ``#`` starts a comment. Returns ``(resolved_rule_ids, resolved_numbers)``.
    """
    rules: set[str] = set()
    numbers: set[str] = set()
    if not path or not path.exists():
        return rules, numbers
    for raw in path.read_text().splitlines():
        line = raw.split("#", 1)[0].strip()
        if not line:
            continue
        if line.startswith("rule:"):
            rules.add(line[len("rule:") :].strip())
        else:
            numbers.add(line)
    return rules, numbers


def is_resolved(alert: Alert, resolved_rules: set[str], resolved_numbers: set[str]) -> bool:
    return rule_id(alert) in resolved_rules or str(alert.get("number")) in resolved_numbers


def rule(alert: Alert) -> dict[str, Any]:
    return alert.get("rule") or {}


def location(alert: Alert) -> dict[str, Any]:
    return ((alert.get("most_recent_instance") or {}).get("location")) or {}


def instance_message(alert: Alert) -> str:
    message = (alert.get("most_recent_instance") or {}).get("message") or {}
    if isinstance(message, dict):
        return str(message.get("text") or "").strip()
    return str(message or "").strip()


def location_key(alert: Alert) -> tuple[str, int, int, int]:
    loc = location(alert)
    return (
        str(loc.get("path") or ""),
        int(loc.get("start_line") or 0),
        int(loc.get("end_line") or 0),
        int(alert.get("number") or 0),
    )


def rule_id(alert: Alert) -> str:
    return str(rule(alert).get("id") or "")


def severity(alert: Alert) -> str:
    return str(rule(alert).get("severity") or "")


def security_severity(alert: Alert) -> str:
    return str(rule(alert).get("security_severity_level") or "")


def description(alert: Alert) -> str:
    rule_data = rule(alert)
    return clean_text(
        str(rule_data.get("full_description") or rule_data.get("description") or "")
    )


def clean_text(text: str) -> str:
    text = re.sub(r"\s+", " ", text).strip()
    return collapse_repeated_sentences(text)


def collapse_repeated_sentences(text: str) -> str:
    sentences = re.split(r"(?<=[.!?`])\s+", text)
    collapsed: list[str] = []
    for sentence in sentences:
        sentence = sentence.strip()
        if sentence and (not collapsed or collapsed[-1] != sentence):
            collapsed.append(sentence)
    return " ".join(collapsed)


def md_escape_cell(text: str) -> str:
    return clean_text(text).replace("|", "\\|")


def format_pos(alert: Alert) -> str:
    loc = location(alert)
    path = str(loc.get("path") or "<unknown>")
    start = loc.get("start_line")
    end = loc.get("end_line")

    if start and end and start != end:
        return f"{path}:{start}-{end}"
    if start:
        return f"{path}:{start}"
    return path


def sort_alert(alert: Alert) -> tuple[int, int, str, str, int, int, int]:
    security = security_severity(alert)
    severity_value = security or severity(alert)
    loc = location(alert)
    return (
        SEVERITY_ORDER.get(severity_value, SEVERITY_ORDER[""]),
        -int(alert.get("number") or 0),
        rule_id(alert),
        str(loc.get("path") or ""),
        int(loc.get("start_line") or 0),
        int(loc.get("end_line") or 0),
        int(alert.get("number") or 0),
    )


def rule_sort_key(item: tuple[str, list[Alert]]) -> tuple[int, int, str]:
    rid, grouped_alerts = item
    representative = grouped_alerts[0]
    severity_value = security_severity(representative) or severity(representative)
    return (
        -len(grouped_alerts),
        SEVERITY_ORDER.get(severity_value, SEVERITY_ORDER[""]),
        rid,
    )


def render_summary_by_rule(alerts: list[Alert]) -> list[str]:
    grouped: dict[str, list[Alert]] = defaultdict(list)
    for alert in alerts:
        grouped[rule_id(alert)].append(alert)

    lines = [
        "## Summary By Rule",
        "",
        "| Rule | Count | Severity | Security | Description |",
        "|---|---:|---|---|---|",
    ]

    for rid, grouped_alerts in sorted(grouped.items(), key=rule_sort_key):
        representative = grouped_alerts[0]
        lines.append(
            "| "
            f"`{md_escape_cell(rid)}` | "
            f"{len(grouped_alerts)} | "
            f"`{md_escape_cell(severity(representative))}` | "
            f"`{md_escape_cell(security_severity(representative))}` | "
            f"{md_escape_cell(description(representative))} |"
        )

    return lines


def render_summary_by_file(alerts: list[Alert]) -> list[str]:
    counts = Counter(str(location(alert).get("path") or "<unknown>") for alert in alerts)

    lines = [
        "## Summary By File",
        "",
        "| File | Count |",
        "|---|---:|",
    ]

    for path, count in sorted(counts.items(), key=lambda item: (-item[1], item[0])):
        lines.append(f"| `{md_escape_cell(path)}` | {count} |")

    return lines


def render_checklist(
    alerts: list[Alert],
    resolved_rules: set[str],
    resolved_numbers: set[str],
) -> list[str]:
    by_rule: dict[str, list[Alert]] = defaultdict(list)
    for alert in alerts:
        by_rule[rule_id(alert)].append(alert)

    lines = ["## Checklist"]

    for rid, grouped_alerts in sorted(by_rule.items(), key=rule_sort_key):
        grouped_alerts.sort(key=lambda alert: location_key(alert))
        representative = grouped_alerts[0]

        lines.extend(
            [
                "",
                f"### `{rid}` ({len(grouped_alerts)})",
                "",
                f"Severity: `{severity(representative)}`. Security: `{security_severity(representative)}`.",
                "",
            ]
        )

        desc = description(representative)
        if desc:
            lines.extend([desc, ""])

        by_file: dict[str, list[Alert]] = defaultdict(list)
        for alert in grouped_alerts:
            by_file[str(location(alert).get("path") or "<unknown>")].append(alert)

        first_file = True
        for path, file_alerts in sorted(by_file.items(), key=lambda item: (-len(item[1]), item[0])):
            file_alerts.sort(key=lambda alert: location_key(alert))
            if not first_file:
                lines.append("")
            first_file = False
            lines.extend([f"#### `{path}` ({len(file_alerts)})", ""])
            for alert in file_alerts:
                number = alert.get("number")
                url = str(alert.get("html_url") or "")
                message = instance_message(alert) or description(alert)
                suffix = f" - {clean_text(message)}" if message else ""
                mark = "x" if is_resolved(alert, resolved_rules, resolved_numbers) else " "
                lines.append(f"- [{mark}] [#{number}]({url}) `{format_pos(alert)}`{suffix}")

    return lines


def render_progress(
    alerts: list[Alert],
    resolved_rules: set[str],
    resolved_numbers: set[str],
) -> list[str]:
    total = len(alerts)
    done = sum(1 for a in alerts if is_resolved(a, resolved_rules, resolved_numbers))
    lines = ["## Progress", "", f"Resolved **{done} / {total}** findings."]
    if resolved_rules:
        lines += ["", "Rules resolved wholesale (verified clean on `main`):", ""]
        lines += [f"- `{r}`" for r in sorted(resolved_rules)]
    lines.append("")
    lines.append(
        "Resolution state is persisted in `code-scanning-resolved.txt` so it"
        " survives regeneration of this report."
    )
    return lines


def render(
    alerts: list[Alert],
    source: Path,
    csv_source: str | None,
    resolved_rules: set[str],
    resolved_numbers: set[str],
) -> str:
    source_note = f"Generated from `{source}`."
    if csv_source:
        source_note += f" Source export: `{csv_source}`."

    lines = [
        "# Code Scanning Todo",
        "",
        source_note,
        "",
        *render_progress(alerts, resolved_rules, resolved_numbers),
        "",
        *render_summary_by_rule(alerts),
        "",
        *render_summary_by_file(alerts),
        "",
        *render_checklist(alerts, resolved_rules, resolved_numbers),
        "",
    ]
    return "\n".join(lines)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Render GitHub code scanning alerts JSON as a Markdown todo list."
    )
    parser.add_argument("input", nargs="?", default="code-scanning.json", type=Path)
    parser.add_argument("-o", "--output", default="code-scanning.md", type=Path)
    parser.add_argument(
        "--csv-source",
        default=None,
        help="Optional CSV export name to mention in the generated report.",
    )
    parser.add_argument(
        "--resolved",
        default="code-scanning-resolved.txt",
        type=Path,
        help="Sidecar file recording resolved rules/alert numbers (persists checkmarks).",
    )
    args = parser.parse_args()

    alerts = load_alerts(args.input)
    resolved_rules, resolved_numbers = load_resolved(args.resolved)
    args.output.write_text(
        render(alerts, args.input, args.csv_source, resolved_rules, resolved_numbers)
    )


if __name__ == "__main__":
    main()
