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
uc="$(git rev-parse --short origin/main)-$(git rev-parse --short origin/$HUC)-$(git rev-parse --short origin/$SAV)${IZLE_B:+-$(basename "$IZLE_B" .env)}"
DURUM=analiz/out/izle-hepsi-durum.txt; mkdir -p analiz/out/izle-hepsi; touch "$DURUM"
if grep -qx "$uc" "$DURUM"; then echo "YENI_YOK $uc"; exit 0; fi
echo "OYNAT $uc"
WT=/tmp/izle-hepsi-wt; rm -rf "$WT"; git worktree prune
git worktree add -q --detach "$WT" origin/main || exit 1
# sırayla birleştir; yalnız oyunu etkilemeyen dosyalarda çakışma kendiliğinden çözülür (bayraklar.env: birleşim, site js/html: ilk taraf)
birles() { git -c user.email=analiz@yerel -c user.name=analiz merge -q --no-edit "$1" >/dev/null 2>&1 && return 0
  for f in $(git diff --name-only --diff-filter=U); do
    case "$f" in
      alan-rs/analiz/bayraklar.env) for i in 1 2 3; do git show :$i:"$f" > /tmp/izb$i 2>/dev/null || : > /tmp/izb$i; done
        awk 'FILENAME==ARGV[1]{b[$0]=1;next} FILENAME==ARGV[2]{o[$0]=1;L[++n]=$0;next} {t[$0]=1;L[++n]=$0}
             END{for(i=1;i<=n;i++){x=L[i]; if(g[x]++)continue; if(b[x]&&!(o[x]&&t[x]))continue; print x}}' /tmp/izb1 /tmp/izb2 /tmp/izb3 > "$f"; git add "$f";;
      alan-v5/*.js|alan-v5/*.html) git checkout --ours -- "$f"; git add "$f";;
      */__pycache__/*) git rm -q -f --cached "$f" >/dev/null 2>&1; rm -f "$f";;
      alan-rs/wasm/src/lib.rs) git show :1:"$f" > /tmp/izl1; git show :2:"$f" > /tmp/izl2; git show :3:"$f" > /tmp/izl3; git merge-file --union -p /tmp/izl2 /tmp/izl1 /tmp/izl3 > "$f"; git add "$f";;
      *) return 1;;
    esac
  done
  git -c user.email=analiz@yerel -c user.name=analiz commit -q --no-edit >/dev/null 2>&1; }
( cd "$WT" && birles "origin/$HUC" && birles "origin/$SAV" ) || { echo "BİRLEŞME ÇAKIŞMASI: main + $HUC + $SAV"; (cd "$WT" && git diff --name-only --diff-filter=U); git worktree remove --force "$WT"; exit 1; }
grep -q 'kip == "analiz"' "$WT/alan-rs/kos/src/main.rs" || { echo "analiz kipi yok (beklenmedik)"; }
# bayraklar: yetkili liste analiz dalındaki analiz/bayraklar.env; dalların listelerinde olup bizde olmayanlar yalnız bildirilir
B="${IZLE_B:-$KOK/analiz/bayraklar.env}"
for d in "$HUC" "$SAV"; do for f in $(git show "origin/$d:alan-rs/analiz/bayraklar.env" 2>/dev/null | grep -o '^ALAN_[A-Z0-9_]*'); do grep -q "^$f=" "$B" || echo "BAYRAK_BIZDE_YOK $d $f"; done; done
for f in $(cd "$WT" && git log --format=%B -12 -- alan-rs/motor | grep -o 'ALAN_[A-Z0-9_]*_IZ' | sort -u); do grep -q "^$f=" "$B" || echo "COMMITTE_YENI_BAYRAK $f"; done
cp "$KOK"/analiz/{olc.sh,cok.sh} "$WT/alan-rs/analiz/"; cp "$KOK"/analiz/py/*.py "$WT/alan-rs/analiz/py/"
mkdir -p "$KOK/target-izle"; ln -sfn "$KOK/target-izle" "$WT/alan-rs/target"
OUT="$KOK/analiz/out/izle-hepsi/$uc"; mkdir -p "$OUT"
( cd "$WT" && git log --format='%h %ci %s' -12 -- alan-rs/motor alan-rs/katman ) > "$OUT/commitler.txt"
cp "$B" "$OUT/bayraklar.env"
# şut tablosu bu bayrak setiyle güncel mi (kafanın şut modeli bayraklarla değişebilir: ör. BEK_DIREK)
( cd "$WT/alan-rs" && env $(grep -v '^#' "$B") ./sina/tablo.sh 2>&1 | tail -1 ) | sed 's/^/TABLO: /' | tee "$OUT/tablo.txt"
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
