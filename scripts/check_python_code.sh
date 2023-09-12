#!/bin/bash

set -euo pipefail

echo "Checking test/requirements/test/*.py"
poetry run black --check tests/requirements/test/*.py
poetry run mypy tests/requirements/test/*.py
poetry run ruff tests/requirements/test/*.py
poetry run pylint tests/requirements/test/*.py

echo "Checking scripts/*.py"
poetry run black --check scripts/*.py
poetry run mypy scripts/*.py
poetry run ruff scripts/*.py
poetry run pylint scripts/*.py
