# Anlar araçları

Sitedeki **Anlar** ekranının (`anlar.html`, veri `anlar/`) üreticisi.

1. `tara.js motorDizini tohum rakipNo çıktıDizini` — maçı node'da oynatır (A aşaması, yavaş: maç başı ~10–16 dk). Her tikin kısa izini (`*.iz.json.gz`) ve her 30 tikte maçın tam kopyasını (`*.kopya.jsonl.gz`, `graf.js` biçimi) yazar. Kurulum: `KURULUM` ortam değişkenindeki kayıt dosyası (sitedeki "Maçı kaydet"/son maç kaydı biçimi, `yeni` alanı); rakip `kadro.js` RAKIP[rakipNo]'dan.
2. `sec.js çıktıDizini anlarDizini motorEtiketi` — B aşaması (hızlı): izlerde sorun türlerini arar (dedektörler dosyanın başındaki `KAT`), her türden en belirgin 8 anı seçer, `anlar/anlar.json` + an başına `a-*.json` (kurulum + başlangıç kopyası) yazar.
3. `sira.sh` — 9 maç, aynı anda 3.

Oynatma: `canli.html?an=ID` kopyayı işçiye `tip: 'an'` ile gönderir; `mac-isci.js` `grafOku` ile kurar, taze maçtan eksik alanları tamamlar, `bitis` tikine kadar **bugünkü motorla** oynar. Motor değişmediyse kopyadan oynatma asıl maçla birebir aynıdır (denendi: 180 tik, fark 0).
"maçta" sayıları (`sayim`) tarandığı motorun; motor değişince yeniden tarayıp karşılaştırılır. Anların kendisi (kopyalar) sabit kalabilir.
