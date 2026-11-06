#!/usr/bin/env bash
# Siteye derleme: şut tablosu eskiyse (şut ya da Bekçi kuralı, kafanın şut/dalış modeli değiştiyse) yeniden üretir ve
# gömer, wasm'ı derler (alan-v5/alan.wasm), sürüm etiketini yeniler. Gönderme yapmaz; commit/push çağıranın işi.
#   alan-rs/sina/site.sh
set -euo pipefail
kok="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$kok/alan-rs"
if ! ./sina/tablo.sh >/dev/null 2>&1; then
  echo "şut tablosu eski: yeniden üretiliyor"
  ./sina/tablo.sh --uret motor/src/kafa/kafa2_imk_sut.bin
  cargo build --release -q -p alan-kos
  ./sina/tablo.sh
fi
(cd wasm && cargo build --release -q --target wasm32-unknown-unknown)
cp target/wasm32-unknown-unknown/release/alan_wasm.wasm "$kok/alan-v5/alan.wasm"
cd "$kok"
eski=$(grep -o "ALAN_SURUM = '[0-9]*" canli.html | grep -o '[0-9]*$')
yeni=$(date -u +%Y%m%d%H%M)
if [ "$eski" != "$yeni" ]; then
  sed -i "s/$eski/$yeni/g" canli.html index.html lab.html taktik.html telefon-oyun.html
fi
echo "site hazır: sürüm $yeni"
