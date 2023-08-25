#!/bin/sh
poetry run flake8 ./*.py
poetry run flake8 ./examples/**/*.py
poetry run mypy ./*.py
poetry run mypy ./examples/**/*.py
