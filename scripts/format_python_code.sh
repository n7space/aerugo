#!/bin/bash

set -euo pipefail

echo "Formatting tests/requirements/test/*.py"
poetry run black tests/requirements/test/*.py
poetry run ruff --fix tests/requirements/test/*.py

echo "Formatting scripts/*.py"
poetry run black scripts/*.py
poetry run ruff --fix scripts/*.py
