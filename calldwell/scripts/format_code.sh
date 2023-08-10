#!/bin/sh
poetry run isort ./*.py
poetry run black ./*.py
