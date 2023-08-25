#!/bin/bash

set -euo pipefail

# Check tests if changed
if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached)" =~ "tests/requirements/test/" ]]; then
    poetry run isort --check tests/requirements/test/*.py
    poetry run black --check tests/requirements/test/*.py
    poetry run flake8 tests/requirements/test/*.py
    poetry run mypy tests/requirements/test/*.py
fi

# Check scripts if changed
if [[ "$(git diff --diff-filter=d --dirstat=files,0,cumulative --cached)" =~ "scripts/" ]]; then
    poetry run isort --check scripts/*.py
    poetry run black --check scripts/*.py
    poetry run flake8 scripts/*.py
    poetry run mypy scripts/*.py
fi
