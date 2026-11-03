# Kafanın ilkeleri: oyuncular nasıl karar verir

3 Kasım 2026. Kafaya (decide.js) yeni bir kural eklerken ya da bir davranışı düzeltirken uyulacak ilkeler. Kullanıcıyla birlikte yapılan işlerden çıktı; her biri bir hatadan ya da bir itirazdan doğdu.

## Davranış nereden gelir

### 1. Davranış oyuncunun kendi mantığından doğar, kalıptan değil

Oyuncu tipik bir davranış kalıbını uygulamaz; anın gereksinimine göre davranır. Sürme ve pas bu yüzden merkezileştirildi ve oyuncunun kendi tartısına bırakıldı.

- Yanlış: "kanat oyuncusu çizgiye iner", "baskı varsa geri ver".
- Doğru: oyuncu seçenekleri maçın kurallarıyla tartar; davranış tartıdan çıkar.

### 2. Tablo yok

Değerler maçta önceden ölçülüp bir tabloya yazılmaz.

- Tablo motor değişse de aynı yolu izler, eskir.
- Öğretilmiş bir oyun oynatır: oyuncu düşünmez, ezberi uygular.
- Değer o anki dünyadan, maçın kurallarıyla kurulur.

### 3. Rakamlı kural yok

Bir kararı elle konmuş bir sayıya bağlayan kural yazılmaz.

- Yanlış örnekler: "yanılma payının iki katından iyiyse değiştir", "hedefe 8 br kalınca niyeti yenile", "arkadaş en çok 10 br koşar", "sürme 3, 6, 10, 15 ya da 20 br".
- Bir eşik gerekiyorsa sayı değil, karşılaştırma olmalı: oyuncunun kendi ölçüsü (yeteneği, yanılma payı), maçın kuralı (hız, tepki, menzil) ya da iki seçeneğin değeri.
- Bir uzunluk, süre ya da mesafe gerekiyorsa durumdan çıkmalı: rakip nerede karşılıyor, arkadaş nereye yetişebiliyor, top ne zaman varıyor.

### 4. Oyuncuyu serbest bırak: nereye ve nasıl

Pasta "nereye" (alıcının çevresi, koşusunun önü, boşluk) ve "nasıl" (şiddet, yumuşak ya da sert, savurma) oyuncuya bırakılınca paslar çok iyileşti. Aynısı her hamle için geçerli. Serbestlik bu oyunun kurallarının izin verdiği kadardır (ilke 10).

- Sürme sabit uzunluklarla değil, bir yön ve bir amaçla yapılır. Uzunluk durumdan çıkar: önüne biri çıkana, beklenen pas açılana, hedefe varılana kadar.
- Adaylar kaba bir ölçüyle elenmemeli. Kaba ölçü düz bir değerden geliyorsa (orta sahada sahiplik alanı gibi) her zaman en kısa hamleyi seçer.
- Sprint planı hazır olunca sürmenin "nasıl"ına (tempo) eklenecek. Dokunuşla sürme ve sprint 3 Kasım 2026'da kullanıcı kararıyla kapatıldı.

### 5. Niyet taşıyıcınındır

Takıma ortak bir karar dayatılmaz; taşıyıcının kendi karar gücü (agency) korunur.

- Taşıyıcı topu alınca kendi konumundan bir niyet seçer: hangi yoldan, hangi bölgeye.
- Niyet kararlıdır: başka bir niyet belirgin biçimde daha iyi olmadıkça değişmez.
- Niyet nedenlidir: oyuncu bir şeyi yaparken neden yaptığını bilir, bu neden yazılabilir ve kafa panelinde gösterilebilir.
- Pas verilince niyet alıcıya devredilmez. Alıcı kendi niyetini seçer; pasın amacı onun için yalnız bir bilgidir.
- Değerler birbirine yakınken seçimi niyet belirler; bu, oyuncunun "bir öyle bir böyle" davranmasını önler.

### 6. Gol amaçtır, zaman bedeldir

Her hamle zaman harcar. Gole yaklaştırmayan zaman, savunmanın yerleşmesi kadar topa sahip olmanın sona ermesi de demek.

- Dolambaçlı, güvenli ama gole gitmeyen yol (kendi kale çizgisi boyunca sürmek gibi) değer kazanmamalı.
- Topu korumak da zaman harcar; bekleyen de o sürede gole yaklaşmaz.

## Nasıl düşünür

### 7. Kafa = maç

Kafa bir şeyi maçın kuralıyla aynı biçimde hesaplamalı (TASARIM-yapi.md, ilke 4). Kafa ile maç ayrışınca oyuncu maçta olmayacak şeylere göre karar verir.

- Örnek: kafada kısa pas 39 tikte varıyor ve pasçı kendi pasını geri alıyordu. Maçta aynı pas 15 tikte vardı ve 30 oynatmada alıcı 30'unu da temiz aldı.

### 8. "Ne olacak" değil, "ne olabilir"

Kafa savunmanın ne yapacağını tek bir tahminle bilmeye çalışmaz; neler yapabileceğini düşünür (kontradaki imkân haritasının mantığı).

- Savunmacı sayısı sınırlıdır ve her biri aynı anda tek bir iş yapar.
- Soru: "Top oraya giderse savunma ne yaparsa yapsın geriye ne açık kalır?"
- Tek bir tahmin ya fazla kesin olur (kale önünü abartır) ya fazla karamsar (kurmayı sıfırlar). Bütün "savunmanın hareketini görme" denemeleri tek bir tahminde takıldı.

### 9. Bir ölçü bütün seçeneklere uygulanır

Yeni bir ölçü yalnız bir seçeneğe eklenirse o seçenek haksız yere öne geçer.

- Örnek: açık kalan fırsat ölçüsü yalnız sürmenin ucuna eklenince her durumda kısa sürme kazandı. Kafanın "bir yere varınca ne yapabilirim" hesabına (`durumD`) taşınınca sürme, tutma ve pas aynı ölçüyle tartıldı.
- Aynı soruyu iki ayrı hesap tartıyorsa iyimser olan kazanır. "Şimdi pas" ile "sürüp sonra pas", şut ile sürme hep bu yüzden ayrıştı.

### 10. Bu oyun futbolun aynısı değil

ALAN futbola benzer ama futbol değil; kendine özgü kuralları var ve bunlar oyunun kimliği. Kafa gerçek futbolun alışkanlıklarını değil, bu oyunun kurallarını bilir. Sohbetler inatla futbola benzetmeye çalışıyor; yapılmayacak.

- **Gövde yönü yok ve olmayacak.** Koşan oyuncunun dönüş bedeli hareketinden gelir (`gec`), duran oyuncu her yöne bedelsiz döner. "Açık gövdeyle almak", "sırtı dönük alıcı", "omuz üstünden bakmak" gibi futbol kavramları kafaya eklenmez. Kafa maçta olmayan bir bedeli hesaplarsa (ilke 7) oyuncu yanlış karar verir.
- Oyunun kendi parçaları (duvarlar ve kenardan pas, Çekirdek ve şarj, Kuyu, Bekçi'nin kuralları, ikili mücadele) futboldaki bir karşılığa çevrilmez; olduğu gibi tartılır.
- Bir davranışı düzeltirken soru "futbolcu ne yapardı" değil, "bu oyunun kurallarıyla en akıllı hamle ne".

### 11. Oyun kurarken gol para birimi olamaz

Kendi yarısında oyun kuran oyuncu için "buradan gole ne kadar sürede varılır" bilinemez. Golü para birimi yapan her ölçü (sahiplik alanı gibi) orada düzdür; seçenekler arasındaki fark gürültüye kalır.

- Oyuncu golü getirmesi muhtemel ara amaçları güder. Bunlar hamleden sonraki durumun nitelikleridir ve o anki dünyadan ölçülür:
  - **güvenlik:** topun gidip gitmeyeceği;
  - **geçilen rakip:** topun gerisinde kalan rakip sayısındaki artış;
  - **zaman:** topu alana rakibin yetişmesinden önce kalan süre;
  - **açık ileri pas:** savunma ne yaparsa yapsın topu alanın önünde açık kalan, rakip geçen paslar.
- Ara amaçlar gole çevrilerek ya da ağırlıkla toplanarak değil, birlikte değerlendirilir. Tek bir amaç seçtirilmez; akıllı hamle birden çok şeyi birden iyi yapandır.
- **Baskınlık:** Bir seçenek başka bir seçenekten bütün niteliklerde en az onun kadar iyiyse ve birinde daha iyiyse, öteki elenir. Ağırlık, toplama, eşik yok. "Daha iyi" oyuncunun kendi algı payını aşan fark demektir: zamanda kafanın konum belirsizliği, güvenlikte oyuncunun olasılığı yanlış kestirme payı (yanılma zarı). Elenmeyenler arasından niyet ya da kafanın kendi değeri seçer.
- İleri hatta gol hesaplanabildiği için değer zaten belirgindir; şut her zaman serbesttir.

## Nasıl çalışılır

- **Sahne sahne:** Bir pozisyon seç, oyuncu orada ne yapmalıydı ve neden yapmadı, düzeltince ne yapıyor. Aynı pozisyonu farklı kafalarla oynat.
- **Uzun ölçüm yok:** Onlarca maçlık ya da 30 tekrarlı ölçümler yerine sahnede tek oynatma ve adım adım izleme.
- **Bayrak:** Her değişiklik bir `self.ALAN_…_ESKI` bayrağının arkasında. Bayrak açıkken maç bire bir eskisi gibi (3 tohumlu parmak izi).
- **Main:** Kullanıcı onay vermeden main'e itilmez.

## Niyet: açık sorular

Niyet iskeleti `yay2` dalında deneniyor (`ALAN_NIYET_ESKI`). Aşağıdakiler henüz yanıtsız ve rakamlı kuralla değil, ilkelerle yanıtlanmalı.

1. **Niyet ile değer çelişince:** Siper'in niyeti orta saha, ama geriye Burç'a pası belirgin biçimde daha değerli görünüyor ve onu seçiyor. Geriye ya da yana pas bir fırsat değilse niyet neden ezildi? Değer mi yanlış, yoksa niyet değere nasıl katılmalı?
2. **Bölgeye varınca:** Mancınık hedef bölgesinin içinde, niyet ona yön vermiyor, topu tutup bekliyor. Bölgeye varınca niyet bir sonraki basamak olmalı (orta sahadan kanada, kanattan arkaya, ceza alanı önünde şut ya da arkaya pas). Bölgeler basamaktır; niyet zinciri buradan çıkmalı.
3. **Değerler hep eksiyken:** Kendi ceza alanında her yol kayıp riskinden ötürü eksi çıkıyor ve niyet bir kararda bir yöne, öteki kararda "koru"ya titriyor.
4. **Deney kodundaki sayılar:** `niyetSec`, `acikKalan` ve yönlü sürmede bugün elle konmuş sayılar var (bölge sınırları, arkadaşın koşabileceği 10 br, hedefe 8 br, yürüyüş adımı 2 br). İlke 3'e göre durumdan çıkan ölçülerle değiştirilmeli.
