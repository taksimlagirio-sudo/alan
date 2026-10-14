# ALAN · Design için plan: taktik kurulumu (sekiz evre yerine iki mod)

Görsel taslak: https://claude.ai/artifact/Bn2wnFmcU468zXobEtyBUZ (beş pano; 2. ve 3. oynanabilir; 4. ve 5. ekran taslağı).
Kullanıcı taslağı onayladı. Bu belge motordaki karşılığıdır.

## 0 · Neden

- Bir hücum birkaç saniye sürüyor. Top 3 saniyede kendi bölgesinden son bölgeye gidiyor. Kurma, ilerleme ve son bölge ayrı ayar değil, aynı hareketin parçaları.
- Bugün sekiz evre var (`EVKOD`), her birinin ayrı ayarı ve talimatı (`tac.evre[...]`, `p.tl['huc.kurma']` …). Oyuncu evre sınırında bir ayar setinden ötekine atlıyor; her evre zamanın yalnız %8–21'i (zip33 ölçümü), ayarların etkisi sahada zor görünüyor.
- Geçiş fazla basit: sabit bir pencere (`WIN_T`, `kpSure`) ve geçişin başında bir kez seçilen sabit sayıda koşucu (`kontraN`). Geçişin nerede başladığı, topun önünde kaç kişi olduğu karara girmiyor.
- Kullanıcının ilkeleri: az ama sahada görünen ayar; gizli katsayı yok; ayrıntı isteyen için ince ayar, istemeyen için kendiliğinden çalışan varsayılan.

## 1 · Yerleşik oyun: topa bağlı tek plan

**Ne:** Her takımın iki planı var: hücum (topa sahipken) ve savunma (topa sahip değilken). Her plan **üç duruştan** oluşur: top kendi bölgemizde, ortada, son bölgede. Her duruş, Bekçi dışındaki altı oyuncunun yeridir (takımın kendi yönüne göre; ileri ve yana).

**Nasıl çalışır:**
- Top ilerledikçe, oyuncunun evi iki komşu duruşun doğrusal karışımıdır. Topun ilerisi (takımın yönünde, 0–100) 20, 50, 80 noktalarında duruşların tam kendisi; arada karışım. 20'nin gerisi ilk duruş, 80'in ilerisi son duruş.
- Top yana kayınca bütün blok onunla kayar (bugünkü `(by - 25) * sh` kayması gibi; hücumda ve savunmada ayrı oran kalabilir).
- Bu yer oyuncunun **evi**dir; bugün `slotPos` / `slotBase`'in verdiği yerin yerine geçer. Aklın boşluk seçimi (`shape.js`'deki aday tartımı), serbestlik bağı (`roleLeash`), koşu penceresi, pas açısını kapatma, markaj: hepsi olduğu gibi kalır, yalnız evden çalışırlar.
- Bekçi kuralı (`bekci`) değişmez.

**Varsayılan plan (ayar yapmayan kullanıcı):** Üç duruş dizilişten ve bugünkü takım ayarlarından (blok, genişlik) kendiliğinden üretilir: bugünkü `slotPos` top 20, 50 ve 80'deyken hesaplanıp duruş olarak kaydedilir. Böylece hiçbir şeye dokunmayan menajerin takımı bugüne çok yakın oynar ve ölçümle karşılaştırılabilir. Blok ve genişlik ayarları ayrı ayar olarak kalmaz: duruşları üreten kısayol olur (blok yüksek = savunma duruşları daha önde, vb.).

**Menajer ne ayarlar:** Tahtada üç sekme (Kendi bölgemizde / Ortada / Son bölgede); oyuncuları sürükler. Hücum ve savunma için ayrı. Bir duruşu ellemezse varsayılan kalır.

**Talimatlar:** Oyuncu talimatları (koşu, serbestlik, destek, pres, adam …) plan genelinde tek; gerekirse bir moda (hücum/savunma) özel. Bugünkü evre başına talimat (`p.tl['huc.kurma']` gibi) kalkar; yerine istenirse duruş başına değişiklik (seyrek kullanılacak bir ince ayar). Ekranda talimat sayısı artmamalı.

## 2 · Geçiş: durumdan doğan an

**Ne zaman var:** Geçiş bir süre ya da bölge değildir. Top el değiştirdiğinde, kaybeden takım dağınıksa (topun gerisinde Bekçi dışında 4'ten az oyuncu: bugünkü `m.gecis` kuralı) başlar; **kaybeden takım yerleşince biter** (topun gerisinde 4+ oyuncu). Bugün koşul yalnız kazanma anında bir kez bakılıyor; artık her tik bakılır. Bittiği an iki takım da yerleşik plana döner.

**Topu kazanınca (hücum geçişi):**
- Menajerin kuralı tek cümle: "Önde **sayıca üstünsek** / **eşitsek de** / **her zaman** / **hiçbir zaman** git."
- Sayı sahadan gelir: topun önündeki bizden (Bekçi hariç) ile topun önündeki rakipten (Bekçi hariç).
- Kural "git" derse katılanlar: topun önündekiler + taşıyıcı; top kendi bölgemizde kazanıldıysa en yakın bir destek. Geri kalanlar güvence olarak evlerinde. **"Kaç kişi koşsun" ayarı yok** (bugünkü `kontraN` kalkar); sayı durumdan çıkar ve geçiş boyunca yeniden bakılır (önde biri yakalanırsa ya da arkadan biri yetişirse değişir).
- Kural "yerleş" derse geçiş oyunu oynanmaz, takım hemen yerleşik plana döner.
- Koşucuların koşusu bugünkü kontra koşusu mantığıyla (`runV`: rakipten önce varabilir miyim × tehlike) kalabilir.

**Topu kaybedince (savunma geçişi):**
- Menajerin kuralı: "**Rakip yarısında** / **her yerde** / **hiçbir yerde** kaybedersek **1 / 2 / 3 sn** karşı pres, yoksa geri dön."
- Kim basar: topun 8 birim içindeki oyuncular (en fazla 3). Kimse yoksa basılmaz. Diğerleri aynı anda savunma planındaki evlerine döner. Süre dolunca basanlar da döner.
- Bugünkü `kpSure` bu sürenin kendisi olur; `kayip` ayarı bu kuralın içine girer.

## 2b · Ekran (zip38'deki taktik ekranına bakılarak)

**Korunacaklar:** Tahta merkezde, şekil sürüklenerek kuruluyor. "Tercihlerinden çıkan tarz" kartı ve "bu kurgunun bedeli" (ayarların ne anlama geldiğini düz dille söylüyor). "Motora giden" kutusu.

**Bugünkü sorunlar:**
1. **Talimatlar tahtadan kopuk.** Oyuncular sekmesinde tek bir uzun liste (11 hücum + 3 savunma kaydırıcısı). Tahta "Top rakipteyken" iken sağda "Topa sahipken" talimatları açık olabiliyor: menajer neyi ayarladığını tahtada görmüyor.
2. **Evreler her yerde.** Oyuncular sekmesinde 7 evre çipi: 6 oyuncu × 14 talimat × 7 evre ayarlanabilir.
3. **Aynı ayar iki yerde.** Pas↔sürme ve tempo hem takım hem oyuncu ayarı; "Kenara sıkıştır" hem Pres tuzağı'nda hem Savunma tarzı'nda; "Adam adama" hem takım savunma tarzında hem oyuncunun "alanı tutar / adam tutar" talimatında. Hangisinin geçerli olduğu belli değil.
4. **"Arkada" motorda yaşıyor.** Ekrandan kalktı ama "Motora giden"de `arkada 2` var; motor bunu kontra penceresinde geride kalan güvence sayısı olarak kullanıyor (`shape.js`, `m.tac[att].arkada`). Kullanıcı bu sayıyı dizilişin belirlemesini istemişti. Yeni geçişte güvence = topun önünde olmayanlar (2. bölüm); `arkada` tamamen kalkar.

**Yeni ekran:**
- **Mod her yerde tek:** Ekranın başında "Topa sahipken / Topa sahip değilken". Tahta, duruş resimleri, talimatlar hep aynı moddadır; ikisi aynı anda farklı olamaz.
- **Tahtada oyuncuya tıklayınca** sağda yalnız o modun talimatları açılır: hücumda koşu, topu nerede ister, destek, serbestlik, boşluk doldurur, pas riski (5–6); savunmada adam/alan, kimi tutar, kişisel pres (3). Serbestlik tahtada oyuncunun evi etrafında bir alan olarak görünür ve değiştikçe büyür/küçülür; koşu, evinin ilerisine bir ok olarak.
- **Tahtanın altındaki küçük resimler:** dört evre yerine üç duruş (kendi bölgemiz, orta, son bölge). Tıklanan duruş tahtada açılır, sürüklenerek değiştirilir. "Kazanınca" ve "Kaybedince" resimleri kalkar; yerine iki cümlelik **Geçiş kartı** (2. bölüm).
- **Oyuncular sekmesi = genel bakış tablosu:** satırda oyuncular, sütunda ana talimatlar (iki modun öne çıkanları). Okumak ve karşılaştırmak için; hücreden değiştirilebilir ama asıl yer tahta. Rolsüz sistemde menajerin kurduğu rolleri tek bakışta gösterir ("Ece: dar serbestlik, topu ayağına, az koşu").
- **Tekrarlar tek yerde:** Takım ayarı temel, oyuncu talimatı ondan sapma ("takımın ayarı" / "daha çok" / "daha az"); pres şiddetinde kurulan kural tempo ve pas↔sürmeye de uygulanır. Kenara sıkıştırma tek yerde (savunma tarzı); adam adama tek yerde (oyuncunun talimatı; takım düğmesi "herkese uygula" kısayolu olur).
- **Evre çipleri kalkar.** Duruşa özel talimat yalnız "ince ayar" açılınca görünür.
- Başlangıç önerisi düğmeleri (Kurucu, Kanat, Süpürücü …) kalabilir: talimatları dolduran kısayol.

## 3 · Kalkanlar ve kalanlar

Kalkar: sekiz evre ve evre başına ayarlar (`EVKOD`, `evreBul`, `evreUygula`, `tac.evre`), evre başına talimatlar, `kontraN`, `arkada`, ayrı blok ve genişlik ayarları (duruşları üreten kısayol olarak kalır).
Kalır: pres şiddeti, pres tetikleri, tempo, risk, sabır, koşullar (skora göre), maç planı (taraf, hedef), tuzak, doldurma, oyuncu talimatları, akıl ve bütün yerleşim kuralları.
Evre ölçüsü ekranda bilgi olarak kalabilir ("top şu an ilerleme bölgesinde") ama ayar anahtarı olmaz.

## 4 · Akıl

- Akıl kafada oynattığı maçta yerleşimi `AlanShape.position` ile kurduğu için ev değişikliği oraya kendiliğinden geçer; ayrı taklit yazılmaz.
- Geçiş kuralları (kim katılır, kim basar, ne zaman biter) maçla aynı fonksiyondan okunmalı; kafada ayrı kopyası olmamalı.
- Akıl farkındalık haritasına iki satır: "topa bağlı plan" ve "duruma bağlı geçiş".

## 5 · Kabul (Claude Code ölçer; birleşik ölçümle, 16 maç, önce/sonra)

1. **Varsayılan plan bugüne yakın:** Ayar yapılmamış takımlarla sayı, pas, pas tutma, durma, koşu ve kalibrasyon zip38'in gürültü sınırında.
2. **Duruş ayarı görünür:** Bir duruşu değiştirmek (örneğin son bölgede iki kanadı genişletmek) o bölgede ölçülen takım şeklini değiştirir, diğer bölgelerde az değiştirir; geçişler kesiksiz (oyuncu ev hızında ani sıçrama yok).
3. **Geçiş duruma bağlı:** Geçişe katılan sayısı topun önündeki adam sayısıyla değişir; "hiçbir zaman" ve "her zaman" kuralları arasında geçiş sayısı ve geçişten gelen şut belirgin farklı; geçiş rakip yerleşince biter (süresi sabit değil).
4. **Karşı pres:** Kural ve kayıp yerine göre açılıp kapanıyor; basan sayısı 0–3; süre sonunda dönüyorlar.
5. **Hız:** Kontrol paketinde zip38'e göre en fazla %15 yavaşlama.
6. **Taktik ekranı:** Ayar sayısı bugünkünden az; tarayıcı testinde hata yok.

## 6 · Sıra

1. Motor: ev = duruş karışımı; varsayılan duruşlar bugünkü `slotPos`'tan. (Önce yalnız bu; ölçüm: kabul 1.)
2. Geçiş: her tik bakılan başlangıç ve bitiş, iki cümlelik kural, sahadan gelen sayı. (Ölçüm: kabul 3–4.)
3. Taktik ekranı: üç sekmeli tahta, iki cümle, talimatlar.
4. Evre ayarlarının kaldırılması (eski kayıtlar için: evre ayarı olan taktik yüklenirse yok sayılır ya da en yakın duruşa çevrilir).

Her adım ayrı zip olursa hangi değişikliğin neyi değiştirdiği ölçümde ayrılır.
