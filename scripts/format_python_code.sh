#!/bin/bash

set -euo pipefail

echo "Formatting tests/requirements/test/*.py"
poetry run isort tests/requirements/test/*.py
poetry run black tests/requirements/test/*.py

echo "Formatting scripts/*.py"
poetry run isort scripts/*.py
poetry run black scripts/*.py
