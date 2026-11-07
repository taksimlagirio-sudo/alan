#!/usr/bin/env bash
# Tek an: secdal_an.sh <klasör> <tohum> <T>   (KOS, ALAN_* ortamdan)
kl="$1"; s="$2"; T="$3"
env ALAN_SECDAL_ALT=1 "$KOS" "$kl/t$s/kayit.json" "$T" secdal "$kl/t$s" 300 once,sut,pas,sur,tut 2> "$kl/t$s/secdal-$T.err" > /dev/null || true
