# Bilgi katmanı: oyuncunun motor hakkındaki bilgisi (tasarım planı)

## Fikir
Oyuncu bir hamle seçerken "bu hamle nelere yol açar" sorusunu bugün **anlık bir fotoğrafla** cevaplıyor (harita, `Sy`, `wf(T,tD)`,
kayıp bedeli V0). Fotoğraf birkaç saniye sonrasını göremiyor:
- yerleşik hücumda zaman lehte: 40 br'de topu tutmak modelde 0.03–0.05, motorun kendi maçlarında %18–27 gol → uzak şut;
- kontrada zaman aleyhte: 5 sn'de kutuya varan kontra %89, 8+ sn %12 (analiz/ZAMAN-DEGERI.md) → modelde kontra deparı değersiz.

İstenen asıl şey oyuncunun hamlelerinin sonuçlarını **hayal etmesi**. Maç içinde hayal (ileri oynatma) çok pahalı. Bu plan hayali
**maç dışına** taşır: motor kendi kendine oynar, hamlelerin sonuçlarını ölçer (gerekirse aynı andan dallanıp her seçeneği ayrı
oynatarak), bunu küçük tablolara damıtır; oyuncu maçta tabloyu okur. Tablolar motor değiştikçe yenilenir; ama her değişiklikte
değil: **elle tetiklenir ya da gece görevi** olarak çalışır.

## Bugün olan (öncüller)
| tablo | ne | üretim | yenilenme | durum |
|---|---|---|---|---|
| `veri/sut-tablosu.json` (ALAN_SHT_VE) | şut girme oranı (fizik) | shot-table.js | elle | kullanılıyor |
| `veri/imkan-sut.json` (imk-11) | imkân haritasının şut tablosu | `alan-kos … imktablo` | `sina/tablo.sh` parmak iziyle **eskime denetimi** | kullanılıyor |
| `kural/gec.rs` önbelleği | varış gecikmesi (80 tik ileri oynatma) | maç içinde, anahtardan | kendiliğinden | kullanılıyor |
| `veri/deger-tablosu.json` (ALAN_VAL v5) | "topla oynamanın değeri": 18 maç, 77 sayı | valuelab.js | **hiç** | yüklü ama **kullanılmıyor** (JS'te `useValTab` kapalıydı, Rust kapalı dalı aldı) |

Ders: değer tablosu bir kez denenmiş, ama **bayat** (eski motorun 18 maçı, hiç yenilenmemiş) ve **kaba** (3 boşluk dilimi × 13
mesafe, değerler 0.20–0.33 arası neredeyse düz). Kapatılma gerekçesi kayıtlı değil. Bu plan aynı fikri yenilenen, güven ölçülü ve
doğrulamalı biçimde kurar.

## İki tür bilgi
**A. Sonuç bilgisi ("bu durumda bu hamle N saniye sonra neye yol açar").** Takımın kendi oyunundan ölçülen değer. Oyuncunun
fotoğrafının göremediği zaman boyutunu verir.

**B. Kalibrasyon bilgisi ("benim tahminim motorun gerçeğine göre ne kadar yanlış").** Oyuncunun kafasındaki modeller (pas
olasılığı, ikili, tutma, varış) motorun gerçek kurallarından sapıyor olabilir. Tahmin → gerçek eğrisi ölçülür; kafa tahminini
bu eğriyle düzeltir. Bu, oyuncunun "motoru tanıması".

## Nerelerde kullanılır (aday listesi, önceliğe göre)
Her satır: soru · bugün nasıl tahmin ediliyor · tablo hücresi · nerede okunur · kanıt · oturum.

### A1. Tutmanın / kurmanın değeri  — ÖNCELİK 1
- Soru: topu bu durumda tutarsam N sn içinde gol (ve kayıp → rakibe gol) olasılığı.
- Bugün: sahiplik haritası (anlık), kayıp bedeli V0.
- Hücre: kaleye uzaklık × topun önündeki rakip sayısı × (sahiplik süresi dilimi).
- Okunur: kayıp bedelinin tabanı (`kafa2_tarti`), tut/pas seçeneklerinin değeri.
- Kanıt: 40 br'de model 0.03–0.05, gerçek %18–27; Liman şutlarının ~%40'ı 30+ br.
- Oturum: hücum.

### A2. Kontranın değeri ve zaman duyarlılığı  — ÖNCELİK 1
- Soru: top kazanıldı; t sn içinde kutuya varılırsa / k savunmacı yetişirse gol olasılığı.
- Bugün: hız kararında hedef hücrenin `Sy`'si, en yakın savunmacının varışı.
- Hücre: kaleye uzaklık × topun önündeki rakip sayısı × kazanımdan bu yana geçen süre.
- Okunur: `sprint_karar3` (hiz2) kontra payı; savunmada geri koşunun aciliyeti, oyalama.
- Kanıt: <5 sn %89, 5–8 sn %29, 8+ sn %12; önde 0/1/2/3 savunmacı → %73/%50/%33/%0. Kontra deparı %2.
- Oturum: savunma (hız), hücum (kontra koşusu seçimi).

### A3. Kaybın bedeli (topu burada kaybedersem)
- Soru: topu bu durumda kaybedersem rakibin N sn içinde gol olasılığı.
- Bugün: tek bir risk tahmini.
- Hücre: kendi kalemize uzaklık × arkamızda kalan savunmacı sayısı.
- Okunur: kendi yarı sahamızda pas/sürüş riski; kendi ceza alanından çıkış (bugün yalnız kenardan/tut üretiliyor, ön süzgeç `s>0`).
- Oturum: hücum.

### A4. Presin değeri
- Soru: bu durumda pres yaparsam top kazanma olasılığı ve arkada açılan boşluğun bedeli.
- Bugün: `pres_kazanc` (deney), presin pozisyon kuralları.
- Hücre: taşıyıcının kalemize uzaklığı × presçi sayısı × presçinin varış süresi.
- Okunur: `pres_sec`, S7 parçaları (kim çıkar, kim bekler).
- Oturum: savunma.

### A5. Kuyu süresinin değeri
- Soru: Kuyu süremi şimdi harcamak ile şut anına saklamak arasındaki fark.
- Bugün: süre yalnız kural (`kuyu.rs`), markaj yeri süreyi tartmıyor.
- Hücre: taşıyıcının kaleye uzaklığı × kalan Kuyu süresi.
- Okunur: `kuyu_girer`, S7j/S7k markaj yeri.
- Kanıt: markajcı Kuyu'da süresini harcayıp şut anında çıkıyor (2 gol); çıkış 130 → 387 tik (analiz/KUYU-MARKAJ.md).
- Oturum: savunma.

### A6. Enerjinin değeri
- Soru: barın şu anki birimi, önümüzdeki saniyelerde ne kadar işe yarar (kontra, geri dönüş, pres).
- Bugün: `HIZ2_BAR` sabit katsayı, dışbükey.
- Hücre: rol × oyun evresi (biz topta / rakip topta / geçiş) × bar dilimi.
- Okunur: `sprint_karar3` bedeli.
- Kanıt: bar kıt; hiz2'de oyuncu-devrelerin %20–25'i 0.3 altına iniyor.
- Oturum: savunma (hız kararı ortak).

### A7. İkinci top / sekme
- Soru: şut ya da blok sonrası top nereye düşer.
- Hücre: şut yeri × engel.
- Okunur: kutu önü yerleşimi (dönen topa hazır olma), Bekçi çıkışı.
- Oturum: ikisi.

### B1. Pas olasılığının kalibrasyonu
- Soru: `ucus_formul`'ün P (arkadaşa / Kuyu'ya / rakibe) tahmini gerçekte ne kadar tutuyor.
- Ölçüm: karar anındaki tahmin ile pasın gerçek sonucu. Karar izine tahmini P eklenmesi gerekir.
- Okunur: kafanın pas değeri (düzeltme eğrisi).
- Oturum: hücum.

### B2. İkili ve tutma kalibrasyonu (`duel_p`, `keep_p`)
- Soru: "bu ikiliyi kazanırım / topu tutarım" tahmini gerçekte ne kadar tutuyor.
- Okunur: tut, çalım, pres kararları.
- Oturum: ikisi.

### B3. Rakibin tepkisi (karar gecikmesi)
- Soru: rakip bir hamleye ne kadar sürede ve nasıl tepki veriyor.
- Bugün: `predictWorld` varsayımları; `gec` tablosu yalnız fizik, karar gecikmesi yok.
- Okunur: hayali dünyalar (kafa2_dunya).
- Oturum: ikisi.

### B4. Şut tablolarının eskimesi
- Durum: `imkan-sut` parmak izli eskime denetimi zaten var (`sina/tablo.sh`).
- Plan: aynı denetimi bütün bilgi tablolarına genellemek.

**Kalibrasyonda kural:** önce ölç, tahmin ile gerçek arasında anlamlı fark çıkan modelde tablo kur. Fark yoksa tablo gereksiz.

## Mimari

### 1. Dosya biçimi
`motor/veri/bilgi/<ad>.json`. Kütüphaneye gömülür (`include_str`, `veri.rs` gibi), maç içinde değişmez; belirlenimcilik korunur.
İçindekiler:
- ad, sürüm, üretim tarihi;
- üretildiği motorun **kural izi** (bkz. 5) ve bayrak seti;
- maç sayısı, tohumlar, taktik çeşitliliği;
- eksenler (hücre sınırları);
- her hücre için değer ve örnek sayısı `n`.

### 2. Okuma ve güven
Kafa tabloyu doğrudan almaz, kendi modeliyle harmanlar:
`değer = (n · tablo + k · model) / (n + k)`.
- Az örnekli hücrede model, çok örnekli hücrede ölçüm baskın olur. Hayal ile bilgi birbirini tamamlar.
- `k` her tablo için ayrı ayarlanır.
- Her okuyucu bir deney bayrağının arkasında başlar (`ALAN_BILGI_<AD>_IZ`).

### 3. Üretim
`sina/bilgi.sh --uret <ad>`:
- N maç oynatır (4 süreç), tohum ve taktik çeşitliliğiyle;
- olayları toplar, hücrelere yazar, az örnekli hücreleri komşulara yumuşatır.
- Üretim betikleri oturumların; `analiz/py/kontra.py` analizcinin ölçüm tanımı (örnek).

### 4. Seçenek dallanması: maç dışında hayal
Sonuç bilgisinin zor kısmı şu: oyuncu bir durumda hep şut atıyorsa, orada "tutsaydı ne olurdu" maç kayıtlarından ölçülemez.
Çözüm `dal` kipinin genişletilmesi:
- üretim maçında bir karar anında maçı kopyala;
- her seçeneği (şut / tut / pas / sür) **zorla**, N sn oynat, sonucu yaz.
- Bu, oyuncunun hamlelerini hayal etmesinin tam karşılığı; yalnızca maç dışında ve önceden yapılıyor.

Gereken motor işi: kafada "bu tik şu seçeneği seç" kancası (deney izinde). `dal` kipi kopyalamayı ve kollara ayırmayı zaten yapıyor.

Maliyet tahmini: 1000 karar anı × 4 seçenek × 300 tik ≈ 1.2 milyon tik ≈ 130 maç. Bu makinede (4 çekirdek, ~3 dk/maç analiz
çıktısıyla) bir gece.

### 5. Eskime denetimi
`sina/bilgi.sh` her tablo için iki şeye bakar:
- **Kural izi:** tablonun ölçtüğü kurallar değişti mi? Örnek: şut fiziği, Bekçi kuralı, Kuyu süresi.
- **Davranış izi:** başvuru maçlarının parmak izi değişti mi?

Değiştiyse yalnızca **"eski"** der; kendiliğinden yeniden üretmez. Üretim elle ya da gece görevinde yapılır.

### 6. Sabit nokta
Tablo davranışı, davranış da tabloyu değiştirir. Döngü şöyle oturur:
- yeni tablo eskisiyle harmanlanır (yeni = %50 eski + %50 ölçüm);
- her turda en büyük hücre değişimi raporlanır;
- değişim küçülene dek (ör. < 0.02) yinelenir.

### 7. Doğrulama
Ölçüt skor değil, davranış:
- `dal` ile aynı anda bayrak açık/kapalı karşılaştırılır;
- davranış ölçüleri (davranis.py, hız tablosu, uzak şut oranı, kontra deparı) bakılır;
- tablo üretiminde kullanılmamış tohumlar ve an an izleme kullanılır.

### 8. Tetikleme
- Elle: `sina/bilgi.sh --uret …`.
- Gece görevi: bütün tabloların eskime denetimi; eskiyse üretim ve doğrulama; sonuç raporu.
- Her motor değişikliğinde çalışmaz.

## Evreler ve iş bölümü
Analizci tablo üretmez, okuyucu yazmaz: ölçüm tanımı verir, sonucu dal ile ve maç izleyerek doğrular, raporlar.
| evre | iş | kim |
|---|---|---|
| 0 | Ortak altyapı: dosya biçimi (sürüm, kural izi, bayraklar, maç sayısı, hücre başına n), `sina/bilgi.sh` (üret / denetle), eskime izi, güvenli okuma (n/k harmanlama), sabit nokta yinelemesi | hücum (sahdeger üreticisini genişletir) |
| 1a | A1 tutma değeri (sahdeger): biçimi ortak altyapıya taşı | hücum |
| 1b | A2 kontra değeri: tablo üretimi (tanım: analiz/py/kontra.py, analiz/ZAMAN-DEGERI.md) | savunma |
| 2 | B1 pas olasılığı kalibrasyonu: karar izine tahmini P, tahmin → gerçek eğrisi | hücum |
| 2 | B2 ikili / tutma kalibrasyonu (duel_p, keep_p) | savunma |
| 3 | Seçenek dallanması: kafaya "bu tik şu seçeneği seç" kancası (deney izinde) + dal kipiyle seçenek değerleri | hücum (kafa) |
| 4 | A3 kayıp bedeli | hücum |
| 4 | A4 pres, A5 Kuyu (S7m ile), A6 enerji, A7 ikinci top | savunma |
| 5 | Gece görevi: denetim → üretim → doğrulama → rapor | analizci kurar (kullanıcı onayıyla), üretim betikleri oturumların |
| her evre | Doğrulama: dal açık/kapalı, davranış ölçüleri, an an izleme; sapma raporu | analizci |

## Riskler
- **Politika yanlılığı:** tablo bugünkü davranışın sonucunu ölçer. Davranış kötüyse değer de yanlış olur. Çare: dallanma (evre 3) ve sabit nokta.
- **Ezber:** tek tohum ya da tek taktikle ölçülen tablo o maça uyar. Çare: tohum ve taktik çeşitliliği, takımdan bağımsız hücreler.
- **Gizli kural:** tablo "neden" sorusunu cevaplamaz. Çare: her tablo bir modelin yanında durur, onun yerine geçmez. Model ile tablo arasındaki büyük fark bir analiz konusu olarak raporlanır.
- **Sessiz eskime:** çare eskime izi; eski tablo uyarı verir.

## İlke (7 Kasım, kullanıcı onayıyla): tablo önce teşhis aracıdır
Bugünkü deneyler: tutma tablosu kafanın tutma değerinin 15–40 kat düşük olduğunu, şutun bedelindeki çift cezayı ve kafanın değer
ölçeğinin kaleden uzaklaştıkça 11 kata kadar düşük olduğunu gösterdi. Kontra tablosu ölçümdeki hayatta kalma seçilimini gösterdi.
keep_p kalibrasyonu, kafanın hamle süresini bilmediğini gösterdi (kural uyuşmazlığı).
Sıra her tabloda şu:
1. Tablo ile kafanın tahmini arasındaki farkı ölç (seçilimsiz: secdal / hizdal kolları ya da sansürlü ölçüm).
2. Fark büyükse önce mekanizmayı bul: kural uyuşmazlığı, hesap hatası (çift ceza gibi), ölçek mi?
3. Kural ya da hesap hatasıysa modelde düzelt (bayraklı); tablo kurma.
4. Tablo yalnız modelin hiç göremediği yerde kalır (zamanın değeri: yerleşik hücumda sabır, kontrada acele) ve orada da
   modelin yerine geçmez, onu tamamlar (ölçek / harmanlama; taban (max) değil).
5. Doğrulama skorla değil davranışla: maçı izleyerek, yan etkiler için ölçütlerle (uzak şut, kutu önü dolanma, top kaybı,
   donma, hattın arkası).
