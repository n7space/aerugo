#!/bin/bash

set -euo pipefail

# Check tests if changed
if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached -- '*.py')" =~ "tests/requirements/test/" ]]; then
    poetry run black --check tests/requirements/test/*.py
    poetry run mypy tests/requirements/test/*.py
    poetry run ruff tests/requirements/test/*.py
    poetry run pylint tests/requirements/test/*.py
fi

# Check scripts if changed
if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached -- '*.py')" =~ "scripts/" ]]; then
    poetry run black --check scripts/*.py
    poetry run mypy scripts/*.py
    poetry run ruff scripts/*.py
    poetry run pylint scripts/*.py
fi
