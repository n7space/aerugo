#!/bin/bash

set -euo pipefail

echo "Formatting ./*.py"
poetry run black ./*.py
poetry run ruff --fix ./*.py

echo "Formatting ./examples/**/*.py"
poetry run black ./examples/**/*.py
poetry run ruff --fix ./examples/**/*.py
