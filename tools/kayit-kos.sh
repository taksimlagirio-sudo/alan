#!/bin/sh
# Birleşik ölçüm: 16 senaryonun kaydını alır. Kullanım: sh tools/kayit-kos.sh <motor> <çıktı klasörü> [paralel=4]
M=$1; O=$2; PAR=${3:-4}; mkdir -p $O
seq 0 15 | xargs -P$PAR -I{} sh -c "[ -s $O/{}.json.gz ] || node tools/kayit.js $M {} $O/{}.json.gz > $O/{}.log 2>&1"
echo bitti > $O/BITTI
