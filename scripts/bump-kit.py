#!/usr/bin/env python3
import json
import re
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CARGO_TOML = ROOT / "Cargo.toml"
CARGO_GENERATE = ROOT / "cargo-generate.toml"

CRATES = [
    "barrzen-axum-core",
    "barrzen-axum-infra",
    "barrzen-axum-obs",
    "barrzen-axum-openapi",
]


def fetch_latest(crate: str) -> str:
    url = f"https://crates.io/api/v1/crates/{crate}"
    with urllib.request.urlopen(url) as resp:
        data = json.load(resp)
    version = data.get("crate", {}).get("max_version")
    if not version:
        raise RuntimeError(f"Failed to resolve latest version for {crate}")
    return version


def update_cargo_toml(content: str, version: str) -> str:
    # Replace all barrzen-axum-* dependency version values with the new version.
    def repl(match: re.Match) -> str:
        prefix = match.group(1)
        return f'{prefix}"{version}"'

    pattern = re.compile(r'(barrzen-axum-(?:core|infra|obs|openapi)\s*=\s*\{[^}]*?version\s*=\s*)"[^"]+"')
    new_content, count = pattern.subn(repl, content)
    if count == 0:
        raise RuntimeError("No barrzen-axum-* dependencies updated in Cargo.toml")
    return new_content


def update_cargo_generate(content: str, version: str) -> str:
    pattern = re.compile(r'(kit_version\s*=\s*\{[^}]*?default\s*=\s*)"[^"]+"')
    new_content, count = pattern.subn(rf'\1"{version}"', content)
    if count == 0:
        raise RuntimeError("No kit_version default updated in cargo-generate.toml")
    return new_content


def main() -> int:
    latest_versions = [fetch_latest(crate) for crate in CRATES]
    # Expect all kits to share same version; use the max to be safe.
    latest = max(latest_versions)

    cargo_content = CARGO_TOML.read_text(encoding="utf-8")
    generate_content = CARGO_GENERATE.read_text(encoding="utf-8")

    if "{{kit_version}}" in cargo_content:
        cargo_new = cargo_content
    else:
        cargo_new = update_cargo_toml(cargo_content, latest)
    generate_new = update_cargo_generate(generate_content, latest)

    if cargo_new == cargo_content and generate_new == generate_content:
        print("No changes needed; already at latest version")
        return 0

    CARGO_TOML.write_text(cargo_new, encoding="utf-8")
    CARGO_GENERATE.write_text(generate_new, encoding="utf-8")

    print(f"Updated kit version to {latest}")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as exc:
        print(f"error: {exc}")
        sys.exit(1)
