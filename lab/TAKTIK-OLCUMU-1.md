# ALAN · Taktik ölçümü 1 (sitedeki motor: main 7ce2527, 19 Ekim 2026)

Soru: taktik farkları sahada ve sonuçta görülüyor mu?

## Kurulum

- İki takımın bütün oyuncuları aynı (bütün özellikler 10). Fark yalnız taktikte. A, maçların yarısında takım 0, yarısında takım 1.
- Taktik alanları taktik sayfasının `tac()` çıktısıyla aynı; varsayılan = sayfanın varsayılanı (pres 1, şiddet 0,5, Orta blok, tempo 0,55, risk 0,5, geçiş "önde sayıca üstünsek", karşı pres "rakip yarısında" 2 sn, diziliş 2-2-2 / 2-1-3).
- **Maç 40 sn** (2400 tik; kullanıcı isteği: hızlı sonuç). 648 maç:
  - 6 hazır tarz, 15 eşleşme × 20 maç = 300 maç.
  - 29 tek ayar (bir ayar uçta, gerisi varsayılan) varsayılana karşı × 12 maç = 348 maç.
- **Uyarı:** 40 sn'lik maçta maç başına 0,19 gol, 1,43 şut atılıyor; maçların %85'i golsüz. Gol farkı bu yüzden neredeyse bilgi taşımıyor; üstünlük **şut farkından** okunuyor. Tek ayarlarda 12 maç yalnız büyük etkileri gösterir.
- Araçlar: `tools/taktik-olc.js`, `tools/taktik-ozet.js`. Ham veri `lab/tk2/sonuc/`, tam tablo `lab/tk2/ozet.md`.

Tarzlar (ayrıntı `taktik-olc.js` AYAR tablosunda):
- **Kale önü:** Düşük blok, pres 0, yavaş, düşük risk, karşı pres yok, 3-2-1.
- **Kontra:** Düşük blok, hızlı ve riskli, kazanınca her zaman geçiş, sabırsız, 2-3-1 / 1-2-3.
- **Topa sahip:** Orta blok, yavaş, güvenli, çok sabırlı, geçiş yok, geniş.
- **Ön alan pres:** Yüksek blok, pres 3, şiddet 0,9, bütün tetikler, her yerde karşı pres 3 sn.
- **Dikine:** hızlı, riskli, sabırsız, çok şut, 1-2-3 hücum.
- **Kanat tuzağı:** Kenara sıkıştır, pres 2, geniş, sürmeye yatkın.

## 1 · Tarzlar sahada ayrışıyor mu? **Evet, açıkça.**

| tarz | top bizde | top bizdeyken rakip yarısında | savunma hattı | presçi (ort.) | pas | şut |
|---|---|---|---|---|---|---|
| Kale önü | %53 | %45 | **%30** | 0,31 | 14,1 | 0,51 |
| Kontra | **%38** | %51 | %36 | 0,35 | 16,5 | 0,73 |
| Topa sahip | **%57** | %38 | %36 | 0,83 | 17,9 | 0,41 |
| Ön alan pres | %54 | %49 | **%43** | **1,35** | 15,3 | 0,79 |
| Dikine | %42 | **%62** | %41 | 0,84 | 16,9 | **1,23** |
| Kanat tuzağı | %55 | — | — | — | 14,1 | 0,61 |

(Savunma hattı: top rakipteyken saha oyuncularının kendi kalemizden ortalama uzaklığı, saha boyunun %'si. Kanat tuzağı listede hep ikinci takım olduğu için yalnız top sahipliği, pas ve şut ölçüldü.)

## 2 · Sonuca etkisi var mı? **Var ama sınırlı; cevap ilişkisi yok, sıralama var.**

Şut farkı (satırın sütuna karşı, 40 sn'de; `*` anlamlı):

| | Kale önü | Kontra | Topa sahip | Ön alan pres | Dikine | Kanat tuzağı | ort. |
|---|---|---|---|---|---|---|---|
| Kale önü | — | −0,30 | −0,10 | −0,35 | −0,70 | −0,05 | −0,30 |
| Kontra | +0,30 | — | +0,10 | −0,05 | −0,95\* | +0,05 | −0,11 |
| Topa sahip | +0,10 | −0,10 | — | −0,10 | −0,35 | −0,50 | −0,19 |
| Ön alan pres | +0,35 | +0,05 | +0,10 | — | −0,20 | +0,65\* | +0,19 |
| **Dikine** | +0,70 | +0,95\* | +0,35 | +0,20 | — | +0,55 | **+0,55** |
| Kanat tuzağı | +0,05 | −0,05 | +0,50 | −0,65\* | −0,55 | — | −0,14 |

- **Dikine hiçbir tarza kaybetmiyor.** Ön alan pres ikinci. Kale önü en zayıf.
- **Taş-kâğıt-makas yok:** her tarz ötekilere karşı aynı sırada. Planın "rakip tarzları sahada gerçekten farklı olmalı, kadro derinliği taktik seçimden gelir" hedefi için bu eksik: bir tarzın başka bir tarza karşı özel bir cevabı yok.
- **Topa sahip olmak şut getirmiyor:** en çok top tutan tarz (Topa sahip) en az şut atıyor.
- Büyüklük: en büyük tarz farkı (Dikine – Kontra, 40 sn'de +0,95 şut) 120 sn'ye ölçeklenince ~+2,9 şut. Hız 16'nın varsayılan takıma karşı farkı ~+5,5 şut (Özellik ölçümü 1, bbd5ead). Yani **en iyi taktik seçimi, tek bir özelliği +6 artırmanın yaklaşık yarısı kadar.** Motorlar farklı (bbd5ead / 7ce2527), karşılaştırma kaba.

## 3 · Tek ayarlar (12 maç, 40 sn)

Hiçbir tek ayarın gol ya da şut farkı 12 maçta anlamlı değil (kısa maç + az maç; ancak çok büyük etkileri görebilirdi). Davranışa etkileri:

| ayar | işliyor mu | sahada ne değişiyor |
|---|---|---|
| **Pres sayısı (0 / 1 / 3)** | **hayır** | pres 0 ile pres 3 **12 maçın 12'sinde birebir aynı maç.** `shape.js:10` `presLim`: "basan sayısı ayarı kapalı, sabit 2". Taktik sayfasında hâlâ seçilebiliyor. |
| Şut isteği (0,1 / 0,9) | zayıf | 12 maçın 6'sı birebir aynı; şut farkı −0,6 / +0,2 |
| Pas–sürme (0,1 / 0,9) | evet | pas sayısı 18 / 11 |
| Geçiş (hiç / her zaman) | evet | top bizdeyken rakip yarısında %41 / %57 |
| Genişlik (Dar / Geniş) | evet | hücum genişliği %45 / %53 |
| Karşı pres yeri (hiç / her yer) | evet | presçi 0,57 / 0,80 |
| Şiddet (0 / 1) | evet | presçi 0,66 / 0,98; top sahipliği −7,3 / +3,2 |
| Diziliş 1-2-3 / 3-2-1 | evet | savunma hattı %44 / %38 |
| **Blok (Düşük / Yüksek)** | zayıf | savunma hattı **%35 / %39**: yalnız 4 puan fark |
| **Pres tetikleri (yok / hepsi)** | ters? | presçi **1,07 / 0,88**: tetik yokken daha çok basılıyor |
| Tempo, risk, sabır, karşı pres süresi | küçük | top sahipliğinde ±3–4 puan, başka belirgin iz yok |

## Bulgular

1. **Taktik oyunun görüntüsünü değiştiriyor.** Tarzlar top sahipliği (%38–57), oyunun nerede oynandığı, hattın yüksekliği (%30–43) ve pres yoğunluğunda (0,3–1,35 presçi) belirgin ayrışıyor. Menajer farkı sahada görür.
2. **Sonuçta taktik fark yaratıyor ama tek yönlü:** Dikine herkesi yeniyor, cevap ilişkisi yok. Planın taktik derinliği için asıl eksik bu.
3. **Çalışmayan ayarlar:**
   - **Pres sayısı** motorda kapalı ama sayfada seçilebiliyor. Ya sayfadan kaldırılmalı ya da motora geri bağlanmalı.
   - **Blok** hattı yalnız 4 puan oynatıyor.
   - **Tetikler** ters görünüyor (12 maç; doğrulanmalı).
   - **Şut isteği** çok az etkili.

## Sınırlar ve sonraki adım

- Kısa maç ve az maç: tek ayarların sonuç etkisi ölçülemedi. Gol yerine şut kullanıldı.
- Tarzlar benim tanımım; menajerin seçebileceği başka bileşimler farklı sonuç verebilir.
- Sonraki adım önerisi:
  1. Dikine'nin neden herkesi yendiğine bakmak (hızlı ve riskli hücum mu, 1-2-3 mü?): Dikine'nin bileşenlerini tek tek varsayılana döndürerek ölçmek.
  2. Pres sayısı ve blok için motor tarafında karar vermek.
