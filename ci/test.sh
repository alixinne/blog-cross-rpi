#!/bin/bash

set -euo pipefail

# Needed because we're capturing Python interpreter output
export PYTHONUNBUFFERED=1

# List of recorded failures for the current test run
FAILURES=()

# List of feature sets we are testing
FEATURE_COMBINATIONS=("" "sql" "python" "sql,python")

# Run tests for all feature combinations
for FEATURE_SET in "${FEATURE_COMBINATIONS[@]}"; do
  echo -e "==== \033[1mTesting '${FEATURE_SET}'\033[0m ====" >&2

  # Build the project
  if cargo build --features "$FEATURE_SET" "$@"; then
    OUTPUT=$(cargo run -q --features "$FEATURE_SET" "$@" 2>&1)
    FILENAME=data/"${FEATURE_SET:-none}".txt

    if [ "${RECORD:-}" = "1" ]; then
      # Record the output in the data/ dir
      cat <<<"$OUTPUT" >$FILENAME
    else
      # Compare the expected output with the actual output
      EXPECTED_OUTPUT=$(cat $FILENAME)

      if ! diff -u --color <(cat <<< "$OUTPUT") <(cat <<< "$EXPECTED_OUTPUT"); then
        # Record execution failure
        FAILURES+=("$FEATURE_SET: invalid output")
      fi
    fi
  else
    # Record build failure
    FAILURES+=("$FEATURE_SET: build failed")
  fi

  echo >&2
done

# Print testing summary and exit
echo -e "==== \033[1mSummary\033[0m ====" >&2
FAILURE_COUNT=${#FAILURES[@]}
if [ $FAILURE_COUNT -eq 0 ]; then
  echo -e "\033[32mAll tests successful." >&2
  echo -e "\033[0m" >&2
  exit 0
else
  echo -e "\033[31mSome tests failed:" >&2
  for FAILURE in "${FAILURES[@]}"; do
    echo "* ${FAILURE}" >&2
  done
  echo -e "\033[0m" >&2
  exit 1
fi

# vim: ft=bash:sw=2:ts=2:et
