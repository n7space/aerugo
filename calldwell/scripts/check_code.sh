#!/bin/sh
poetry run flake8 ./*.py ./examples/**/*.py
poetry run mypy ./*.py ./examples/**/*.py
