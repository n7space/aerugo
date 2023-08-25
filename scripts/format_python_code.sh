#!/bin/sh
poetry run isort tests/requirements/test/*.py
poetry run black tests/requirements/test/*.py

poetry run isort scripts/*.py
poetry run black scripts/*.py

pushd ./calldwell > /dev/null
sh ./scripts/format_code.sh
popd > /dev/null
