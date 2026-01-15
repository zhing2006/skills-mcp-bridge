#!/usr/bin/env bash
set -euo pipefail

# Ensure CODEX_HOME points at the .codex directory under the current working directory.
export CODEX_HOME="$(pwd)/.codex"

GREEN=$'\033[32m'
RESET=$'\033[0m'

mode=""
mode_label=""
if [[ $# -gt 0 ]]; then
  case "$1" in
    -a1)
      mode="--full-auto"
      mode_label="full-auto"
      shift
      ;;
    -a0)
      mode="--dangerously-bypass-approvals-and-sandbox"
      mode_label="dangerously-bypass-approvals-and-sandbox"
      shift
      ;;
  esac
fi

cmd=(codex)
if [[ -n "$mode" ]]; then
  printf 'Launching Codex in %s%s%s mode...\n' "$GREEN" "$mode_label" "$RESET"
  cmd+=("$mode")
fi
if [[ $# -gt 0 ]]; then
  cmd+=("$@")
fi

exec "${cmd[@]}"
