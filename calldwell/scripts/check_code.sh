#!/bin/sh
poetry run isort --check ./*.py
poetry run black --check ./*.py
poetry run flake8 ./*.py
poetry run mypy ./*.py

poetry run isort --check ./examples/**/*.py
poetry run black --check ./examples/**/*.py
poetry run flake8 ./examples/**/*.py
poetry run mypy ./examples/**/*.py
