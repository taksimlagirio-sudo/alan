#!/usr/bin/env bash
# Saatlik izleme ("hepsi açık"): main + hücum dalı + savunma dalı birleşik uçlarını ayrı worktree'de birleştirir, bütün
# deney bayraklarıyla (analiz/bayraklar.env; dalın kendi bayraklar.env'i varsa o) tohumlar oynatır, davranis.py ölçülerini
# yazar. Uçlar son turla aynıysa oynatmaz. Durum: analiz/out/izle-hepsi-durum.txt; sonuçlar analiz/out/izle-hepsi/<uçlar>/.
# Kullanım: analiz/izle-hepsi.sh [tohumlar...]   (varsayılan 217468 1 2 3)
set -u
cd "$(dirname "$0")/.."
KOK=$(pwd)
HUC=${IZLE_HUC:-claude/wizardly-knuth-1vc0a3}
SAV=${IZLE_SAV:-claude/youthful-cerf-w5mm8b}
T=${*:-217468 1 2 3}
for i in 1 2 3 4; do git fetch -q origin main "$HUC" "$SAV" && break; sleep $((2**i)); done
uc="$(git rev-parse --short origin/main)-$(git rev-parse --short origin/$HUC)-$(git rev-parse --short origin/$SAV)"
DURUM=analiz/out/izle-hepsi-durum.txt; mkdir -p analiz/out/izle-hepsi; touch "$DURUM"
if grep -qx "$uc" "$DURUM"; then echo "YENI_YOK $uc"; exit 0; fi
echo "OYNAT $uc"
WT=/tmp/izle-hepsi-wt; rm -rf "$WT"; git worktree prune
git worktree add -q --detach "$WT" origin/main || exit 1
( cd "$WT" && git -c user.email=analiz@yerel -c user.name=analiz merge -q --no-edit "origin/$HUC" "origin/$SAV" ) > /dev/null 2>&1 || { echo "BİRLEŞME ÇAKIŞMASI: main + $HUC + $SAV"; (cd "$WT" && git diff --name-only --diff-filter=U); git worktree remove --force "$WT"; exit 1; }
grep -q 'kip == "analiz"' "$WT/alan-rs/kos/src/main.rs" || { echo "analiz kipi yok (beklenmedik)"; }
# bayraklar: dalda daha yeni bir liste varsa onu kullan
B="$KOK/analiz/bayraklar.env"; [ -f "$WT/alan-rs/analiz/bayraklar.env" ] && B="$WT/alan-rs/analiz/bayraklar.env"
cp "$KOK"/analiz/{olc.sh,cok.sh} "$WT/alan-rs/analiz/"; cp "$KOK"/analiz/py/*.py "$WT/alan-rs/analiz/py/"
mkdir -p "$KOK/target-izle"; ln -sfn "$KOK/target-izle" "$WT/alan-rs/target"
OUT="$KOK/analiz/out/izle-hepsi/$uc"; mkdir -p "$OUT"
( cd "$WT" && git log --format='%h %ci %s' -12 -- alan-rs/motor alan-rs/katman ) > "$OUT/commitler.txt"
cp "$B" "$OUT/bayraklar.env"
( cd "$WT/alan-rs" && env $(grep -v '^#' "$B") analiz/cok.sh "$OUT" $T ) > "$OUT/cok.txt" 2>&1 || { echo "OYUN HATASI"; tail -5 "$OUT/cok.txt"; }
for t in $T; do [ -f "$OUT/t$t/kararlar.txt" ] && ANALIZ_OUT="$OUT/t$t" python3 analiz/py/davranis.py > "$OUT/t$t.davranis.json"; done
python3 - "$OUT" "$(tail -1 "$DURUM" 2>/dev/null)" $T <<'P'
import json,sys,os
out,onceki,ts=sys.argv[1],sys.argv[2],sys.argv[3:]
def ort(d):
  R=[json.load(open(f'{d}/t{t}.davranis.json')) for t in ts if os.path.exists(f'{d}/t{t}.davranis.json')]
  if not R: return None
  o={}
  for takim in R[0]:
    o[takim]={}
    for k in R[0][takim]:
      v=[r[takim][k] for r in R if r[takim][k] is not None]
      if v and all(isinstance(x,(int,float)) for x in v): o[takim][k]=round(sum(v)/len(v),1)
      else: o[takim][k]=[x for x in v if x] or None
  return o
a=ort(out); print('DAVRANIŞ (ortalama, %d tohum):'%len(ts)); print(json.dumps(a,ensure_ascii=False))
b=ort(os.path.join(os.path.dirname(out),onceki)) if onceki else None
if b:
  print('ÖNCEKİ TURA GÖRE (%s):'%onceki)
  for t in a:
    for k in a[t]:
      if a[t][k]!=b.get(t,{}).get(k): print(f'  {t} {k}: {b.get(t,{}).get(k)} → {a[t][k]}')
P
echo "$uc" >> "$DURUM"
rm -f "$WT/alan-rs/target"; git worktree remove --force "$WT"
