# Rapor: SAVUNMA (S1–S3) — savunma oturumundan analizciye

Dal: `claude/youthful-cerf-w5mm8b` (analiz dalı `claude/youthful-pascal-i91fpn` içinde). Hepsi **deney bayrağı arkasında,
varsayılan kapalı**; bayraklar kapalıyken davranış main ile aynı (`sina/ayni.sh`). `cargo test --release` yeşil
(14 / 0 / 12).

Not: dalda ayrıca kullanıcının istediği, bu görevden önceki bir savunma değişikliği var: **yerleşik hücumda adam adama**
(top kalemize 30 br'den yakın ve rakibin en az yarısı yarı sahamızdayken: taktik alan olsa da adam adama, bölge
yarıçapı yok, blok kalkar, topa en çok bir presçi). O **varsayılan açık**; eski davranış `ALAN_YERLESIK_ADAM_ESKI=1`
(sitede `?yerlesik=0`). Aşağıdaki bütün ölçümler onu kapatarak (`ALAN_YERLESIK_ADAM_ESKI=1`) alındı, yani main + yalnız
S bayrakları.

## Commit'ler ve bayraklar

| Görev | Commit | Bayrak (yerelde; `iz_acik` gerekir, adı `_IZ` ile biter) | Dosya |
|---|---|---|---|
| S1 baskının değeri | `faa18ba` | `ALAN_PRES_KAZANC_IZ` (`PRES_KAZANC_ZORLA`) | `esles.rs` (taşıyıcı işinin `w0`'ı) |
| S2 atamada süreklilik | `ab9aaf6` | `ALAN_UA_HIST_IZ` (`UA_HIST_ZORLA`, `UA_HIST = 0.25`) | `esles.rs` (eşleşme), `savunma.rs` (`markaj`) |
| S3 savunma tabanı | `0d96a22` | `ALAN_SAV_TABAN_IZ` (`SAV_TABAN_ZORLA`; `SAV_TABAN_MIN = 8 GKX`, `SAV_TABAN_ARA = 45 GKX`) | `savunma.rs` (`taban`) |
| sınama düzeltmesi | (son commit) | — | `tests/mac_kafa.rs` (`Pl` alanı; main'de `1f0ec1a`'dan beri derlenmiyordu) |

### S1 — baskının değerine kendi kazancımız
Taşıyıcı işinin ağırlığı `w0 = t_p.h × K` idi (kafanın "o olmasa"sı); bayrakla
`w0 += max(0, tac.pres) × max(0, threat(savunan takım, taşıyıcının yeri)) × K`. Topu orada kazanırsak rakip kalesine
yakınlık; kafanın haritasından (hücum oturumunun `kafa2_zemin`) bağımsız. `pres = 0` olan takımda (Kale Duvarı) 0.

### S2 — iş değiştirme maliyeti
Eşleşme atamasında önceki tikin işi (adam işi: `uaPrev`/`markRef` aynı adam; taşıyıcı işi: `_savCar` bir önceki tik)
değerde × 1,25. Adam adama atamada (`savunma.rs markaj`) önceki adama ayrıca yetişme süresinde `0,25 × SH` tik pay ve
değerde × 1,25 (yetişme sınırındaki adam her tik verilip alınıyordu: `markaj · Ok ↔ arkada · alan`).

### S3 — en geri çizgi için taban
`yedek`'ten sonra `markaj` / `arkada` / `kapatma` / `bölge` işindeki savunmacının hedef derinliği
`taban = min(blok çizgisi, max(8 GKX, top derinliği − 45 GKX))`'nin gerisine inmez (pres, kovalama, geri koşu, Bekçi
dışarıda). Top 64 br'den yakınken taban yok.

## `olc.sh` önce / sonra (maç 217468, 9000 tik; Liman savunurken)

| Ölçü | Önce | S1 | S2 | S3 | S1+S2+S3 |
|---|---|---|---|---|---|
| Skor (Liman – Kale Duvarı) | 3-0 | 3-2 | 0-1 | 1-1 | 2-3 |
| Liman savunma tiki | 3643 | 2770 | 4742 | 1584 | 1871 |
| Kimse pres işinde değil | %75 | %27 | %78 | %58 | %38 |
| En yakın savunmacı 15+ br | %40 | %17 | %65 | %13 | %15 |
| En uzun baskısız dönem | 28 sn | 2,8 sn | — | — | 4,1 sn |
| İş değişimi / oyuncu / savunma dk | 93 | 151 | 50 | — | 105 |
| 12 br+ hedef sıçraması / savunma dk | 201 | 269 | 109 | — | 223 |
| 20 tikte eski hedefe dönüş / savunma dk | 62 | 85 | 40 | — | 52 |
| Top 80+ br uzakken en geri oyuncu derinliği (ortanca) | 13,7 br | 29,2 | — | 37,4 | 46,0 |
| … 15 br'den derinde olduğu tik | %54 | %25 | — | %1 | %0 |
| Kaleye 12 br içinde Liman saha oyuncusu (oyuncu-tik) | 1594 | 708 | — | 107 | 0 |
| "Kuyu alanından çıkıyor" (oyuncu-tik) | 136 | 91 | — | 5 | 0 |

Yorum:
- S1 baskıyı kuruyor (asıl hedef). Yan etkisi iş değişiminin artması (baskıcılar pres / yolu kapat / markaj arasında).
- S2 tek başına git-geli ~yarıya indiriyor; S1 ile birlikte S1'in getirdiği artışı kısmen geri alıyor. Dikkat: S2 tek başınayken
  "en yakın savunmacı 15+ br" %40 → %65 kötüleşti (maç farklı aktı; baskı işini sürdürme ayrıcalığı, baskıya geçişi de
  zorlaştırıyor olabilir). S1'siz kullanılmamalı.
- S3 çizgiyi tutuyor, Kuyu girişlerini bitiriyor.
- **Skor tek tohumda çok oynak** (S2 tek başına baskıya ve çizgiye dokunmadığı hâlde 0-1). S1+S2+S3'te Kale Duvarı 3 gol
  attı: önde baskı + yüksek çizginin arkada boşluk bırakıp bırakmadığı başka tohumlarla ölçülmeli (aşağıdaki çok
  kurulumlu kontrol).
- Ölçü betikleri (git dışı, oturumun çalışma klasöründe): iş değişimi/sıçrama/dönüş ve taban ölçüleri `analiz.jsonl`'den;
  istenirse `analiz/py/`'ye taşınabilir.

## Çok kurulumlu kontrol (20 kurulum × 4000 tik, bayrak kapalı → S1+S2+S3)

(eklenecek)
