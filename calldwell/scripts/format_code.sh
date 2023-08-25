#!/bin/sh
poetry run isort ./*.py
poetry run isort ./examples/**/*.py
poetry run black ./*.py
poetry run black ./examples/**/*.py
