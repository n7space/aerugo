#!/bin/bash

set -euo pipefail

if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached -- 'calldwell/*.py')" ]]; then
    poetry run black --check ./*.py
    poetry run mypy ./*.py
    poetry run ruff ./*.py
    poetry run pylint ./*.py
fi

# Check examples if changed
if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached)" =~ "calldwell/examples/" ]]; then
    poetry run black --check ./examples/**/*.py
    poetry run mypy ./examples/**/*.py
    poetry run ruff ./examples/**/*.py
    poetry run pylint ./examples/**/*.py
fi
