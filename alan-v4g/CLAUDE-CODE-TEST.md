# ALAN v4g · Claude Code için test görevi

Sistem: `alan-v4g/` (sayfa: `ALAN v4g Maç.html`, web sunucusundan açılır). Motor: `core.js` (fizik), `match.js` (maç adımı), `shape.js` (refleks yerleşim), `decide.js` (aday hamleler, değer tablosu okuma), `beyin.js` (tek beyin: ileri oynatma, seçim), `ogrenme.js` + `ogrenci.js` (hafıza: değer tablosu kendi maçlarından öğrenir), `mac-isci.js` / `beyin-isci.js` (işçiler).

## 1 · Asıl görev: uzaktan göndermede beynin hayali iyimser mi?
Gözlem: 35+ birimden göndermeler sık. Tek örnekte beyin 35+ birimden bir göndermeye +0,82 değer verdi; gerçekte Bekçi hattın üstündeydi (0,1 birim) ve tuttu.

Deney (en az 300 gerçek gönderme, karışık taktikler, `Herkes 10`):
1. Gerçek maçta her gönderme anında kaydet: uzaklık, açı, şarj, Bekçi'nin Kuyu'ya ve gönderme hattına uzaklığı, beynin o adaya verdiği değer (`m._beyinSon.aday[secilen].v`), gerçek sonuç (sayı / Bekçi tuttu / bizden biri aldı / rakip aldı) ve 10 sn içinde sıradaki sayı.
2. Aynı anı `cloneMatch` ile 20'şer kez yeniden oynat, dört koşulda:
   - `B.A.hafif` true / false
   - algı hatası açık / kapalı (`B.A.algi = 0`)
   - Bekçi derinliği planı ileri oynatmada da çalışsın / çalışmasın (`bekPlan` şu an `m._ic` iken atlanıyor)
3. Rapor: uzaklık dilimine göre (≤20, 20–35, 35–50, 50+) beynin ortalama değeri ile gerçek sonucun değeri (sayı = +1, rakip sayı = −1, aksi hâlde tablo değeri). Hangi koşulda fark kapanıyor? Sebep o.

Kural koyma, katsayıyla gizleme yok: fark bir mekanikten geliyorsa onu söyle.

## 2 · Gerçek maçta gönderme sonuçları (Bekçi)
Temiz testte (Bekçi Kuyu ağzında, gönderme ağza) sayı oranı: 15 birim %82, 25 %51, 35 %18, 50 %6. Maçta sayı oranı bundan belirgin yüksek görünüyor (90 sn başına 6–9 sayı). Maçta Bekçi'nin gönderme anındaki yeri ve tepkisi testtekinden farklı mı? Uzaklığa göre gerçek maç sayı oranını ve Bekçi konum dağılımını çıkar.

## 3 · Hareketin doğallığı (kullanıcı: "hâlâ doğal değil")
Ölçülecekler:
- Yön değiştirme sıklığı ve açısı (oyuncu başına saniyede 45°+ dönüş sayısı), ivme sıçramaları.
- Topsuz oyuncular: hedef noktası her tik değişiyor mu (titreme), hedefe varınca durup tekrar kalkma.
- Taşıyıcı: sürme planı başlarken/biterken ani dönüş, "kıvrak" sürmede kırma anı.
- Beyin yerleşimi (`m.yer`): 2 sn'lik kayma bittiğinde ani sıçrama oluyor mu?
Çıktı: en çok sıçrama üreten kaynak (refleks hedef değişimi mi, beyin kayması bitişi mi, plan geçişi mi). Düzeltme önerisi mekanik olsun (ör. hedefe yumuşak geçiş, dönüş ivmesi sınırı fizikte).

## 4 · Hız ve belirlenimcilik
- 90 sn maç, 4 çekirdek: yükleme süresi.
- Aynı tohum iki kez: birebir aynı maç mı (işçili ve işçisiz)?

## 5 · Hafıza (öğrenme)
`ogrenci.js` 50 maç oynasın (kusursuz okuma). Tablo değişimi: 35+ birimden göndermenin uç durumları (top bizde, rakip Kuyu'ya 25–35 birim, önde 0–2 savunmacı) öğrendikçe düşüyor mu? 0, 25, 50 maçta uzak gönderme sıklığını karşılaştır.

## 6 · Top saklama ve savunma zekâsı (yeni)
Kullanıcı gözlemi: top saklama fazla işe yarıyor; savunmanın yerleşimi, baskısı ve top kapması zayıf.
- **Top saklama:** Taşıyıcı dibinde rakipken ne kadar süre topu koruyabiliyor (ikili sayısı, kazanma oranı, kayba kadar geçen süre)? Gövdeyle koruma (`shielded`, `shieldK .4`) ikiliyi ne kadar zorlaştırıyor? Yön, Sürme–Kesme farkı ve ikinci presçiye göre dağılım.
- **Kapma:** Savunmacının topa uzandığı anların kaçında giriyor (beynin ikili kararı), kaçında kazanıyor? Kaybedilen ikililerde savunmacının açısı (önden/yandan/arkadan).
- **Baskı:** Pres sayısı ayarına göre (0–3) taşıyıcıya ulaşma süresi; ikinci presçinin kaçış açılarını kapatma oranı; presçinin arkasında açılan boşluğun kullanılma oranı.
- **Yerleşim:** Savunmanın Çekirdekle Kuyu arasına girme oranı, en tehlikeli rakibin markajsız kaldığı süre, iki savunmacının aynı bölgede (≤3 birim) üst üste durma süresi, beyin yerleşim kararlarında "refleks yeri" dışı seçim oranı.
Çıktı: en zayıf halka ve mekanik sebebi (kural ya da katsayı önerisi değil).

## Son değişiklikler (bilgi)
- Bekçi: kendi elleri (uzanma 3,2, kapasite ×1,7, tutamadığını yana çeler)
- Tek dokunuş beynin kararı (gönderme ya da kontrol, ileri oynatarak)
- İki adımlı derin bakış en iyi 8 + her ailenin (her alıcıya pas, sürme, bekleme) en iyisi; son seçim yalnız derinleştirilenler arasında
- Şarj: oyuncunun kendi hareketine harcamadığı enerji (duran tam, tam hızla koşan %10)
- Sürme: 6/12/20 birim planlar, yavaş ve kıvrak sürme, rakipten yana sıyrılan doğal yol, topla hız Sürme'ye bağlı
- Yerleşim: beyin seçer (refleks yeri, 7 birim çevresi, yerinde dur, üstüne çık, ikisini kapat); "ikisini kapat" artık otomatik değil; geçilen savunmacı engel olmaz
- Bekleme planı (0,5 / 1 sn), koşularla birlikte tartılır
- Değer tablosu 2700 hücre (uzaklık, kanat, önde savunmacı, baskı, geçiş, Bekçi konumu), yumuşak okunur
