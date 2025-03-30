#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 Shun Sakai
#
# SPDX-License-Identifier: Apache-2.0 OR MIT

set -euxCo pipefail

scriptDir=$(cd "$(dirname "$0")" && pwd)
cd "$scriptDir"

curl -L -O --skip-existing https://github.com/google/zopfli/archive/refs/tags/zopfli-1.0.3.tar.gz
echo "e955a7739f71af37ef3349c4fa141c648e8775bceb2195be07e86f8e638814bd  zopfli-1.0.3.tar.gz" | sha256sum -c
gunzip zopfli-1.0.3.tar.gz
mkdir gzip zopfli
rsync -ac zopfli-1.0.3.tar gzip
rsync -ac zopfli-1.0.3.tar zopfli

autocast --overwrite demo.yaml demo.cast
agg --font-family "Cascadia Code,Hack,Source Code Pro" demo.cast demo.gif
gifsicle -b -O3 demo.gif
