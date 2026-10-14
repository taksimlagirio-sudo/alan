# ALAN · zip49 ölçümü: direk (yarıçap 0,3)

Kaynak: 16 maç, 120×60, herkes 20 (Alan gücü ve Soğukkanlılık 10), ayarsız takımlar. Karşılaştırma: aynı 16 senaryo zip48 (direksiz). Maçlar ilk saniyelerden ayrışıyor (yeni gönderme tablosu kararları değiştiriyor), yani bunlar aynı maçlar değil, aynı kurulumda yeni maçlar. Hücreler küçük: yön gösterir.

## 1 · Genel
| | zip48 | zip49 |
|---|---|---|
| Sayı (16 maç) | 50 | 50 |
| Gönderme | 197 | 210 |
| Direkten sekme / maç | – | ~0,56 (9) |
| Direkten sekip giren sayı | – | 1 |
| Hesap süresi / maç | 157 sn | 162 sn (+%3) |

Direk sayısı (9) kayıttan bulundu: Çekirdeğin direk merkezine tam yarıçap uzaklıkta yön değiştirdiği anlar (direksiz kayıtlarda aynı yöntem 1 sahte sonuç veriyor). Çarpanların geliş açıları: 22° 27° 50° 70° 73° 78° 82° 88° 90°; çoğu dar.

## 2 · Gönderme açısına göre
| Açı | zip48 gönderme · oran | zip49 gönderme · oran | Sayıların payı (48 → 49) |
|---|---|---|---|
| 0–30° | 52 · %21 | 85 · %26 | %22 → %44 |
| 30–60° | 86 · %29 | 72 · %18 | %50 → %26 |
| 60–75° | 33 · %30 | 36 · %39 | %20 → %28 |
| **75°+** | **26 · %15** | **17 · %6** | **%8 → %2** |

- **Direk hedefini tuttu:** 75°+ sayı 4 → 1; akıl da bu açıdan daha az gönderiyor (26 → 17).
- Göndermeler ortaya kaydı (0–30° 52 → 85).

## 3 · Açı × mesafe (zip49)
| Açı | Yakın (0–15) | Orta (15–25) | Uzak (25+) |
|---|---|---|---|
| 0–30° | 8/14 (%57) | 7/24 (%29) | 7/47 (%15) |
| 30–60° | 4/8 (%50) | 3/17 (%18) | 6/47 (%13) |
| 60–75° | **6/7 (%86)** | 5/16 (%31) | 3/13 (%23) |
| 75°+ | 1/4 (%25) | 0/10 (%0) | 0/3 (%0) |

- Uzak ve geniş (25+, 60–75°): %31 → %23. Düştü ama hâlâ uzaktan düzden (%15) yüksek; 13 göndermede 3 sayı, kesin değil.
- **Yeni göze batan:** yakından 60–75° 6/7. Yakından düz (%57) ve yakından 30–60° (%50) daha düşük. Az örnek, ama zip48'de de bu hücre 2/3 idi. Dar açıdan yakın gönderme, düz yakından kolay olmamalı.
- Ölçüt (kullanıcı) hâlâ tam tutmuyor: geniş açı, aynı mesafeden düzden kolay olmamalı.

**Öneri:** 60–75° için sebep büyük ihtimalle Bekçi'nin yakın direkteki duruşu ve uzanışı (zip48'de bu sayılarda Çekirdek Bekçi'nin 2 birim yakınından geçiyordu). Bekçi'nin açıortay kuralı geniş açıda yakın direğe yeterince yaklaşıyor mu, bakılmalı. Claude Code istersen 60–75° sayılarında Bekçi'nin konumunu ve hareketini anlık izler.

## 4 · "Direk 0 = zip48 ile birebir" tutmuyor
DIREK = 0 ile 4 maç zip48 ile 0 / 4 aynı; ayrışma 2. saniye civarı, orta sahada. Sebep: `SHT_SURUM = 'v5-15 · direk ' + DIREK`. Direk 0'da sürüm "v5-15 · direk 0" oluyor, hiçbir tabloyla uyuşmuyor, motor gönderme hücrelerini kendisi ölçüyor ve kararlar değişiyor. Direk 0'da sürüm zip48'inki ('v5-14 · Bekçi derinliği ağza göre') olsun ve eski tablo da yüklenebilsin; ya da not "direk 0 = direksiz ama zip48 ile birebir değil" diye düzeltilsin. Oyunu etkilemiyor (varsayılan 0,3), yalnız deney için.

## 5 · Kayıt isteği
Direk çarpması topun kendi günlüğünde (`b.log`) var, maç olaylarında yok. Maç olayına "direk" yazılırsa sayım kesin olur (bugün kayıttan geometriyle bulunuyor).
