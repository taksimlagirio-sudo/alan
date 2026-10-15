# ALAN · zip67 ve zip68–69: koşmaya devam etme hâlâ %53; A ve B maçta etki göstermedi

Claude Code'dan Design'a. 16'şar maç; hız aynı makinede, aynı anda (senaryo 0–1).

## 1 · zip67 (sürme kopyası zip65 kapsamına döndü)
| | zip65 | zip66 | zip67 | Beklenen |
|---|---|---|---|---|
| Koşarak alıp koşmaya devam eden % | 58,6 | 50,4 | **53,4** | ≥ 58 ✗ |
| Koşarak alıp ilk 20 tikte duran % | 25,6 | 33,5 | 30,1 | ≤ 26 ✗ |
| Pas tutma % | 92,3 | 92,2 | 92,7 | |
| Hesap (zip66'ya göre, aynı makine) | | | +%13 | ≤ +%20 ✓ |
| "Tut"tan sonra 1 sn'de kayıp % | 29,8 | 18,2 | 24,1 | |

zip65'in %58,6'sına dönmedi. zip65'te buluşma pası da açıktı; ayrıca bu satır 16 maçta birkaç puan oynuyor. zip65 bayrak ölçümündeki (aynı 8 maç) 53 → 59 farkı muhtemelen kısmen gürültü. Sürme kopyasının kalıcı etkisi bu tabloyla küçük görünüyor (~+3 puan).

## 2 · zip68–69 (A: alıcı topa gelir · B: top gelmeden karar)
| Satır | zip67 | zip69 | Beklenen |
|---|---|---|---|
| Ayağa pasta son 10 tikte toptan uzaklaşan alıcı % | 26,6 | **25,8** | ↓ (A; kısa sınamada %39 → %19) ✗ |
| "Kesin" pas kaybı % | 6,28 | **6,78** | ↓ (A) ✗ |
| — bunlarda alıcıya varmadan kesilen | | %64 | ↓ ✗ |
| Pas tutma % | 92,7 | 91,4 | ↑ ✗ |
| Alıştan ilk karara (tik) | 3,72 | **2,80** | ~1 (B) kısmen |
| Önceden kararın kullanıldığı alış (`m.st.onceden` / pas) | | **%28** | çoğu |
| Koşarak alıp koşmaya devam eden % | 53,4 | **53,5** | ≥ 60 (B) ✗ |
| Koşarak alıp ilk 20 tikte duran % | 30,1 | 29,5 | ↓ ✗ |
| Hesap (zip67'ye göre, aynı makine) | | +%6 | ± %5 ✓ (sınırda) |
| Sayı / maç | 2,88 | 3,63 | |
| Gönderme başına sayı % | 12,7 | 16,1 | |

**A maçta görünmüyor.** Kısa sınamadaki iyileşme (%39 → %19) 16 maçta yok (%27 → %26); kesin pas kaybı da düşmedi. Olasılıklar:
1. Ayağa pasta alıcının uçuş sırasındaki hareketi `alici()`'den başka bir yerden geliyor (ör. alıcı görevine geçmeden önceki plan, ya da `alici()` yalnız bazı pas türlerinde / ilk tikte çağrılıyor).
2. Yolda rakip "önden varıyor" koşulu nadiren doğru çıkıyor; akıldaki rakip varış hesabı (tepki, hız) maçtakinden iyimser (kesin pas kayıplarının %64'ü hâlâ yolda kesilme). Bu, akıldaki kesme hesabı ile `contact`'ın karşılaştırılması işiyle aynı kök.
Öneri: bir alıcının pas uçuşunda tik tik `alici()` seçimini (buluşma noktası, yoldaki en küçük pay) yazdıran izle, kesilen bir pası incelemek.

**B kısmen çalışıyor ama koşmaya devam etmeyi değiştirmedi.** Önceden karar alışların yalnız %28'inde kullanılıyor (sapma eşikleri — rakip 1,5 yaklaştı / alıcı 4,5 saptı — çoğu alışta tetikleniyor). Kullanıldığında da koşmaya devam etme değişmiyor: karar önceden verilse de **içeriği** aynı (fake / tut / pas). Yani durmanın sebebi zamanlama değil, seçilen hamle. Öneri: koşarak alınan anlarda seçilen ilk hamlenin dağılımı (sür / pas / fake / tut) ve "sür"ün değerinin diğerlerinden ne kadar düşük olduğu; Claude Code karar kaydından çıkarabilir (bir sonraki adım).

## 3 · Akış
Sayı 2,88 → 3,63; dar açıdan ve yakın geniş açıdan sayılar arttı; Bekçi "sayılarda topa değmiş" %36 → %28. A/B'nin doğrudan Bekçi'ye etkisi beklenmez; akış değişikliği. Bayraklarla ayırmayı (8'er maç) istersen yaparım; ama A ve B hedef satırlarında etkisiz olduğu için önce yukarıdaki iki inceleme daha verimli.
