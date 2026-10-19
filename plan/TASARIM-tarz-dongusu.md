# Tasarım planı · Topa sahip olmanın getirisi ve tarz döngüsü

> Durum: fikir aşaması, kod yok. 19 Ekim 2026. Kardeş belge: `TASARIM-arka-hat.md`. Ölçümler: `lab/TAKTIK-OLCUMU-1.md`, `lab/TAKTIK-OLCUMU-2.md`.

## Hedef: her tarzın bir panzehiri olsun

Bugün tarzlar arasında bir sıralama var (Dikine hiçbir tarza kaybetmiyor), cevap ilişkisi yok. İstenen döngü:

| | yener | yenilir |
|---|---|---|
| **Pres** | topa sahip takımı | direkt oyuna (arkada boşluk) |
| **Topa sahip** | sabırsız ya da hattı koruyan bloğu | prese |
| **Direkt oyun** | prese ve yüksek hatta | derin bloğa |
| **Derin blok** | direkt oyunu | topa sahip takıma (sabır, kayma) |

Bu dört ilişki kurulursa tek bir baskın tarz kalmaz. Menajer rakibe ve maçın gidişine göre seçmek zorunda kalır.

Bugünkü durum (ölçüm 2):
- Pres → topa sahip: **var.**
- Direkt → pres: **yarım.**
- Derin blok → direkt: **yok.**
- Topa sahip → derin blok: **yok.**

## Bugün topa sahip olmak neden kısır

Ölçümlerde Topa sahip tarzı:
- topun %57'sine sahip, ama top ondayken oyunun yalnız %38'i rakip yarısında (en düşük);
- en az şutu atıyor; varsayılana karşı +0,08 xG (gürültü);
- prese karşı çöküyor: xG 0,18, kendi yarısında kayıp %27.

Okunan sebepler (doğrulanmadı):
1. **Savunma yıpranmıyor.** Kondisyon bilerek yok; topa sahip olmanın futboldaki yorma ve dikkat dağıtma getirisi burada yok.
2. **Savunmanın kayması bedelsiz.** Top taraf değiştirdiğinde blok neredeyse kusursuz kayıyorsa sabır boşluk açmaz.
3. **Arkada boşluk yok.** Ofsayt olmadığı için arkadakiler hep en derin rakibin gerisinde; kilit açmanın klasik bitişi (arkaya koşu) yok. Bkz. `TASARIM-arka-hat.md`.
4. **Pres kırma zayıf.** Basan en çok 2 kişi; geride kalan 4'e karşı üstünlük olmalı. Ama üçüncü adam, duvar pası, Bekçi'nin oyun kurmaya katılması ya yok ya da yeterince değerli tartılmıyor.

## Fikirler

### 1 · Kayma gecikmesi (kondisyonsuz yıpratma)
- Top hızlı taraf değiştirince savunmacılar yeni yerlerine **Tepki ve Yer alma kadar geç** kayar. O arada bir koridor açılır.
- Uzun, sabırlı topa sahip olma böylece ödüllenir; tempo talimatı (hızlı çevirme) anlam kazanır.
- Ölçümde etkisiz çıkan **Yer alma**'ya iş çıkar.
- Kondisyon kuralıyla çelişmez: bedel yorgunluk değil, okuma ve tepki.

### 2 · Sabırsızlanan savunma (talimat)
- Savunma talimatı: **"bekle" / "bas"** (ya da kayan "sabır").
- Uzun süre topu kazanamayan blok, sabırsız ayardaysa öne çıkıp basmaya başlar. Basınca arkası ve hatlar arası açılır.
- Sabırlı hücum sabırsız savunmayı cezalandırır; sabırlı savunma ise topa sahip takımı kısır bırakır. Kararı menajer verir.
- Yapay zekâ menajer skora göre değiştirir (gerideyken sabırsızlanır).

### 3 · Arka hat ikilemiyle bağlantı
- Forvet derinde bekler, adamını takip eden savunmayı geri iter, hatlar arası açılır, topa sahip takım oradan girer.
- Savunma hattı korursa arkaya uzun top açılır.
- Topa sahip takımın derin bloğa karşı bir seçimi olur: "hatlar arasına mı, arkaya mı?"

### 4 · Pres kırma araçları
- **Bekçi oyun kurmaya katılır** (planda Bekçi pası var): basan 2 kişiye karşı sayıca üstünlük.
- **Üçüncü adam:** A'dan B'ye pas, B'den koşan C'ye. Değer tartısında, presçiyi geride bırakan ikinci pası ödüllendirmek.
- **Duvar pası:** verip koşma; presçinin arkasına geçmek.
- Bunlar olursa pres riskli hale gelir: kırılan pres arkasında boşluk bırakır, "pres topa sahip takımı yener" ilişkisinin de panzehiri olur.

### 5 · Bölgede sayıca üstünlük
- Talimatlarda `doldur` ve `bindir` var. Bunların bir bölgede gerçekten fazla adam yaratıp yaratmadığı ölçülmeli.
- Topa sahip olmanın asıl silahı: topun olduğu bölgede bir kişi fazla olmak, sonra hızla öbür tarafa çevirmek (1 ile birleşir).

## Özelliklere bağları

| fikir | değer kazanan özellik |
|---|---|
| Kayma gecikmesi | Tepki, Yer alma (savunmada); Pas, Görüş (hızlı çevirmede) |
| Pres kırma | Bekçi'nin Pas'ı ve Kontrol'ü, Sürme, Soğukkanlılık |
| Üçüncü adam, duvar | Görüş, Hız (koşucu) |
| Sabırsız savunma | Tepki (öne çıkarken arkayı okumak) |

Bu, özellik ölçümünde etkisiz ya da tek yönlü çıkan özelliklerin taktiğe bağlı değer kazanmasını sağlar. Planın "özelliğin değeri taktiğe göre değişmeli" şartı da böyle karşılanır.

## Kabul (ölçüm, ileride)

1. Dört ilişkinin her biri, eşleşme başına en az 20 maçta xG ve şut farkıyla görünür.
2. Hiçbir tarz bütün tarzlara karşı kazanmaz (her satırda en az bir eksi).
3. Topa sahip tarzının derin bloğa karşı rakip yarısında geçirdiği pay ve xG artar.
4. Mekanik izler ayrıca ölçülür: kayma gecikmesinin açtığı koridor, kırılan presin arkasındaki boşluk.

## Açık sorular

- Kayma gecikmesi sadece okumaya mı bağlı olsun, yoksa topun yön değiştirme hızına (pas hızı) da mı?
- "Sabırsız savunma" ayrı talimat mı, yoksa mevcut pres şiddeti ya da tetiklerle mi birleşsin?
- Bekçi'nin oyun kurmaya katılması riskli olmalı mı (Bekçi'nin topu kaybetmesi)?
- Tarz döngüsünü yapay zekâ menajerler maç içinde nasıl okur: rakibin tarzını kaç dakikada tanır?
