#!/bin/bash

CURRENT_TAG=$(curl -s https://api.github.com/repos/lcrownover/sexpand/tags | grep name | head -n1 | awk '{print $2}' | tr -d '[,"]')

if [[ "$(uname -a)" == "Darwin"* ]]; then
	ARCH=apple-darwin
else
	ARCH=unknown-linux-gnu
fi

URL="https://github.com/lcrownover/sexpand/releases/download/${CURRENT_TAG}/sexpand-x86_64-${ARCH}.tar.gz"

OUTPUT="/tmp/sexpand-x86_64-${ARCH}.tar.gz"

curl -s -L -o "$OUTPUT" "$URL"

tar -xzf "$OUTPUT" -C /usr/local/bin/
