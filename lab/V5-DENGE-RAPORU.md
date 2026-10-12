# ALAN v5 (zip 17) · taktik, özellik ve ayar ölçümü

1066 maç, 90 sn, herkes 10 (aksi belirtilmedikçe), diziliş 2-2-2 / 2-2-2, rol atanmamış. Her deney aynı tohumlarla; A takımı maçların yarısında taraf değiştirir.
Gürültü: aynı iki takım (temel, 24 maç) sayı farkı +0,08 ±0,37. `*` = fark standart hatanın 2 katından büyük.
Ayrıntılı tablo: `lab/v5-deney-ozet.txt`, ham veri: `lab/v5-deney.jsonl`, araçlar: `tools/v5-deney.js`, `v5-havuz.js`, `v5-deney-ozet.js`.

## 1 · Taktik (altı hazır tarz, her ikili 30 maç; satırın sütuna karşı ortalama sayı farkı)
|  | Dengeli | Sabırlı | Dikine | Kontra | Ön alan | Kuyu önü | ort. |
|---|---|---|---|---|---|---|---|
| Dengeli | — | +0,77* | +0,17 | +1,07* | +0,07 | +0,30 | +0,47 |
| Sabırlı | −0,77* | — | +0,47 | +0,33 | −1,07* | +0,43 | −0,12 |
| Dikine | −0,17 | −0,47 | — | +0,17 | −1,03* | +0,70* | −0,16 |
| Kontra | −1,07* | −0,33 | −0,17 | — | −1,20* | −0,27 | −0,61 |
| Ön alan | −0,07 | +1,07* | +1,03* | +1,20* | — | +0,73* | +0,79 |
| Kuyu önü | −0,30 | −0,43 | −0,70* | +0,27 | −0,73* | — | −0,38 |

- Cevap ilişkisi yok, sıralama var: Ön alan hiçbirine kaybetmiyor (Dengeli ile berabere), dördünü belirgin yeniyor. Kontra hiçbirini yenmiyor.
- Ön alan'ın sistemi "Adam adama" ama motor bu sistemi tanımıyor (aşağı bakın). Yani Ön alan'ın gücü 3 presçi + yüksek blok + geniş hücum bileşiminden geliyor. Tek başına pres 3 (öteki ayarlar varsayılan) fark yaratmadı (+0,00): güç bileşimden.

## 2 · Özellik (A'nın saha oyuncularında tek özellik 16, gerisi 10; 16 maç)
| özellik | sayı farkı | G-B-M | gönderme farkı |
|---|---|---|---|
| Hız | +1,00 ±0,53 | 10-1-5 | +2,1 |
| Okuma | +0,88 ±0,30 * | 11-3-2 | +1,0 |
| Sürme | +0,63 ±0,33 | 10-4-2 | +1,4 |
| Yoğunluk | +0,13 ±0,54 | 9-1-6 | +0,8 |
| Kesme | −0,19 ±0,40 | 6-6-4 | −0,3 |
| Cesaret | −0,19 ±0,54 | 7-1-8 | −0,5 |
| Aktarım | −0,38 ±0,34 | 2-6-8 | −0,3 |
| Tutuş | −0,88 ±0,46 | 3-2-11 | −0,9 |

- Tutuş ters çalışıyor (yüksek Tutuş kaybettiriyor, 3-2-11; sınırda anlamlı). Aktarım da ters yönde işaret veriyor. Sebebini incelemedim; mekanik bir yerde Tutuş'un bir yan etkisi olabilir (ör. topu daha çok tutup daha az oynamak).
- Kesme, Yoğunluk, Cesaret: 16 maçta etkisi görünmüyor.
- Hız en güçlüsü ve doğrudan gönderme sayısını artırıyor (v4'teki baskınlık burada da var).

## 3 · Ayar (A'da tek ayar uçta, B varsayılan; 16 maç)
- Belirgin: top kaybından sonra geri çekilme (`kayip` 0,1) −0,94*; pres 0 −0,75; hep pas (`pasSurme` 0,1) −0,63; `sut` iki uçta da kötü (−0,63 / −0,81: ortası en iyi).
- Oyunu değiştiriyor, skoru değil: `sabir` 0,9 topa sahip olma %60 (−0,56), `pasSurme` 0,9 %58.
- 16 maçta ayırt edilemeyen (±0,4 içinde): blok, genişlik, tempo, risk, kazanınca, arkada, Kenara sıkıştır.
- **Hiç çalışmayan: `sistem: 'Adam adama'`.** 16 maçın 16'sı varsayılanla birebir aynı. `shape.js` yalnız `'Kenara sıkıştır'`ı tanıyor; 'Adam adama' sadece `smallside.js`'te (maçta yüklenmiyor). Menajer/maç ekranında seçilebiliyor ve Fırtına'nın kimliği bu.
- Pres tetikleri tasarım gereği yalnız pres ≥ 2'de etkili (`presLim`). Pres 2 ile: varsayılan tetik +0,13, tetik yok +0,00, bütün tetikler −0,25 (hepsi gürültü içinde). Ayar çalışıyor (maçlar farklı), etkisi küçük.
