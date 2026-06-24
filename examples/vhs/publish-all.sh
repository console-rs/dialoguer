#!/usr/bin/env bash
set -euo pipefail

# Publish all VHS examples to the VHS website.
# requires https://github.com/charmbracelet/vhs to be installed

examples=(
    confirm
    confirm-with-default
    input
    password
    editor
    select
    fuzzy-select
    multi-select
    sort
)
for example in "${examples[@]}"; do
    echo "Publishing ${example}..."
    vhs --quiet --publish examples/vhs/${example}.tape
done
