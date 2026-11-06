#!/usr/bin/env bash
# Değişiklik izleyici (analizci). Uzak dallarda motoru (alan-rs/motor, alan-rs/katman) değiştiren yeni uç commit'leri bulur;
# her biri için ayrı worktree'de, bu klasörün araçlarıyla (analiz kipi + olc.sh) maç 217468'i oynatır ve ozet.py ile başvuruya
# (baz-1f0ec1a.json: analizin yapıldığı main) karşı farkı yazar. Durum: analiz/out/izle-durum.txt (dal uç), sonuçlar analiz/out/izle/<dal>-<commit>/.
# Kullanım: analiz/izle.sh [--hepsi]   (--hepsi: durum dosyasını yok say)
set -u
cd "$(dirname "$0")/.."
KOK=$(pwd); BEN=claude/youthful-pascal-i91fpn
DURUM=analiz/out/izle-durum.txt; mkdir -p analiz/out/izle; touch "$DURUM"
[ "${1:-}" = "--hepsi" ] && : > "$DURUM"
ESIK=${IZLE_ESIK:-2026-11-06T16:28:00Z}   # bundan eski commit'ler izlenmez
for i in 1 2 3 4; do git fetch -q --all --prune && break; sleep $((2**i)); done
git for-each-ref --format='%(refname:short) %(objectname) %(committerdate:iso-strict)' refs/remotes/origin | while read -r ref sha tarih; do
  dal=${ref#origin/}
  [ "$dal" = "$BEN" ] || [ "$dal" = "HEAD" ] && continue
  [[ "$tarih" < "$ESIK" ]] && continue
  grep -q "^$dal $sha$" "$DURUM" && continue
  # aynı commit başka dal adıyla zaten oynandıysa yalnız kaydet
  if grep -q " $sha$" "$DURUM"; then echo "$dal $sha" >> "$DURUM"; echo "AYNI $dal ${sha:0:7} (başka dal adıyla oynandı)"; continue; fi
  # yalnız motoru değiştiren (analizin başvurusuna göre)
  if git diff --quiet 1f0ec1a "$sha" -- motor katman 2>/dev/null; then echo "$dal $sha" >> "$DURUM"; echo "ATLA $dal ${sha:0:7} (motor aynı)"; continue; fi
  ad="${dal//\//_}-${sha:0:7}"; OUT="$KOK/analiz/out/izle/$ad"; WT=/tmp/izle-wt-$ad
  echo "OYNAT $dal ${sha:0:7} ($tarih)"
  rm -rf "$WT"; git worktree prune; git worktree add -q --detach "$WT" "$sha" || continue
  # analiz kipi yoksa ekle (kos/src/main.rs), araçları kopyala
  if ! grep -q 'kip == "analiz"' "$WT/alan-rs/kos/src/main.rs"; then
    git show edc112d -- kos/src/main.rs > /tmp/izle-kip.patch; (cd "$WT" && (git apply --3way /tmp/izle-kip.patch 2>/dev/null || patch -p1 -s < /tmp/izle-kip.patch)) || { echo "  analiz kipi eklenemedi"; }
  fi
  rm -rf "$WT/alan-rs/analiz"; cp -r analiz "$WT/alan-rs/analiz"; rm -rf "$WT/alan-rs/analiz/out"
  mkdir -p "$OUT"
  ( cd "$WT/alan-rs" && CARGO_TARGET_DIR="$KOK/target-izle" bash -c '
      set -e; cargo build --release -q -p alan-kos
      B=$CARGO_TARGET_DIR/release/alan-kos
      $B analiz/mac-217468.json 9000 analiz > "'"$OUT"'/analiz.jsonl" 2> "'"$OUT"'/analiz.err" &
      $B analiz/mac-217468.json 9000 kararlar > "'"$OUT"'/kararlar.txt" 2> "'"$OUT"'/kararlar.err" &
      wait' ) > "$OUT/derleme.txt" 2>&1 || { echo "  DERLEME/OYUN HATASI (bkz $OUT/derleme.txt)"; tail -5 "$OUT/derleme.txt"; echo "$dal $sha" >> "$DURUM"; git worktree remove --force "$WT"; continue; }
  git log --format='%h %ci %s' 1f0ec1a.."$sha" -- motor katman | head -20 > "$OUT/commitler.txt"
  export ANALIZ_OUT="$OUT"
  for b in zincir pres sut pas oyuncu sekil firsat topsuz; do python3 analiz/py/$b.py > "$OUT/$b.txt" 2>&1; done
  python3 analiz/py/ozet.py "$OUT" > "$OUT/ozet.txt"
  head -1 "$OUT/ozet.txt" > "$OUT/ozet.json"
  echo "  $(cat "$OUT/analiz.err")"
  python3 - "$OUT/ozet.json" analiz/baz-1f0ec1a.json <<'P'
import json,sys
a=json.load(open(sys.argv[1]));b=json.load(open(sys.argv[2]))
for k in a:
  if a[k]!=b.get(k): print(f'  {k}: {b.get(k)} → {a[k]}')
P
  echo "$dal $sha" >> "$DURUM"
  git worktree remove --force "$WT"
done
