#!/usr/bin/env bash

set -e
set -o pipefail

THIS_SCRIPT_DIR="$( cd "$( dirname "$(readlink -f "${BASH_SOURCE[0]}" )" )" && pwd )"

function installprogram {
    pushd ../
    cargo install --path .
    popd
}

installprogram 1>&2

BINARY="$(which session_scripts)"

ARGS="$1"

CONTENT="$(cat "$THIS_SCRIPT_DIR"/session-scripts.service.template)"

CONTENT="${CONTENT//\$\{BIN\}/${BINARY}}"
CONTENT="${CONTENT//\$\{ARGS\}/${ARGS}}"

echo "$CONTENT"
