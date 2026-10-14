# ALAN · zip49b kontrolü ve Bekçi izi

## 1 · zip49b ✓
- Direk 0,3 (varsayılan): zip49 ile 4 / 4 maçta her tikte aynı (konum, hız, hedef, görev, skor).
- Direk 0: zip48 ile 4 / 4 maçta her tikte aynı, maçın tamamı.
- Direk olayı kayıtta görünüyor (4 maçta 4).

## 2 · Bekçi izi: 60–75° sayılarının sebebi
Kaynak: 32 maç (zip48 ve zip49 kayıtları; Bekçi kuralı ikisinde aynı), 120×60. Her göndermede Bekçi'nin gönderme anındaki yeri, hedefi, görevi ve öncesi.

### Sayılarda Bekçi çoğunlukla yerinde değil
"Yerinde" = gönderme anında kendi hedefine (açıortay noktası) 3 birimden yakın.
| Açı | Sayılarda Bekçi yerinde | Kaçanlarda Bekçi yerinde |
|---|---|---|
| 0–30° | 12/29 (%41) | 93/104 (%89) |
| 30–60° | 12/35 (%34) | 104/120 (%87) |
| 60–75° | **6/22 (%27)** | 37/45 (%82) |
| 75°+ | 1/3 | 25/37 (%68) |

Bu her açıda geçerli; geniş açıda en belirgin. 60–75° sayıların dörtte üçünde Bekçi yerinde değil.

### Neden yerinde değil: pası kovalayıp kaybediyor
Bekçi yerinde değilken giren 58 sayının:
- 53'ünde gönderen topu pasla almış; pas uzun (ort. 37 tik), alıştan göndermeye 4 tik.
- 50'sinde Bekçi o pası kovalamış (görev "kovala"), yetişememiş, geri dönerken gönderme gelmiş. Gönderme anında ağız ortasına ort. 12,5 birim uzakta.
- Pası atan oyuncuya çıkmış değil (pasçıya ort. 27 birim): pası arada kesmeye koşuyor.

Genel tablo (gönderme anından geriye 2,5 sn içinde Bekçi bir pası kovalamış mı):
| | Sayı | Kaçan |
|---|---|---|
| 0–60° | 67/71 (**%94**) | 123/224 (%55) |
| 60–90° | 27/29 (**%93**) | 71/83 (%86) |

Bekçi maç başına ~17,6 kez pas kovalıyor (6 tik ve üstü); bunların **%65'inde top rakibe gidiyor** (564'te 365).

### Kural
`shape.js` ~112: Bekçi bir pası kovalamaya, **kendi takımında topa ilk o varacaksa** çıkıyor (`r.est <= bestE`, `bestE` yalnız kendi takımından). Rakibin varış süresi hiç karşılaştırılmıyor. Yani forvet ondan önce varacak olsa da Bekçi kalesini bırakıyor. Top Kuyu'ya doğru geliyorsa (`kaleye`) ayrıca pay da alıyor (+5 + Yerleşim payı).

## 3 · Öneri
Bekçi bir pası, **rakipten önce** (belirgin payla) varacaksa kovalasın; yoksa açıortayda kalsın ya da açıortaya göre yeniden yerleşsin (pasın varış noktası zaten `E` olarak kullanılıyor). Pay sabiti Okuma / Yerleşim'e bağlanabilir: iyi Bekçi ne zaman çıkacağını daha iyi bilir. Futboldaki karşılığı: kaleci ancak topa forvetten önce varacaksa çıkar.

Bu, 60–75° sorununu ve genel olarak "kaleci yerinde değil" sayılarının çoğunu birlikte çözmeli. Sayı sayısı düşebilir; o zaman sayının nereden geldiğine yeniden bakılır.

**Claude Code ölçecek (16 maç):** Bekçi kovalaması ve kazanma oranı, sayılarda Bekçi'nin yerinde olma payı, açı × mesafe tablosu, toplam sayı.
