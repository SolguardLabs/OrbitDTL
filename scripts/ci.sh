#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

WINDOWS_ROOT=""
if command -v wslpath >/dev/null 2>&1 \
  && command -v powershell.exe >/dev/null 2>&1 \
  && powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "exit 0" >/dev/null 2>&1; then
  WINDOWS_ROOT="$(wslpath -w "$ROOT_DIR" 2>/dev/null || true)"
fi

run_powershell() {
  local command="$1"
  local escaped_root="${WINDOWS_ROOT//\'/\'\'}"
  powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "Set-Location -LiteralPath '$escaped_root'; $command"
}

run_cargo() {
  if command -v cargo >/dev/null 2>&1; then
    cargo "$@"
  elif [[ -n "$WINDOWS_ROOT" ]]; then
    run_powershell "cargo $*"
  else
    echo "cargo no esta disponible en PATH" >&2
    exit 127
  fi
}

run_bun() {
  if command -v bun >/dev/null 2>&1; then
    bun "$@"
  elif [[ -n "$WINDOWS_ROOT" ]]; then
    run_powershell "bun $*"
  else
    echo "bun no esta disponible en PATH" >&2
    exit 127
  fi
}

run_bun install --frozen-lockfile

run_cargo fmt --all -- --check
run_cargo build --all-targets --locked
run_cargo test --locked
run_cargo clippy --all-targets --all-features --locked -- -D warnings

run_bun run fmt:check
run_bun run build
run_bun test --timeout 30000 ./tests/node
