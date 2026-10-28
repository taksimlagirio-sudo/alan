# ALAN · Paralel kafa: ikinci çekirdek neden kapanıyor, maç neden takılıyor (28 Ekim 2026)

> Motor: main 3efa691. Maç: 775141 kaydı (`kayit.json`), başsız Chromium, çapraz köken yalıtımlı, 2 işçi.
> Araçlar: `tools/paralel/` (sunucu, tarayıcı sürücüsü, iz yaması, karşılaştırıcılar, Node denemeleri). Motor dosyalarına dokunulmadı; izler sitenin bir kopyasında.

## Kısa sonuç

- **Takılma, iki işçinin birbirini 15 sn beklemesi** (`paralel.js` `SURE`). Ardından paralel kip kapanıyor, yardımcı işçi kapatılıyor, maç tek çekirdekte sürüyor.
- **Beklemenin sebebi, iki işçinin maçı aynı oynamaması.**
  - Konumlar önce 1e-8 düzeyinde ayrışıyor. Kaos bunu büyütüyor.
  - Birkaç yüz tik sonra bir kararın iş sayısı iki işçide farklı çıkıyor (örnek: yardımcıda 34, ana işçide 33).
  - Bir işçi dalgayı bitirip sonrakine geçiyor. Öbürü hiç gelmeyecek 34. işi bekliyor.
- **Ayrışmanın kökü:** bir önbelleğin değeri, sorulduğu bağlama (`gam()`) bağlı, ama anahtarında bu yok.
  - Tek çekirdekte hücreyi hep aynı kişi (dalganın işi) ilk dolduruyor.
  - Paralelde o işi öbür işçi hesapladıysa, hücre bu işçide sonradan, başka bir `gam()` ile doluyor.
- Bu, kodda zaten bilinen "ilk soranın değeri" sorununun bir türü. Hücrenin noktası düzeltilmiş (değer hücrenin merkezinde). Bağlam düzeltilmemiş.

## Ölçüm

Her koşu sonunda yardımcı işçi kapanıyordu:

| koşu | ne oldu |
|---|---|
| tam maç ×3 | 17–18 sn duraklama, sonra paralel kapalı |
| neden yazısı | `süre doldu (yuva 9, dalga 1335)` ya da `kazanç yok: bedel 15340 ms …` (bedelin 15 sn'si bekleme) |
| `aw` | iki işçi hep bir dalga arayla: [652, 653], [1335, 1334] |

Duraklama tek işçide (paralel=1) hiç yok. Kapanma anı koşudan koşuya değişiyor: tik 836 ile 3000 arası.

### İzlerle bulunanlar

1. **Bekleme çapraz:** yardımcı dalga 687'de `DONE ≥ 34` bekliyor. Ana işçi 687'yi kendi iş sayısıyla bitirmiş, 688'de yardımcının gelmesini bekliyor. İkisi 15 sn birbirini bekliyor.
2. **İş paketleri aynı:** 13.861 işin paketi iki işçide birebir aynı (`DKH`). Sorun paketlemede, sırada ya da ortak nesne değişikliğinde değil:
   - Dalga boyunca ortak nesnelerde değişen alan yok (`MUT`).
   - İşleri ters ya da karışık sırada hesaplamak tek çekirdekte bit bit aynı maçı veriyor (`sira.js`, 3000 tik).
3. **Tik tik durum karşılaştırması** (paylaşımlı bellekte her tikin durum özeti):
   - Ayrışma her koşuda **tik 102**'de başlıyor. Cansu'nun kenardan pas seçeneğinin değeri 0,049374485 ile 0,049374481.
   - Farklı olan döküm terimleri `kayip` ve `surme`. İkisi de kaybın bedeli fonksiyonundan geçiyor.
   - Değer koşudan koşuya da değişiyor. Hangi işçinin hangi işi aldığına bağlı.

### Kök: `kayipF` hücresinin değeri `gam()`'a bağlı

`decide.js` `kayipF` (807–809):
- `hucre(X, Y)`, `max(kazanImk, zeminS, pasR)` değerini `c` önbelleğinde X, Y anahtarıyla saklıyor.
- `pasR` sonucu `gam() * b`. `gam()` = `D._gPr ?? D.combo`.
- Bu kaybın bedelini iki yer soruyor:
  - **Dalganın işleri** (bekleme sonrası, sürme sonrası): bu sırada `D._gPr` boş, `gam()` = `D.combo` (0,9).
  - **Dalgadan sonra `prensipDegerle`**: `D._gPr = prGam(h, tac)` (bu maçta 0,96 ya da 0,87).
- Tek çekirdekte hücreyi ilk soran çoğu zaman bir dalga işi, değer 0,9 ile saklanıyor.
- Paralelde o işi öbür işçi hesapladıysa, bu işçide hücreyi ilk `prensipDegerle` soruyor, değer 0,96 ile saklanıyor.

Tek çekirdekte bağlam denetimi (`baglam.js`, hücre dolduğu bağlamdan farklı bağlamda okunursa sayar): 600 tikte 30.000'den çok okuma. Değişen etken yalnız `gam` (`IMK_OFS` ve sahiplik alanı 2000 tikte hiç değişmiyor).

**Sınama:** sitenin kopyasında hücre anahtarına `gam()` eklendi (`?gamk=1`). Sonuçlar:
- Tik 102'deki ayrışma kalkıyor.
- 3 tam maç (9000 tik) ve 5 × 2100 tik: hiç ayrışma yok, hiç duraklama yok.
- Ama 4500 tiklik 2 koşuda yine tik 1988'de bir ayrışma çıktı, iki koşuda da aynı değerlerle. Yine `kayip` ve `surme` terimleri, ama kaynağı `kayipF` değil (bağlam denetiminde yok).

Yani **en az bir ikinci kaynak** var. Daha seyrek: 10 koşunun 2'sinde. Bulunamadı.

## Bu ayrıca bir kafa tutarsızlığı

Paralelden bağımsız olarak, tek çekirdekte de aynı kararın içinde aynı noktanın kayıp bedeli, hücreyi ilk kimin sorduğuna göre 0,9'luk ya da 0,96'lık pas payıyla çıkıyor. Fark küçük (bu maçta 1e-8 ile 1e-4), ama sıradan bağımsızlık kuralını bozuyor.

## Öneriler (motor oturumu için, karar onların)

1. **`kayipF`'in hücresi bağlamdan bağımsız olmalı.** Üç yol:
   - **(a)** `pasR`'nin `gam()`'ını `kayipF` kurulurken bir kez almak (kararı verenin payı). Bağlamdan bağımsız olur.
   - **(b)** Hücrede `b`'yi çarpansız saklayıp okurken `gam()` ile çarpmak. Bu bugünkü "o anki bağlam" anlamını korur.
   - **(c)** Anahtara `gam()`'ı katmak (benim sınadığım). En basiti, ama aynı hücre iki kez kurulur.

   Hangisi doğru anlam, bu motor oturumunun kararı. Üçü de tek çekirdek sonucunu biraz değiştirir.
2. **İkinci kaynak için** (tik 1988): aynı yöntemle bulunabilir.
   - Kaybın bedeli (`VoK`, `kontraKur`) ve `surD`'nin önbelleklerine `baglam.js`'teki gibi bağlam damgası koymak.
   - Daha iyisi: "dalganın içinden doldurulup dışından okunan" bütün önbellekler için değeri yeniden hesaplayıp karşılaştıran bir sınama kipi.
3. **Takılmanın kendisi için paralel.js'e erken uyarı.** Ayrışma olunca 15 sn beklemek yerine hemen tek çekirdeğe geçmek:
   - Yardımcı, `WID` geldiğinde ana işçinin yazdığı `NN`'yi kendi `n`'iyle karşılaştırsın. Farklıysa hemen `boz` etsin. Bu, gördüğüm bütün takılmaları anında bitirirdi.
   - Daha sağlamı: her dalgada iki işçi kararın durum özetini (taşıyıcı ve oyuncuların konumu) yazsın, ana işçi karşılaştırsın.
   - Bu ikincisi önemli: iş sayısı tutup durum ayrışırsa ana işçi öbür işçinin başka bir dünyada hesapladığı sonucu sessizce kullanır, maç tek çekirdektekinden ayrılır. Bugünkü 15 sn bekleme bu ayrışmanın tek belirtisi.

## Notlar

- Başsız Chromium'da tam maç 9000 tik, paralel kapanmadan da kapanarak da ~270 sn sürdü. Bu konteynerde iki işçinin hız kazancı ölçülemiyor; ölçüm yalnız doğruluk için.
- İz yaması: `tools/paralel/iz-yamasi.diff` (sitenin kopyasına, motora değil).
