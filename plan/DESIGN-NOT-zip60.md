# ALAN · zip60 kontrol tablosu ve Bekçi'nin asıl sorunu

Claude Code'dan Design'a. Kullanıcı onayladı.

## 1 · Bekçi: tepki süresi şuttan uzun (öneri A2)

Kaynak: zip59 ve zip60 kayıtları, 32 maç, 662 gönderme. Mesafe ağız ortasına. "Yerleşmiş": hız < 0,05 ve hedefine < 1 (zip59 kuralı).

| | Yakın (<15) | Orta (15–25) | Uzak (25+) |
|---|---|---|---|
| Gönderme başına sayı | %49 | %20 | %4 |
| Gönderme anında Bekçi yerleşmiş | %11 | %16 | %57 |
| Gönderme anında Bekçi'nin hızı (birim/tik) | 0,20 | 0,20 | 0,07 |
| Çekirdek Bekçi'nin hizasına (sayılarda) | **1,8 tik** | **3,4 tik** | 7,3 tik |
| Çekirdek Bekçi'nin hizasına (kaçanlarda) | 3,4 tik | 8,2 tik | 11,0 tik |
| Çekirdeğin yolu Bekçi'ye 2,9'dan yakın (gövde + uzanma) | %74–83 | %76–83 | %86–94 |

**Ne gösteriyor:**
1. **Bekçi yerinde, ama kurtaramıyor.** Sayıların %75'inde Çekirdek Bekçi'nin uzanabileceği mesafeden geçiyor. Sorun duruş değil.
2. **Tepki şuttan uzun.** `bekR`: uzanma ancak `bekReact − Okuma payı` (≈ 7,5–8 tik) dolduktan sonra başlıyor, `bekDiveV` (0,32 + Hız payı) ile büyüyor. Yakından gelen Çekirdek Bekçi'nin hizasına 2, ortadan 3–4 tikte varıyor. Bekçi hiç uzanamıyor; yalnız gövdesi (0,9) kapatıyor. Sayı ile kaçanı ayıran şey neredeyse yalnız yol süresi (sayılarda 1,8 / 3,4 tik, kaçanlarda 3,4 / 8,2).
3. **Futbolla oran ters.** 16 metreden şut kaleye ~0,5 sn'de gelir, kalecinin tepkisi ~0,2 sn; kaleci uzanacak zaman bulur. Burada şut süresi tepkiden kısa.
4. **"Yerleşmiş" kuralı (zip59) gereken yerde çalışmıyor.** Yakın ve orta göndermelerde Bekçi gönderme anında neredeyse hep hareket hâlinde (top yeni geldi ya da taşıyıcı sürüyor); yerleşmiş olma payı %11–16. Yerleşmiş olsa da tepki (×0,6 ≈ 4,5 tik) 2–3 tiklik şuta yetmez. zip59'da sayının azalmamasının sebebi bu (gönderme başına sayı %15,8 → %17,1).

**Kural (tek cümle):** Bekçi, Çekirdek ayaktan çıktığı an tepkiye başlar; tepki süresi Okuma'ya göre 2–4 tik (Okuma 20'de 2, 10'da 3, 0'da 4 gibi), uzanma hızı bugünkü gibi Hız'a göre.
- "Yerleşmiş" kuralı (zip59 madde 4) kaldırılır: tek kural, istisna yok.
- Akıl aynı `bekR`'i kullandığı için gönderme tablosu yeniden üretilmeli (`SHT_SURUM`).
- Bayrak: örn. `ALAN_BEKCI_TEPKI_ESKI`.

**Beklenen (kontrol tablosu):**
- Gönderme başına sayı **↓** (bugün %19); yakın ve orta mesafede belirgin.
- Sayı / dk **↓** (bugün 2,34; zip60'ta maç başına 4,7).
- Yakın direkten sayı / dk **↓** (≤ 0,5).
- Bekçi Okuma'sı farklı iki takımda fark görünmeli (Claude Code ayrıca ölçebilir: Bekçi Okuma 10 ve 20).
- Hesap değişmesin; diğer satırlar değişmesin.

**Not:** Oran ayarı sayıyı epey düşürebilir. Hedef sayı düzeyi kullanıcının kararı; tepki süresi bunun için tek ve anlaşılır düğme olur.

## 2 · zip60 kontrol tablosu (16 maç, taban zip59)

### 2a · Tam tablo (16 maç)
| Ölçü | zip59 | zip60 |
|---|---|---|
| Sayı / maç | 3,88 | **4,69** |
| Sayı / dk | 1,94 | 2,34 |
| Gönderme başına sayı % | 17,1 | 19,1 |
| Hesap ms/tik | 32,4 | **26,7** ✓ |
| Bekçi pas kovalaması / dk | 0,38 | 0,63 ⚠ (kazanma %67 → %95) |
| Yakın direkten sayı / dk | 0,66 | 0,81 ⚠ |
| Baskı altında "tut" seçimi % | 15,5 | 22,7 ⚠ |
| "Tut"tan sonra 1 sn'de kayıp % | 35,0 | 29,7 ✓ |
| Pastan sonra geri koşan pasçı % | 18,6 | 22,6 ⚠ |
| Boş hücumcu (rakip yarısı) % | 49,2 | 43,1 |
| Boş adam + çift adam anı / dk | 12,1 | 12,2 (hedef ≤ 5) |
| bunlarda iki kişi "arkada" / dk | 2,50 | 0,22 ✓ |
| Taşıyıcıya baskı yok % | 17,8 | 13,7 ✓ |
| Topsuz rakipte 4 br içinde 2 savunmacı / dk | 28,4 | **44,0** ⚠ |
| Geçiş "git" tik / dk | 39,0 | 77,6 ✓ |
| Uzun tutuşta 1 sn+ duran hücumcu | 0,14 | 0,00 ✓ |

### 2b · Bayraklarla ayırma (senaryo 0–7, 8'er maç)
| Ölçü | zip59 | zip60 | B kapalı | C3 kapalı |
|---|---|---|---|---|
| Sayı / maç | 3,25 | 4,63 | 3,50 | 3,88 |
| Gönderme başına sayı % | 14,5 | 18,1 | 15,5 | 15,6 |
| Topsuz rakipte 4 br içinde 2 savunmacı / dk | 30,6 | 38,7 | **29,4** | 45,0 |
| Pastan sonra geri koşan pasçı % | 17,6 | 23,2 | **17,2** | 21,2 |
| Boş adam + çift adam anı / dk | 13,3 | 12,1 | 11,9 | 12,9 |
| Baskı altında "tut" seçimi % | 14,3 | 22,1 | 26,2 | 23,5 |
| Geçiş "git" tik / dk | 52,9 | 66,1 | 72,1 | **43,3** |
| Top kazanınca 3 sn içinde gönderme % | 16,2 | 16,8 | 15,0 | 20,7 |
| Hesap ms/tik | 32,3 | 26,8 | 27,4 | 25,3 |

**Sonuç:**
- **B (atama) sayıyı artırıyor ve hedefini tutmuyor.** B kapalıyken sayı 4,63 → 3,50; topun yakınında çift savunmacı 38,7 → 29,4 (zip59 düzeyi); geri koşan pasçı 23,2 → 17,2. Asıl hedef "boş adam + çift adam" değişmiyor (12,1 / 11,9). Yani atama savunmacıları topsuz rakiplerin dibine topluyor (iki kişi aynı rakibin 4 birim içinde), ama boş adamı azaltmıyor; savunma zayıflıyor. Muhtemel sebep: hedef `kapatHat` (adam ile Kuyu arası) presçinin ve atananın aynı bölgeye düşmesi; ve "atanmayan, başkasına atanmış rakibin peşindeyse evinde boşluk tut" yerine görevini sürdürüyor olabilir. Öneri: B şimdilik kapalı (`ALAN_ATAMA_ESKI` varsayılan), atama "aynı rakibe 4 birim içinde ikinci kişi yok" koşuluyla yeniden; kabul: 4 br çift savunmacı ≤ zip59, boş adam + çift adam ↓, sayı artmasın.
- **C3 (geçiş "eşitsek de") istediğini yapıyor:** "git" 43 → 66, top kazanınca 3 sn'de gönderme yükselmiyor ama sayıya +0,75 / maç katkısı var. Kontra oyunu açılıyor; beklenen bir etki. Kalsın.
- **C2 (kopya yalnız gerektiğinde) hızı getirdi** (32 → 27 ms/tik, zip50'den hızlı). Ama baskı altında "tut" seçimi her kolda arttı (%14 → %22–26); bunu B de C3 de açıklamıyor, ortak olan C2. Sebep olabilir: atlanan anda "tut" ilk (iyimser) değerinde kalıyor. Baskı varken hep oynatılıyor deniyor; ama "baskı" eşiği (4 birim) ile tablonun eşiği aynı, sınırdaki anlar kaçıyor olabilir. Claude Code `ALAN_BEKLE_HEP` ile 8 maçta doğrulayabilir.
- **C1 (bekleme kaydı) çalışmıyor:** `match.js` `lastDec.opts` seçenekleri yeniden kurarken `hw`'yi atıyor (`opts: r.opts.slice(0, 4).map(o => ({ kind, to, q, P, v }))`). `mac-isci` `o.hw`'yi `d.opts`'tan okuyor, hep boş. Düzeltme: o map'e `hw: o.hw` eklemek. Kayıt aracı hazır; düzelince kalibrasyon satırları dolacak.
