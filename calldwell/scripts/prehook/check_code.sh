#!/bin/bash

set -euo pipefail

if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached -- './*.py')" ]]; then
    poetry run isort --check ./*.py
    poetry run black --check ./*.py
    poetry run flake8 ./*.py
    poetry run mypy ./*.py
fi

# Check examples if changed
if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached)" =~ "examples/" ]]; then
    poetry run isort --check ./examples/**/*.py
    poetry run black --check ./examples/**/*.py
    poetry run flake8 ./examples/**/*.py
    poetry run mypy ./examples/**/*.py
fi
