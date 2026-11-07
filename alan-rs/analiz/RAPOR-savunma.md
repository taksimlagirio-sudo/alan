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

Oturumun kendi 20 kayıtlı kurulumu (farklı takımlar ve taktikler; `ALAN_YERLESIK_ADAM_ESKI=1` ikisinde de). Maç başına,
eşleşmiş fark ± standart hata:

| | Kapalı | S1+S2+S3 | Fark | Artan/azalan |
|---|---|---|---|---|
| Gol | 1,20 | 1,30 | +0,10 ± 0,33 | 8/6 |
| Şut | 2,30 | 2,15 | −0,15 ± 0,44 | 6/8 |
| Pas | 24,4 | 29,7 | +5,4 ± 2,9 | 12/7 |
| Pas isabeti (%) | 81,1 | 70,3 | **−10,8 ± 3,2** | 5/15 |
| Top kaybı | 9,9 | 13,4 | **+3,5 ± 1,4** | 14/5 |

Gol ve şut değişmiyor: maç 217468'deki 2-3 tek tohumun oynaklığı gibi görünüyor, çok kurulumda "arkada boşluk, fazla gol"
yok. Belirgin olan pas isabetinin düşmesi ve top kaybının artması: baskı topu kazanıyor (S1'in amacı). Analizcinin
başka tohumlarla ölçmesi önerilir: Liman/Kale Duvarı gibi pres = 1 / pres = 0 eşleşmelerinde ayrı ayrı.

## Main birleştikten sonra yeniden ölçüm ve S7 (6 tohum, `cok.sh`)

Dal main'i (`441ce0b`, `afc56a9`, `da32777`, `c6e692f`, `e0411be`, `d0a344f`) ve analizci dalını (`1ec253c`) içeriyor.
Yerleşik adam adama artık **varsayılan kapalı** deney (`ALAN_YERLESIK_ADAM_IZ`, sitede `?yerlesik=1`); main ile birleşince
`tests/ayni.rs` onun yüzünden ayrışıyordu. `ALAN_YERLESIK_ADAM_ESKI` artık gerekmiyor. `tests/mac_kafa.rs` düzeltmesi main'e
ayrı commit olarak gitti (`63197f0`).

### S7 — taşıyıcıyı durdurmak (`62c21f0`, `sekil/pres.rs`, `savunma.rs`, `esles.rs`; varsayılan kapalı)
Tehlike bölgesi: taşıyıcı kale ortasına `S7_D = 32 GKX`'ten yakın.
- **a** `ALAN_S7A_IZ`: kale tarafında olmayan ilk presçi yandan dolanmaz ("pres · yandan giriyor / yanında / yolu kapat"
  yerine), taşıyıcı → kale ortası çizgisinde taşıyıcıdan önce varabildiği ilk noktaya koşar: **"pres · şut hattı"**.
  Dibindeyken (1,6 br, gövde arkasında değil) topa girme yolu aynı.
- **b** `ALAN_S7B_IZ`: tehlike bölgesinde tetik beklemeden `p_lim` ≥ 2.
- **c** `ALAN_S7C_IZ`: topun gerisinde kalmış savunmacı adamına değil kale önüne, şut hattının %55'ine:
  "geri koşu · kale önü".
- `ALAN_S7_IZ` üçü birden; ölçüm/wasm için `S7_ZORLA` (bit 1/2/4).

### Sonuçlar (tohum 217468, 1–5; skor Liman–Kale Duvarı; `analiz/py/hatta.py`)

| | Skorlar | Liman G/M | KD şut | KD gol | gol/şut | hattı boş şut / gol |
|---|---|---|---|---|---|---|
| Kapalı (yeni main) | 2-2 2-4 0-5 2-3 0-1 1-6 | 0/5 | 39 | 21 | 0,54 | 26 / 16 |
| **S7a** | 2-2 0-2 2-0 0-1 0-0 4-3 | 2/2 | **18** | **8** | 0,44 | 11 / 5 |
| S7b | 1-3 1-4 2-5 2-4 2-2 3-4 | 0/5 | 40 | 22 | 0,55 | 23 / 14 |
| S7c | 4-3 3-4 2-4 1-8 3-3 3-4 | 1/4 | 44 | 26 | 0,59 | 21 / 15 |
| S7 (a+b+c) | 1-2 0-1 0-0 1-3 1-3 2-4 | 0/5 | 23 | 13 | 0,57 | 8 / 3 |
| S1+S2+S3 | 2-4 1-4 3-2 2-4 3-2 5-2 | 3/3 | 32 | 18 | 0,56 | 16 / 9 |
| S1+S2+S3+S7 | 2-2 2-5 2-2 2-5 2-1 2-2 | 1/2 | 25 | 17 | 0,68 | 19 / 14 |

Yorum:
- **Analizcinin 4 tohumluk sonucu yeni main'de tersine döndü:** S1–S3'ün sorunu değil, main'in kendisi (kapalıyken Liman 0/6
  kazanıyor, KD 21 gol, 16'sı hattı boş şuttan). S1+S2+S3 bunu kısmen düzeltiyor (3/6 galibiyet, hattı boş gol 16 → 9).
- **S7a işin özü:** KD şutu 39 → 18, golü 21 → 8. Tek savunmacının yandan dolanması gerçekten sorundu.
- **S7b etkisiz, S7c zararlı** (KD 26 gol): geçilen savunmacıları kale önüne çekmek adamlarını boş bırakıyor; pas alan boş adam
  şut çekiyor. c'yi açmayın.
- S1+S2+S3+S7'deki hattı boş gollerin çoğu **kontra**: Liman hücumdayken top kaybı, savunmacılar 50–90 br geride tam hızla
  dönüyor (iz: t2 5438–5478). S7'nin konusu değil; geçiş (hücumdan savunmaya) ayrı iş.
- Gol/şut hâlâ yüksek (0,44–0,68): hatta savunmacı olsa da şutlar giriyor. Kaleci / şut sapması tarafı ayrıca bakılmalı.
- Baskıcının barı (geç gollerde .04–.11) bu turda ele alınmadı.

## Hepsi açık setinden çıkanlar (analizci değerlendirmesiyle)
- **S3 (`ALAN_SAV_TABAN_IZ`, `savtaban`) bırakıldı:** ofsayt yok; çizgiyi topa göre tabanda tutmak rakibin en önündeki
  hücumcusunu çizginin arkasında bırakıyor (217468, Kale Duvarı topta: en geri Liman adamının arkasında hücumcu bayraksız %4,
  S3 ile %49). Çizgi eskisi gibi rakibin en önündeki hücumcusuna göre kurulur. Kod bayrak arkasında duruyor; `yeni=1` ve
  `bayraklar.env` artık açmıyor (açıkça `savtaban=1` ile açılır).
- **S7b etkisiz, S7c zararlı (yerini S7d aldı):** `s7=1` artık a, d, e, f, g'yi açar (bit 121); `bayraklar.env`'de
  `ALAN_S7_IZ` yerine `ALAN_S7A_IZ`.
- **bek_direk** yalnız yakın direk şutunda (`c718f9b`; top kale çizgisine 12 br içinde, en az 45° yanda): Kale Duvarı 30+ br'den
  6 şut / 0 gol (önce 25 / 19).

## B2 (KP_HAMLE) ve S7o, 16 tohum (7 Kasım ~15:35)
Tohum 301–316 × 4500 tik, `analiz/bayraklar.env` (S7n dahil); kollar: taban (6699e8b sonrası dal), S7o (`ALAN_S7O_IZ`),
KP_HAMLE (`ALAN_KP_HAMLE_IZ`). Ölçü betikleri: oturumun çalışma klasöründe `b16/pas2.py`, `hat.py` (git dışı).

| | taban | S7o | KP_HAMLE |
|---|---|---|---|
| kaleye ≤25 br pas | 193 | 176 | 210 |
| … alıcıya | %80 | %85 | %91 |
| … (b) uçuşta rakibe | %17 (33) | %14 (24) | %8 (16) |
| (a) pas çıkmadan top kaybı (kaleye ≤25 br) | 14 | 16 | 13 |
| rakip topta en geri savunmacının arkasında rakip | %3,9 | %3,7 | %4,9 |
| 'öne çık' (S7i) olan tik | %2,3 | %1,2 | %2,4 |
| goller (iki takım, 16 maç) | 43 | 39 | 61 |

- KP_HAMLE: rakibe pas ve pas çıkmadan kayıp artmadı, azaldı (4 tohumdaki %11 → %16 gürültüydü). Sansürlü keep_p kalibrasyonu
  (4 tohum): bayraksız kutu önü pre 0,34 → gerçek tutma 0,90, 0,46 → 1,00; KP_HAMLE'de düşük tahminler gerçek kayıplara denk.
- S7o: arkadaki rakip artmadı (tohum başına yayılım %0–8). Kutuda takas ayrıca sayılmadı.
- Karar analizcide (izleme seti, site).

## Kontra tablosu (A2) ve hız kancası
- `motor/veri/bilgi/kontra.json` sürüm 2 (8fae8d7): yakınsamadı (0,16 > 0,03) ve hayatta kalma seçilimi (kazanım sonrası
  örnekler): 40–60 br'de öndeki savunmacı arttıkça değer artıyor. Kazanım-anı sınaması (86 kayıt): önde 1 → %19, 2 → %12,
  3 → %8 (ZAMAN-DEGERI yönü). Tablo izleme setine / siteye girmiyor; ktab yeni=1 dışında.
- Nedensel ölçü: `alan-kos hizdal` + `sina/bilgi/hizdal.{sh,py}` (6b9f94b): kontra anında koşucu 90 tik depar / tempo
  (mac::enerji::HIZ_ZORLA), iki kol 360 tik; sonuç 12 sn içinde gol, kutuya varış, bitiş. 200 an, 301–316 çalışıyor.
- hizdal ara sonuç (70 an, kanca kafanın kopyasında kapalıyken): 12 sn içinde gol depar %1,4 / tempo %20,0; kutuya varış
  %35,7 / %41,4. **Geçersiz:** taşıyıcının kafası koşucuyu zorlanmamış (tempo) hızla öngörüp pası oraya atıyor, depar atan
  koşucu başka yerde kalıyor; ölçülen depar değil, kafayla gerçeğin uyuşmazlığı. Kanca kafanın kopyasında da açıldı, ölçüm
  yeniden çalışıyor (83 an).
- hizdal 2. deneme (83 an, zorlamasız kol eklendi): gol depar %13,3 / tempo %9,6 / zorlamasız %7,2. **Bu da geçersiz:**
  taban maçlar (kontra anı seçimi) eski bayraklar.env ile, kollar S7o + KP_HAMLE eklenmiş setle oynandı; maç tike dek farklı
  akabilir. Bütünüyle tek setle yeniden çalışıyor (taban maçlar dahil); aynı 16 maçta S7o + KP_HAMLE açıkken "en geri
  savunmacının arkasında rakip" de ölçülecek.
- **hizdal 301–316, tek setle (geçerli):** 77 kontra anı, 1 geçersiz (zorlamasız kol taban maçtan ayrıştı); 76 an: 12 sn içinde
  gol depar %11,8 / tempo %7,9 / zorlamasız %6,6; kutuya varış %32,9 / %39,5 / %42,1. Fark küçük ve örnek az (±4 puan);
  S1 (317–348, koşucu) ve S2 (317–348, savunma kolu) işçileri çalışıyor. Kısa sonuç: `analiz/olcum/hizdal-kosucu-301-316/`.
- Aynı 16 taban maçta (bayraklar.env, S7o + KP_HAMLE dahil) en geri savunmacının arkasında rakip %4,8.

## İş / hedef gidip gelmesi (S7q, S7r)
Ölçüler `analiz/py/gidip.py` (markaj ↔ geri koşu A→B→A, 30 tik), `analiz/py/ters.py` (savunan takımda hedef 10 br+ sıçrayıp
yönü 90°+ dönerse; 30 tikte geri dönen), `analiz/py/hat.py` (arkadaki rakip). 4 tohum × 4500 tik, bütün set:

| | taban | S7q | S7q + S7r |
|---|---|---|---|
| markaj ↔ geri koşu gidip gelme / maç | 53 | 16 | 19 |
| hedef ters dönüşü / maç (30 tikte geri dönen) | 264 (120) | 260 (116) | 230 (90) |

En büyük ters dönüş kaynağı 'önde bekle' → 'önde bekle' idi (95 / 4 maç): onde_ac'ın yer koruması iş adına bakıyordu, iş
adı her tik sıfırlandığı için hiç çalışmamış. S7r ile önde bekleyenin hedefi tikte en çok 1 br kayıyor (o kaynak listeden çıktı;
toplam 4 tohumda akış farkıyla değişmedi). Kalan büyük kaynaklar: markaj → markaj (adam ya da adamın yeri), 'boşluk' ↔
'arkada · blokta' (top el değiştirirken; gerçek gidip gelme değil). 16 tohumla doğrulanmalı.
