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

Dosya eşlemesi (modüller fiziksel olarak yerinde; başka bir oturum aynı kodda çalıştığı için dosya taşımak
Aşama 7'ye kaldı, kural `sina/katman.sh`'ta):

| Katman | Dosyalar |
|---|---|
| kural | `kural/*` |
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
| 3 | Açık bağlam | Kafa iş parçacığına özel değişkenden değil, maçın kutusundan (`m.mac.kafa`) alınır; kafanın durumuna bağlı olmayan değerlendirmeler saf işlev; gizli bağlam (sprint niyeti, kafanın tiki) açık alan | **3a–d ✅**, **3e-1 ✅** `SF_NIYET` → `MacKural::niyet`, **3e-2 ✅** `D_TICK` → `Mc::dtik`, **3e-3 ✅** `KafaD` yalnız değişmez ayar, geçici bağlam `KafaBag`. Kalan 3e-4: bkz. 3.1 |
| 4 | Tek dünya | `Dunya` + `Gorunum`; kimlik `Pid`; köprü çevirileri kalkar | `ow`, `op_dunya`, `dunya_kopya`, `Varlik` yok; `Arc::ptr_eq` yok |
| 5 | Veri | `Oyuncu` → `OyuncuDurum` + `OyuncuGovde`; `KafaD` → `Ayar` + geçici durum; `JsDeger` ve gereksiz `Option`'lar | `Oyuncu` kopyası ~300 bayt; `model.rs` üretilmiş taslak değil |
| 6 | JS bağı | **Karar (5 Kasım): Rust tek doğru kaynak.** JS ile bit bit aynılık kısıtı kalktı; doğrulama `sina/ayni.sh` ve maç istatistikleriyle. Kalan: JS karşılaştırma sınamalarının (`tests/kafa1.rs`, `kafa2_saf.rs` taklitleri, vektör üreticileri) temizliği, Rust adlandırması, sadeleştirme | Karar verildi; temizlik sürüyor |
| 7 | Crate'ler | Katmanlar ayrı crate; döngüyü derleyici yasaklar | `cargo build` katman kuralını zorlar |

### 3.1 Aşama 3e ve 4'ün birlikte planı (5 Kasım)

3e (kafanın içindeki gizli durum) ile 4 (tek dünya) iç içe: kafanın önbelleklerinin çoğu dünyanın gösterimine
(kopya `Op`'lar, işaretçi kimliği) bağlı. Sıra, her adımın bir sonrakini küçültmesine göre:

| Adım | Ne | Neden bu sırada | Bitti sayılır |
|---|---|---|---|
| 3e-1 ✅ | Sprint niyeti `SF_NIYET` → `MacKural::niyet` (kafa tartı süresince dış dünyasının anlık görüntüsüne koyar) | Davranışı etkileyen gizli durum; küçük | `SF_NIYET` yok; yol çalışıyor (900 tikte 93 konur, 37 816 okunur) |
| 3e-2 ✅ | Kafanın tiki `D_TICK` → `Mc::dtik` (pres → `presKesis` → `yolBag` → `hizYanilgi` boyunca parametre) | Davranışı etkileyen gizli durum; küçük | `D_TICK`, `d_tick_ile` yok |
| 3e-3 ✅ | `KafaD`'nin 22 `_` alanı (geçici bağlam: `_tick`, `_gPr`, `_sav*`, `_kenar` …) ayrı yapıya (`Kafa::bag`); `KafaD` yalnız değişmez ayar | Saf değerlendirmeler (`pressAt_d` …) ve yerleşimin `c.d`'si gerçekten değişmez ayarı görsün; "geçici değiştir, geri koy" kalıpları görünür olsun | `KafaD`'de `_` alanı yok (geçici bağlam `KafaBag`, `Kafa::bag`; ayarın hiçbir alanı yazılmıyor: kaydet/geri koy yalnız bağlamı korur) |
| 4a | Kimlik envanteri: 46 `ayni` (`Arc::ptr_eq`) yerinin anlamı (gerçek oyuncu mu, kopyası mı, aynı dünyada mı) | 4b/4c'den önce: paylaşım ve görünümler kimliği bozmasın | Her yer belgeli; `Pid` ile karşılaştırılabilenler çevrilmiş |
| 4b | `Gorunum` (taban dünya + değişen oyuncu konumları): yalnız `x, y, vx, vy` değiştiren 35 kopya yeri | Kopya maliyetinin çoğu; kimlik 4a ile güvende | 35 yerde `Oyuncu` kopyası yok |
| 4c | Tek dünya: yerleşimin `Varlik`'i ile kafanın `Dunya`'sı birleşir; `ow`, `op_dunya`, `dunya_kopya`, `hafif` çevirileri ve tamponları (`OW`, `DUNYA_KOPYA`, `ILK_TAMPON`) kalkar | 4a–b'nin üstüne | Çeviri yok; `Dis`'in yerleşim yöntemleri doğrudan çağrı |
| 3e-4 | Kafanın önbellekleri ömürlerine göre (`Onbellek`: tik ömürlü `ge`, `rnb`, `keepP_son` …; saf kalıcı `ucus`, `suty` …), tik başında açıkça temizlenir | 4c'den sonra: anahtarlar artık işaretçi değil | `Kafa`'da dağınık `RefCell` önbellek yok |

Kalan `thread_local`'lar (13): saf önbellekler (`GEC`, `GTB`, `GECY_K`, `BEK_DER`, `BEK_YOL`: değer yalnız anahtardan,
sıradan bağımsız; iş parçacığı başına kabul edilir), tamponlar (`OW`, `DUNYA_KOPYA`, `ILK_TAMPON`, `SAHTE`, `SABLON`: 4c'de
çoğu kalkar), sayaç/iz ve deney (`*_SAY`, `*_OLC`, `SON_KUME`, `ONDEN_SAY`), paralel işçi durumu (`DAG`, `ICTE`, `TABLO`).
Davranışı etkileyen gizli durum yok.

Her aşamanın sonunda: `sina/ayni.sh` aynı, `cargo test` geçen sayısı düşmemiş, `alan-kos` hızı ölçülmüş
(hız hedef değil, ama yapı değişikliği yavaşlatmamalı), wasm kareleri sitedekiyle aynı.

## 4. Dış kütüphaneler

- **Bevy (ECS motoru):** motora gerek yok. ECS'in özü (veri bileşen dizilerinde, mantık işlevlerde) bu
  tasarımın `Dunya`'sı ile aynı fikir; ama 22 oyuncu + 1 topluk, sırası belirlenimli bir benzetim için
  Bevy'nin zamanlayıcısı (sistemleri kendi seçtiği sırayla paralel çalıştırır) belirlenimliliği zorlaştırır,
  wasm boyutunu çok büyütür. Telefon/PC uygulamasının **ekranı** için aday olabilir: motor kütüphane kalır,
  Bevy onu bağlayıp çizer.
- **Fyrox:** 3B ve görsel editör odaklı; bu iş için gereksiz.
- **wgpu:** ekran kartıyla doğrudan konuşmak için; yalnız uygulamanın çizimi kendimiz yazılırsa. Motorun
  hesabı (dallı, sıralı mantık) ekran kartına uygun değil.
