# ALAN · Alan kullanımı 1: alan neden az kullanılıyor (maç 885639, 21 Ekim 2026)

> Motor 1ab1167 (maçın oynandığı). Maç 885639 (Liman 5–1 Kale Duvarı) yeniden oynatıldı; `match.js` `alanSec`'e yalnız sayaç eklendi (maç birebir aynı, skor 5–1). Araç `lab/maclar/885639/oynat-alan.js`, sayılar `lab/maclar/885639/alan-sayac.json`.

## Kural (koddan)

`alanSec` (`match.js:63`), her tik:
- Yalnız **Çekirdek havadayken** (taşıyıcı yokken). Bu maçta 9000 tikin 5426'sı.
- Her oyuncu için sırayla:
  1. Pastan beri tepki süresi geçmediyse (10 − Tepki × 0,4 tik) → kapalı.
  2. Barı 0,12'nin altındaysa → kapalı.
  3. Alanının topa etkisi 0,02'nin altındaysa (topun **şu anki yerine** ~1,8 × R ≈ 14 br'den uzak) → kapalı.
  4. Kalanlar sabit kuralla seçer:
     - şut: hücum eden hızlandırır, savunan yavaşlatır;
     - kendi pası: rakip araya girebilecekse hızlandır; alıcı ancak yolun sonunda yetişiyorsa yavaşlat; **yoksa bırak**;
     - rakip pası: takımımızdan biri alıcıyla aynı anda varabilecekse yavaşlat; alıcı ancak yolun sonunda yetişiyorsa hızlandır; **yoksa bırak**;
     - boş top: önce varacak takım yavaşlatır, öbürü bırakır.
- Bedel: tik başına 1/50 × etki (paylaşılan alanda azalır). Ortanca etki 0,23 → tik başına ~0,005 bar; 1 sn'lik bir pasta ~0,14 bar.

## Sayılar (oyuncu × uçuş tiki = 75 964)

| sebep | pay |
|---|---|
| **topa uzak (etki < 0,02)** | **%71,6** |
| tepki süresi dolmamış | %3,7 |
| bar 0,12'nin altında | %2,9 |
| kurala ulaşan | %21,8 |

Kurala ulaşanların (16 557) içinde:

| durum | bırak | hızlandır | yavaşlat |
|---|---|---|---|
| kendi pası | 5514 | 1726 | 53 |
| rakip pası | 3723 | 9 | 2243 |
| boş top | 1352 | – | 1423 |
| şut | – | 166 | 348 |
| **toplam** | **%64** | %11 | %25 |

## Okuma

1. **Asıl sebep menzil:** alan yalnız topun ~14 br yakınında etkili; oyuncuların çoğu çoğu zaman o kadar yakında değil. Uçuş anlarının %72'sinde oyuncu için alan söz konusu bile değil.
2. **İkinci sebep kuralın dar tanımı:** topun yakınındaki oyuncu bile %64 bırakıyor. Kural yalnız **topa kim önce varır** yarışına bakıyor:
   - Kendi pasımızı rakip kesemeyecekse kimse hızlandırmıyor. Oysa hızlanan pas savunmanın kaymasına (ölçülen kayma yavaş: taraf değiştirmede %17) ve alıcıya basmasına daha az zaman bırakır.
   - Rakip pasına yetişemeyecekse kimse yavaşlatmıyor. Oysa tek enerjide yavaşlatma **Çekirdeğin şarjını da düşürür**: alıcı daha boş Çekirdekle başlar, şutu zayıflar. Bu maçta Kale Duvarı'nın şutları 0,51 şarjla çıktı; Liman bunu daha da düşürebilirdi.
3. Yani alan bir **değer kararı** değil, sabit bir yarış kuralı. Kafa da alanı yalnız bu kuralla bekliyor (`core.js` `amOf`). Alanın taktik değeri (tempo, şarj eritme, kaymaya zaman bırakmama) hiçbir yerde tartılmıyor.
4. Bar kısıtı küçük (%3). Bedel sorun değil; karar sorun.

## Öneriler (motor oturumu için, karar onların)

- Alan seçimini kuraldan **değere** çevirmek: hızlandır / yavaşlat / bırak, sonucu kafanın değeriyle tartılsın (sarj kararındaki gibi). Bedel enerji, kazanç: varış sırası + alıcının şarjı + alıcıya basacak rakibin süresi.
- Rakip pasında "şarj eritme": yetişemesem de yavaşlatırsam Çekirdeğin şarjı düşer. Bu, derin bloğa (Kale Duvarı) Dikine'nin uzak şutlarına karşı da bir araç olur.
- Menzil (R, Alan gücü) ölçülmeli: herkes 10'da. Alan gücü yüksek oyuncunun alanı çok daha sık devreye girer; özelliğin değeri buradan gelir.
