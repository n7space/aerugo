#!/bin/bash

set -euo pipefail

if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached -- 'calldwell/*.py')" ]]; then
    poetry run black --check calldwell/*.py
    poetry run mypy calldwell/*.py
    poetry run ruff calldwell/*.py
    poetry run pylint calldwell/*.py
fi

# Check examples if changed
if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached)" =~ "calldwell/examples/" ]]; then
    poetry run black --check calldwell/examples/**/*.py
    poetry run mypy calldwell/examples/**/*.py
    poetry run ruff calldwell/examples/**/*.py
    poetry run pylint calldwell/examples/**/*.py
fi
