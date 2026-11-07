# A2 kontra değer tablosu (savunma dalı)

Biçim ve altyapı bilgi tablolarınınki (analiz/TASARIM-BILGI.md; hücum dalının `sina/bilgi.sh`, `sina/bilgi/ortak.py`,
`kafa2_bilgi::Bilgi`, bicim 1). Ayrı biçim yok.

- Dosya: `motor/veri/bilgi/kontra.json` (derlemeye gömülü, wasm'da da). Yerelde `ALAN_KONTRA_TABLO=<yol>` başka dosya okur.
- Üretim: `sina/bilgi.sh --uret kontra [tur]` (tarif `sina/bilgi/kontra.conf`, üretici `sina/bilgi/kontra.py`); denetim `sina/bilgi.sh`.
- Bayrak: `ALAN_BILGI_KONTRA_IZ` (= `ALAN_KTAB_IZ`, sitede `ktab=1`); `hiz2` ile çalışır. `HIZ2_BAR` ve `Sy` değişmez.

## Ölçüm (kontra.py)
Kontra: top rakip kaleye 50+ br'de kazanılır, topun kale tarafında en çok 3 rakip saha oyuncusu vardır (analiz/py/kontra.py).
Kontra boyunca (12 sn'ye, gole ya da rakip topu alana dek) her 5 tikte durum örneklenir; sonuç: o andan 12 sn içinde gol.
Eksenler: `uzaklik` (top / taşıyıcının rakip kale merkezine, br: 10–90) × `onde` (rakip saha oyuncularından kale çizgisine toptan
yakın olanlar: 0–5) × `sure` (top kazanılalı geçen süre, sn: 0–12). Az örnekli hücre komşularla yumuşatılır (en az 20 bağımsız kontraya dek);
`n` ölçülen örnek.

## Okuma (mac/kontra_tablo.rs, enerji::sprint_karar3)
Koşucunun her varış süresi seçeneği için: `uzaklik` = hedefin rakip kaleye uzaklığı; `onde` = varışta topun önüne dönebilmiş
savunmacı (rakip saha oyuncularının hedefe varış sürelerinden yumuşak sayım: `Σ 1/(1+exp((t_q − T)/20))`); `sure` = top kazanılalı
geçen + varış (en az pasın varışı). Değer `(n·tablo + k·model)/(n + k)`, model = 0/1/2/3+ savunmacı → %73/%50/%33/%0
(analiz/ZAMAN-DEGERI.md). Kontra payı hedefin `Sy·g` değerine eklenir. Başlangıç tablosunda bütün `n` = 0: davranış tablosuzla aynı.
