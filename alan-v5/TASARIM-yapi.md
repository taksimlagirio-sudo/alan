# Motorun yapısı: bütünlük ve WebAssembly'ye hazırlık

30 Ekim 2026. Paralel kafa, hız ve WebAssembly denemelerinden çıkan ilkeler. Amaç: kurallar oturunca motor tek seferde, bit bit aynı maçla WebAssembly'ye (Rust) taşınabilsin. Bu ilkeler yeni kafa kuralı yazılırken de geçerli; taşıma zamanı geldiğinde iş küçülür.

## Neden şimdi değil

- Kafanın kuralları neredeyse her gün değişiyor. Taşınan bir bölgedeki her değişiklik iki dilde (JS ve Rust) yapılmak zorunda kalır.
- Süre küçük işlevlere dağılmış (profil düz). Küçük çekirdek taşımak maçı hızlandırmıyor. Ölçüm: `KpkSec` ile `exp`/`tanh` wasm'da ~1,5–2× hızlı, maç süresi değişmedi (çekirdeklerin payı ~%1,5 ve ~%3). Dal: `wasm-deney`.
- Kazanç ancak büyük bir sıcak bölge bütünüyle taşınırsa gelir: pasın uçuş hesabı (`ucusFormul`, kapsayıcı ~%34), imkân haritası (`imkan`, ~%17). İkisi de motorun birçok parçasına bağlı.

## İlkeler

### 1. Dünya tek bir veri yapısında

Oyuncular ve top, alan sırası sabit sayı dizilerinde (oyuncu başına bir satır; `core.js pack` buna bir örnek: oyuncu başına 16 sayı).

- Bugün her hayali dünya `{ ...p, x, y }` kopyalarıyla kuruluyor: 50–100 alanlı nesneler. 3000 tikte oyuncu başına 107 alan, 14 oyuncu 14 ayrı sırayla (14 ayrı gizli sınıf). Tik başına ~30 MB bellek ayrılıyor.
- Alan sırasını eşitlemek tek başına işe yaramadı (%7 yavaş: kopyalar büyüdü). Çözüm nesne değil dizi.
- Dizi yapısı hem JS'te hızlandırır hem wasm belleğine doğrudan geçer.

### 2. Gizli durum yok

Bir işlev ihtiyaç duyduğu bağlamı açıkça almalı.

- Bugün kafa `D._…` alanlarından (`D._gPr`, `D._sav*`, `D._kenar`…), modül değişkenlerinden (`SAH_F`, `IMK_OFS`, `DP_K`…), işlevlere asılı yan kanallardan (`keepP.son`) ve önbelleklerden okuyor.
- Örnek hata: kayıp haritasının hücresi ilk sorulduğu andaki `D._gPr`'yi kullanıyordu. Tek çekirdekte ve paralel kafada farklı bağlamla dolunca 1 ve 4 işçinin maçı ayrıldı (`ALAN_KAYIP_BAGLAM_ESKI`).
- Gizli durum dedektörü (dalga işlerini modül önbellekleri boşken ve başka sırayla yeniden hesaplayıp karşılaştıran sınama) işlerin modül önbelleklerine göre saf olduğunu gösterdi. Kararın kendi kapanışındaki önbellekler ve bağlam değişkenleri ise ayrıca sınanmalı.

### 3. Önbellekler sıradan bağımsız

Önbelleğin değeri, anahtarın temsil ettiği sabit noktada ve sabit bağlamda hesaplanmalı. İlk soranın tam noktasındaki değeri saklamak sıraya bağımlılık yaratır.

- Doğru örnekler: `kayipF` hücreleri (ızgara köşesinde, kararın başındaki bağlamla), `RNB` (yarım birimlik noktada), `gecC` (anahtarda hız da var).
- Kaçınılacak: yuvarlanmış anahtarla ilk sorgunun değeri (`durumD cx.P`, `zeminH cx.IM` bugün böyle; karar içinde sıra sabit olduğu için şimdilik zararsız).

### 4. Kafa = maç, tek işlev

Maçın kuralı ile kafanın tahmini aynı işlevi çağırmalı: varış, temas, uzanma, fren, dönüş, ikili. Taşıma da ancak o zaman tek yerde yapılır; kafa ile maç da ayrışmaz.

### 5. Belirlenimli matematik

`sabit-mat.js` yalnız + − × ÷ √ kullanıyor (fdlibm algoritmaları). Rust karşılığı bit bit aynı (`exp`, `tanh`: 2 milyon değerde sınandı).

- Rust'ta FMA kullanılmamalı (varsayılan zaten kullanmaz).
- `x ** 2` V8'de `x * x` ile aynı (5 milyon değerde sınandı).
- `Math.round`, `floor`, `ceil` wasm'da aynı komutlar.

### 6. Sıcak döngüde bayrak ve global

- Deney bayrakları globalde tanımlı olmalı (`core.js`, `ALAN_BAYRAK_TANIM_ESKI`). Tanımsız bayrağın her okunuşu yavaş bir aramaydı (tarayıcıda %6).
- Sıcak döngüde nesne kopyası kurulmamalı. Örnek: `ucusFormul`'da alıcının kopyası her yol noktasında kuruluyordu (`ALAN_PG_TEK_ESKI`).

## Taşıma zamanı geldiğinde

- **Araçlar:**
  - maç parmak izi (her tikte top ve oyuncular, SHA-256),
  - 3 tohumlu doğrulama takımı,
  - gizli durum dedektörü,
  - tarayıcı ölçüm düzeneği (Playwright, Chromium),
  - `wasm-deney` dalındaki Rust iskeleti (`alan-v5/wasm-kaynak/`; wasm baytları `cekirdek-wasm.js`'te, `ALAN_WASM` bayrağı).
- **Sıra:** önce dünya dizisi (ilke 1) JS'te, bit bit aynı maçla. Sonra en sıcak kapalı bölge (`ucusFormul` iç döngüsü), sonra imkân haritası.
- **Her adımda:** parmak izi aynı, bayrakla geri alınabilir.
