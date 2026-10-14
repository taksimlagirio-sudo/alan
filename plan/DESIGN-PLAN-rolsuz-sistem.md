# ALAN · Design için plan: rolsüz oyuncu sistemi ve özellik standardı

Hazırlayan: Claude Code (ölçüm ve test tarafı) · 13 Ekim 2026 · Motor: zip32
Bu not tek başına okunabilsin diye yazıldı: bağlam, ölçümlerden öğrendiklerimiz, yeni yön, yapılacak işler, korunacaklar ve nasıl test edileceği.

---

## 0 · Bağlam (kısaca)

- **Çalışma düzeni:** Design motoru ve sayfaları geliştirir, zip gönderir. Claude Code her zip'i ölçer, siteye koyar (GitHub `taksimlagirio-sudo/alan`, `main` = yayındaki site; ölçümler ve araçlar `arsiv` dalında).
- **İlkeler (kullanıcının):** Davranış mekanikten çıkar; yapay kural ve "katsayıyla gizleme" yok. Neyin neden olduğu anlaşılmalı. Kafadaki hesap (akıl) maçtaki kuralın aynısıyla çalışır; kafada ayrı bir taklit yazılmaz.
- **Yeni:** Her zip artık bir **kontrol paketinden** geçiyor (`arsiv` dalı `tools/kontrol/kontrol.sh`). Tarayıcıda açılış, donma, hız, oyun istatistikleri (16 kurulum, önceki sürümle yan yana), taktik etkisi (blok, şiddet, tempo, genişlik), hızlandırmaların ve `yz` bayrağının yerinde olması. Sonuç tek sayfalık rapor (`RAPOR.md`), kullanıcı aracılığıyla sana da gelir.

## 1 · Ölçümlerden öğrendiklerimiz (motor için önemli olanlar)

| Konu | Sonuç |
|---|---|
| Blok | En etkili savunma ayarı. Görevi pres olan oyuncu düşük → yüksek: 0,25 → 0,79; pres başlatma 31 → 87/maç. Yüksek blok daha çok sayı atar ve yer. |
| Pres şiddeti | Çalışıyor (doğrudan ölçü): presçi 0,39 → 0,56, çıkış mesafesi 8,1 → 10,1, ikili 12,7 → 21,7/maç. Ama top kapma değişmiyor (~10,5). Sebebi bölüm 6'da (ikili ölçümü). |
| Basan sayısı ayarı | 1–3 sahada aynıydı; zip32'de kapatıldı (doğru karar). |
| Tempo | Çalışıyor: yüksek tempoda top daha az bekliyor, daha çok pas. |
| Genişlik | Çalışıyor: takım genişliği dar 15,5 · normal 20,9 · geniş 24,3. |
| Fake | Kopyalı ölçüm: fake top kaybını 10 puan azaltıyor; aklın aldanma tahmini doğru (%42 → gerçek %46). Pas fake'inde planlanan pas yalnız %22 atılıyor (sebebi bulunamadı; fake dünyasını ilerletmek denendi, işe yaramadı, uygulanmadı). |
| Topsuz yerleşim | Doğru: oyuncuyu yakındaki "daha açık" noktaya koşturmak takımın sonucunu değiştirmiyor (80 an). Tek ipucu: ileride açık nokta varsa oraya koşmak daha iyi olabilir (9 an, kesin değil). |
| Hız | zip29'a göre ~%20 yavaş; sebep yeni bir ağır hesap değil, daha çok karar (pas 68 → 94/maç). Birebir aynı kalan hızlandırma payı küçük (%3–5). |

## 2 · Yeni yön: rolsüz oyuncu sistemi (kullanıcı kararı)

**Amaç:** Menajer kendi rolünü kendisi kurar, olabildiğince yaratıcı olur; oyun buna uyar. Stoper hücumda kanatta oynayabilir, forvet geride kalabilir. "Role uygunluk" diye bir şey yoktur. Uyum, oyuncunun **özellikleri** ile menajerin **istediği işin** karşılaşmasından doğar: yavaş oyuncuya arkaya koşu istenirse ceza verilmez, sadece geç varır.

**Bekçi bir kuraldır, rol değildir.** Kullanıcı kararı: Bekçi gerçekten Bekçi olarak kalır (alan, Bekçi süresi, özel haklar aynen). Önerim (kullanıcı onayıyla kesinleşsin): Bekçi unvanını menajer istediği oyuncuya verebilir; unvanı taşıyan oyuncu için Bekçi kuralları geçerlidir.

**Bugünkü durum (kod taraması):**
- **Hat rolleri (Bek, Orta, Kanat, Uç) zaten yalnız etiket.** Motorda bunlara göre dallanan kod yok. Yer, dizilişten geliyor ve savunma ile hücum dizilişi zaten ayrı (`sav`, `huc`).
- **Davranışı rol paketleri belirliyor:** hücumda `p.rh` (Kurucu, Koşucu, Kanat, Pivot, Serbest), savunmada `p.rs` (Süpürücü, Markajcı, Presçi, Önde, Bölge). Kullanıldığı yerler (zip32):
  - `match.js` 55: rol ataması (`p.rh`, `p.rs`, `p.rslot`)
  - `match.js` 56: Markajcıya otomatik adam seçimi (Pivot/Koşucu öncelikli)
  - `match.js` 62–74: role uyum notları (Kanat kenarda mı, Kurucu geride mi …)
  - `shape.js` 100: Süpürücü ve Önde seçimi (arkada kalanlar)
  - `shape.js` 103: pres sırası, Presçi önce
  - `shape.js` 104: **Presçi'nin pres değeri ×1,3** (gizli avantaj)
  - `shape.js` 132, 134, 150: Markajcının adamı ve takip kuralları
  - `shape.js` 178: `roleLeash` (role göre yerinden ne kadar uzaklaşabilir)
  - `shape.js` 179: `roleY` (Pivot boşluğu, Kanat genişliği, Kurucu gerisi)
  - `shape.js` 180: `roleRun` (Koşucuya ek koşu isteği)
  - `shape.js` 192, 198, 199: role göre aday nokta üretimi
  - `shape.js` 212: görev etiketi

## 3 · Senden istenen (Aşama 1: şimdi başlanabilir)

> **Yapı notu (önemli):** Talimatları baştan **evreye bağlı** kur: `evre → talimat`. Aşama 1'de yalnız iki evre kullanılır (bugünkü savunma ve hücum), ama veri yapısı Aşama 3'teki evrelere hazır olsun; sonradan evre eklemek yeniden yazma gerektirmesin. Değeri verilmeyen her talimat bir üst seviyeden **miras** alsın (takım → evre → oyuncu). Bkz. bölüm 4b.

### 3.1 Rol paketlerini oyuncu başına talimatlara ayır
Her oyuncu için hücum ve savunma ayrı. Önerilen talimatlar (adlar ve kademeler senin):

**Hücumda**
- Yer: hücum dizilişinde serbest nokta (bugünkü `huc` slotu)
- Koşu isteği: ileri / arkaya koşu ne sıklıkla (bugün `roleRun`, Koşucu aday noktaları)
- Destek mesafesi: taşıyıcıya yakın mı, uzak mı (bugün Kurucu `roleY`)
- Kenar / iç tercihi: kenarda kal ↔ içe kat (bugün Kanat `roleY`, Kanat aday noktaları)
- Derinlik / hat önü: rakip savunmanın önünde boşluk tut (bugün Pivot boşluğu)
- Serbestlik: yerinden ne kadar uzaklaşabilir (bugün `roleLeash`)
- Pas ↔ sürme ve risk eğilimi (bugün `p.eg`, varsa)

**Savunmada**
- Yer: savunma dizilişinde serbest nokta (bugünkü `sav` slotu)
- Adam ↔ alan (bugün Markajcı ↔ Bölge)
- Kimi tutar: en tehlikeli / belirli bir rakip (bugün `match.js` 56 otomatik)
- Kişisel pres isteği (takımın şiddet ayarının üstüne; bugün Presçi)
- Geride kal / arkayı koru (bugün Süpürücü)
- Önde bekle / kontraya hazır (bugün Önde)

**Hazır paketler silinmesin:** Kurucu, Koşucu, Süpürücü, Markajcı … bu talimatların **hazır ayarları** olarak kalsın. Menajer bir paket seçip sonra istediği talimatı değiştirebilsin. Paketler bugünkü davranışı yaklaşık aynen üretmeli (kontrol paketi bunu ölçer: paketlerle oynanan maçlar bugünkü istatistiklere yakın çıkmalı).

### 3.2 Gizli rol avantajlarını kaldır
Davranışı **talimat** belirlesin, başarıyı **özellik**. Etikete bağlı çarpan ya da öncelik kalmasın. Örnek: `shape.js` 104'teki Presçi ×1,3 → "kişisel pres isteği" talimatının değeri olsun; aynı talimatı alan herkes aynı davransın.

### 3.3 Uyum notları
`match.js` 62–74'teki "role uyuyor mu" kontrolleri rol yerine talimata göre yazılsın (ör. "kenarda kal" talimatı alan oyuncu kenarda mı).

### 3.4 Kabul ölçütleri
- Motorda hat etiketine (Bek, Orta, Kanat, Uç) ya da paket adına bağlı davranış kalmasın; davranış yalnız talimat + özellik + Bekçi kuralından gelsin.
- Aynı talimatı alan iki oyuncu, özellikleri aynıysa aynı davransın (etiketleri farklı olsa da).
- Her oyuncu her yere konabilsin (Bekçi kuralı dışında sınır yok).
- Hazır paketlerle oynanan maçlar bugüne yakın sonuç versin.

## 4 · Aşama 2: özellik standardı (Claude Code'un ölçümünü bekle)

**Kullanıcının gözlemi:** Özellik az; aynı kaynaktan beslenen çok yetenek var. Kod taraması bunu doğruluyor (zip32, motorda kullanıldığı yer sayısı):

| Özellik | Yer | Not |
|---|---|---|
| **Okuma** | **~70** | tepki süresi, algı hatası, yer tutma, fake'i fark etme, yanılmadan dönme, kafada oynatma (otOk), kovalama, alıcı tepkisi, Bekçi yerleşimi, pres tepkisi … |
| Hız | 30 | fiziksel, makul |
| Kesme | 9 | |
| Sürme | 6 | |
| Çeviklik | 5 | |
| Aktarım | 4 | |
| Yoğunluk | 4 | |
| Tutuş | 3 | |
| **Cesaret** | **1** | yalnız karardaki risk ağırlığı |

**Standart (önerilen üç şart):**
1. Her özelliğin tek ve net bir mekanik ailesi olsun.
2. Bu anlam herkes için aynı işlesin; oyuncu nerede oynarsa oynasın.
3. Özellik yükseldikçe sonuç hiçbir zaman kötüleşmesin.

**Yön (ölçüm sonrası kesinleşecek):** Okuma'yı ayırmak (ör. Tepki · Görüş/pas seçeneği görmek · Yer tutma · Karar/baskı altında doğru seçim); Cesaret'e gerçek bir anlam vermek (ör. ikiliye girme isteği, riskli pas/gönderme) ya da kaldırmak; Bekçiye özel özellikler gerekip gerekmediği (refleks/uzanma, alan hakimiyeti).

**Özellik ölçümü (ertelendi, kullanıcı kararı):** Şu an taktik derinlik aşamasındayız. Özellik ölçümü yeni özellik listesiyle (ayrılmış Okuma vb.) daha anlamlı olacak; liste yeniden tanımlanınca yapılacak: her özellik tek tek ±4; genel etkisi, tek tek mekaniklere etkisi ve taktiğe göre değeri (çeşitlilik için: hız kontrada, okuma topa sahip oyunda daha değerli mi).

**Ekrana etkisi:** Menajere gösterilecek "uyum" bilgisi kurala değil ölçüme dayansın: "Bu oyuncu, istediğin bu işte şu kadar etkili." (Ölçüm tablosundan türetilebilir.)

## 4b · Aşama 3: oyun evreleri (isteğe bağlı derinlik)

**Amaç (kullanıcı):** Detayla uğraşmayı seven menajer dizilişe ve savunma/hücum şekillerine boyut katabilsin; en ince ayrıntısına kadar. Herkesin yapması gerekmez: isteyen ayarlar, istemeyen bugünkü sadelikte oynar.

**Evreler (önerilen; adlar ve sınırlar senin):**

| | Evreler |
|---|---|
| Savunma | Rakip oyun kuruyor (top onların arka bölgesinde) · Rakip orta sahada · Rakip son bölgemizde · **Top kaybı sonrası geçiş** (ilk birkaç saniye: hemen geri bas ya da yerleş) |
| Hücum | Oyun kurma (arkadan) · İlerleme (orta saha) · Yerleşik / set hücum (son bölge) · **Top kazanınca geçiş** (kontra) |

Gerekirse ince ayrımlar: rakibin hangi kanatta olduğu, set hücumunda savunma geçildiyse (ör. çizginin arkasına inildiyse) farklı davranış.

**Her evrede ayarlanabilenler:** diziliş şekli, blok yüksekliği, pres şiddeti, tempo, genişlik, sabır/risk ve **her oyuncunun talimatları** (bölüm 3.1'deki talimatların evre başına değeri).

**Miras (anahtar):**
- Basit mod: tek takım ayarı; bütün evreler onu kullanır (bugünkü gibi).
- Gelişmiş mod: menajer istediği evrede istediği ayarı değiştirir; değiştirmediği her şey üst seviyeden gelir: **takım ayarı → evre ayarı → oyuncunun o evredeki talimatı**.
- Yapay zekâ takımları hazır evre ayarlarıyla oynar (tarz şablonları evre değerleri olarak).

**Bugün motorda olan parçalar (evre sistemine toplanacak):** ayrı savunma ve hücum dizilişi (`sav`, `huc`), top kazanınca davranış (`kazaninca`: Dengeli / Kontra / Yerleş; `kontraN`), top kaybı sonrası pres süresi (`kpSure`, `kayip`), pres tetikleri (`tetik`: geri pas, kanatta), blok hattı, tempo, sabır, risk.

**Şartlar:**
1. **Evre mekanik ve görünür olsun.** Hangi evrede olunduğu net ölçülerle belirlenir (topun yeri, top el değiştirdikten sonra geçen süre). Sınırlar tanımdır, gizli katsayı değildir; menajere gösterilir (canlı maçta "şu an: rakip oyun kuruyor").
2. **Geçiş yumuşak olsun.** Şekil değişince oyuncular yeni yerlerine koşar, ışınlanmaz; geçiş anındaki açıklar gerçek ve oyunun parçası.
3. **Akıl da evreyi bilsin.** Kafada oynatma (imagine, predictWorld) savunmanın o evredeki şeklini maçtaki kuralla kullansın (kafada ayrı taklit yok).

**Test (Claude Code, kontrol paketine eklenecek):** her evre ayarı sahada fark yaratıyor mu; değiştirilmeyen ayar mirası doğru alıyor mu; evre geçişlerinde donma ya da ışınlanma yok mu; basit modda sonuçlar bugüne yakın mı.

## 5 · Korunacaklar (her zip'te)

- **Hızlandırmalar** (sonucu bit bit koruyan): `hız:` ile başlayan yorumlu satırlar. zip32'de `core.js` 4, `decide.js` 8, `match.js` 4, `shape.js` 2 yer. Dosyaları yeniden yazarken korunmalı; kontrol paketi sayar.
- **`tac.yz` bayrağı:** Skora göre taktiği kendiliğinden değiştiren kısım yalnız yapay zekâ takımında (`shape.js` `macPres` koşulu, Canlı'da rakibe `yz: true`). Kontrol paketi denetler.
- **Kafada ayrı taklit yok:** Akıl, maçtaki kuralın aynı fonksiyonunu çağırır (`alici`, `govdeAyir`, `ivme` gibi).
- **DEGISIKLIK-GUNLUGU.md:** Her değişiklik orada; yer bilgisiyle.

## 6 · Açık küçük işler

1. **YZ şiddet tepkisi ikiliye ulaşmıyor:** `shape.js`'te yenik YZ için şiddet +0,25 (pres kararı), ama `match.js` ikili döngüsü (`sdq`) takımın kendi şiddetini kullanıyor. İstenen buysa sorun yok; değilse aynı +0,25 ikiliye de.
2. **DEGISIKLIK-GUNLUGU.md satır 133:** "Yeni sürümlerde bu bayrağı **korumayın**" → niyet "koruyun / kaldırmayın". Yanlış okunmasın.
3. **CLAUDE-CODE-NOT.md eski:** "sayfanda bu düzeltme yok", "sayfalar repoda yok" artık doğru değil; silinsin ya da güncellensin.
4. **Sert pres top kapmaya dönüşmüyor (ikili ölçümü, 36 maç, Liman):**

   | / maç | şiddet 0 | 0,5 | 1 |
   |---|---|---|---|
   | ikiliye giriş | 4,0 | 7,0 | 14,4 |
   | ikili kazanma oranı | %31 | %35 | %30 |
   | kazanılan ikili → topa sahip olundu | 1,0 | 1,8 | 2,5 |
   | kazanılan ikili → top boşa düştü, rakip geri aldı | 0,3 | 0,6 | 1,8 |
   | pas keserek kazanılan | 10,4 | 9,7 | 9,7 |
   | toplam top kapma | 10,4 | 10,7 | 10,7 |
   | kaybedilen ikiliden sonra 2 sn içinde rakip gönderdi | 1,0 | 0,8 | 2,7 |

   Sebep üç yerde: (a) ikili kazanma oranı şiddetle hiç değişmiyor; daha çok ikili, aynı oran. (b) Kazanılan ikilinin yarısı topu dürtüp boşa düşürüyor; yön rastgele (`match.js` ikili, `(m.r() - .5) * 1.6`), presçinin yanında toplayacak arkadaş olmadığından sert takımda boştaki topun ~yarısını rakip alıyor. (c) Geçilen savunmacılar pas kesmeyi biraz azaltıyor.
   **Düzeltme (sonraki analiz):** Pres sayıya dönüşüyor: top sayısı değil kazanılan yer değişiyor. Şiddet 0 → 1: rakip yarısında kazanılan %31 → %38, Liman sayısı 1,25 → 2,25, gönderme başına sayı %15 → %24. Blok düşük → yüksek: rakip yarısında kazanılan %25 → %41, Liman sayısı 1,25 → 2,58, rakip sayısı 0,67 → 2,00 (arkada açılan boşluk). Yani pres gerçek bir risk/ödül kararı; eksik olan yalnız **zorlanan hata** (madde 5).
   Mekanik öneriler (kural değil): **ikinci kişi desteği** (presçinin yanında boştaki topu toplamaya hazır biri; "ikinci presçi kaçış yolunu kapatır" fikriyle aynı aile) · **ikili kararının kalibrasyonu** (aklın ikiliye girerken kullandığı kazanma tahmini ile gerçek sonuç; şiddet daha düşük ihtimalli ikililere soktuğu hâlde oran düşmüyor, tahmin ile sonuç zayıf ilişkili olabilir; Claude Code ölçebilir) · **dürtülen topun yönü** savunmacının gövde yönüne ya da arkadaşa doğru (rastgele yerine).

5. **Baskı pasın isabetini etkilemiyor (kodda):** `decide.js` `applyError` pas sapmasını yalnız Aktarım ve tek dokunuşla hesaplıyor; baskı yalnız gönderme için (`match.js` `ek *= D.gonEk(...)`, yalnız `o.kind === 'gönder'`). Dibinde iki rakip olan oyuncu, boştaki kadar isabetli pas atıyor; presin "rakibi hataya zorlama" yolu yok. Öneri: göndermedeki baskı terimini (gonEk) pasa da uygulamak; akıl da aynı fonksiyonla tahmin etsin (kafada ayrı taklit yok). **Ölçüldü (3.057 pas, zip32):** Pasçıya en yakın rakip 2 birimden yakınken pas tutma %88, baskısızken %91: baskı isabeti neredeyse hiç etkilemiyor. Pres yine de çalışıyor ama başka yoldan: akıl baskı altında geriye ve kısa oynuyor (geriye pas %19 → %37; 2+ rakip varken %52; ileri gidiş 13,7 → 4,8 birim). Yani pres bugün rakibi **geri oynamaya** zorluyor, **hataya** zorlamıyor.
   **Ek bulgu: aklın pas tahmini yeniden iyimserleşti.** Ortalama tahmin %94–98, gerçek %87–91 (≈ +7 puan; pas kalibrasyonundan sonra +1'di). En büyük sapma: kendine pas (kenardan) akıl %99 · gerçek %78; rakip üçte birinde akıl %93 · gerçek %82. Olası sebep: kafada oynatma savunmanın yeni mekaniklerini (mekanik pres, ortak atama, alan paylaşımı) maçtaki kuralla değil eski bir kestirmeyle hesaplıyor (kafada ayrı taklit). Pas kalibrasyonu yeni savunmayla tekrarlanmalı; Claude Code ölçebilir.
6. **Saha ölçüleri tek yerden, oransal (ÖNCELİKLİ, Aşama 1'in başında):** Kullanıcı sahanın eninde sonunda büyüyeceğini düşünüyor (daha geniş alan → daha çeşitli taktik). Motorda 100, 50, 25 (orta çizgi), pres hattı (42, 62), bölge sınırları, Kuyu ağzı, Bekçi alanı ve gönderme tablosu sabit sayı. Bunlar tek bir yerden okunan, sahaya oransal ayarlara çevrilsin; rolsüz sistem ve evreler yazılırken sabitler çoğalmadan. Sonra Claude Code aynı motoru üç boyutta ölçer (100×50, 115×57, 130×65): laboratuvarın tarz × tarz tablosu (taktik çeşitliliği: birden çok geçerli tarz, gerçek karşılıklar), taktik etkilerinin büyüklüğü, sayı, pas, pres, hesap süresi. Maç süresi saha ile birlikte ayarlanabilmeli. 7'ye 7 şimdilik kalır.

7. **"Arkada kalan" takım ayarı kaldırılsın.** `tac.arkada` üç yerde "en geride duran N oyuncu"yu seçiyor (`shape.js` 100 savunmada süpürücü, `match.js` 120 hücumda güvence, `shape.js` 169 kontrada geride kalan). Dizilişle ve Süpürücü rolüyle çakışıyor; menajerin çizdiği ile sahada olan farklılaşıyor. Yerine oyuncu başına **"geride kal / arkayı koru"** talimatı (savunma, hücum ve kontra evresinde ayrı). Taktik ekranındaki bedel kartı da bu sayı yerine talimatlara baksın.
8. **Saha büyütmede sabitlerin üç türü:** (a) **fiziksel** sabitler değişmez: koşu hızı, gövde, uzanma, top fiziği, pas hızları (saha büyür, oyuncu aynı kalır); (b) **geometrik** sabitler sahayla oranlı: pres hattı, bölgeler, ön alan ızgarası (`decide.js` `FRT`: x 36–92, y 6–44), tehdit ölçeği (`threatK`), diziliş yerleri, aday nokta mesafeleri; (c) **zaman ufukları** gerekirse uzar: kafada oynatma (`simMax` 180), alıcı (240). Gönderme ve değer tabloları yeniden üretilir. Hesaplayarak düşünen kısım (kafada oynatma) kendiliğinden uyum sağlar; sayılara gömülü kestirmeler sağlamaz. Test: kontrol paketi + pas kalibrasyonu büyük sahada.

## 7b · İleride: menajerlik ve dünya katmanı (motor dışı; motorla bağları)

Kullanıcının yönü: Oyun hem bilgisayarda hem mobilde, offline ve online; aynı anda tek cihazda oynanır, kariyer cihazlar arasında taşınır. Bunlar motorun dışında, ama motordaki kararlar bunlarla uyumlu kalmalı. Ayrıntılı tasarım sonra; burada yön ve motor bağları.

**Offline dünya: ligler, kümeler, kupalar (kullanıcı kararı: 7 lig).** Gerçek ligler takip edilmez; gerçek liglerin değeri (birbirinden farklı olmaları) kurgusal bir dünyada kurulur.
- **7 lig, her birinin karakteri var.** Ligi ayıran şey isim ya da renk değil, menajere farklı iş yaptıran özellikler:
  - *Oyun kültürü:* o ligin yapay zekâ kulüpleri belli tarzlara yatkın (birinde çoğu takım yüksek pres, ötekinde düşük blok ve kontra). Lig değiştiren menajer taktiğini yeniden düşünür; farklı tarzlara karşı farklı oyuncu gerekir (kadro derinliğinin anlamı).
  - *Ekonomi:* maaş tavanı, bütçe, altyapı gücü, kulüp sayısı. Zengin lig (kaliteli kadro, büyük beklenti) ile yetenek yetiştiren lig (güçlü altyapı, küçük bütçe, en iyisini satmak zorunda) bambaşka menajerlik.
  - *Sporun kuralları her ligde aynı* (saha, oyuncu sayısı, süre): neyin neden olduğu anlaşılır kalsın.
  - Örnek karakterler (taslak; ad ve ayrıntı sonra): zengin merkez lig; yetenek fabrikası; pres kültürü; düşük blok ve kontra kültürü; sabırlı, teknik lig; yükselen lig; küçük, uzak lig (yeni menajerin kariyere başladığı yer).
- **Kümeler:** Ligler büyüklüğe göre 2–3 kademe (örnek: büyük ligler 3 × 10 takım, küçükler 2 × 10). Yükselme–düşme (örnek: 2 yukarı, 2 aşağı; istenirse bir eleme maçı). Her kademenin kendi maaş tavanı: alt kademede kadro kurmak ucuz, iyi oyuncuyu tutmak zor.
- **Kupalar:**
  - *Lig kupası:* eleme usulü, bütün kademeler. Alt ligin büyüğü elemesi; şansı bilerek az tuttuğumuz bu dünyada hikâyenin büyük kısmı buradan doğar.
  - *Ligler arası turnuva:* her ligin en iyileri (grup + eleme). Farklı oyun kültürleri burada çarpışır.
  - *Lig katsayısı:* bir ligin kulüpleri turnuvada başarılı oldukça (son birkaç sezonun ortalaması) o lig turnuvaya daha çok takım gönderir; ligler zamanla güçlenip zayıflar, dünya durağan kalmaz.
- **Menajer kariyeri:** Ligler ve kademeler arasında iş değiştirir; teklifler itibara göre (kazandığına değil, eldeki kadroyla beklentiyi ne kadar aştığına) gelir.
- **Dünyanın kurulumu:** Liglerin karakteri elle tasarlanır (oyunun kimliği); kulüpler ve oyuncular her yeni kariyerde yeniden üretilir (her kariyer farklı).
- **Önkoşul: hızlı sonuç modeli.** 7 lig × ~25 kulüp ≈ 175 kulüp, haftada ~85 maç. Motor bir maçı dakikalar içinde hesaplıyor; menajerin izlemediği maçlar tam motorla oynatılamaz. Gerekli: takım gücü ve taktik eşleşmesinden gerçekçi skor üreten, motordan ölçülerek kurulmuş ve motorla tutarlılığı ölçülen bir sonuç modeli (aşağıdaki "hafif mod"). Lig sistemi ondan önce kurulamaz. Claude Code'un kayıt ve ölçüm araçlarıyla yapılabilir.

**Online yapı:** turnuvalar, aşamalar, gruplar, sezonlar; tabakalı ligler ve yükselme–düşme (yeni ve küçük kulüpler alttan başlar; güçlü kulüplerin üstte kalması sorun değil).

**Oyuncu yaşam döngüsü (yaş yok, yaşlanma yok):** Oyuncular insan değil; özellikler yaşla düşmez. Dolaşım ve statikleşmeye karşı:
1. **Kariyer süresi (kullanıcı kararı):** Her oyuncunun baştan belli, görünür bir kariyer süresi var (örnek: 6–10 sezon, oyuncudan oyuncuya farklı; bir kuşak aynı anda gitmesin). Süre dolunca oyuncu ayrılır; **özellikleri son güne kadar düşmez**. Adı dünyaya uygun seçilir (oyuncular insan değil; "emeklilik" yerine örneğin lisans süresi, Design karar verir).
   - Neden: ayrılan, yeteneğinden bağımsız ayrılır; havuz yeteneklilerle dolmaz. Dünyadaki toplam yetenek ≈ sezon başına gelen yeni oyuncu × kariyer süresi (tek, anlaşılır ayar).
   - Kalan süre piyasa değişkeni olur: kalan 1 sezonlu yıldız ucuz ve hemen işe yarar, 5 sezonlu genç uzun vadeli yatırım.
   - Giden oyuncu kayıp değil hikâye: kulübün efsaneler listesi gibi.
   - (Vazgeçilen fikirler: "uzun süre kulüpsüz kalan dünyadan çıkar" havuzu yetenekliler lehine çarpıtır; "seviye oynanan lige çekilir" gereksiz karmaşıklık.)
2. **Kullan ya da kaybet yalnız profili şekillendirir:** Oyuncu sahada hangi işi yapıyorsa o yanı gelişir, kullanılmayan yanı yavaşça körelir (koşucu olarak oynayan koşuculaşır). Rolsüz sistemle örtüşür. Sonuç: kimse her şeyde 20 olmaz, "en iyi oyuncu" tek bir sıralama değil; bir taktik için en iyisi başkası için vasat olabilir.
3. **Sözleşme ve oyuncu iradesi:** Sözleşmesi biten oyuncu, oynamıyorsa ya da daha iyi teklif varsa ayrılabilir; serbestler havuza döner.
4. **Özellik tavanı (20):** Zirvede kadrolar benzeşince farkı taktik yaratır; oyunun asıl değeri öne çıkar.

**Altyapı (yaşsız dünyada):** "Genç" yaşı küçük değil, henüz şekillenmemiş oyuncu: özellikleri düşük, gelişmeye açık, profili belirsiz. Asıl değeri: **hazır oyuncu yerine kendi taktiğine göre şekillendirdiğin oyuncu.** Seçim: hazır oyuncu (pahalı, hemen, profili sabit) ya da yetiştirmek (ucuz, zaman ister, belirsiz, tam senin sistemine göre).
- **Gelişim payı (gizli):** Ne kadar ve ne hızla gelişebileceği; oynadıkça ortaya çıkar. Scout'un işi bunu erken tahmin etmek.
- **Gelişim maç süresi ister:** Genç oynarsa bugün takım zayıflar, oynamazsa gelişmez (gerçek gerilim). Çıkışlar: kiralık (alt lig kulübünde oynar; online'da iki tarafa da yarar), altyapı maçları (yavaş gelişim, risksiz).
- **Yatırım ve kimlik:** Yatırım sezon başına aday sayısını ve kalitesini belirler; kulüp bir yön seçebilir (hızlı oyuncu ya da pasör yetiştiren altyapı) ve bu zamanla kulübün kimliği olur.
- **Dünyaya giriş:** Sezon başına yeni oyuncu sayısı kulüp sayısıyla orantılı (kulüp başına yetenek sabit kalır).

**Kimse domine etmesin (kullanıcı şartı):** Tek fren yetmez; birbirini tamamlayanlar:
1. **Ligin maaş tavanı:** Her ligde kulübün toplam maaşına görünür, basit bir üst sınır (gizli katsayı değil, herkesin bildiği kural). Para biriktirmek işe yaramaz.
2. **Maaş kaliteyle orantıdan hızlı artar:** 18'lik oyuncu 14'lüğün çok katı pahalı. Tavanla birlikte seçim doğar: birkaç yıldız + vasat ekip mi, dengeli kadro mu (ikisinin taktik sonuçları farklı).
3. **Kadro sayısı sınırı:** Oyuncu biriktirmeyi engeller; iyiler kulüplere dağılır.
4. **Gelir beklentiyi aşmaya bağlı** (aşağıda, performans ve ekonomi): güçlü kadro kazanınca az prim alır.
5. **Motor tarafı:** Bütçe kadroların ne kadar farklı olabileceğini sınırlar; o farkın sahada ne kadar avantaja dönüştüğünü motor belirler. Küçük kalite farkı neredeyse kesin galibiyetse bütçe ne yaparsa yapsın güçlü domine eder. Zayıf takımın kazanma şansı hep olmalı ve taktik farkı kapatabilmeli. Özellik ölçümünde ölçülür: belli bir kalite farkında zayıf takım yüzde kaç kazanıyor, taktik bunu ne kadar değiştiriyor.

**Olmayacaklar (kullanıcı kararı):** Kondisyon (maç içi de maçlar arası da), sakatlık, zorunlu rotasyon. Gerekçe: menajerin elinde olmayan sebeplerle düzeni bozarlar, taktik özgürlüğü kısıtlarlar. Kaybın sebebi menajerin kararı olmalı, şans değil. Yüksek presin bedeli kondisyondan değil oyunun kendisinden gelir: arkada bırakılan alan (ofsayt yok, bu yüzden risk büyük).

**Kadro derinliğinin anlamı taktik seçimden gelir:** Mesele farklı taktiklere uygun oyunculara sahip olabilmek. Düşük bloğa karşı sabırlı kurucu, yüksek prese karşı hızlı koşucu; maç içinde plan değiştirmek için oyuncu değişikliği (yorgunluk için değil). Bunun şartları:
- Rakip tarzları sahada gerçekten farklı olmalı (taktik derinlik aşaması).
- Bir özelliğin değeri taktiğe göre değişmeli (yukarıdaki çeşitlilik şartı); yoksa her taktiğe aynı 7 oyuncu en iyidir ve geniş kadro boşa durur. Özellik ölçümü bunu da ölçmeli: aynı oyuncu iki farklı taktikte ne kadar farklı değer üretiyor.

**Gerçek kişi rolleri (özgün mekanik):** Scout, transfer yöneticisi, ileride direktör; menajer bu görevleri gerçek oyunculara açabilir, istemeyen yapay zekâ çalışanla oynar.
- **Bilgi gizli olmalı:** Özelliklerin tam değeri görünmez, gözlemle netleşir (maç izleme, istatistik, scout raporu). Yoksa scout'a iş kalmaz.
- **Ölçülebilir itibar:** Sistem gerçek değerleri bildiği için scout'un isabeti otomatik ölçülür ("tahminleri ortalama ±1,2 sapıyor").
- **Güven ve hile koruması:** Sınırlı yetki (bütçe tavanı, satışta alt fiyat, onay gereken işlemler), her işlemin kaydı, bağlantılı hesapların şüpheli transferlerinin işaretlenmesi; ödeme oyun içi para ve itibarla (gerçek parayla değil).
- **Ayrı kariyer yolu:** scout → transfer yöneticisi → direktör → menajer.
- **Zamanlama:** Kritik kitle gerektirir; önce yapay zekâ çalışanlar, oyuncu sayısı arttıkça insanlara açılır.

**Performans ve ekonomi:** Başarı "ne kadar kazandın" ile değil "elindekiyle beklenenden ne kadar fazlasını yaptın" ile ölçülür (ayın menajeri, ödüller, gelir).
- Her maç için **beklenen sonuç**: sonuçlarla güncellenen takım puanı (Elo gibi) + kadro gücü (özellik ölçümünden). Beklenti piyasa değerinden değil **sahaya çıkan kadronun gerçek gücünden** hesaplanır (kadroyu zayıf gösterip ödül toplamaya karşı).
- Gelir: **sabit taban** (lig payı, taraftar) + **performans primi** (beklentiyi aşma). Kazanamayan kulübün bütçesi zorlanır, oyuncu satmak gerekebilir.
- Frenler: ölüm sarmalına karşı sabit taban ve yükselme–düşme; zenginleşme sarmalına karşı değerle artan maaş ve kadro sınırı.
- **Çeşitlilik şartı:** Bir özelliğin değeri taktiğe göre değişmeli; yoksa herkes aynı oyuncuyu ister, pazar ve oyun tekdüzeleşir (ölçülebilir; özellik ölçümünün parçası).

**Gerçek para (kullanıcı kararları):** Reklam hiçbir yerde yok, mobilde de. Güç satılmaz (para ile oyuncu, bütçe, özellik yok): online rekabette taktik yerine cüzdan kazanırdı. Analiz araçları satılmaz. Gelir modeli sonra konuşulacak.

**İleride, fikir aşamasında:** Başkanlık sistemi (kulübün sahibi ya da seçilmiş başkanı: bütçe, menajer ve çalışan atama), taraftarlar (kulübe bağlılık; gelire, atmosfere, başkanlık seçimine etkisi). Tasarımı sonra; ilke: her biri gerçek bir karar ve ödünleşim yaratsın, süs olmasın.

**Motor bağları (şimdiden önemli olanlar):**
- **Özellik ölçümü** → oyuncunun değeri, maaşı, gelişim önceliği (keyfi değil, sahadaki etkiden).
- **Özellikler sahada okunabilir olmalı:** Gizli bilgi sisteminde hızlı oyuncu hızlı görünmeli, iyi pasçı iyi pas atmalı; sahada izi olmayan özelliği ne scout ne menajer fark edebilir. Özellik ölçümü bunu da sınar.
- **Oyuncunun maçta yaptığı işin kaydı** (görev, ikili, pas, koşu) → "kullan ya da kaybet" profil gelişimi. Motor bunu büyük ölçüde zaten biliyor; ileride oyuncu başına özetlenmeli.
- **Hafif mod ve hesap bütçesi:** İzlenmeyen maçlar ve online dünyada sunucu hesabı için.
- **Kayıtlar sonucu tutsun:** Kariyer kaydı maç sonuçlarını saklar, yeniden oynatmaya dayanmaz; kayıtlar motor sürümünü taşır. Yarım kalan maç taşınacaksa zarın (rastgele sayı üreteci) durumu kayda sayı olarak girmeli.

## 7 · Sıra ve teslim

1. **Aşama 1** (şimdi): önce saha ölçülerini tek yerden ve oransal yap (6.6), sonra talimat sistemi + hazır paketler + gizli avantajların kaldırılması (3.1–3.4), **veri yapısı baştan evreye bağlı ve miraslı**. Bekçi unvanının atanabilmesi (kullanıcı onayıyla).
2. Her zip → Claude Code kontrol paketi → rapor (açılış, donma, hız, istatistik, taktik etkisi). Ek olarak: hazır paketlerle oynanan maçların bugüne yakınlığı ve "aynı talimat = aynı davranış" testi.
3. **Aşama 2** (taktik derinlikten sonra): özellik listesinin yeniden tanımı (4), ardından özellik ölçümü.
4. **Aşama 3:** oyun evreleri, isteğe bağlı derinlik (4b).
5. Ara işler: bölüm 6'daki açık işler (özellikle ikili/pres mekaniği, madde 4) uygun bir zip'e.
6. Sonra: menajere uyum bilgisi (ölçümden).
