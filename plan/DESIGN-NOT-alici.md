# ALAN · Alıcı: topa doğru gelme ve top gelmeden karar (kullanıcı önerisi)

Claude Code'dan Design'a. Kullanıcı onayladı.

**Kullanıcı:** "Topu alacak oyuncu topa varmadan karar verse? Ayrıca pas atılan oyuncu topa doğru koşmuyor; top hareket hâlindeyken çok daha iyi bir noktada yakalayacakken topun üstüne gelmesini bekliyor."

## 1 · Ölçüm (zip66, 16 maç; araç `tools/alici-ucus.js`)
Alıştan önceki son 10 tikte alıcı:
| | Önüne | Ayağa (pas) | Kenardan |
|---|---|---|---|
| Duruyor | %4 | %7 | %1 |
| Topa doğru koşuyor | %59 | %62 | %85 |
| **Toptan uzaklaşıyor** | **%24** | **%30** | %8 |
| Son 10 tikin 6+'sında duran | %2 | %5 | %0 |

- Alıcı neredeyse hiç durup beklemiyor. Ama ayağa pasların %30'unda son anlarda **toptan uzaklaşıyor**: kendi hedef noktasına koşuyor, top onu arkadan kovalıyor. Ekranda "topun üstüne gelmesini bekliyor" gibi görünen bu (alıcı topa gelmiyor, top ona yetişiyor).
- Bağlantı: "kesin" pas kayıplarının %60'ı **alıcıya varmadan araya girme** (zip66 notu). Alıcı topa doğru gelse pasın yolu kısalır, rakibin araya girme şansı düşer.
- Alıştan ilk karara ~3,7 tik; ilk kararların ~%39'u fake / tut (durmak). Koşarak alıp koşmaya devam eden %50 (zip66).

## 2 · Öneri (tek kural, iki parça)

**A · Topa gelme (ayağa pas ve kenardan):** Pas havadayken alıcının hedefi pasın varış noktası değil, **top ile alıcının en erken buluşabileceği nokta** olsun: alıcı topun yoluna doğru koşar (pasın yönüne karşı), buluşma öne çekilir. Ne kadar geleceği:
- bir rakip pas yoluna yetişebiliyorsa (akıldaki kesme hesabı) tam gelir,
- yol tamamen boşsa yerinde kalabilir ya da koşusunu sürdürebilir (alıcının bir sonraki hamlesi için daha iyi yer).
Önüne pasta (koşu yoluna atılan) alıcı koşusunu sürdürür; bu kural ona uygulanmaz.
Görüş / Okuma ne kadar erken ve doğru geleceğini belirleyebilir ("iyi oyuncu pasa gelir").

**B · Top gelmeden karar:** Pas havadayken alıcı bir sonraki hamlesini seçsin (aynı akıl, topun alınacağı nokta ve an varsayılarak). Alışta:
- ilk dokunuş o hamlenin yönüne: sürecekse koşu yönüne, pas atacaksa pas açısına, gönderecekse Kuyu'ya,
- alıştan sonra yeniden karar yalnız durum belirgin değiştiyse (ör. rakip beklenmedik yaklaştı).
Bu, "koşarken alıp devam etme"yi kendiliğinden getirmeli; fake / tut seçimi alıştan önce tartıldığı için alıştaki duraklama azalır.

Bayraklar ayrı: örn. `ALAN_TOPA_GEL_ESKI`, `ALAN_ONCEDEN_KARAR_ESKI`. Hesap: B, pas başına bir karar daha (alıştaki kararın yerine geçtiği için toplamda artmamalı).

## 3 · Kontrol tablosunda beklenen
| Satır | Bugün (zip66) | Beklenen |
|---|---|---|
| "Kesin" pas kaybı (P ≥ 0,99) | %6,6 | ↓ (A) |
| Ayağa pasta son 10 tikte toptan uzaklaşan | %30 | ↓ (A) |
| Pas tutma | %92,2 | ↑ |
| Koşarak alıp koşmaya devam eden | %50 | ↑ (B; ≥ %60) |
| Koşarak alıp ilk 20 tikte duran | %33,5 | ↓ (B) |
| Alıştan ilk karara (tik) | 3,7 | ≈ 0 (B) |
| Hesap | | aynı makinede zip66 ± %10 |

Bu iki satırı (toptan uzaklaşan, kesin pas kaybı) kontrol tablosuna ekliyorum.
