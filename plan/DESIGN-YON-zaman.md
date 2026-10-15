# ALAN · Yön notu: kurallar zamanı ve rakibin bir sonraki hamlesini görsün

Claude Code'dan Design'a. Kullanıcı onayladı. Yama listesi değil: üç düzeltme, ortak bir teori, her biri için ölçüt ve doğrulama.

## 0 · Neden (kısa)
Ayrıntı: `plan/MEKANIK-HARITASI.md`, `plan/OLCUM-1-donuk-dunya.md`, `plan/OLCUM-3a-pres.md`.

- Ölçümlerde **doğru çalışan** kararların hepsi, sonucu maçın kuralıyla (Çekirdeğin yolu, rakibin tepkisi ve ivmesi) kısa vadede hesaplayan yerler: orta ihtimalli pas, kenardan pas, sürmenin riski, bekleme ortalaması, gönderme.
- **Çalışmayanlar** (kontrada durmak, alıcının topa gelmemesi, yanlış yerde pres, arkada bırakılan boşluk, Bekçi, markaj), değeri ya **donuk bir resimden** (hızlar yok) ya da **sabit kuraldan** alıyor.
- **Ölçüm 1 (64 maç, 5220 karar):** Akıl "sür" ile "fake / tut"u eşit görüyor. Gerçekte hızlı taşıyıcıda sürmek %38 daha çok sayı getiriyor; yavaş taşıyıcıda fark yok. Hız varken değer yanlış ölçülüyor.
- **Ölçüm 3a (kopyalı, 156 pres anı):** Orta sahada ve rakip sahasında basmak doğru (kural %61–84 en iyi). **Kalemizin önünde** basmak 3 sn'de yenen sayıyı 0,14'ten 0,21'e çıkarıyor, top kazanmayı artırmıyor.
  - Önceki notumdaki "orta sahada pres arkada boşluk bırakıyor" teşhisi bu ölçümle **yanlış çıktı** (kayıttan ilişki, kopyalı ölçümde tutmadı).

## 1 · v4 dersleri: ne yapmayacağız
Kullanıcı hatırlattı: v4 (her kararı ileri oynatan tek beyin) dört sebeple kötü oynadı.
1. Gürültü: seçenekler arası fark zar gürültüsünden küçüktü.
2. Ufkun sonu, kendi kötü oyunundan öğrenen tablo.
3. Hız.
4. v3d'nin inceltilmiş topsuz aklını basit rollere çevirmek.

Bu yüzden:
- Maçta kararlar **v3d aklıyla, kurallarla** verilmeye devam eder. Derin oynatma, kâhin, ortalama alma yok.
- Kurallar yalnız **eksik girdilerini** alır: zamanı (hızlar, varış süreleri) ve rakibin **bir adım sonraki** cevabını.
- Kopyalı ölçüm maçta karar vermez. **Çevrimdışı öğretmen**dir: çok sayıda an üzerinden kuralın neyi görmediğini gösterir. Toplu ölçüm gürültüyü yok eder. Değer tablo değil, gerçekleşen sonuçtur (sayı, gönderme, top kimde).
- (Kâhin denemesini — maçta kararı kopyayla seçmek — tek zarla ve 1,5 sn ufukla v4'ün küçük tekrarı olduğu için durdurdum.)

**Ortak ilke:** Bir durumun tehlikesi oyuncuların nerede olduğuna değil, **kimin nereye ne zaman yetişebileceğine** bağlı. Her karar aynı ölçüyü kullanır: varış süreleri (maçtaki ivme kuralıyla) ve rakibin en iyi bir sonraki hamlesi.

Motorda bu parçalar zaten var:
- `MM.ivme`;
- topsuz hücumcunun `teamV`'sinde kullanılan `gec` (dönüş / hızlanma gecikmesi);
- taşıyıcının seçenek değerleri;
- `K.predict`.

## 2 · Düzeltme 1 · Donuk dünya (bayrak örn. `ALAN_DONUK_ESKI`)
**Bugün:**
- Taşıyıcının hamle değeri, hamleden sonraki dünyanın tahminiyle ölçülüyor.
- `predictWorld` hızları sıfırlıyor: geri koşan savunmacı "yerine varmış", taşıyıcının ivmesi kayıp.
- `respond`, `recvVal`, yerleşme cezası da bu resme bakıyor.
- Sonuç: koşarken sürmek ile durmak aynı değerde.

**Düzeltme:**
- Hamle sonrası dünya, hızları korunarak ve maçtaki ivme kuralıyla (`MM.ivme`, oyuncunun hedefine doğru) hamle süresi kadar ileri kaydırılsın.
- Taşıyıcının hızı da korunsun.
- Değeri okuyan ölçüler (boşluk, pas yolu, tehlike), mümkün olan yerde konumdan değil varış süresinden okusun (`freeAt`, `laneOpen` zaten zamanla çalışıyor).
- Ayrı bir "koşu bonusu" yazılmaz.

**Ölçüt:**
- Ölçüm 1 tekrarı: başa baş kararlarda hızlı taşıyıcının "sür"ü ile "fake / tut"u arasındaki gerçekleşen fark (bugün sayı %14,3 / %10,4; gönderme %44 / %33) **kaybolmalı**. Akıl farkı kendisi görmeli; başa baş kararların dağılımı değişir.
- Kontrol tablosu:
  - koşarak alıp koşmaya devam eden ≥ %60 (bugün %53);
  - önü açıkken ilk karar fake / tut ≤ %30 (bugün ~%61).
- Pas tutma, top kaybı kötüleşmesin. Hız aynı makinede ± %10.

## 3 · Düzeltme 2 · Presin bedeli: geçilirsem rakip ne yapar (bayrak örn. `ALAN_PRES_BEDEL_ESKI`)
**Bugün:**
- `prV` = kazanma ihtimali × kazanç − bedel.
- Bedel yalnız presçinin evinin yakınındaki (< 9) ya da tuttuğu rakibin tehlikesi.
- Presçi geçilirse taşıyıcının **ne yapabileceğine** (kalemizin önünde şut) bakmıyor.
- İlk presçi ayrıca her zaman basıyor.

**Düzeltme:**
- **Bedel** = geçilme ihtimali × presçi yerinden çıkmışken (basmaya giderken) taşıyıcının **en iyi seçeneğinin değeri**.
  - Bu değeri taşıyıcının kendi seçenek değerleri verir (`options` ve kaba ön süzme değerleri: pas, `shotQ`, sürme). Presçi taşıyıcıya doğru gitmiş varsayılır. Bir adım; derin oynatma yok.
  - Topsuz hücumcunun `teamV`'si zaten "en yakın iki savunmacı nasıl karşılık verir" diye bir adım bakıyor; bu onun savunma tarafındaki eşi.
- **Geçilme ihtimali:** Presçinin varış süresi (hızıyla, `gec`) ile taşıyıcının bir sonraki karar / pas süresi karşılaştırması.
- İlk presçi de bu tartıyla karar versin. Değer negatifse basmaz, bölgesinde kalır.
- "Son bölgede basma" kuralı yazılmaz: kalemizin önünde şutun değeri yüksek olduğu için ayrım kendiliğinden çıkar.

**Ölçüt:**
- Pres kopyalı ölçümünün tekrarı (`tools/k3-pres.js`, aynı 8 maç): kuralın seçimi ile kopyadaki en iyi kolun **uyum oranı**. Bugün kalemizin önünde %38, genelde %54. Yükselmeli.
- Kalemizin önündeki pres anlarında 3 sn'de yenen sayı 0,21 → ~0,14'e doğru.
- Kontrol tablosu: toplam top kazanma ve yenen sayı kötüleşmesin.

## 4 · Düzeltme 3 · Akılla çalışan alıcı (bayrak örn. `ALAN_ALICI_AKIL_ESKI`)
**Bugün:**
- `alici()` topun yolunu doğru biliyor (`K.predict`).
- Buluşma noktasını şuna göre seçiyor: rakiplerin **durdukları yer** ve tepki süresi (hızları yok), kendi hızı yok, **topu aldıktan sonra ne yapacağı** yok. Tek ölçü "o nokta ne kadar boş".
- zip68'in "yol payı" düzeltmesi maçta görünmedi (son 10 tikte toptan uzaklaşan %27 → %26). zip69 "top gelmeden karar" tek noktada çalışıyor ve içerik değişmedi.

**Düzeltme:** Topun yolu üzerinde 3–5 aday buluşma noktası. Her biri için:
1. **Ben ne zaman varırım:** şu anki hızım ve yönümle, maçtaki ivme kuralıyla (dönüş ve hızlanma dahil).
2. **Rakip ne zaman varır:** her rakibin şu anki hızıyla, aynı kuralla. Bugün eksik olan bu.
3. **Orada alırsam sonra ne yaparım:** taşıyıcının aklı o noktada bir kez (zip69'un önceden kararı, tek nokta yerine adaylar için). Arkadaşların ve rakiplerin durumu buradan gelir; değer Düzeltme 1'deki hızları koruyan dünyayla.

**Seçilen nokta:** rakipten önce varma ihtimali × oradan en iyi hamlenin değeri. Seçilen noktadaki karar alışta uygulanır (zip69).

Kendiliğinden çıkması beklenenler:
- rakip yola yetişiyorsa topa gelmek;
- önü boşsa koşuyu sürdürüp ileride almak;
- bir sonraki hamleye uygun yerde almak.

**Maliyet:** aday başına bir karar hesabı (zip69'un tek hesabının 3–5 katı). Aday sayısı hıza göre ayarlanır; gürültü yok (varış süreleri deterministik, tek hesap).

**Ölçüt:**
- "Kesin" pas kaybı (bugün %6,8) ve bunun yolda kesilme payı (%64) ↓.
- Ayağa pasta son 10 tikte toptan uzaklaşan ↓ (%26).
- Koşarak alıp koşmaya devam eden ↑ (%53).
- Pas kalibrasyonunda P ≥ 0,99 sınıfında akıl / gerçek farkı (%99 / %94) ↓.

**Doğrulama ayrıca:** Kesilen paslarda akıldaki rakip ile maçtaki rakip (Claude Code, "ikiz iz"). Düzeltme sonrası bu fark kapanmıyorsa sebep başka (akıldaki rakip modeli); o zaman ayrı not.

## 5 · Sıra ve çalışma düzeni
1. **Düzeltme 1 önce:** Diğer ikisinin değer ölçüsü buna dayanıyor (alıcı ve pres, hamle sonrası değeri kullanacak).
2. **Düzeltme 2.**
3. **Düzeltme 3.** Düzeltme 1 ölçülüp tuttuktan sonra.
- Her biri ayrı bayrakla. Claude Code her pakette:
  - kontrol tablosu (16 maç, aynı makinede bir önceki sürümden 2 maç);
  - ilgili kopyalı ölçüm / kalibrasyon tekrarı;
  - "ne öğrendik" notu.
- Bir düzeltme ölçütünü tutmazsa üstüne yama yapılmaz. Önce neden tutmadığı ölçülür.
- Biriken bayraklar: ölçümle kalıcılaşanların eski hâlleri bir temizlik turunda silinir (kural ve istisna sayısı azalsın).
