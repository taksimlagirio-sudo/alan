# ALAN · Sitedeki motor değişti (zip74): kafa = maç, şekil onayı, adama bağlılık

Claude Code'dan Design'a. Kullanıcının isteğiyle bu üç değişikliği **ben uyguladım ve siteye koydum** (main `bfc0866`, taban zip71). Bir sonraki paketin bunları içermesi gerekiyor, yoksa ezilir. Kod farkı ekte: `zip74-motor.diff` (`decide.js`, `match.js`, `shape.js`; diğer dosyalar zip71'le aynı).

## 1 · Kafa = maç, yalnız gönderme (`decide.js`, `match.js`)
Ayrıntı ve ölçüm: `DESIGN-NOT-kafa-mac.md`.
- `imagine` içinde gönderme denemesi `simOne` yerine `ucusMac` ile yapılır: maçın kopyası (`_lite`), maçın kendi adımı.
- `match.js` adım başında `D.D._mac = m`.
- **Bayraklar:**
  - `ALAN_KAFA_ESKI`: eski taklit;
  - `ALAN_KAFA_PAS`: aynı yolu paslara da açar. Varsayılan kapalı: zar gürültüsü ve 3 kat yavaşlık. Önce `contact` olasılık / zar diye ayrılmalı.
- **Gönderme kalibrasyonu (8 maç, akıl / gerçek):**
  - orta %24 / %12 → %18 / %10;
  - uzak %24 / %4 → %8 / %1.
- **Hız bedeli:**
  - node'da +%25–30;
  - **tarayıcıda maçın hazırlanması 38–46 sn → 57–75 sn** (Playwright, aynı makine, ikişer ölçüm).

## 2 · Şekil onayı (`shape.js`, `position` başı; bayrak `ALAN_SEKIL_ONAY_ESKI`)
- **Sorun:** Top havadayken `flightRead`'in "topu kim alacak" tahmini her 3 tikte yenileniyor; `att` buna göre hemen değişiyordu.
  - Takım şekli dakikada 48 kez hücum ↔ savunma arasında değişiyordu; gerçek el değişimi dakikada 14.
  - Değişimlerin yarısı yarım saniyede geri dönüyordu.
  - Her değişim, topun önündekilere `p.rec` (geri koşu) da veriyordu.
- **Kural:** Tahmin değişince şekil, tahmin 10 tik aynı kalırsa değişir. Top birine geçince hemen değişir.

## 3 · Adama bağlılık (`shape.js`, arkadakilerin `swT` ataması; bayrak `ALAN_ADAM_BAGLI_ESKI`)
- **Sorun:** Arkadakiler her tik sıfırdan, sıra numarasıyla atanıyordu: birinci arkadaki birinci tehlikeliyi, ikinci ikinciyi tutuyordu.
  - Tehlike eşitken (0,047 / 0,047) adamlar her tik takas ediliyordu.
  - Tehlike pas yoluna bağlı olduğundan üç tikte 0,028 → 0,009 → 0,029 zıplayabiliyordu.
  - Mert'in tuttuğu adam dakikada 77 kez değişiyordu.
- **Kural:** Arkadaki, tuttuğu rakip hâlâ en tehlikeli `arkadaki sayısı` rakip arasındaysa onu tutmaya devam eder. Boşta kalan, kalan tehlikelilerden kendine en yakını alır. `p.swPrev` tutulur.

## Ölçüm (tek maç: senaryo 0, aynı tohum)
Kullanıcı tek maçta kalmamı istedi. Maçı birebir yeniden oynatarak ölçtüm.

| | zip71 + kafa = maç | + 2 + 3 |
|---|---|---|
| Şekil değişimi / dk | 48 | 15 |
| bunlardan yarım saniyede geri dönen | 48 | 3 |
| Mert / Deniz adam değişimi / dk | 77 / 51 | 33 / 29 |
| Geri koşu (savunma süresine göre) | %16 | %5 |
| Görev değişimi / sn (oyuncu başı) | 3,9 | 3,0 |
| Mert'in maç boyu görev gidip gelmesi | 194 | 86 |

- **Değişmeyen:** Koşarken çeyrek saniyede ters yöne dönüş (~24 / dk). Bunların %70'e yakını pas çıkarken ya da top el değiştirirken. Sebep, dakikada ~120 pasın temposu.
- **Denenip bırakılan:** Evin kayma hızını oyuncunun depar hızıyla sınırlamak. Evin sıçramasını bitirdi (dakikada 58 → 2), ama oyuncuların hareketini değiştirmedi.

## Sonra bakılacak (aynı maçtan)
- **Hücumda arkada kalan:** Bek Mert, top kaybında rakip ceza sahasının dibindeydi (ileri 104). Bir sayı, topu kaybettikten 3,6 sn sonra kontradan geldi.
- **Ayrıntı:** sayfa `plan/liman-uc-oyuncu.html`, araçlar:
  - `tools/sahne-iz.js`,
  - `tools/ev-iz.js`,
  - `tools/arkada-iz.js`,
  - `tools/baglilik-kiyas.js`.
