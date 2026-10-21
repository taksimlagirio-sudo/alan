# Maç 885639 · Liman – Kale Duvarı (21 Ekim 2026, 20:32)

> Kullanıcının oynadığı maç. Kayıt `kayit.json`; yeniden oynatma `oynat.js` (mac-isci.js'in 'yeni' yolu). Motor: o saatte yayındaki main 1ab1167 (`d-1ab1167.json`). Bir önceki motor 33c66f8 aynı kayıtla 2–3 veriyor (`d-33c66f8.json`). Kullanıcı sitede 5–1 gördüğünü doğruladı.

**Skor 5–1** (devre 2–0). Şut 18–10, xG 11,6–2,5, top %60–40, pas 103 (97 isabetli) – 58 (47).

## Liman'ın kurulumu
Yüksek blok, pres şiddeti 1, her yerde 3 sn karşı pres, beş tetik; düşük tempo (0,25), sabır 0,75, dinlenme ve yıpratma 0,75. Hücum duruşunda iki oyuncu çizgide (y 2 ve 48): saha tam genişlikte.

## Ne oldu
1. **Liman topu rakip yarısında tuttu:** topun %58'i rakip yarısında, top ortalama x 67. Kale Duvarı yattı (Düşük blok, 1 önde).
2. **Şarjlı şut farkı:** Liman şutları ortalama 0,70 şarjla çıktı (sabırlı, düşük tempo: şarj edecek zaman), Kale Duvarı 0,51 (kontra, aceleci). Liman'ın 5 golü 23–31 br'den, 0,65–0,83 şarjla. Uzaktan şut, yüksek şarjla gol oluyor.
3. **Kale Duvarı'nın kontrası işlemedi:** Önde bekleyen Ok topu 582 tik tuttu, 7 şut attı (xG 1,36), hepsi 30–38 br'den; 6'sını Aras tuttu, 9 top kaybı. Liman'ın 3 golü Ok'un kaybından (Aras tutuyor) başlayan ataklardan.
4. **Kale Duvarı'nın tek golü santradan:** Liman'ın 4. golünden 6 sn sonra, Mancınık → Hendek → Siper, Kuyu önünden (19 br). Yüksek blok gol sonrası yerleşmeden.
5. **Genişlik:** Liman'ın 18 şutunun 12'si kanattan (y < 15 ya da > 45). Önceki ölçümde genişlik etkisizdi; burada çizgiye konan oyuncular kanattan şut atıyor.

## Oyuncular (Liman)
| | pas | şut | xG | gol | top kazanma | kayıp | bar | ort. x |
|---|---|---|---|---|---|---|---|---|
| Aras (Bekçi) | 9 | 0 | – | – | 7 | 1 | – | – |
| Deniz | 9 | 0 | – | – | 4 | 1 | 0,93 | 31 |
| Mert | 15 | 4 | 2,39 | 1 | 4 | 3 | 0,66 | 61 |
| Ece | 13 | 3 | 2,36 | 2 | 0 | 4 | 0,58 | 69 |
| Cansu | 18 | 3 | 1,92 | 1 | 2 | 4 | 0,74 | 59 |
| Kaan | 21 | 4 | 2,21 | 1 | 2 | 3 | 0,58 | 81 |
| Tuna | 18 | 4 | 2,76 | 0 | 0 | 3 | 0,58 | 77 |

## Dikkat
- **xG şişkin:** Liman 11,6 xG'den 5 gol. 30 br'den şutlara 0,8–1,0 xG verilmiş (95 sn Tuna 0,96 → Bekçi tuttu). Şut anatomisi 1'deki bulgu: shotPk uzak şutlarda fazla.
- Bütün oyuncular 20 (kadro.js HEPSI_20): fark yalnız taktikten.

## Ek · Önceki motorda neden 2–3?

- İki motor arasında tek fark 1ab1167: ayak tercihi kalkar (`core.js`, 1 satır). Bu kurulumda iki takımın tarafları farklı (Liman'da sol ve iki ayaklılar var, Kale Duvarı'nda hepsi sağ).
- Maçlar **ilk 20,5 sn birebir aynı** (Liman'ın 9. sn golü ikisinde de var). 614. tikte bir Kale Duvarı oyuncusu binde bir br farklı yerde; 10 sn sonra top artık başka oyuncularda. Motor belirlenimci ama kaotik: en küçük fark birkaç saniyede bambaşka bir maç doğurur.
- Aynı kurulum, 12 farklı tohum, iki motor (`tohum/`):

| motor | G-B-M (Liman) | gol | gol farkı | şut | xG | top |
|---|---|---|---|---|---|---|
| 1ab1167 | 6-4-2 | 2,42 – 1,25 | +1,17 ±0,53 | 14,7 – 10,6 | 9,1 – 4,7 | %62 |
| 33c66f8 | 8-1-3 | 2,75 – 1,67 | +1,08 ±0,51 | 14,8 – 11,2 | 9,5 – 4,7 | %58 |

- **İki motor aynı:** Liman ortalama ~1 gol farkla kazanıyor. 5–1 ile 2–3 arasındaki fark motor değişikliği değil, tek maçın şansı. Aynı kurulum 0–1'den 5–0'a kadar skorlar veriyor.
- Tasarım notu: tek maçın sonucu çok oynak. Menajer oyununda bu istenebilir, ama oyuncu taktiğinin işe yarayıp yaramadığını tek maçtan okuyamaz; maç sonu ekranında xG ya da "pozisyon üstünlüğü" gibi bir gösterge bunu dengeler (xG'nin kendisi düzeltildikten sonra).

## Ek · Motor bu maçı nasıl belirledi (ALAN'ın kendi mekanikleriyle)

Kaynak: `d-1ab1167-ayrinti.json` (m.st, alan seçimi, şarj, kafa ↔ maç kayıtları).

1. **Şarj (Çekirdeğin enerjisi) maçı belirledi.** Liman şuta 0,70, Kale Duvarı 0,51 şarjla vurdu. Liman şarja 6,4, Kale Duvarı 3,6 enerji harcadı. Taşıyıcının şarj ettiği zaman ikisinde de benzer (%46 / %43), ama Liman topu daha çok tuttu (kafada 114 "tut" kararı, KD 68) ve şarjı şuta taşıdı. Liman'ın 5 golü 23–31 br'den 0,65–0,83 şarjla; KD'nin 10 şutunun 7'sini Bekçi tuttu.
2. **Duvar (kenardan pas):** iki takım da çok kullandı (22 / 18). Liman'ınki 22'de 2 kez rakibe geçti, KD'ninki 18'de 7 kez: KD'nin kontra çıkışı duvardan ve orada kırıldı.
3. **Alan neredeyse kullanılmadı:** oyuncu-zamanının %2'sinde hızlandıran, %6'sında yavaşlatan alan açık. Harcamanın ~%9'u alan. Bu maçta alan sonucu belirleyen bir araç değildi.
4. **İkili yok denecek kadar az:** 3 + 2 ikili, kazanılan yok; top kazanmalar (18 / 13) pres ve pas kesmeyle.
5. **Kafa ↔ maç:** 480 kararın 41'inde (%8,5) sapma. Çoğu (29) "alıcı topu kafadakinden ~1 sn erken aldı": kafa alıcının topa gelişini yavaş tartıyor. 11'i kayıp: kafa %81–99 "bizde kalır" dedi, rakip aldı. Şutta kafa %82–96 gol bekledi, olmadı: şut değeri kafanın içinde de şişkin, oyuncular bu yüzden uzaktan vuruyor.
6. **Bekçi oyun kuruyor:** Aras 7 top kazandı (Ok'un uzak şutları), Liman'ın 3 golü onun elinden başladı. KD'de Sur 11 (Liman'ın şutları).
7. **Önde bekleyen (Ok):** topu en çok tutan (582 tik), en çok kaybeden (9). Kafa ona hep çıkış veriyor; tek başına uzak şuttan başka seçeneği olmuyor.
