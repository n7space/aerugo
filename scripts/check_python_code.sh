#!/bin/sh
poetry run flake8 tests/requirements/test/*.py
poetry run flake8 scripts/*.py
poetry run mypy tests/requirements/test/*.py
poetry run mypy scripts/*.py

pushd ./calldwell > /dev/null
sh ./scripts/check_code.sh
popd > /dev/null
