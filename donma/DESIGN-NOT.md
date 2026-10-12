# Claude Code → Design · donma: motor gerçekten donuyor (hesap yığılması değil)

## Ölçüm
Sayfanın kurduğu maç (Liman varsayılan kadro/taktik – Demir Blok), işçideki kurulumla, 4 tohum (11–14), tam maç. "Hareketsiz": hiçbir oyuncu bir tikte 0,01 birimden fazla kımıldamıyor.

| tohum | skor | en uzun hareketsiz | 0,5 sn üstü adım | en yavaş adım |
|---|---|---|---|---|
| 11 | 3–3 | 0 | 119 | 2,1 sn |
| 12 | 1–1 | **2058 tik (34 sn)** | 45 | 2,2 sn |
| 13 | 0–1 | **1909 tik, maç sonuna kadar** | 31 | 2,5 sn |
| 14 | 2–1 | **936 tik, maç sonuna kadar** | 81 | 2,3 sn |

Ekranda yükleme çıkmaz çünkü kareler geliyor; karelerde herkes duruyor. Sitedeki Canlı sayfası sizinkiyle bayt bayt aynı (%30 ve "Hesap yetişiyor" var). Repodaki motor da zip 27 ile aynı.

## Sebep: bekle-sonra-oyna döngüsü
Tohum 13, tik 3600: Liman 0–1 geride, Cansu topta, her yarım saniyede "tut 0,5 sn bekle" seçiyor; Demir Blok önde, çekilmiş, basmıyor. Herkes hedefinde, kimse kımıldamıyor.
- Şimdi en iyi hamle: kenardan → Tuna, **0,094**.
- Bekleme: 0,147. Değeri "30 tik sonraki dünyada en iyi hamle" = **fake** (0,181, inanılırlık %88).
- O dünyada (`predictWorld`) Demir Blok'un 6 oyuncusu da **8,67 birim** topa yaklaşmış. Gerçekte kimse gelmiyor. 30 tik sonra dünya aynı, karar aynı: sonsuz bekleme.

Tohum 14, tik 4700: bu sefer topu Demir Blok tutuyor; hayalde Liman'ın 6 oyuncusu 8,67 yaklaşıyor, beklemenin değeri 0,113 ("sonra kenardan"), şimdi en iyisi 0,076. Aynı döngü.

**1 · `predictWorld` maçtaki kural değil.** "En yakın 2 rakip Çekirdeğe koşar, diğerleri %25/%50 topa kayar" sabit bir varsayım. Maçta rakip yerleşimi `shape` ile (blok, pres, tetik) belirleniyor; düşük blok ya da basmayan takım gelmiyor. Pas hesabındaki ilkeyle aynı: hayal, maçın yerleşim kuralıyla ilerlemeli.

**2 · Zamanın değeri kovalara takılıyor.** Tohum 13: `macZ(10) = macZ(30) = macZ(60) = −0,034`. Tohum 14 (önde): `macZ(30) = +0,009`, `macZ(60) = −0,018` (önde olan için 1 sn beklemek aleyhte çıkıyor). Sebep `match.js` `macEff`: `w` 120 tiklik, `eT` 60 tiklik kovada ve ikisi de kovaya ilk girildiği andaki değerle önbellekte kalıyor. Geride olan için beklemenin ek bedeli yok, işaret bile tutarsız.

## Öneri
- 1 asıl sebep. Düzeltme kuralı değiştirir (davranış değişir), o yüzden size bırakıyorum; isterseniz ben de yaparım.
- 2'de `zaman(dt)`, `w.e0` ve `eT` aynı ve kesintisiz bir zamandan hesaplanmalı.
- Ayrı konu: 1–2 sn süren adımlar gerçek (maç başına 31–119 tane 0,5 sn üstü). Bunlar %30 kuralıyla ekranda yükleme olarak görünür, donma olarak değil.

Araçlar: `donma/don.js <motor> <tohum>` (tarama), `donma/don5.js <motor> <tohum> <tik>` (o andaki karar ve bekleme hesabı).
