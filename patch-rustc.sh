#!/usr/bin/env bash
ARGS=()
for arg in "$@"; do
  # Drop the legacy feature flag if passed down
  if [[ "$arg" == *"panic_immediate_abort"* && "$arg" != *"-Cpanic="* ]]; then
    continue
  fi
  ARGS+=("$arg")
done

exec "${ARGS[@]}" -C panic=immediate-abort -Z unstable-options
