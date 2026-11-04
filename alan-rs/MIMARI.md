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
  karar      taşıyıcının kararı (bugünkü kafa2: dünyalar, seçenekler, tartı, prensipler)
  yerlesim   topsuz oyuncuların yeri (bugünkü sekil: hücum, savunma, eşleşme, pres, Bekçi)
  ─────────────────────────────────────────────────────────────────────────────────────────
  deger      ortak değerlendirme: tehlike, boşluk, pas yolu, tutma, kesişme, uçuş, şut, ikili,
             yer değeri, kayıp haritası (bugünkü kafa1 + kafa2'nin yerleşimin de kullandıkları)
  ─────────────────────────────────────────────────────────────────────────────────────────
  dunya      tek dünya gösterimi ve varsayımsal görünümler; oyuncu kimliği = sıra (Pid)
  kural      maçın kuralları, saf: enerji/sprint/koşu, tepki, varış tabloları, ivme/kütle/direnç
  ─────────────────────────────────────────────────────────────────────────────────────────
  temel      mat · saha · top · temas · donus · yetenek · rng            (bugün de böyle, sağlam)
```

Karar yerleşimi çağırabilir (Bekçi'nin yeri, `slotBase`, pres öngörüsü: hepsi aşağı doğru); yerleşim kararı
çağırmaz, ihtiyaç duyduğu değerlendirmeler `deger`'dedir. Bugünkü döngünün kaynağı tam bu: yerleşimin
kullandığı `kayipYer`, `yerDeger`, `bekYol` kafa2'de duruyor.

**Rust'a özgü güvence:** katmanlar sonunda ayrı crate'ler olur (`alan-temel`, `alan-kural`, `alan-dunya`,
`alan-deger`, `alan-yerlesim`, `alan-karar`, `alan-mac`). Cargo crate'ler arası döngüye izin vermez: mimari
derleyicinin denetimine girer. O zamana kadar aynı crate içinde modül; kuralı bir betik (`sina/katman.sh`, Aşama 1'de eklenir) denetler.

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

| # | Aşama | Ne | Bitti sayılır |
|---|---|---|---|
| 0 | Güvenlik ağı | Parmak izi (`motor/src/parmak.rs`), `alan-kos … parmak`, `sina/ayni.sh`, başvurular (`sina/ayni/`) | ✅ Başvurular oturum başındaki motorla (b711370) aynı: 4 Kasım'ın bütün hız işleri bit bit davranış korumuş |
| 1 | Kural katmanı | Enerji/sprint/koşu (`mac/enerji`), tepki, varış tabloları (`sekil/gec`), hareket fiziği (ivme, kütle, direnç, omuz) → `kural/`; saf işlevler (oyuncu + ayar girer). `Dis`'in bunlara karşılık gelen yöntemleri doğrudan çağrı olur | `Dis` 66 → ~30 yöntem; `kural` yalnız `temel`'e bağlı |
| 2 | Değerlendirme katmanı | kafa1'in değerlendirme işlevleri + kafa2'den yerleşimin kullandıkları (`kayipYer`, `yerDeger`, `bekYol`) → `deger/` | `sekil` hiçbir şekilde kafa2'ye bağlı değil; `sekil/gecici_dis.rs` yalnız `deger` çağırır |
| 3 | Açık bağlam | `Baglam`, `Ayar`, `MacAni`, `Onbellek`; `kafa_ile` ve `KAFA`/`DIS`/`YAN` kalkar. Önce `deger`, sonra `yerlesim`, sonra `karar` | `kafa_ile` 0; `thread_local` yalnız sayaç/iz için |
| 4 | Tek dünya | `Dunya` + `Gorunum`; kimlik `Pid`; köprü çevirileri kalkar | `ow`, `op_dunya`, `dunya_kopya`, `Varlik` yok; `Arc::ptr_eq` yok |
| 5 | Veri | `Oyuncu` → `OyuncuDurum` + `OyuncuGovde`; `KafaD` → `Ayar` + geçici durum; `JsDeger` ve gereksiz `Option`'lar | `Oyuncu` kopyası ~300 bayt; `model.rs` üretilmiş taslak değil |
| 6 | JS bağı (karar gerekir) | Rust tek doğru kaynak olursa: JS ile bit bit aynılık kısıtı kalkar → Rust adlandırması, `f32` denemesi, sadeleştirme | Kullanıcı kararı |
| 7 | Crate'ler | Katmanlar ayrı crate; döngüyü derleyici yasaklar | `cargo build` katman kuralını zorlar |

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
