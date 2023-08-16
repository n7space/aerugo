#!/bin/sh
poetry run flake8 .
poetry run mypy .
