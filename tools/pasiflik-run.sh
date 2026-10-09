#!/bin/sh
# Pasiflik deneyi parçası: dört motoru kurar, lab/pasiflik/jobs-<k>.txt içindeki maçları tüm çekirdeklerde oynatır → lab/pasiflik/out-<k>.jsonl
set -e; k="$1"; cd "$(dirname "$0")/.."; sh tools/deney-pasiflik-kur.sh >/dev/null
: > "lab/pasiflik/out-$k.jsonl"; grep . "lab/pasiflik/jobs-$k.txt" | xargs -P "$(nproc)" -L 1 node tools/pasiflik.js >> "lab/pasiflik/out-$k.jsonl"; wc -l "lab/pasiflik/out-$k.jsonl"
