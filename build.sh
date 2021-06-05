#!/bin/bash
poetry install
poetry run maturin develop
poetry run pytest -s