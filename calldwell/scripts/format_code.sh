#!/bin/bash

set -euo pipefail

echo "Formatting ./*.py"
poetry run black ./*.py

echo "Formatting ./examples/**/*.py"
poetry run black ./examples/**/*.py
