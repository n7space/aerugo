#!/bin/sh
poetry run isort ./*.py ./examples/**/*.py
poetry run black ./*.py ./examples/**/*.py
