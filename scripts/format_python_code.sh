#!/bin/bash

set -euo pipefail

poetry run isort tests/requirements/test/*.py
poetry run black tests/requirements/test/*.py

poetry run isort scripts/*.py
poetry run black scripts/*.py
