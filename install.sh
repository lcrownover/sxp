#!/bin/bash

CURRENT_TAG=$(git tag --sort=taggerdate | tail -1)

if [[ "$(uname -a)" == "Darwin"* ]]; then
	ARCH=apple-darwin
else
	ARCH=unknown-linux-gnu
fi

URL="https://github.com/lcrownover/sexpand/releases/download/${CURRENT_TAG}/sexpand-x86_64-${ARCH}.tar.gz"

OUTPUT="/tmp/sexpand-x86_64-${ARCH}.tar.gz"

curl -s -L -o "$OUTPUT" "$URL"

tar -xzf "$OUTPUT" -C /usr/local/bin/
