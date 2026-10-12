#!/bin/sh
# Kullanım: sh tools/v5-kalib-kos.sh <motor> <çıktı.jsonl> [maç=36]
seq 1 ${3:-36} | xargs -P 4 -I{} sh -c "node tools/v5-kalib.js $1 {} > /tmp/kal-\$\$-{}.j" ; cat /tmp/kal-*-*.j > $2; rm -f /tmp/kal-*-*.j
