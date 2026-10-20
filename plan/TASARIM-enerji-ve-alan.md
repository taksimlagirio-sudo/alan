# Tasarım planı · Enerji: oyuncu, Çekirdek ve alanın ortak kaynağı

> Durum: fikir aşaması, kod yok. 19 Ekim 2026. Fikir kullanıcının. Kardeş belgeler: `TASARIM-tarz-dongusu.md`, `TASARIM-arka-hat.md`.

## Fikir

Her oyuncunun bir **enerji barı** var. Çekirdeğin şarjı gibi **saniyeler içinde dolup boşalır**. Yorgunluk değildir:
- Maç boyunca biriken düşüş yok.
- Maçlar arası taşınmaz.
- Zorunlu rotasyon yok.

Plandaki "kondisyon olmayacak" kararıyla çelişmez (o madde buna göre güncellendi). Enerji bir **zamanlama** kaynağıdır: ne zaman harcanır, ne zaman toplanır.

**Çekirdeğin şarjı ile oyuncunun enerjisi aynı para birimidir.** Topu tutan oyuncu enerjisini Çekirdeğe aktarır. Güçlü şut ya da pas, oyuncunun barından gelir.

## Harcayan ve dolduranlar

| harcar | doldurur |
|---|---|
| Sprint, sert ivmelenme ve yön değişimi | Yürümek, yerinde durmak |
| Pres atağı, ikili mücadele | Top oyun dışındayken |
| Alanı etkin kullanmak (aşağıda) | |
| Çekirdeğe şarj aktarmak | |

Ayarlanacak iki sayı ölçümle oturur:
- **Boşalma:** tam bir pres atağı ya da uzun bir sprint barı bitirmeli.
- **Dolma:** birkaç saniyelik toparlanma.

Dolma çok hızlıysa sistem bir şey değiştirmez; çok yavaşsa yorgunluğa döner.

## Alanın etkin kullanımı (kullanıcı önerisi)

Bugün alan pasiftir: rakibin topunu yavaşlatır, takım arkadaşının pasını hızlandırır (`match.js` p.R / p.D, `ownFlow`). Öneri: **oyuncu o an hangisi avantajlıysa onu seçer.** Seçim, alanından geçen her Çekirdek için geçerlidir; topu kimin vurduğu fark etmez.

İki durum: **hızlandır** ya da **yavaşlat**. **Pasif etki tamamen kalkar** (kullanıcı kararı): oyuncu alanını kullanmadıkça alan Çekirdeğe hiçbir şey yapmaz.

**Menajere talimat bırakılmaz** (kullanıcı kararı). Alanı ne zaman, hangi yönde kullanacağı tamamen motorun ve oyuncunun işidir: oyuncunun zekâsı (Tepki, Görüş) ve özellikleri (Alan gücü, enerji) belirler. Menajer yalnız sonucunu görür ve kadrosunu buna göre kurar.

Örnek kullanımlar:

| kim | ne yapar | neden |
|---|---|---|
| Hücumcu | Kendi pasını **yavaşlatır** | Alıcı topa yetişsin, top dışarı gitmesin |
| Hücumcu | Kendi pasını **hızlandırır** | Araya giren rakipten önce ulaşsın |
| Savunmacı | Rakibin pasını **yavaşlatır** | Araya girip kessin (bugünkü davranış) |
| Savunmacı | Rakibin pasını **hızlandırır** | Alıcı kaçırsın ya da top taca gitsin |
| Savunmacı | Şutu **yavaşlatır** | Bekçi'ye zaman kazandırsın |
| Taşıyıcı | Kendi sürüşünde kısa **hızlandırma** | Patlama, presten kurtulma (sprintten ayrı, alandan) |

Kurallar:
- **Bedeli enerji.** Etkinin büyüklüğü ve süresiyle orantılı. **Alan gücü**, harcanan enerji başına etkiyi belirler.
- **Karar değerle.** Seçenekler "hızlandır / yavaşlat / bırak". Motor topun yolunu zaten tahmin ediyor (`predict`); her seçeneğin sonucunu tartar, enerji bedelini düşer. Tepki: ne kadar hızlı devreye girer. Görüş: durumu doğru okur.
- **Çekişme.** Bir Çekirdek iki alandan aynı anda geçiyorsa (hücumcu hızlandırıyor, savunmacı yavaşlatıyor) sonuç ikisinin Alan gücü × harcanan enerjisiyle belirlenir. Sahada görünür bir mini düello olur.
- **Sınır.** Hız değişiminin bir üst sınırı ve kısa bir bekleme süresi olur; yoksa top kontrolü kaosa döner.
- **Bekçi.** Kuyu alanındaki Bekçi'nin alanı ayrıdır (Alan hakimiyeti). Şutu yavaşlatma ya da sektirme onun işi olabilir; bugün etkisiz çıkan Alan hakimiyeti'ne iş çıkar.
- **Aynı takımdan birden çok alan üst üste biner** (kullanıcı fikri). Çekirdek aynı anda birkaç takım arkadaşının alanından geçerse etkiler toplanır ve harcanan enerji paylaşılır: her biri daha az harcar, toplam etki daha büyük olur. Sonuç: **kompakt oyunun gücü.**
  - **Savunmada:** sık bir blok, içinden geçen pası birkaç alanla birden yavaşlatır; dar alanda araya girmek kolaylaşır.
  - **Hücumda:** yakın üçgenlerde kısa paslar birkaç alanla hızlanır; kombinasyon oyunu presi kırar.
  - **Bedeli:** kompakt takım sahanın başka yerini boş bırakır. Taraf değiştiren uzun pas, alanların olmadığı boşluktan geçer. Genişlik ve taraf değiştirme anlam kazanır (`TASARIM-tarz-dongusu.md`: kompakt blok ↔ geniş oyun).
  - **Sınır:** üst üste binen etki azalan getiriyle toplanır (ör. ikinci alan yarım, üçüncü çeyrek) ve hız değişiminin üst sınırı geçerlidir; yoksa sık blok aşılmaz bir duvara döner.
  - Rakip alanlar da aynı yerdeyse çekişme her iki tarafın toplamıyla belirlenir: kalabalık bölgede sayıca üstün olan kazanır.
- **Görünürlük.** Alanın rengi seçimi gösterir: hızlandıran bir renk, yavaşlatan başka bir renk, kapalı soluk. Menajer kimin ne yaptığını sahadan okur.

## Çekirdek ile enerjinin bağı

- Şarj, oyuncunun barından Çekirdeğe akar. **Yükleme** bu aktarımın verimidir (bugün etkisiz bir özellik).
- Barı boş oyuncu şarj edemez ya da yavaş eder. Yorgun anda atılan şut zayıftır.
- Paslaşmak şarjı taşır: pasla gelen şarj, verenin harcadığı enerjidir. "Kısa paslarla şarjı topla" ile "tek oyuncuda bekleyip şarj et" farklı tercihler olur.
- Takım arkadaşının barı pas seçiminde bir ölçüt olur: barı dolu olana ver.

### Çekirdeğin tek enerjisi (kullanıcı kararı, 20 Ekim 2026)

Çekirdeğin enerjisi ile oyuncunun enerjisi **aynı şeydir**: oyuncu enerji üretir, Çekirdeğe aktarır, Çekirdek uçarken harcar, kalanı bir sonraki oyuncuya geçer.

Bugün motorda (main 58dd0d2) Çekirdeğin üzerinde iki ayrı sayı var:
- **Şarj** (`m.ch`): oyuncunun barından gelir (Yükleme verimi), vuruş hızını (`chgMul`) ve rakip alanını delmeyi (`pierce × ch`) belirler. Tasarıma uygun.
- **Uçuş enerjisi** (`b.e`): her vuruşta sabit `P.e0` ile başlar (`core.js` `makeBall`). Yolda (`attn`), rakip alanında (`drain`), duvarda ve direkte (`wallLoss`), sekmede (`deflE`) azalır; sıfırlanınca Çekirdek "söner". **Şarjla da oyuncunun barıyla da bağı yok.**

İkisi tek sayı olmalı:
1. **Vuruşta:** Çekirdeğin enerjisi şarjdan gelir. Çok şarjlı pas ya da şut daha uzağa gider, alanlara daha iyi direnir, sekince sönmez; az şarjlı pas kısa sürede söner.
2. **Uçuşta:** enerji bugünkü gibi harcanır (yol, rakip alanı, duvar, sekme).
3. **Alışta:** alıcı topu aldığında **kalan enerji Çekirdekte kalır**, şarj oradan devam eder. Pasla taşınan şarj gerçekten verenin harcadığı enerjidir.
4. **Söndüğünde:** boşta kalan Çekirdeği alan oyuncu sıfırdan (ya da çok azdan) şarj etmeye başlar.

Sonuçları:
- Kısa, temiz, takım arkadaşlarının alanından geçen paslar enerjiyi taşır. Uzun, rakip alanından ve kenardan geçen pas enerjiyi harcar. "Enerjiyi takımda dolaştır" ile "tek oyuncuda bekleyip şarj et" gerçek bir tercih olur.
- Derin bloğun alanları şarjı tüketir; kuşatma sonunda Çekirdek zayıf gelir, hücum yeniden şarj etmek zorunda kalır (Kuyu önü düellosu).
- Ölçek dikkat: bugünkü `e0` ve harcama katsayıları sabit başlangıca göre ayarlı. Şarja bağlanınca az şarjlı pasın hiç ulaşamaması gibi uç durumlar ölçülmeli. Gerekirse şarjsız vuruşa bir taban enerji kalır.
- Görünümde (canlı ekran) Çekirdeğin tek bir halkası olur: şarj ile uçuş enerjisi aynı halka.

## Sprint ve harcama çeşitleri (20 Ekim 2026)

Bugün (main 58dd0d2) sprint açık-kapalı: temel hız ya da 1,25 katı; hangi işte sprint atılacağı sabit bir listeden (`ACIL`). Ölçüm (pres – topa sahip, 90 sn): saha oyuncuları zamanın **%52–59'unda sprintte**. Harcama yolları beş: sprint, ivme, ikili, alan, şarj.

### Sprinti ayrıştırmak
1. **Tempo kademeleri:** yürüyüş, tempo, sprint, patlama.
   - **Patlama:** ilk 1–2 saniyenin sert ivmesi; çok pahalı, kısa. **Çeviklik** ucuzlatır.
   - **Uzun sprint:** en yüksek hızı korumak; süre uzadıkça bedeli artar. **Hız** en yüksek hızı, **Enerji** sürdürmeyi belirler.
   - Hız tek başına üstünlük olmaktan çıkar: "patlayıcı ama çabuk biten" ile "dayanıklı ama ağır kalkan" oyuncu ayrışır.
2. **Sprint değerle seçilir, listeyle değil:** "sprint atarsam yarışı kazanıyor muyum, kazanmazsam ne kaybederim, enerji bedeli ne?" Yarış zaten kaybedilecekse ya da sprintsiz de yetişilecekse atılmaz. Hedef: sprint payı %15–25.
3. **Göreve göre sprint şekli:**

   | görev | şekil |
   |---|---|
   | Pres | Uzaktan sprint, yaklaşınca fren (frenin de bedeli var) |
   | Arkaya koşu | Geç başla, pas anında patla; erken başlayan boşa yanar |
   | Geri koşu | Uzun sprint, en pahalısı: top kaybının gerçek bedeli |
   | Topla sürme | Ayrı bedel; Sürme ucuzlatır |

### Harcama çeşitleri ve verim özellikleri

| harcama | verimi belirleyen |
|---|---|
| Tempo ve sprint | Hız (en yüksek hız), Enerji (süre) |
| İvme, fren, yön değiştirme | Çeviklik |
| Geri geri savunma (jokey) | Çeviklik (normal koşudan pahalı) |
| İkili, gövdeyle koruma | Güç |
| Uzanma ve kayma hamlesi | Uzanma, Kesme |
| Çalım ve fake (sert yön değişimi) | Kontrol, Çeviklik |
| Topla sürme | Sürme |
| Şarj aktarımı | Yükleme (var) |
| Alan kullanımı | Alan gücü (var) |
| Bekçi'nin dalışı ve çıkışı | Refleks, Yerleşim |

**Toparlanma:** yalnız bedene bağlı (aşağıda "Enerjinin dinamiği"). Takımı topa sahipken arkada bekleyen oyuncunun barının dolması bir kural değil, az koşmasının sonucudur.

**Menajer talimatları:** geri koşu "her topta sprint / ekonomik"; arkaya koşu "sık / seçici"; pres "dalga / sürekli". Talimat, enerjinin sprint kararındaki ağırlığını değiştirir.

**Başlangıç:** az ve anlamlı türlerle (tempo kademeleri, fren, uzanma hamlesi); her biri değer birimiyle yazılır, kafadaki maç da bilir.

## İkili mücadele ve enerji (20 Ekim 2026)

Bugün: savunmacı kazanma ihtimali eşiği aşınca girer (`duelP`, `commitThr`), iki hamle var (omuz: Güç farkı; uzanma), sonuç tek zar, girmek bardan sabit 0,08 götürür, bar 0,1'in altındaysa girilmez. Enerji sonucu etkilemiyor.

**İlke (kullanıcı kararı): top kapmak yalnız enerjiye bağlı olmaz.** Sonucu önce **beceri ve durum** belirler; enerji bunlardan biri, sınırlı bir çarpandır.
- **Beceri:** Güç, Uzanma, Kesme (savunmacı); Kontrol, Çeviklik, Sürme (taşıyıcı); Tepki (girme zamanlaması).
- **Durum:** yaklaşma açısı (kale tarafından mı, arkadan mı), taşıyıcının yönü ve hızı, gövdenin Çekirdekle arasında olup olmadığı, momentum, yanında ikinci bir presçi.
- **Enerji:** iki tarafın barı arasındaki fark ihtimali **sınırlı bir payla** kaydırır (ör. en çok ±%15–20). Barı boş ama doğru açıdan, doğru anda giren becerikli savunmacı yine kazanabilir; barı dolu ama yanlış açıdan dalan kaybeder.
- **Şans:** küçük bir pay kalır.

Öneriler:
1. **İki taraf da harcar.** Savunmacı girmek için, taşıyıcı gövdeyle korumak için.
2. **Kısa çekişme:** ikili birkaç tik sürer, her tik iki taraf da harcar; Güç harcamanın verimidir. Çekişmenin sonucu beceri ve duruma göre ağırlıklı ilerler, bar biten taraf o anki dengesini kaybeder (sonucu tek başına belirlemez).
3. **Hamleye göre bedel:**

   | hamle | bedel ve risk | özellik |
   |---|---|---|
   | Omuz | Orta, kaybedince hafif sarsılma | Güç |
   | Uzanıp dürtmek | Az bedel, az etki; bar düşükken de yapılabilir | Uzanma |
   | Kayma | Büyük; kaçırınca uzun süre oyun dışı ve bar boşalır | Kesme, Uzanma |
   | Jokey (oyalamak) | Sürekli küçük bedel, zaman kazandırır | Çeviklik, Tepki |
   | Gövdeyle koruma (taşıyıcı) | Sürekli bedel | Güç |
   | Çalım, patlama (taşıyıcı) | Ani büyük bedel | Kontrol, Çeviklik, Sürme |

4. **Alan ikiliye girer:** temas anında taşıyıcı alanıyla Çekirdeği yanında tutar, savunmacı alanıyla yavaşlatıp koparmaya çalışır (alan düellosu ayakta).
5. **Kaybetmenin bedeli:** kaybeden savunmacı bugünkü sarsılmanın üstüne enerji de kaybeder; kazanan ivmesini korur.
6. **Çift baskı:** ikinci presçi ihtimali yükseltir, ikisi de harcar; ikisinin barı birden boşalırsa arkada boşluk (pres dalgasıyla bağ).
7. **Karar değerle:** girme eşiğine kendi barım, rakibin barı ve kaybedersem açılacak boşluk eklenir; barı az olan çoğu zaman jokeye geçer.
8. **Menajer:** "ikiliye gir / oyala" talimatı enerjiyle gerçek bir seçim olur (çok top kazanıp çabuk tükenmek ya da enerjiyi korumak).

Dikkat: kafadaki maç aynı kuralı bilmeli; ölçülecekler ikili sayısı, kazanma oranının **beceri farkına** ve **enerji farkına** göre ayrı ayrı değişimi (enerjinin payı becerininkini geçmemeli), Güç ve Uzanma'nın etkisi.

## Enerjinin dinamiği (20 Ekim 2026)

### İlke (kullanıcı kararı)

**Enerji yalnız oyuncunun kendi bedenine bakar.** Dolum ve harcama için taktiğe ya da sahadaki duruma bağlı kural yazılmaz ("topa sahipken hızlı dolar", "kompakt blokta dinlenir", "baskı altında yavaş dolar" gibi kurallar yok). Her taktiğin artısı ve eksisi **mekanikten kendiliğinden çıkar.**

Amaç: taktiksel durumları çeşitlendirmek ve "her durumda işe yarayan" hatalı taktiklerin önünü kesmek. Bir taktik bedelsiz çalışıyorsa onu yasaklayan bir kural eklenmez; o taktiğin **hangi harcamayı bedelsiz bıraktığı** bulunur ve o harcama bedellendirilir.

### Beden kuralları

- **Harcatan:** oyuncunun ne yaptığı. Hız, ivme, fren ve dönüş, temas (ikili, gövdeyle koruma), alan kullanımı, şarj.
- **Dolduran:** yapmadığı. Yavaş hareket ya da durmak.
- **İki depo:**
  - **Patlama deposu:** küçük, saniyeler içinde dolar. İlk 1–2 saniyenin sert ivmesi buradan.
  - **Ana bar:** büyük, daha yavaş dolar. Uzun sprint ve sürekli işler buradan.
- **Nefes gecikmesi:** harcamadan hemen sonra dolum başlamaz; kısa bir bekleme.
- **Dipten dönüş yavaş:** bar çok düşükken dolum yavaşlar, üst yarıda hızlı. Barı tamamen boşaltmak pahalıdır.
- **Kısa süreli yük:** son 10–15 saniyede art arda yapılan sprintler bir sonrakini pahalılaştırır; birkaç saniyede silinir, maç boyunca birikmez (yorgunluk değil).

### Özellikler: üç katman

| katman | özellik |
|---|---|
| Kapasite (barın büyüklüğü) | **Dayanıklılık** (yeni) |
| Dolum (hız, nefes gecikmesi, dipten dönüş) | **Toparlanma** (yeni) |
| Harcamanın verimi | Her işin kendi özelliği (yukarıdaki tablo: Çeviklik, Güç, Sürme, Yükleme, Alan gücü…) |

Bugünkü tek "Enerji" özelliği (kapasite ve dolumu birlikte büyütüyor: harcama ÷ S, dolum × S²) ikiye ayrılır. Ortaya çıkan oyuncu tipleri:
- Kapasitesi büyük, toparlanması yavaş: uzun koşar, bir kez boşalırsa uzun süre toparlanamaz.
- Kapasitesi küçük, toparlanması hızlı: kısa patlar, çabuk döner; pres dalgasına uygun.

### Taktiğin artısı ve eksisi buradan çıkar (kural değil, sonuç)

| taktik | mekanikten çıkan |
|---|---|
| Pres | Çok sprint ve fren: bar düşer; pres kırılınca presçiler boştadır |
| Kompakt blok | Kayarken az mesafe: az harcar |
| Geniş oyun, taraf değiştirme | Savunma her çevirmede uzun mesafe kayar: barlar düşer (topa sahip olmanın yıpratması) |
| Topa sahip, yavaş dolaşım | Oyuncular az koşar: barlar dolar |
| Kontra | Kısa sürede uzun sprintler: patlama deposu ve ana bar birlikte boşalır; başarısız kontradan dönüş yavaş |
| Kuyu önü bloğu | Her pasta alan kullanılır: top dolaştıkça barlar erir |

### Bugünkü açık (ölçüm)

Saha oyuncuları zamanın %52–59'unda sprintte (main 58dd0d2): sprintin bedeli dolumla hemen kapanıyor. Enerji 10'da tam sprint dolu barı ~6 sn'de bitiriyor; durarak boş bar ~2,3 sn'de, yürüyerek ~6 sn'de doluyor. Çözüm sprinti yasaklamak değil: nefes gecikmesi, dipten dönüş ve kısa süreli yükle sprintin bedeli doğal olarak büyür.

### Doğrulama

Her tarzın **enerji profili** ölçülür: barın ortalaması, dibe vurma sıklığı, harcamanın türlere göre dağılımı (sprint, ivme, ikili, alan, şarj). Tarzların profilleri ayrışıyorsa ilke çalışıyor demektir. Hiçbir tarzda bir harcama türü bedelsiz kalmamalı.

## Enerjiyi oyunun parçası yapmak: kafa ve menajer (20 Ekim 2026)

Bugün enerji bir ceza: harcanır, oyuncu yavaşlar. Kafa yalnız anlık etkiyi görüyor (yorgun oyuncu şu an yavaş). Sonrasını görmüyor: "şimdi dinlenirse birazdan hızlı olur", "rakip şimdi kayarsa birazdan yorulur", "kontra için şarj lazım". Enerjinin oyunun parçası olması için bunlar kafaya girmeli, ama menajerin işini elinden almadan.

### İlke (kullanıcı kararı): kafa kuralı bilir, stratejiyi menajerden alır

- **Kafa bilir (kural bilgisi):** sprintin, ivmenin, ikilinin, kaymanın, şarjın bedeli; yorgun oyuncunun yavaşladığı; şarjın pası ne kadar uzağa taşıdığı; sabrın ve tempo farkının enerjiye ne yaptığı. Bunları bilmeyen oyuncu saçma görünür (yetişemeyeceği topa sprint, sönecek pas).
- **Menajer seçer (strateji):** rakibi yormaya ne kadar önem verileceği, takımın ne zaman dinleneceği, şarjın kontra için mi biriktirileceği, presin dalga mı sürekli mi olacağı, sabırlı mı direkt mi oynanacağı. Kafa bu ağırlıkları kendisi seçmez, talimattan okur. **Talimatsız takım nötr oynar:** enerjiyi ne özellikle korur ne özellikle kullanır, rakibi yormayı hedeflemez.
- **Oyuncu kusursuz değil:** enerji tahminleri oyuncunun okumasıyla (Görüş) hatalı. Kafa birkaç saniyelik ufku görür; maçın geri kalanını (ikinci yarıya saklamak) menajer düşünür.
- **Maç içi yönetim:** menajer barları ekranda görür, taktiği maç içinde değiştirir (rakip yorulduysa presi artır, kendi takımın yorulduysa tempoyu düşür). Enerji, menajere maç boyunca okunacak ve tepki verilecek bir durum verir.
- **Değer birimi (kullanıcı kararı): mekanikten başla.** Bar farkı → hız farkı (ALAN_HZ, ALAN_SPR) → topa ve pas yoluna varış farkı → değer. Ölçüm gelince katsayılar düzeltilir.

### Sabırlı ile direkt oyun

Sabır ve tempo talimatları zaten var; enerjiyle gerçek bir seçim olurlar:

| | sabırlı, sakin, an kovalayan | tempolu, hızlı, direkt |
|---|---|---|
| Kendi takımı | Az sprint, top dolaşırken barlar dolar | Çok sprint, barlar çabuk düşer |
| Rakip | Her taraf değişiminde kayar, yorulur; ama blok yerleşir | Yerleşemeden yakalanır; az kayar, az yorulur |
| Çekirdek | Tutup dolaştırırken şarj birikir | Şarj az, uzun pas sönebilir |
| Eksisi | Yerleşik blok, Kuyu önü alanları her pasta şarjı eritir | Top kaybında takım yorgun ve dağınık yakalanır |

Kafanın bunu bilmesi gereken yer: sabırlı takım "şimdi atmayayım, rakip biraz daha kaysın, şarj biraz daha dolsun" diye bekleyebilmeli (beklemenin enerji ve şarj getirisi değere girer); direkt takım "rakip yerleşmeden, şarj az da olsa şimdi" diyebilmeli (yerleşmemiş savunmanın değeri, bekleyince kaybolur). Hangisinin ağır basacağı talimattan.

### Kafanın öğreneceği mekanikler

1. **Enerji muhasebesi (temel):** her seçenek için o süre içinde kimin ne kadar harcayacağının tahmini. Kafa seçeneğin sonunda savunmanın nereye kayacağını zaten hesaplıyor (sonDunya, respond); kayma mesafesi × sprint ve ivme bedeli = rakibin harcaması. Kendi takımı için de aynı (koşu mesafesi, sprint gerekip gerekmediği).
2. **Takım arkadaşını dinlendirmek:** sprint gerektirmeyen pas, top dolaştırma, tutma barları doldurur. Değeri: birazdan o oyuncu daha hızlı (koşu, savunmaya dönüş). Bar dolumu → gelecekteki hız → değer.
3. **Rakibe enerji harcatmak:** taraf değiştirme, uzun dolaşım, presi kırmak savunmayı uzun mesafe kaydırır. Rakibin tahmini harcaması → birazdan daha yavaş savunma → değer. "Topa sahip olmanın yıpratması" kural olmadan buradan çıkar.
4. **Kontra için şarj deposu:** top el değişince şarj kapanın olur.
   - Şarjlı topu kesmek hazır bir kontra demek: kesmenin değerine Çekirdekteki şarj girer.
   - Kontra uzun pas ister, uzun pas şarj ister: kafa "bu şarjla kaç br'lik pas sönmeden gider" sorusunu kontra için uzun mesafeyle sorar (bugün tekF tipik 18 br).
   - Ters yönü: şarjı yüksek topu kaybetmenin bedeli büyük; riskli pas topu rakibe dolu verebilir.
5. **Savunma kararlarında bar farkı:** yorgun taşıyıcıya pres, dengesi ve barı düşük oyuncuya ikili daha değerli; kendi barı az olan savunmacı ikiliye girmek yerine oyalar.
6. **Talimatlar ağırlıkları değiştirir:** enerji kullanımı (koru / dengeli / harca), pres (dalga / sürekli), geri koşu (her topta / ekonomik), arkaya koşu (sık / seçici), sabır ve tempo, kontra. Aynı akıl farklı disiplinlerle oynar.
7. **Görünürlük:** kararın amacı ekranda: "dinlendiriyor", "harcatıyor", "şarj biriktiriyor", "an kovalıyor", "yerleşmeden". Maçta kafanın bunu gerçekten düşünüp düşünmediği görülür.

### Sıra

1. Enerji muhasebesi (kafa harcamayı tahmin eder; Görüş'le hatalı)
2. Dinlendirme ve harcatma değerleri, **talimatlarla birlikte** (talimatsız nötr)
3. Sabırlı / direkt: beklemenin ve yerleşmemiş savunmanın değeri
4. Şarj deposu ve kontra
5. Savunma kararlarında bar farkı
6. Ekranda amaç yazıları

Her adım ayrı bayrakla, kafa ve maç aynı kuralla. Doğrulama: aynı takım farklı talimatlarla farklı enerji profili ve farklı sonuç üretmeli; talimatsız takım hiçbir stratejiyi kendiliğinden seçmemeli.

## Ölçümdeki sorunlara etkisi

| sorun (ölçüm) | enerjiyle |
|---|---|
| Yüksek presin bedeli yok | Pres dalga dalga gelir. Bastıktan sonraki birkaç saniye presçinin barı boştur; presi kıran takım boşluğu o anda bulur. |
| Kompakt ve geniş oyun arasında fark yok | Üst üste binen alanlar kompakt oyuna güç verir; taraf değiştiren pas bunun panzehiri olur. |
| Hız baskın (puan başına 2,5 kat) | En yüksek hız kısa patlamalarla. Hız bir patlama gücü olur, sürekli üstünlük değil. |
| Topa sahip olmak kısır | Topu hızla dolaştırmak savunmayı kaydırır, barlarını düşük tutar. O anda taraf değiştirmek koridoru açar (atak içinde yıpratma). |
| Yükleme, Alan hakimiyeti etkisiz | Şarj aktarım verimi; Bekçi'nin alanı. |

## Talimatlar (menajer)

Alan menajere bırakılmaz; **enerji yönetimi bırakılır** (kullanıcı kararı: enerjinin nerede harcanacağı menajerlik işi).

- **Enerji kullanımı:** "hep kullan" / "dengeli" / "kritik anlara sakla" (takım ya da oyuncu).
- **Pres:** "dalga" (bas, toparlan) / "sürekli" (bar düşükken de bas, daha az etkili).

## Özellikler

| özellik | enerjideki işi |
|---|---|
| **Dayanıklılık** (yeni) | Barın büyüklüğü (kapasite) |
| **Toparlanma** (yeni) | Barın dolma hızı, nefes gecikmesi, dipten dönüş |
| Yükleme | Enerjiyi Çekirdeğe aktarma verimi |
| Alan gücü | Harcanan enerji başına alan etkisi |
| Alan hakimiyeti (Bekçi) | Kuyu alanında şutu yavaşlatma ve sektirme |
| Tepki, Görüş | Alanı zamanında ve doğru seçmek |

Hepsi "yükseldikçe kötüleşmez" kuralına uyar.

## Risk: Kuyu önüne dizilen blok geçilmez olmasın

Üst üste binen alanlar en çok, Kuyu önüne dizilmiş sık bir blokta toplanır. Derin blok güçlü olmalı (zayıf takımın şansı), ama geçilmez olmamalı. Panzehirler:

1. **Şarj alanı deler.** Alanın Çekirdeğe etkisi, Çekirdeğin şarjı arttıkça azalır. Hücumcu enerjisini Çekirdeğe yükleyip sık bloğu delmeye çalışır; savunma enerjisini alana harcayıp durdurmaya çalışır. Kuyu önü bir **enerji düellosu** olur: hücumun şarjı ile savunmanın toplam alanı.
2. **Alan kullanmak dolumu durdurur.** Bekleyen ama alanını kullanan savunmacının barı dolmaz. Topu sürekli dolaştıran hücum, savunmayı her pasta alan kullanmaya zorlar. Uzun kuşatma bloğun barlarını eritir; bir an gelir alan yetmez (atak içinde yıpratma).
3. **Alanın olmadığı yoldan oynamak.** Alan yalnız içinden geçen Çekirdeği etkiler. Kenardan sektirme (motorda var), bloğun yanından dolaşan pas ve geniş açıdan şut alanları atlar. Genişlik ve kenar oyunu derin bloğun panzehiri olur.
4. **Azalan getiri ve üst sınır.** Üçüncü, dördüncü alan çok az ekler; hız değişiminin sınırı aşılmaz.
5. **Taraf değiştirme.** Blok kayarken alanlar dağılır; o anda açılan koridordan geçilir (`TASARIM-tarz-dongusu.md`, kayma gecikmesi).
6. **Bedeli kendi hücumu.** Bütün takım Kuyu önündeyse topu kazanınca kimse ileride değildir; derin blok gol yemez ama atamaz da.

Kabul ölçütü: Kuyu önü bloğu, topa sahip ve geniş oynayan takıma karşı belirgin kaybetmeli; direkt oyuna karşı iyi olmalı (tarz döngüsü).

## Motorda dikkat (Claude Code için)

- **Ölçek:** Enerji bedeli karar değeriyle aynı birimde yazılmalı (ev bağındaki ölçek sorunu tekrarlanmasın).
- **Kafadaki maç:** Kendi ve rakibin barını bilmeli; yoksa sprint, pres ve alan kararları yanlış tartılır.
- **Belirlenimcilik:** Enerji maçın durumunun parçası. Kopyaya (`kopya`) ve an kaydına girer.
- **Hız:** Oyuncu başına birkaç sayı; maliyeti küçük. Alan kararı top uçarken alanın içindeki birkaç oyuncu için tartılır.

## Kabul (ölçüm, ileride)

1. Barlar maç içinde dolup boşalır, maç boyunca düşmez.
2. Yüksek pres tarzında pres dalgalar halinde görünür; presi kırılan takımın arkasından şut ve xG artar.
3. Hız'ın puan başına etkisi düşer; öbür özelliklerle arası kapanır.
4. Alan kararlarının dağılımı makul: dört kullanımın hepsi maçta görülür.
5. Etkisiz özellikler (Yükleme, Alan hakimiyeti) ölçülebilir etki kazanır.

## Açık sorular

- Patlama deposunu Çeviklik mi belirlesin, yoksa ayrı bir özellik mi?
- Şarj aktarımı otomatik mi (topu tutan hep aktarır), yoksa bir karar mı ("şimdi şarj et")?
- Üst üste binen alanların azalan getirisi ne olsun (yarım, çeyrek mi)?
- Rakibin barı görünür mü (menajere ve oyuncuya)?
