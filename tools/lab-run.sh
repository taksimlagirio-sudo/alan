#!/bin/sh
# Laboratuvar parçası: lab/jobs-<k>.txt içindeki maçları (motor hücre i) tüm çekirdeklerde oynatır, sonucu lab/out-<k>.jsonl'a yazar.
# eski = düzeltmeler öncesi (b0eaf0e), yeni = üç düzeltme sonrası (d99794b). İki motor da git geçmişinden çıkarılır.
set -e; k="$1"; cd "$(dirname "$0")/.."; T=$(mktemp -d)
for v in eski:b0eaf0e yeni:d99794b; do n=${v%%:*}; c=${v##*:}; mkdir -p "$T/$n"; git archive "$c" alan-v3f | tar -x -C "$T/$n"; done
: > "lab/out-$k.jsonl"
sed "s#^\(eski\|yeni\) #$T/\1/alan-v3f #" "lab/jobs-$k.txt" | grep . | xargs -P "$(nproc)" -L 1 node tools/lab.js >> "lab/out-$k.jsonl"
wc -l "lab/out-$k.jsonl"
