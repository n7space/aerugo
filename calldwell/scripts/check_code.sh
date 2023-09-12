#!/bin/bash

set -euo pipefail

echo "Checking ./*.py"
poetry run black --check ./*.py
poetry run mypy ./*.py
poetry run ruff ./*.py
poetry run pylint ./*.py

echo "Checking ./examples/**/*.py"
poetry run black --check ./examples/**/*.py
poetry run mypy ./examples/**/*.py
poetry run ruff ./examples/**/*.py
poetry run pylint ./examples/**/*.py
