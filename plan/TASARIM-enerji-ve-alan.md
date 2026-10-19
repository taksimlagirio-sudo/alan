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
| Alanı etkin kullanmak (aşağıda) | (Belki) takım arkadaşının yanında, kendi bloğunun içinde durmak |
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

## Ölçümdeki sorunlara etkisi

| sorun (ölçüm) | enerjiyle |
|---|---|
| Yüksek presin bedeli yok | Pres dalga dalga gelir. Bastıktan sonraki birkaç saniye presçinin barı boştur; presi kıran takım boşluğu o anda bulur. |
| Kompakt ve geniş oyun arasında fark yok | Üst üste binen alanlar kompakt oyuna güç verir; taraf değiştiren pas bunun panzehiri olur. |
| Hız baskın (puan başına 2,5 kat) | En yüksek hız kısa patlamalarla. Hız bir patlama gücü olur, sürekli üstünlük değil. |
| Topa sahip olmak kısır | Topu hızla dolaştırmak savunmayı kaydırır, barlarını düşük tutar. O anda taraf değiştirmek koridoru açar (atak içinde yıpratma). |
| Yükleme, Alan hakimiyeti etkisiz | Şarj aktarım verimi; Bekçi'nin alanı. |

## Talimatlar (menajer)

- **Enerji kullanımı:** "hep kullan" / "dengeli" / "kritik anlara sakla" (takım ya da oyuncu).
- **Pres:** "dalga" (bas, toparlan) / "sürekli" (bar düşükken de bas, daha az etkili).

## Özellikler

| özellik | enerjideki işi |
|---|---|
| **Dayanıklılık** (yeni) | Barın büyüklüğü ve dolma hızı |
| Yükleme | Enerjiyi Çekirdeğe aktarma verimi |
| Alan gücü | Harcanan enerji başına alan etkisi |
| Alan hakimiyeti (Bekçi) | Kuyu alanında şutu yavaşlatma ve sektirme |
| Tepki, Görüş | Alanı zamanında ve doğru seçmek |

Hepsi "yükseldikçe kötüleşmez" kuralına uyar.

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

- Dayanıklılık yeni özellik mi, yoksa Güç'ten mi türesin?
- Şarj aktarımı otomatik mi (topu tutan hep aktarır), yoksa bir karar mı ("şimdi şarj et")?
- Enerji talimatları (enerji kullanımı, pres dalgası) da alan gibi tamamen motora mı bırakılsın?
- Üst üste binen alanların azalan getirisi ne olsun (yarım, çeyrek mi)?
- Rakibin barı görünür mü (menajere ve oyuncuya)?
