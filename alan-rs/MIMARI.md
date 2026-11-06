# ALAN motoru: hedef mimari ve geçiş planı

4 Kasım 2026. `PLAN.md` taşımanın planıydı; bu belge taşımadan sonraki yapının planı. İlkeler
`alan-v5/TASARIM-yapi.md`'den gelir (tek dünya, gizli durum yok, önbellekler sıradan bağımsız, kafa = maç);
burada Rust'ta nasıl kurulacakları ve oraya davranışı bozmadan nasıl gidileceği yazılı.

## 1. Bugünkü durum (ölçülmüş)

Taşıma modül modül yapıldı (`AJAN.md`): her modül ötekini `gecici_dis.rs` imzalarından çağırdı, birleştirmede
bu köprüler gerçeklerine bağlandı ve kaldı. Sonuç:

| Konu | Bugün |
|---|---|
| Bağımlılık | `mac ⇄ sekil ⇄ kafa` döngüsel: mac→kafa 27, mac→sekil 15, sekil→mac 16, sekil→kafa 8 başvuru; kafa dışarıya `Dis` arayüzüyle (66 yöntem, `MacDis` uygular, 2'si `todo!()`). |
| Bağlam | Kafa iş parçacığına özel değişkende (`KAFA`), 52 yerden `kafa_ile` ile çağrılır. İç içe çağrıda (kafa → dış dünya → yerleşim → kafa) yedek kafalar (`YAN`). 16 `thread_local`, 35 `static`. |
| Dünya | Üç gösterim: maç `Vec<Oyuncu>`, yerleşim `Varlik`, kafa `Dunya = Vec<Arc<Oyuncu>>`. Aralarında çeviri (`ow`, `op_dunya`, `dunya_kopya`), oyuncu kopyalarının çoğu bunlardan. Kafa 46 yerde kimliği işaretçiyle karşılaştırır (`ayni` = `Arc::ptr_eq`). |
| Veri | `model.rs` JS nesne biçiminden üretildi: `Oyuncu` 143 alan (2776 bayt), `Mac` 56, `KafaD` 120 (22'si `_` önekli geçici bağlam). Adlar JS'teki gibi, çoğu alan `Option`. |
| Doğrulama | Sınama vektörleri git'te değil (temiz klonda 5 sınama düşer). Birebir aynılık denetimi yoktu (Aşama 0 ekledi). |
| Süre | Profil düz: tek bir sıcak bölge yok. Paralellik ölçüldü, kazanç küçük (dalgalar ~%10, kayıp ızgarası %5–16). |

## 2. Hedef yapı

### 2.1 Katmanlar

Her katman yalnız altındakileri çağırır. Döngü yok; geri çağrı gerekirse alttaki katman bir trait tanımlar,
üstteki uygular (bugünkü 66 yöntemlik `Dis` yerine birkaç küçük, anlamlı trait).

```
  kabuk      kos · wasm · uygulama bağı                         (girdi/çıktı, kare biçimi)
  ─────────────────────────────────────────────────────────────────────────────────────────
  mac        tik döngüsü: kararı ve yerleşimi çağırır, hareketi ve teması uygular, olaylar
  ─────────────────────────────────────────────────────────────────────────────────────────
  karar      taşıyıcının kararı: seçeneklerin tartısı, prensipler, ileriye bakış
  yerlesim   topsuz oyuncuların yeri: hücum, savunma, eşleşme (position)
  ─────────────────────────────────────────────────────────────────────────────────────────
  deger      ortak değerlendirme: tehlike, boşluk, pas yolu, tutma, kesişme, uçuş, şut, ikili,
             yer değeri, kayıp haritası; ve yerleşimin hizmetleri: Bekçi'nin yeri, slotBase,
             pres öngörüsü (ilkPres), tehlike listesi, alıcının direnci
  ─────────────────────────────────────────────────────────────────────────────────────────
  dunya      tek dünya gösterimi ve varsayımsal görünümler; oyuncu kimliği = sıra (Pid)
  kural      maçın kuralları, saf: enerji/sprint/koşu, tepki, varış tabloları, ivme/kütle/direnç
  ─────────────────────────────────────────────────────────────────────────────────────────
  temel      mat · saha · top · temas · donus · yetenek · rng            (bugün de böyle, sağlam)
```

Yerleşim ile karar birbirini çağırmaz; ikisi de `deger`'i çağırır.

**Aşama 2'nin bulgusu (5 Kasım):** yerleşimin bazı işlevleri (Bekçi'nin yeri, `slotBase`, pres öngörüsü
`ilkPres`, tehlike listesi, `aliciDirenc`, `gside`) ile kafanın değerlendirmeleri **gerçekten karşılıklı**
çağrışır. Kafa rakibin presini maçın kendi pres kuralıyla öngörür ("kafa = maç"); pres kuralı da ikili
değerlendirmesini (`girThr`, `duelP`) kullanır. Bu bir kod kusuru değil, modelin doğası. Bu yüzden bu
hizmetler `deger` katmanındadır; katman içinde karşılıklı çağrı serbesttir, katmanlar arasında yasak.

Dosya eşlemesi (Aşama 7c'den beri her katman ayrı crate, `katman/<ad>/`; kaynak dosyalar `motor/src`'de yerinde,
crate kökü `#[path]` ile bağlar, 3.9. Katmanlar arası kuralı derleyici, katman içi kuralları `sina/katman.sh` zorlar):

| Katman | Dosyalar |
|---|---|
| kural | `kural/*` |
| dunya | `dunya/*` (`Varlik`, `Beden`, `Op`, `Dunya` ve aralarındaki çeviriler) |
| deger | `kafa/kafa1_*`, `kafa/kafa2_{zemin,imkan,dunya,bag}`, `sekil/{yer,pres,takim,ucus,gor,varlik,sabit,gecici_dis}` |
| yerlesim | `sekil/konum.rs`, `sekil/konum/*` |
| karar | `kafa/kafa2_{karar,tarti,eski}` |
| (Aşama 3'te dağılacak) | `kafa/kafa2_baglam` (bütün katmanların önbellek tipleri), `kafa/gecici_dis1` (`Dis` köprüsü) |

**Rust'a özgü güvence:** katmanlar sonunda ayrı crate'ler olur (`alan-temel`, `alan-kural`, `alan-dunya`,
`alan-deger`, `alan-yerlesim`, `alan-karar`, `alan-mac`). Cargo crate'ler arası döngüye izin vermez: mimari
derleyicinin denetimine girer. O zamana kadar aynı crate içinde modül; kuralı `sina/katman.sh` denetler.

### 2.2 Açık bağlam

Gizli küresel durum yerine her hesap bağlamını parametre olarak alır:

```rust
pub struct Baglam<'a> {
    pub dunya: &'a Dunya,           // ya da bir görünümü (2.3)
    pub ayar: &'a Ayar,             // değişmez ayarlar: bugünkü KafaD'nin sabit kısmı, taktik, kural katları
    pub mac: &'a MacAni,            // tikin anlık görüntüsü: şarj, sahip, son pres tartısı, tik
    pub onb: &'a mut Onbellek,      // tik ve karar ömürlü önbellekler (sıradan bağımsız anahtarlarla)
}
```

- `KAFA`, `DIS`, `YAN` ve `kafa_ile` kalkar; iç içe çağrı sıradan bir işlev çağrısı olur, yedek kafa gerekmez.
- Bugünkü `D._gPr`, `IMK_OFS` gibi "geçici değiştir, sonra geri koy" alanları ya `Baglam`'ın bir kopyasında
  değişir ya da açık parametre olur.
- Ödünç alma kuralları (`&` ile `&mut` aynı anda olmaz) gizli durumu zaten yasaklar: derleyici bir kez daha
  mimari bekçisi olur.

### 2.3 Tek dünya ve görünümler

> **6 Kasım notu:** aşağıdaki `Gorunum` tasarımı ölçümle bırakıldı (3.1, 4c-2c): dünyalar çoğunlukla sıfırdan
> kuruluyor, gövdeler zaten paylaşımlı. Kalan: `Dunya` tipi, değer kimliği (`Oyuncu::kim`), paylaşımlı gövde/maç durumu.

```rust
pub struct Dunya {
    pub durum: Vec<OyuncuDurum>,        // sık değişen: konum, hız, hedef, enerji, sprint, iş … (~300 bayt)
    pub govde: Vec<Arc<OyuncuGovde>>,   // nadiren değişen: yetenek, rol, talimat, geçmiş … (paylaşımlı)
    pub top: TopDurum,
}
pub struct Gorunum<'a> {                // varsayımsal dünya: taban + değişen oyuncular
    pub taban: &'a Dunya,
    pub degisen: SmallVec<[(Pid, OyuncuDurum); 4]>,
}
```

- Kopyaların 51 sitesinin 35'i yalnız `x, y, vx, vy` değiştiriyor: bunlar kopya yerine `Gorunum` olur.
- Kimlik işaretçi değil `Pid`: "aynı oyuncu mu" sorusu tek tamsayı karşılaştırması. Bugünkü 46 `ayni`
  yerinin anlamı (gerçek oyuncu mu, kopyası mı) Aşama 4'te tek tek belgelenip korunur.
- `Varlik`, `Op`, `ow`, `op_dunya`, `dunya_kopya` kalkar.

### 2.4 Önbellekler

- Anahtar, değerin hesaplandığı sabit noktayı ve bağlamı tam taşır; değer ilk soranın tam noktasında değil,
  anahtarın temsil ettiği noktada hesaplanır. Böylece sıradan bağımsızdır (bugün çoğu böyle; 4 Kasım'da
  kayıp ızgarası ters sırayla ve önceden hesaplamayla doğrulandı).
- Ömrü belli: tik ömürlü ve karar ömürlü önbellekler `Onbellek`'te ayrı alanlar; tik başında temizlenir.

### 2.5 Paralellik

Ölçümler (4 Kasım) büyük paralel kazanç göstermedi. Yapı paralelliğe engel olmayacak şekilde kurulur
(`Baglam` + sıradan bağımsız önbellekler + değişmez `Dunya` = işler iş parçacıklarına güvenle dağıtılabilir),
ama paralellik bir hedef değil.

## 3. Geçiş: aşamalar

Kural: **her commit davranışı korur** ve `sina/ayni.sh`'tan geçer (3 maç × 900 tik, her tikte maçın
gözlemlenebilir durumu bit bit aynı). Davranış değiştiren bir iş (kural, ayar) yapısal commit'lerle karışmaz,
ayrı commit'te ve başvuruların bilerek yenilenmesiyle (`sina/ayni.sh --yaz`) gelir. Başka bir oturum aynı
kodda çalıştığı için aşamalar küçük parçalarla ve sık `main`'e birleştirilerek ilerler.

**İş bölümü (5 Kasım, kullanıcı kararı).** İki oturum aynı kodda çalışıyor:
- *Mimari oturumu:* bu belgedeki aşamalar (yapı, katmanlar, bağlam, dünya, veri). Yapıya dokunan her iş onun.
- *Davranış oturumu:* yapıya dokunmayan işler — oyun davranışı ve ayarları (bayrak arkasında, ayrı commit, `--yaz`),
  ölçüm ve maç izleme, hata ayıklama (ör. tarayıcıdaki bellek kaçağı), siteye gönderme. Mimarinin sıradaki
  aşamasının taşıyacağı dosyalara (köprüler: `sekil/gecici_dis`, `mac/kafa_dis`, `kafa/gecici_dis1`) hız yaması
  yapmaz; yerleşim davranışı işleri (`sekil/konum`) Aşama 3'le çakışmasın diye önceden haber verilerek yapılır.

| # | Aşama | Ne | Bitti sayılır |
|---|---|---|---|
| 0 | Güvenlik ağı | Parmak izi (`motor/src/parmak.rs`), `alan-kos … parmak`, `sina/ayni.sh`, başvurular (`sina/ayni/`) | ✅ Başvurular oturum başındaki motorla (b711370) aynı: 4 Kasım'ın bütün hız işleri bit bit davranış korumuş |
| 1 | Kural katmanı | Enerji/sprint/koşu, tepki, varış tabloları, ikili ve hareket fiziği → `kural/` (saf işlevler; oyuncu + `MacKural`/`&dyn Kural` girer). Kafa bunlara `Dis` yerine `KuralErisim` ile, maçın anlık görüntüsünden (`Dis::kural`) ulaşır | ✅ `kural/{enerji,gec,kos,ikili,hareket}.rs`; `sekil`'deki yinelenen kurallar teke indi; `Dis` 66 → 27 yöntem; `sina/katman.sh` tutuyor |
| 2 | Değerlendirme katmanı | Yerleşim → kafa iç içe çağrısını kaldırmak; yerleşimin ve değerlendirmenin karar dosyalarına bağını kesmek; katman eşlemesi | ✅ Şut gücü `kural/sut.rs`'e (yedek kafaya düşen çağrı 300 tikte yüz binlerce → 0); `kayipYer`, `riskOf`, `bekYol` `kafa2_zemin`'e; `sina/katman.sh` 4 kural tutuyor. Kalan `Dis` yerleşim yöntemleri (`bekciFm`, `slotBase`, `ilkPres` …) dünya çevirisi gerektirdiği için Aşama 4'te kalkar |
| 3 | Açık bağlam | Kafa iş parçacığına özel değişkenden değil, maçın kutusundan (`m.mac.kafa`) alınır; kafanın durumuna bağlı olmayan değerlendirmeler saf işlev; gizli bağlam (sprint niyeti, kafanın tiki) açık alan | **3a–d ✅**, **3e-1 ✅** `SF_NIYET` → `MacKural::niyet`, **3e-2 ✅** `D_TICK` → `Mc::dtik`, **3e-3 ✅** `KafaD` yalnız değişmez ayar, geçici bağlam `KafaBag`, **3e-4 ✅** önbellekler `Onbellek`'te ömürlerine göre |
| 4 | Tek dünya | `Dunya` + `Gorunum`; kimlik `Pid`; köprü çevirileri kalkar | **✅ kapandı (6 Kasım, ölçümle yeniden kapsamlandı):** kimlik değer (`Oyuncu::kim`), dünya kendi tipi (`Dunya`, temel katmanda), gövdeler paylaşımlı, köprülerdeki çift biçimler kalktı. `Gorunum` (taban + değişenler) yapılmadı: dünyaların çoğu sıfırdan kuruluyor (3.1, 4c-2c) |
| 5 | Veri | **5-1, 5-2 ✅** (3.6: `Oyuncu` 2704 → 288 bayt). `Oyuncu` → `OyuncuDurum` + `OyuncuGovde`; `KafaD` → `Ayar` + geçici durum; `JsDeger` ve gereksiz `Option`'lar | `Oyuncu` kopyası ~300 bayt; `model.rs` üretilmiş taslak değil |
| 6 | JS bağı | **Karar (5 Kasım): Rust tek doğru kaynak.** JS ile bit bit aynılık kısıtı kalktı; doğrulama `sina/ayni.sh` ve maç istatistikleriyle | ✅ Temizlik: JS vektörüne bağlı 12 sınama gerekçeli `#[ignore]` (vektör üretilince `--ignored` ile çalışır; temiz klonda düşen ya da sessizce geçen sınama kalmadı); `tests/ayni.rs` `cargo test`'in kendisinde parmak izi başvurusunu denetler; `tests/kafa1.rs`'in JS taklit artıkları silindi; motor ve sınamalar uyarısız. Rust adlandırması ertelendi (bkz. Geri dönülecekler) |
| 7 | Crate'ler | Katmanlar ayrı crate; döngüyü derleyici yasaklar | **7a, 7b ✅** (3.8), **7c ✅** (3.9): `alan-temel` ← `alan-kural`, `alan-dunya` ← `alan-kafa` ← `alan-sekil` ← `alan-mac`; `alan-motor` hepsini eski yollarla yeniden dışa verir. Bitti: üst katmana bağlanmak derlenmez |

### 3.1 Aşama 3e ve 4'ün birlikte planı (5 Kasım)

3e (kafanın içindeki gizli durum) ile 4 (tek dünya) iç içe: kafanın önbelleklerinin çoğu dünyanın gösterimine
(kopya `Op`'lar, işaretçi kimliği) bağlı. Sıra, her adımın bir sonrakini küçültmesine göre:

| Adım | Ne | Neden bu sırada | Bitti sayılır |
|---|---|---|---|
| 3e-1 ✅ | Sprint niyeti `SF_NIYET` → `MacKural::niyet` (kafa tartı süresince dış dünyasının anlık görüntüsüne koyar) | Davranışı etkileyen gizli durum; küçük | `SF_NIYET` yok; yol çalışıyor (900 tikte 93 konur, 37 816 okunur) |
| 3e-2 ✅ | Kafanın tiki `D_TICK` → `Mc::dtik` (pres → `presKesis` → `yolBag` → `hizYanilgi` boyunca parametre) | Davranışı etkileyen gizli durum; küçük | `D_TICK`, `d_tick_ile` yok |
| 3e-3 ✅ | `KafaD`'nin 22 `_` alanı (geçici bağlam: `_tick`, `_gPr`, `_sav*`, `_kenar` …) ayrı yapıya (`Kafa::bag`); `KafaD` yalnız değişmez ayar | Saf değerlendirmeler (`pressAt_d` …) ve yerleşimin `c.d`'si gerçekten değişmez ayarı görsün; "geçici değiştir, geri koy" kalıpları görünür olsun | `KafaD`'de `_` alanı yok (geçici bağlam `KafaBag`, `Kafa::bag`; ayarın hiçbir alanı yazılmıyor: kaydet/geri koy yalnız bağlamı korur) |
| 4a ✅ | Kimlik envanteri: 46 `ayni` (`Arc::ptr_eq`) yerinin anlamı (gerçek oyuncu mu, kopyası mı, aynı dünyada mı) | 4b/4c'den önce: paylaşım ve görünümler kimliği bozmasın | Her yer belgeli ve sınıflı (3.2), adlar anlamı söylüyor (`ayni_nesne`, `ayni_oyuncu`, `ayni_secenek`). Sıraya (`Pid`) çeviri 4c'de: kafanın dünyasındaki oyuncular (`Op`) bugün sıra taşımıyor |
| 4b ✅ (yeniden kapsamlandı) | Ölçüm (5 Kasım, 60 tik): oyuncu kopyası artık sürenin yalnız %2,4'ü (önceden ~%6; davranış oturumunun hız işleri en sıcak yerleri temizledi), yarısı `ow` çevirisi (4c'nin konusu). 35 yere tek tek görünüm koymak ~%1 getirir: yapılmadı. Yapılan: B sınıfı "değişti mi" soruları nesne kimliğinden bağımsız: `bekKay_d` (Bekçi yer değiştirdiyse `Some`), `respond_d` (değişen oyuncuların açık listesi) | 4c dünyanın gösterimini değiştirince kimliğe dayanan bu sorular kırılmasın | B sınıfının 10 yeri (çalışmayan 7'si dahil) `ayni_nesne` kullanmıyor |
| 4c-1 ✅ | Dünya katmanı: `Varlik`/`Beden`/`hayal`, `Op`/`Dunya` ve dört çeviri (`op_dunya`, `ow_kur`/`ow`/`ow_paylas`, `dunya_kopya`) `dunya/`'da; kimlik kuralı orada belgeli; `sina/katman.sh`'a kural | Çeviriler tek yerde, kimlik kuralı açık | `dunya` üst katmanlara bağlı değil |
| 4c-2 (kapsam kararı) | Kafanın `Op` yerine görünüm (`Varlik`) üzerinde çalışması; çevirilerin kalkması | Ölçüm: çeviriler artık sürenin ~%1,5'i; kafanın ~14 bin satırı `&[Op]` üzerinden alanlara doğrudan erişiyor: çok günlük iş. Ayrıca çeviriler arasında nesne paylaşmak `dolanma`'nın davranışını değiştirir (3.2 C): önce o davranış kararı | Kullanıcı kararı |
| 4c-2a ✅ | Varlık kimliği `Oyuncu::kim` (gerçek oyuncu ve kopyaları: `id`; hayalet: `HAYALET + id`; sahte/boş: yok). `ayni_nesne`'nin 19 yeri `ayni_varlik`'e (aynı nesne ya da aynı kim) | Çevirilerin nesne paylaşabilmesi ve görünümler için kimlik dünyanın gösteriminden bağımsız olmalı | Ölçüm: işaretçi ile kim yalnız `dolanma`'da ayrışıyor; orada anlam birebir korundu (takım + id; sahte oyuncular kim taşımadan gerçek id taşır). `ayni_nesne` yalnız `dolanma`'nın eski yolunda. Main ile 20 maç × 2700 tik aynı |
| 4c-2b ✅ (ölçülüp yeniden kapsamlandı) | Yalnız yeri değişen taşıyıcı kopyaları görünüme: `dunya::Yerde { govde, x, y }` (`Fm`/`BekM` taşıyıcısı; `sutTab`, `bekKay_d` yolu, `imkan`). Adres anahtarları incelendi: hepsi ya nesneyi tutuyor (`PWM`, `GECY_K`, `DUNYA_KOPYA`, `paket`) ya da kapsam boyunca ödünçte (`OW`): adres yeniden kullanılıp karışamaz, değiştirmek bir şey kazandırmaz | Ölçüm (m00, 300 tik, valgrind): oyuncu kopyası sürenin %2'si; en büyük kaynak `ow_kur` (%0,48, tik başına ~500 kopya), sonra `predictWorld`, `pwKos`, `bekKay_d` (~%0,23'er). Çeviride bedeni paylaşmak denendi: varlıkların yalnız ~%11'i bedeninin yerinde (yerleşim gerisini oynatmış), anlık görüntü kurmak kazandırdığından pahalı: geri alındı. Asıl kaldıraç kopyanın boyu: `Oyuncu` **2704 bayt** (Aşama 5) | Main ile 20 maç × 2700 tik aynı; `Yerde` ile kopya komutları −%3,7, süre gürültü içinde |
| 4c-2c-1 ✅ | Köprüdeki çift biçimler: kafanın `GecTab`/`GecY`'si kuralınkinin `Vec`'li kopyasıydı ve her `gecTab`'da çevriliyordu (300 tikte 1,7 milyon ayırma; `gecOf` zaten aynıydı): kafa kuralınkini kullanıyor, `gecTab_k`/`GECY_K` kalktı. `MacGor.tac` paylaşımlı (`Arc<[JMTacE]>`; `sutTab`/`imkan` her çağrıda kopyalıyordu) | Ölçüm (5 Kasım): dünya çevirileri artık profilde görünmüyor; maliyet köprülerin kendi çevirilerinde | Komutlar 34,45 → 32,68 milyar (−%5,1), duvar saati 12,24 → 11,32 ms/tik (**−%7,5**); 20 maç × 2700 tik aynı |
| 4c-2c-2 ✅ | Sıcak döngülerde maçın kural görüntüsü (`Dis::kural`, 128 bayt, sanal çağrı) bir kez alınır (`laneOk`, `imkan`'ın varış kapanışları): 14 milyon çağrının ~yarısı | | Komutlar −%0,5 |
| 4c-2c ✅ kapandı | 1. adım ✅: `Dunya` ayrı tip (temel katmanda `dunya_tip.rs`, `Deref<Target=[Op]>`), kuruluşlar `Dunya::from`/`koy`/`extend` ile. Görünüme (taban + değişenler) geçilmedi | Ölçüm (6 Kasım, 300 tik): tik başına ~268 dünya sıfırdan kuruluyor (bütün oyuncular eşlenerek), ~35 kopyalanıyor, yalnız ~4,6 kez bir oyuncu değiştiriliyor. Değişmeyen oyuncular kuruluşta zaten `Arc` ile paylaşılıyor: görünümün kazancı bugünkü yapıda var. Görünüm, kuruluşların ~268'ini yeniden yazmayı ve kafanın en sıcak döngülerinde her erişime "değişmiş mi" denetimini gerektirirdi; dünya çevirileri gerçek sürenin ~%0'ı | Kullanıcı kararı: kapat. `ow`/`op_dunya` (yerleşim ve maç → kafa) çeviri olarak kalır; gerekirse iç yapı tek dosyadan (`dunya_tip.rs`) değiştirilir |
| 3e-4 ✅ | Kafanın önbellekleri tek yerde, ömürlerine göre: `kafa::onbellek::Onbellek` (`Kafa::ob`). Tik ömürlü (`ge`, `rnb`, `pwm`, `bsk`) `kafa_hazirla`'da `tik_basi` ile birlikte temizlenir (eskiden her biri kendi etiketiyle tembelce); görünüm ömürlü (`imkk`, `it`) etiketlerini kendinde tutar, çünkü `ileriKur` tik içinde tik 0'lık görünüm kurar; kalıcı saf önbellekler (`ucus`, `suty`, `sht_ek`, `imkb`). `keepP_son`/`kesisK_son` önbellek değil, yan çıktı: yerinde | 4c'den sonra: anahtarlar artık işaretçi değil | `Kafa`/`Kafa2Durum`'da dağınık önbellek ve `*T` etiketi yok; 20 maç × 2700 tik aynı |

Kalan `thread_local`'lar (13): saf önbellekler (`GEC`, `GTB`, `GECY_K`, `BEK_DER`, `BEK_YOL`: değer yalnız anahtardan,
sıradan bağımsız; iş parçacığı başına kabul edilir), tamponlar (`OW`, `DUNYA_KOPYA`, `ILK_TAMPON`, `SAHTE`, `SABLON`: 4c'de
çoğu kalkar), sayaç/iz ve deney (`*_SAY`, `*_OLC`, `SON_KUME`, `ONDEN_SAY`), paralel işçi durumu (`DAG`, `ICTE`, `TABLO`).
Davranışı etkileyen gizli durum yok.

### 3.2 Kimlik envanteri (Aşama 4a, 5 Kasım)

Kodda üç ayrı "aynı mı" sorusu var; 4a'dan önce üçü de `ayni` adını taşıyordu, artık adları anlamlarını söylüyor:

| İşlev | Anlam | JS |
|---|---|---|
| `ayni_nesne` (`kafa1_js`; `kafa2_baglam` yeniden dışa açar) | Aynı nesne mi: bir oyuncunun kopyası aslıyla aynı değil; kimlik numarası `NaN` olan hayalet yalnız kendisiyle aynı | `a === b` |
| `ayni_oyuncu` (`kafa1_secenek`) | Aynı takımın aynı numaralı oyuncusu mu: kopyası da sayılır | `a.team === b.team && a.id === b.id` |
| `ayni_secenek` (`mac/kafa_bag`) | Seçenek kafanın şu an yürüttüğü karar mı (`_kafaId`); oyuncu kimliğiyle ilgisi yok | |

**Ölçüm.** Her çağrı yerinde nesne kimliği ile oyuncu kimliğinin ne sıklıkla ayrıştığı sayıldı
(`#[track_caller]` ile geçici ölçüm; 3 maç × 600 tik ve 2 maç × 2700 tik):

| Sınıf | Yerler | Ayrışma | 4b/4c'de |
|---|---|---|---|
| **A. Dünyadaki kendisi mi** (taşıyıcıyı/alıcıyı dünyada bulmak ya da dışlamak) | `kafa1_js::sira`, `kafa1_enerji` 262/275, `kafa1_savur` 282/334, `kafa1_ikili:1206` (kütle: `p === h`), `kafa1_secenek` (5 yer, oyuncu kimliği), `kafa2_tarti` 263/451/456/547/714, `kafa2_zemin:401` | Hiç | Sıra (`Pid`) ile: karşılaştırılan hep dünyanın kendi öğesi |
| **A'. Kendisi mi, ama karşılaştırılan hayalet olabilir** | `kafa1_ikili` 272/1379 | Yalnız nesne eşit: hayaletler (`NaN`) 7–566 bin | Hayaletin sırası yok (`Pid` = yok); oyuncu kimliği kullanılamaz |
| **B. Değişti mi** (cevap dünyasında hangi oyuncu yeni kopya) | `kafa2_dunya:410` (Bekçi'nin yerleşmiş kopyası), `kafa2_tarti` 101/117 (cevap dünyası `Rw` ile `src`); çalışmayan aynı kalıp: `kafa2_karar` 226/231/312, `kafa2_eski` 114/646/852/855 | Yalnız oyuncu eşit: kopyalar, 114–330 bin | Görünümde değişenler açık liste (`Gorunum::degisen`): kimlik sorusu kalkar |
| **C. Kopya dünyada kendi aslını atlamamak** | `kafa1_ikili:235` (`dolanma`: yoldaki gövdelerin etrafından dolanma) | İkisi de: kopyalar 4 milyon, hayaletler 2,8 milyon | **Bulgu**: `p` hayali bir kopyayken dünyadaki asıl gövdesi atlanmıyor, oyuncu kendi gerçek gövdesinin etrafından dolanmaya çalışıyor (JS'ten gelen davranış). Düzeltmek davranış değişikliği: davranış oturumunun kararı. 4b/4c bu yerde nesne kimliğini korur |
| **D. Önbellek/dünya anahtarı** | `kafa2_dunya:518` (taşıyıcının tutma önbelleği: aynı nesne + dünya adresi), `kafa2_tarti` 686/693/695 (`o olmasa` dünyaları), `mac/kafa_dis` `DUNYA_KOPYA` (`Arc::ptr_eq`) | Hiç (tarti) | 4c'de dünya sürümü/kimliği; işaretçi ve adres anahtarı kalkar |
| **Çalışmayan** → silindi | `kafa2_eski` (7 yer), `kafa2_karar` 226/231/312, `kafa2_dunya` 259/518 | Ölçüm (kapsam: 8 maç × 1500 tik) ve kanıt: `holdWorld` hiç çağrılmıyordu ve `_inLook`'u `true` yapan tek yerdi; dolayısıyla `PR = !_inLook` hep `true`, `!PR` arkasındaki eski kafa ölüydü | Silindi (5 Kasım): `kafa2_eski.rs`, `holdWorld` ve hayali maç düzeneği, kontra haritası, zincir, sahte hamle tartısı, `_inLook`; ölü kalan 21 öğe (~1700 satır). Motor uyarısız derleniyor |

Sonuç: 4b/4c'de kimlik **sıra (`Pid`) + kaynak** (gerçek oyuncu / kopya / hayalet) olarak taşınır. A sınıfı sırayla
karşılaştırılır, B sınıfı görünümün değişen listesinden okunur, C sınıfı (tek yer) kopyanın aslından ayrı olduğunu
açıkça bilir, D sınıfı dünya sürümüyle anahtarlanır.

### 3.3 Geri dönülecekler (5 Kasım)

Bilerek ertelenen işler; her biri bir karar ya da uygun bir an bekliyor:

| Konu | Neden ertelendi | Ne zaman |
|---|---|---|
| ~~**`dolanma` davranışı**~~ (3.2 C): hayali bir kopya oyuncu, dünyadaki kendi gerçek gövdesinin etrafından dolanmaya çalışıyordu | **Yapıldı (5 Kasım)**: kopya aslını oyuncu kimliğiyle tanıyıp atlıyor; varsayılan, eski davranış `ALAN_DOLANMA_ESKI` ile (yalnız yerelde). Ölçüm 3.4; başvurular `sina/ayni.sh --yaz` ile yenilendi | — |
| **4c-2: tek dünya** (kafanın `Op` yerine görünüm `Varlik` üzerinde çalışması, çevirilerin kalkması) | Çeviriler artık sürenin ~%1,5'i; kafanın ~14 bin satırı `&[Op]` üzerinden alanlara doğrudan erişiyor (çok günlük iş, davranış oturumuyla sürekli çakışır); çeviriler arasında nesne paylaşmak `dolanma`'yı değiştirir | `dolanma` kararı verildi; kafa kodu durulduğunda |
| ~~**Rust adlandırması**~~ | **Yapıldı (6 Kasım, 3.10)**: 1198 ad Rust biçiminde; kalanlar (serde/JSON adları, tek harfli matematik adları) bilerek bırakıldı | — |
| **Aşama 5: veri** (`Oyuncu` sıcak/soğuk, `JsDeger`, gereksiz `Option`'lar) | Kopya maliyeti %2,4'e indi; `model.rs` her katmanın ortak tipi, bölmesi 4c-2'ye bağlı | 4c-2 ile |
| ~~**3e-4: önbelleklerin ömürleri**~~ | **Yapıldı (6 Kasım)**: `Onbellek` | — |

Her aşamanın sonunda: `sina/ayni.sh` aynı, `cargo test` geçen sayısı düşmemiş, `alan-kos` hızı ölçülmüş
(hız hedef değil, ama yapı değişikliği yavaşlatmamalı), wasm kareleri sitedekiyle aynı.

### 3.4 Ölçüm: mimari değişikliğin ve `dolanma` bayrağının etkisi (5 Kasım)

Taban `1cd1dde` (mimari çalışmadan önceki main), şimdi `fe295eb` (Aşama 0–6 + davranış oturumunun ölçüm
commit'leri). 10 farklı kayıtlı maç kurulumu ve her birinin tohum+1000 eşi: 20 maç × 2700 tik.

**Davranış:** 20 maçın 54 000 tikinin hepsinde parmak izi taban ile bit bit aynı. Mimari değişiklik maçı
değiştirmedi.

**Hız** (tek iş parçacığı, sırayla, dönüşümlü, 2700 tik; ms/tik):

| | Taban | Şimdi |
|---|---|---|
| m00 (3 tekrar) | 13,99 / 13,67 / 13,96 | 14,77 / 14,65 / 14,46 |
| m10 (3 tekrar) | 11,91 / 11,99 / 11,80 | 12,34 / 12,28 / 12,34 |
| 4 maç ortalaması | 13,50 | 14,15 |

Duvar saatinde %3–6 yavaş. Ama işin kendisi azaldı (valgrind, m00 300 tik): komut sayısı 34,66 → 34,20
milyar (−%1,3); önbellek benzetiminde (150 tik) I1 ıska 16,4 → 15,3 M, LL veri ıskası 1,08 → 0,60 M, dal
yanlış tahmini 79,6 → 75,1 M (dolaylı dallar 5,9 → 3,0 M). Benzetim her ölçüde yeniyi iyi gösteriyor; gerçek
işlemcideki fark büyük olasılıkla kod yerleşiminden (hizalama, sıcak döngülerin yeri). Yapıdan gelen bir
iş artışı yok. İzlenecek: Aşama 7'de (crate'lere bölme) yeniden ölçülmeli; gerekirse PGO.

**Kod:** motor 39 231 → 37 898 satır (davranış oturumunun eklemeleriyle birlikte; mimari tarafı ~1700
ölü satır sildi). `Dis` 66 → 27 yöntem; `kafa_ile` 51 çağrı → 0; `thread_local` 21 → 13; uyarılar 0;
katman kuralları (`sina/katman.sh`) 5/5; testler 13 geçen, 0 kalan, 12 JS karşılaştırması `ignore`.

**`ALAN_DOLANMA_KIMLIK`** (bayrağın ilk adı; artık varsayılan, eski davranış `ALAN_DOLANMA_ESKI`) (aynı 20 maç, kapalı / açık; maç başı ortalama, eşleşmiş fark ± standart hata):

| | Kapalı | Açık | Fark/maç | Artan/azalan maç |
|---|---|---|---|---|
| Gol | 1,25 | 1,10 | −0,15 ± 0,29 | 6/9 |
| Pas | 41,35 | 42,60 | +1,25 ± 1,52 | 11/8 |
| Pas isabeti (%) | 84,8 | 84,3 | −0,5 ± 1,8 | 8/12 |
| Şut | 4,10 | 4,10 | 0,00 ± 0,52 | 7/8 |
| Top kaybı | 7,50 | 8,05 | +0,55 ± 0,94 | 12/8 |

Rakibe giden pas %7,9 → %9,5; sahiplik ve enerji (bar ortalaması 0,778 → 0,793) yakın. Hiçbir fark bir
standart hatanın dışına çıkmıyor; bayrak maçı ilk ~40 tikte ayırıyor (kaos), ama maçın istatistiğini
ölçülebilir biçimde değiştirmiyor. Hız farkı da maçların ayrı yollardan gitmesinden; dolanma döngüsünün
maliyeti değişmiyor. Sonuç: düzeltme güvenli ve doğru (kopya kendi gövdesini engel saymamalı); varsayılan
yapıldı (kullanıcı kararı, 5 Kasım); 4c-2'nin önü açık.

### 3.5 Bulgu: `kenardan` pasının `endD`'si ve 12 GB (5 Kasım)

4c-2a ölçülürken bir maç (m05, 2700 tik) belleği tüketti. Kaynak `kafa1_ucus`'ta pasın varış ortası `endD`:
işaretli ağırlıkların (m − s) ortası, s bir yerde m'yi aşınca sahadan binlerce, bir kez 160 milyon br uzağa
düşüyordu (2000 tikte 1–6 kez, çoğu `kenardan`). Bu nokta şut yeri olarak `sutTab` → `bekci` → `bek_derinlik0`'a
gidiyor; kalecinin şut yolu taraması yol boyunu topun hızıyla adımladığı için 160 milyon br'lik yol ~30 milyon
nokta × yol sayısı ayırıyordu. Main'de m05'in bellek tepesi **12 GB** (16 GB'lık makinede sağ kalıyor; tarayıcıda
wasm'ın 4 GB sınırını aşar). Düzeltme (ayrı commit, varsayılan; eski davranış `ALAN_ENDD_ESKI`): `endD` sahadan
uzaksa `endM` (mD ≈ 0'daki gibi). m05'in tepesi 186 MB. 20 maçta yalnız m05 değişti; başvuru maçları aynı.

### 3.6 Aşama 5'in envanteri: `Oyuncu`'nun alanları (5 Kasım)

`Oyuncu` 144 alan, **2704 bayt**; kafanın her kopyası (tik başına yüzlerce) bunun hepsini taşır. Envanter
`sina/alan-envanter.sh` ile (geçici kopyada alanlar yeniden adlandırılır, derleyicinin her hatası bir kullanım
yeri: ortak adlar, ör. `x`, başka yapılarınkiyle karışmaz); çıktı `sina/oyuncu-envanter.txt`. Gözden geçirilen
kör noktalar: `ref mut` desen yok; yapı ifadesi yalnız `kurulum`'da; tek yanlış pozitif `motor.rs:153`
(`== … =>` karşılaştırması).

**Bulgu.** Kafa ve karar (kopyalar üzerinde) yalnız 13 alanı yazar: `x, y, vx, vy, en, den, ca, _don, _dke,
pplan, _sf, sprint, noTouch`. Kafanın kimlik/yetenek alanlarına yazdığı yerlerin hepsi boştan kurulan kısmi
bedenler (`hayal`, `govdeF`, `sahte`, `kafa2_zemin`'in `dinc`'i, `kafa1_sut`'un `hh`'si). Geri kalan 108 alanı
yalnız maç ve yerleşim yazar.

**Önerilen bölme** (alan sayısı; kullanım yeri okuma/yazma):

| Parça | Alanlar | Yer | Kopyada |
|---|---|---|---|
| Anlık durum (yapının kendisinde) | 19: `x, y, vx, vy, tx, ty, en, den, ca, D, cd, sprint, press, noTouch, _don, _dke, _sf, pplan, job` — kafanın değiştirdikleri + her tik yazılan küçükler (`D` enerjiyle her tik ölçeklenir) | 2760 / 530 — **değişmez** (alanlar yerinde kalır) | 243 bayt kopyalanır |
| Gövde (`Arc<OyuncuGovde>`) | 17: `id, team, i, role, slot, name, a, R, yet, rslot, slot0, rslot0, rh, rs, tl, eg, kim` — kurulumda (ve taktik değişiminde) yazılır | 978 / 54 → `p.govde.yet` | paylaşılır |
| Maç durumu (`Arc<OyuncuMac>`, yazarken kopyala) | 108: işler, hamleler, sayaçlar, yerleşimin anıları (`hamle, savur, adandi, lock, tutQ, _kv` …) | 268 / 231 → `p.mac.hamle`, yazma `p.mac_mut().hamle` | paylaşılır |

Kopya ~260 bayt (bugün 2704: ~10 kat küçük). Maç durumuna yazma, nesne o tik kafanın bir anlık görüntüsünde de
duruyorsa bir kez kopyalar (oyuncu başına tik başına en çok bir); bu sayı uygulamada ölçülür. Kısmi bedenler
kendi gövdesini kurar (bugünkü gibi; paylaşılan gövde onlara verilirse boş bıraktıkları alanlar dolu görünür ve
davranış değişir).

**5-1 ✅ maç durumu** (5 Kasım): 108 alan `OyuncuMac`'e, `Oyuncu::hal: Arc<OyuncuMac>`; okuma `p.hal.f`, yazma
`p.hal_mut().f` (`Arc::make_mut`). Yerler derleyicinin hatalarından düzenlendi (~600 yer, 42 dosya; elle yalnız
`sekil/konum.rs`'in geçmiş kaydında iki ödünç sırası). Ölçüm (m00, valgrind 300 tik): komutlar 35,86 → 34,65
milyar (−%3,4), oyuncu kopyası %2,0 → %0,5; yazarken kopyalama tik başına ~44 `OyuncuMac` (önce tik başına ~500
tam kopya). Duvar saati 13,40 → 12,57 ms/tik (**−%6,2**; küçük kopyalar önbelleğe sığıyor). Main ile 20 maç ×
2700 tik aynı. `sina/alan-envanter.sh` üç parçayı da sayar (alanın parçası yazılır; paylaşımlı bir parçaya kafa/karar yazıyorsa `!`: bugün yalnız `role`, kısmi bedenlerin kuruluşu).

**5-2 ✅ gövde** (5 Kasım): 13 alan (`i, role, slot, name, a, yet, rslot, slot0, rslot0, rh, rs, tl, eg`)
`OyuncuGovde`'ye, `Oyuncu::govde: Arc<OyuncuGovde>`; yazma `govde_mut()` (yalnız kurulum, rol atama, kayıt ve kısmi
bedenler). Önerideki `id, team, kim` yapıda kaldı: ~600 sıcak karşılaştırmada (`c.team == o.team && c.id == o.id`)
ek bir işaretçi atlaması olmasın diye (18 bayt); `R` de (her tik yazılıyor). `kafa2_zemin`'in `dinc`'i gövdenin 13
alanının hepsini kopyalıyordu: artık paylaşıyor. **Boyut: `Oyuncu` 2704 → 288 bayt** (`OyuncuMac` 2024, `OyuncuGovde`
416, paylaşımlı). Ölçüm: komutlar 34,65 → 34,44 milyar, oyuncu kopyası %0,51 → %0,18; duvar saati 12,58 → 12,21
ms/tik (−%2,9). 5-1 ile birlikte 13,40 → 12,21 (**−%8,9**). Main ile 20 maç × 2700 tik aynı.

**5-3 ✅ ölü tipler ve alanlar** (5 Kasım): `model.rs` JS'in çalışma anı envanterinden üretilmiş taslaktı (328 tip,
6015 satır). Hiç başvurulmayan 208 tip (çoğu iç içe anlık görüntü tipleri: `JMCtlRBest_pwE…`) yinelemeli olarak
silindi. Alanlar derleyiciyle ölçüldü: bütün alanlar geçici kopyada yeniden adlandırılır, her hata bir kullanım yeri;
bir zincirdeki ilk hata sonrakileri gizlediği için yalnız adaylar yeniden adlandırılarak sabit noktaya dek
yinelenir; motor derlenemeyince `alan-kos`/wasm/sınamalar hiç denetlenmediği için onlarda adı geçen her aday
korunur (ihtiyatlı). Hiç okunmayan ve yazılmayan ya da yalnız yapı ifadesinde kurulan alanlar silindi (kurulduğu
ifadeler de). Sonuç: `model.rs` 6015 → 2379 satır, 328 → 120 tip. Davranış aynı (20 maç × 2700 tik). Yazılan ama
okunmayan 57 alan (yazma deyimleri yan etkili olabilir) ayrı adımda.

**5-4 ✅ yazılan ama okunmayan alanlar:** 23 alan (Secenek'in günlük alanları, Mac'in istatistik ve anıları, JMSt
sayaçları, KafaBag'in üç bağlamı); her yazma yerine elle bakıldı (`Mac.ctl` aslında okunuyor: kaldı), onları besleyen
ölü hesaplar da (iki `sonDunya`, değerlendirmede iki ilerleme hesabı). 39 tip daha ölü.
**5-5 ✅ `JsDeger` kalktı:** tek gerçek kullanımı `KafaBag._sut` (kararın başında taktiğin şut eğilimi, hayali şutta
şutun türü) → `SutBag { Egilim(f64), Tur(String) }`; diğer 7 `JsDeger` alanı hiç okunmuyordu.
**5-6 ✅ gereksiz `Option`'lar (ölçüldü):** `sina/option-olc.rs` (4 maç × 2700 tik, `{:#?}` dökümünden): 272 `Option`
alanından 41'i hep dolu, 60'ı hep boş, 171'i ikisi de. Hep boşlardan yalnız 3'üne hiçbir yerde değer yazılmıyor
(`Secenek.next`, `Secenek.why`, `TopEk.bekM`): silindi, okudukları ölü dallar ve yalnız onlar için hesaplanan
`dis_lookTo` da. Kalan 57'ye yazılıyor (seyrek yollar, bu maçların kullanmadığı taktik ayarları). Hep dolu 41 alan
(6 Kasım'da yeniden ölçüldü: 42) tek tek incelendi; **9'u `Option`'sız** (boşluğu hiçbir okumada gözlenmiyordu):
`Mac._mac_c` (`get_or_insert_with(Default)`), `Mac.tac0`, `Mac.evre` (tembel `vec![None, None]`: boş `Option<Vec>` →
boş `Vec`, okumalar `get`), `OyuncuMac.hist` (boş `Arc<Vec>`), `lock` (`p.lock > t`, t ≥ 0: yok = 0), `rec`, `_yog`
(`unwrap_or(0.0)`), `_evx`, `_evy` (yalnız yazılıyor). Kalan 33 bilerek `Option`: `Tac.*` (8; taktik JSON'u,
`ust` katmanlamasında "verilmedi" anlamlı), `Oyuncu.en` (yok = enerji kuralı kapalı), `Oyuncu.job`, `Mac.E`/`att`/`sh`/
`ra`/`not` (maçın ilk adımından önce yok; okumalar boşluğa göre dallanıyor), `_kuyu_top` (`== Some(takım)`; yok ≠ 0),
`_D0`/`_enOnce` (`unwrap_or(NaN)`/`unwrap_or(1.0)`: düz tipte özel varsayılan isterdi), kafanın karar tipleri
(`Secenek.*`, `Karar.best`, `JODet.P`: JS'in `undefined` karşılaştırmaları) ve olay/not kayıtları. Davranış aynı
(20 maç × 2700 tik).
Sonuç: `model.rs` 6015 → 1950 satır, 328 → 82 tip.

Sıra: (1) maç durumu (108 alan, ~500 yer, kafaya dokunmaz: yalnız `mac/`, `sekil/`, `temas`), (2) gövde
(~1000 okuma, çoğu `p.yet`/`p.team`/`p.id`: düz değiştirme), her biri `sina/ayni.sh` + 20 maçlık parmak iziyle.

### 3.7 Gerçek zamanın profili (5 Kasım)

valgrind komut sayar; sıcak hesaplarda komut sayısı ile süre ayrışıyor. Süre örneklemesi: `perf record -e cpu-clock`
(sanal makinede donanım sayaçları yok; zamanlayıcıyla örnekleme çalışıyor; `apt-get install linux-tools-generic`,
`/usr/lib/linux-tools-*/perf`). m00, 900 tik, payı %1'in üstündekiler:

| İşlev | Süre | | İşlev | Süre |
|---|---|---|---|---|
| `kural::gec::gecOf` | %10,5 | | `top::tahmin` | %2,4 |
| `sekil::yer::bek_derinlik0` | %8,4 | | `mat::atan` | %1,8 |
| `kural::gec::gecYk` | %7,6 | | `ALAN_SPR`, `kosuPr`, `ivme` | %1,7'şer |
| `mat::exp` | %6,6 | | `_int_malloc` | %1,6 |
| `kafa2_imkan::sutTab` (+ kapanışı) | %7,0 | | `savurPlan` | %1,6 |
| `ucusFormul` (+ kapanışı) | %4,5 | | `gecTabV` | %1,5 |
| `imkan`'ın kapanışları | %4,0 | | `floor` | %1,5 |

Gecikme tabloları (`gecOf` + `gecYk` + `gecTabV`) birlikte **~%20**: en büyük hedef.

**Gecikme tabloları ✅** (5 Kasım; davranış aynı, 20 maç × 2700 tik). Ölçüm (m00, 900 tik): `gecOf` 31,5 milyon
çağrı, `gecYk` 617 bin çağrı / 318 bin hesap (100 binlik önbellek 3 kez toptan temizleniyor; sınırsızda bile 264 bin
farklı eğri: azami hız enerjiyle kayıyor, anahtar hep yeni). İki değişiklik:
(1) `gecK`'nın araması: eğrinin başında 11'lik özet (`m[0], m[8], …, m[80]`); önce özette, sonra tek 8'lik blokta.
`m` azalmayan ve NaN'sızsa (`temiz`) ikili aramanın bulduğu sıra tektir (m[lo] < s ≤ m[lo+1]); NaN'lı eğride eski
arama. Eğrinin ~6 önbellek satırı yerine 2: `gecOf`+`gecK` %10,3 → %7,9.
(2) `gecTabV`'nin 9 yönü birlikte: her eğri 80 adımlık bir zincir (her adım öncekinin hızını bekler; süre `sqrt`/`div`
gecikmesinde); 9 bağımsız zincir aynı adımda ilerleyince işlemci üst üste yürütür (`gec_egriler<N>`). Her zincirin
işlemleri ve sırası tek başınakiyle aynı. Eğri hesabı %7,5 → ~%5.
Süre: örneklemede 24 328 → 22 246 örnek (m00 900 tik, −%8,6); duvar saati m00 −%7, m05 −%4, m15 −%1 (eğri hesabının
çok olduğu maçta çok).

**`bek_derinlik0` ✅ (küçük)** (5 Kasım). Gerçek maliyeti iki kez hesaplatarak ölçüldü: ~%8 (m00, 900 tik, iki kez
hesaplayınca +0,8–1,3 ms/tik). Zamanın çoğu `sqrt`'un gecikmesi ve aday × yol döngüsü. İki kesin kısaltma:
(1) `hypot(ax, ay) <= er` → `q = ax·ax + ay·ay`, `er²`'nin 1e-12'lik bandının dışındaysa sonuç karekök alınmadan
belli (`sqrt` doğru yuvarlanır); bandın içinde, NaN'da, sonsuzda, `er ≤ 0`'da eski karşılaştırma.
(2) Aday sayımı, kalan yolların hepsi kapansa bile adayın seçilmesine yetmeyecekse bırakılır.
İşlevin kendi süresi −%14 (örnekleme), toplam ~−%1 (m00 duvar saati −%0,7; gürültüye yakın). Kalan maliyet çağrı
sayısında: tik başına ~400 hesap (kafanın varsayımsal şut yerleri; önbellek %81 isabetli); aşağısı ya kafanın
çağırma biçimine ya da yaklaşıma (davranış) dokunur.

**Denendi, alınmadı: `bek_derinlik0`'ın sayım önbelleği.** Adayların kapattığı yol sayısı Bekçi'nin şimdiki
derinliğine bağlı değil (yalnız eşitlikte seçer): 118 bin hesabın 55,5 binde farklı (top yeri, adım) var. Sayımı
bunlarla önbelleğe almak komutları −%4,2 indirdi (işlevin kendi payı −%43), davranış aynıydı (20 maç). Ama süre:
20 dönüşümlü koşuda 11,20 → 11,18 ms/tik, örneklemede −%1,4 (önbelleğin dağınık girişleri L1 ıskasını %11 artırdı).
Gürültüden ayrılmayan kazanç için ikinci önbellek alınmadı.

### 3.8 Aşama 7a–7b: döngülerin kaldırılması (5 Kasım)

Ölçüm: `kafa` yalnız `kural` ve `dunya`'ya bağlıydı (crate'e hazır). İki döngü vardı: temel → maç (`model.rs` 13,
`temas.rs` 2 başvuru) ve yerleşim ↔ maç (yerleşim → maç 27).

**7a ✅** Maçın veri tipleri (`Tac` ve makrosu, `Durus`, `Kosul`, `MacZ`, `MacW`, `Olay`, `Perde`, `KenarW`, `Diz`,
`DizV`, `Yer`, `Karar`, `FakeShow`, `EnSayac`) temel katmanda `mac_veri.rs`'e; maç eski yerlerinden yeniden dışa
açıyor (`crate::mac::Tac` çalışıyor). `temas` `EN3`'ü doğrudan `kural`'dan alıyor.
**7b ✅** Yerleşimin maçtan aldığı saf işlevler yerlerine: taktik (`tacEff`, `macEff`, `kpAkt`, `mac/tac.rs` →
`kural/tac.rs`), talimat (`tl`, `TLV` → `kural/tac.rs`), `kuyuSafe` → `kural/hareket.rs`, `kafa_tac` →
`kafa2_baglam`, `mac_dokunus` → `gecici_dis1`; `spd`, `ALAN_EN3_DOL` `kural::enerji`'den, `op_dunya` `dunya`'dan, `kafa_d`
yerine `kafa2_baglam::ayar()`. Sahanın ve sayıların yardımcıları (`KX`, `KY`, `dirOf`, `ownX`, `SN` → `saha.rs`;
`to_fixed`, `js_num` → `mat.rs`). Davranış aynı (20 maç × 2700 tik).

**Kök kaldırıldı: `MacEk` `Mac`'ten çıktı ✅.** Temel maç modeli üst katmanların durumunu (kafa kutusu, yerleşimin belleği,
kafanın pres tablosu) taşıyordu. Şimdi maç katmanında `MacTam { m: model::Mac, mac: MacEk }`; maç katmanında `Mac`
bu tiptir (`use crate::mac::MacTam as Mac`), temel alanlara `Deref` ile ulaşır (`m.tick`), `m.mac.kafa` aynen kaldı,
alt katmanlara `&Mac` olarak kendiliğinden geçer. Aynı anda iki temel alanı ödünç alan 35 satır `m.m.ps` biçimine
çevrildi (Deref bütün nesneyi ödünç alır). Yerleşim kafaya artık maçın kutusundan değil, maçın bağlamı kurarken
verdiği elden ulaşır (`Mc::kafa: KafaEl`, `sekil::gecici_dis::kafa_ile`); `laneOpen`, `yolP`, `yerV` ve köprünün 13
işlevi `m` yerine bu eli alır. Düz veri `kosulAkt` temel modele (taktiğin kendisi temelde). Davranış aynı (20 maç ×
2700 tik), hız aynı. `sina/katman.sh`'ın 5. ve 6. kuralı istisnasız; katmanlar arasında döngü kalmadı.

Yan bulgu: kural 2 bugün davranış oturumunun bir deneyinde bozuk (`kafa2_imkan.rs` → `kafa2_karar::decide`,
`ALAN_SUT_GERCEK_IZ`, bayrak arkasında).

### 3.9 Aşama 7c: katmanlar ayrı crate (6 Kasım)

Crate'ler ve bağımlılıkları (her crate'in `Cargo.toml`'u katman kuralının kendisi):

| Crate | Modül | Bağlı olduğu |
|---|---|---|
| `alan-temel` | kökteki dosyalar (`model`, `mat`, `saha`, `temas`, `mac_veri`, `dunya_tip`, `paket`, `oyun`, `paralel`, `parmak` …), `iz_acik`, `HizliMap` | — |
| `alan-kural` | `kural` | temel |
| `alan-dunya` | `dunya` | temel |
| `alan-kafa` | `kafa` | temel, kural, dünya |
| `alan-sekil` | `sekil` | temel, kural, dünya, kafa |
| `alan-mac` | `mac` | hepsi |
| `alan-motor` | yalnız yeniden dışa verir (`alan_motor::mac::…` gibi eski yollar, `kos`, `wasm`, testler değişmedi) | hepsi |

Kaynak dosyalar taşınmadı: `katman/<ad>/lib.rs` katmanın modülünü `#[path = "../../motor/src/<ad>/mod.rs"]` ile
bağlar ve alt katmanları yeniden dışa verir (`pub use alan_temel::*;`), bu yüzden `crate::model::…` gibi yollar
değişmedi. Neden: davranış oturumu aynı dosyalarda çalışıyor; taşıma her birleştirmede çakışma demekti. Dosyalar
ileride `katman/<ad>/`'a taşınabilir, yalnız `#[path]` satırları değişir.

Bölme derleyicinin bulduğu dört gizli bağı açtı (hepsi davranışı değiştirmeden çözüldü):
- `temas` (temel) → `kural::enerji::EN3`: `En3`/`EN3` temele (`core_sabit`), `kural::enerji` yeniden dışa verir.
- `MacZ`'nin (temel tip) yöntemleri kural katmanındaydı: `macW`, `fak`, `MAC_ORAN`, `MAC_UC` ve `impl MacZ`
  `mac_veri`'ye; `kural::tac` yeniden dışa verir.
- `impl Kural for MacKural` maç katmanındaydı ve kafa da kullanıyordu (kafa → maç bağı): uygulama ve
  `ALAN_EN3_KUL` `kural::enerji`'ye. `ALAN_EN3_KUL` kafanın `threat`'ini kullanıyordu (kural → kafa): tehdit
  geometrisi temelde (`saha::tehdit`), sabitleri (`TEHDIT_K`, `TEHDIT_YAN`, `EN_FARK`) `core_sabit`'te; kafanın
  `D.threatK`/`D.sideK`/`DU.enFark`'ı ve maçın `DU::enFark`'ı aynı sabitlerden alır (tek kaynak).
  `MacKural::of(m)` → `mac_kurali(m)` (yabancı tipe yöntem eklenemez).
- `Default for Rng` maç katmanındaydı: `rng.rs`'e.

Doğrulama: `sina/ayni.sh` aynı, main ile 20 maç × 2700 tik parmak izi birebir, testler 14/0/12, wasm derleniyor.
Hız gürültü içinde (m00/m05/m10, 600 tik, üçer koşu: 19,05/15,41/17,15 → 19,42/15,34/17,24 ms/tik); release
derlemesi 60 → 30 sn (crate'ler paralel derleniyor).

### 3.10 Rust adlandırması (6 Kasım)

Derleyicinin `non_snake_case` / `non_upper_case_globals` / `non_camel_case_types` uyarıları (izinler kaldırılınca 1544
farklı ad) önerdiği adlarla, araçla (`sina/adlandir.py`) değiştirildi: **1198 ad** (işlevler, yöntemler, trait
yöntemleri, tipler, sabitler, serde dışı alanlar, değişkenler; ör. `ALAN_SPR` → `alan_spr`, `JP_bkE` → `JpBkE`).
Eşleme `sina/ad-esleme.tsv`'de (eski ad, yeni ad, tür).

Yöntem: eski adın her kod tokeni (yorum ve dizgiler dışında) yeni adla değişir; yeni ad kodda hiçbir yerde geçmiyorsa.
Bu birebir bir yeniden adlandırmadır, anlam değişemez; adın dışarıdan göründüğü yerler hariç tutuldu:
- serde alanları (türetilmiş ya da `tac_yap!` gibi makroyla kurulan tipler) ve girdi JSON'larının anahtarları: maçın
  girdisi ve sitenin okuduğu çıktı aynı kalır (131 ad);
- biçim dizgisinde yakalanan adlar (`"{ad}"`, 5), wasm'ın dışa verdiği işlevler;
- yeni adı zaten var olanlar (195; çoğu tek harfli matematik adı: `T` → `t`, `D` → `d`), farklı öneri alanlar (7),
  aynı yeni ada düşenler (4), aynı tokenin başka türde de kullanıldığı 4 ad (`KOP`, `UA`, `engage`, `oyala`).
Yorumlardaki JS adları (`/// JS macW`) bilerek değişmedi: JS motoruyla karşılık onlarda.

Kalan adlar yüzünden 48 dosyada izin satırı duruyor; uyarısı kalmayan dosyalarda izin kaldırıldı (yeni kodda uyarı verir).

Eski adlarla yazılmış bir dalı güncellemek (davranış oturumunun dalında denendi, çakışmasız derlendi):
1. Dala adlandırmadan önceki main'i birleştir: `git merge 6f76ff4` (7c dahil).
2. Bu commit'in `sina/adlandir.py` ve `sina/ad-esleme.tsv`'sini al (`git checkout <bu commit> -- sina/adlandir.py sina/ad-esleme.tsv`),
   `python3 sina/adlandir.py --hazir sina/ad-esleme.tsv` (yalnız eşlemedeki adları değiştirir), `cargo check`, commit.
3. Yeni main'i birleştir; çakışan dosyalarda dalın kendi hâlini al (`git checkout --ours <dosya>`): main'in o dosyalardaki
   tek değişikliği adlandırma, dalınki zaten adlandırılmış.

Doğrulama: `sina/ayni.sh` aynı, 20 maç × 2700 tik parmak izi birebir, testler 14/0/12, wasm derleniyor. İlk denemede
`tac_yap!` makrosunun serde alanları (`kpSure` …) yakalanmamıştı ve başvurular ayrıştı; araç makroları ve girdi JSON
anahtarlarını da dışarıda bırakacak biçimde düzeltildi.

## 4. Dış kütüphaneler

- **Bevy (ECS motoru):** motora gerek yok. ECS'in özü (veri bileşen dizilerinde, mantık işlevlerde) bu
  tasarımın `Dunya`'sı ile aynı fikir; ama 22 oyuncu + 1 topluk, sırası belirlenimli bir benzetim için
  Bevy'nin zamanlayıcısı (sistemleri kendi seçtiği sırayla paralel çalıştırır) belirlenimliliği zorlaştırır,
  wasm boyutunu çok büyütür. Telefon/PC uygulamasının **ekranı** için aday olabilir: motor kütüphane kalır,
  Bevy onu bağlayıp çizer.
- **Fyrox:** 3B ve görsel editör odaklı; bu iş için gereksiz.
- **wgpu:** ekran kartıyla doğrudan konuşmak için; yalnız uygulamanın çizimi kendimiz yazılırsa. Motorun
  hesabı (dallı, sıralı mantık) ekran kartına uygun değil.
