# ALAN · zip66: hız zip63 düzeyinde · koşmaya devam etme kazancı gitti · "kesin" pas kayıpları

Claude Code'dan Design'a.

## 1 · Hız: makine değişmiş, ölçümüm yanıltıcıydı
Konteyner yeniden başladıktan sonra makine yavaşladı (4 maç 3,5 dk yerine 5 dk). zip63 eski makinede, zip64–66 yeni makinede ölçüldü; tablodaki hesap satırları bu yüzden karşılaştırılamaz.

**Aynı makinede, aynı anda** (senaryo 0–1, ikişer maç):
| | ms/tik |
|---|---|
| zip63 | 39,6 |
| zip66 | **38,8** |

zip66 zip63 ile aynı hızda: hedef tuttu. **Düzeltme:** zip64 notundaki "buluşma pası +%41" makine değişiminden etkilendi, o rakam geçersiz. zip65'teki "sürme kopyası +%19" aynı makinede bayrakla ölçülmüştü, o geçerli. Bundan sonra kontrol paketine her seferinde bir önceki sürümden aynı anda 2 maç ekleyeceğim (hız satırı makineye bağlı kalmasın).

## 2 · Kontrol tablosu (16 maç)
| Satır | zip63 | zip65 | zip66 | Beklenen |
|---|---|---|---|---|
| Koşarak alıp koşmaya devam eden % | 54,4 | 58,6 | **50,4** | ≥ 58 ✗ |
| Koşarak alıp ilk 20 tikte duran % | 28,8 | 25,6 | **33,5** | ≤ 26 ✗ |
| Pas tutma % | 91,5 | 92,3 | 92,2 | ≥ 92 ✓ |
| "Tut"tan sonra 1 sn'de kayıp % | 26,7 | 29,8 | **18,2** | izlenecek ✓ |
| Sayı / maç | 3,06 | 3,00 | 3,88 | |
| Gönderme başına sayı % | 14,3 | 13,3 | 16,3 | |
| Sayılarda Bekçi topa değmiş % | 40,8 | 34,8 | 33,9 | zip63'e dönsün ✗ |
| Yakın direkten sayı / dk | 0,34 | 0,44 | 0,56 | zip63'e dönsün ✗ |
| Boş adam + çift adam / dk | 13,0 | 11,1 | 10,2 | ✓ |
| Geçiş "git" tik / dk | 103 | 120 | 129 | ✓ |

**Sürme kopyası ucuzlayınca kazancı da gitti:** koşmaya devam eden %58,6 → %50,4 (zip63'ün de altında), ilk 20 tikte duran %25,6 → %33,5. "Önü 6 birim açık ve sür ilk iki seçenekte" koşulu, kazancın geldiği anları dışarıda bırakıyor gibi: koşarak alan taşıyıcının önünde çoğu zaman 6 birim içinde biri var (pres), ama ilerlemek yine de değerli.

**Öneri:** Sürme kopyasını zip65'teki kapsamına geri al (en iyiye 0,1 yakınken, 30 tik). Bedeli aynı makinede +%19 ölçülmüştü; buluşma pası kapandığı için toplam hız zip63'ün ~%20 üstünde kalır. Hız daha da kısılacaksa "önü açık" koşulu yerine "taşıyıcı koşuyor (hız > 0,15)" denenebilir (kullanıcının şikâyeti tam bu anlar).

Sayı arttı (3,06 → 3,88) ve Bekçi satırları kıpırdadı; zip63'ten bu yana akıştaki değişikliklerin toplamı. Tek başına bir değişikliğe bağlayamıyorum.

## 3 · "Kesin" pasların kaybı (zip64 + zip65, 3095 pas, P ≥ 0,99)
Kaybedilen 203 (%6,6):
| Sebep | Pay |
|---|---|
| Top alıcıya varmadan rakip kesti | **%60** |
| Top alıcının 2,5 birim yakınına geldi, alıcı alamadı, rakip aldı | **%36** |
| Arada sekme / çarpma | %3 |

Türe göre kayıp: ayağa %3,2 · önüne %6,5 · kenardan %7,8 · **kendine %16,8**.

**Yorum:**
- Asıl iyimserlik **araya girme**: akıl rakibin pas yoluna yetişmesini olduğundan az görüyor. Akıldaki kesme hesabı (`laneOk`, `oppReach`, rakibin tepkisi) maçtaki `contact` ile aynı mı, karşılaştırılmalı. Kendine pas en kötüsü (top rakibe yakın bırakılıyor).
- Alıcının yakına gelen topu alamaması (%36): kontrol kapasitesi mi (göreli hız), yoksa alıcının konumu mu; bir sonraki izde temas kancasıyla (Bekçi için yaptığımız gibi, arkadaş teması) ayrılabilir.
Araç: `tools/pas-kayip.js`.
