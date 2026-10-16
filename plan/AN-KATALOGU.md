# An kataloğu

Kullanıcının maçta gördüğü sorunlu anlar. Amaç tek tek yama değil: anlar biriktikçe ortak ilkeleri bulup toplu çözüm üretmek.
Her an: kimlik (Anlar ekranı `a-…` ya da tohum + saat), kullanıcının sözü, hangi ilkeye bağlandığı. Motor sürümü yazılır; o sürümden eski anlar yeni motorda başka oynar.

## Genel kural · Kararlar görev yazısını koşul olarak okumaz
Görev ataması kalır (savunmada kim kime basar, kim markaj, kim arkada: bunu atmak savunmayı dağıtır) ve görev kartındaki yazı kalır. Ama hiçbir karar görev **yazısına** bakmaz ("görevi 'koşu' içeriyorsa…"); sahada görünen duruma bakar (hız, yön, konum, kimi tuttuğu). Örnek: Şimşek kaleye koşarken görevi "boşluk" yazdığı için buluşma pası üretilmedi (D1).
Bugün motorda görev yazısını okuyan ~17 yer var (match.js 8, shape.js 8, decide.js 1). İlkeler çözülürken bu kurala göre temizlenir. Kullanıcıyla 16 Ekim'de kararlaştırıldı.

## İlke A · Baskının yönü: presçi, top sahibinin en tehlikeli seçeneklerini gövdesiyle kapatarak yaklaşır
Topa düz koşmak yerine, top sahibiyle onun en tehlikeli seçenekleri (kale, en boş arkadaş) arasına doğru yaklaşmak. İkinci presçi topa değil, kalan en tehlikeli pas yoluna.
Tartı olasılık değil, **olasılık × bize zararı** (varış yerinin hücum değeri − şimdiki değer; negatifse 0). Kullanıcı: "taşıyıcı arkaya pası da ihtimal olarak görüyorsa sorun yaratmaz mı" — geri pas olası ama zararsız; presçi onu bilerek açık bırakır, top sahibini geriye zorlar. Bu yüzden A, C'ye (değerlerin doğruluğu) bağlı.

- **A1 · Deniz–Ece (Liman–Demir Blok, 01:40, motor 6ad507e, eski an a-52-0-2845-kontra; yeni motorda yok, ekran görüntüsünden).** Şimşek ve Gökay ikisi de Deniz'in üstüne düz koşuyor. Kullanıcı: "biri baskı yaparken diğeri sağ köşede durup pas açısını kapatarak baskı yapmalı." Ayrıca Deniz'in kararı: sür 0.198 / pas → Ece 0.192, oysa Ece çok boş; pas belirgin daha değerli olmalı (bkz. İlke C).
- **A2 · Yelkovan–Tuna (Liman–Saat Kulesi, 01:55, motor 82a0ff0, `a-129-2-3473-iki-topta`).** Yelkovan Tuna'ya arkasından baskıya gidiyor. Kullanıcı: "prese gitmesi sorun değil ancak arkasındaki Kaan'ı da düşünerek, ayrıca Tuna'nın kaleye bakış açısını da sınırlamaya çalışarak durmalı."

## İlke C · Değer: çok boş bir arkadaşa pas, sürmekten belirgin değerli
- **C1 · Deniz (A1 ile aynı an).** Sürme değeri, "sürdükten sonraki en iyi hamle"den geliyor (`ALAN_SUR_SONRA_ESKI`); sürünce pasın hâlâ açık kalacağı varsayılıyor olabilir. Doğrulanmadı.

## İlke D · Koşucunun önüne ara pas: koşunun yönünde, kaleye doğru boşluğa
Pas hedefleri bugün üç kaynaktan: arkadaşın ayağı, "boşluğa pas" taraması (arkadaş savunmadan önce varmalı), buluşma noktası (yalnız görevi "koşu" olana). Koşan arkadaşın **önüne, kaleye doğru** atılan ara pas sistematik olarak ya üretilmiyor ya çok düşük değerleniyor.

- **D1 · Rüzgâr–Şimşek (Liman–Demir Blok, 03:50, motor 82a0ff0, `a-52-0-6951-kontra`).** Kullanıcı: "Şimşek gayet müsait pozisyondayken önüne ara pası hiç düşünmüyor bile, en sonunda atağın tehlikesini yiyor." Döküm (tik 6903, 19 seçenek): Şimşek (37.9, 46.0) kaleye doğru koşuyor (hız −0.32, −0.17). Ona pas adayları var ama hedefleri koşunun **yanında ya da gerisinde**: önüne → (36.1, 51.7) 0.089, pas → ayağa 0.010; koşunun önünde, kaleye doğru (≈ 28–30, 42) hiç aday yok. Seçilen sür 0.284. Görevi o anda "boşluk" (6916'da "kontra koşusu") olduğu için buluşma pası da üretilmiyor.
- Bağlı eski şikâyetler: Yıldırım'ın arkasına düşen pas; Volkan–Kartal kale ağzı pası (`ALAN_AGIZ_PAS_ESKI` yamasıyla kapatılmıştı); "Koşucunun arkasına pas" Anlar türü (9 maçta 23 kez).

## Daha önce konuşulup aynı ilkelere bağlananlar (16 Ekim, ekran görüntüleri)
- Volkan önü boş sürerken Deniz yavaşlatmalıydı (sahte pres, yolu kapatma) → A'nın "yardım yoksa uzaktan yavaşlat" hâli.
- Kaan ve Mert ikisi de Volkan'a, Yıldırım boş → A (ikinci presçi pas yoluna).
- Savunma hattı baskısız topta çok yüksek; hücumcular arkadaki boşluğu kullanmıyor → (ayrı ilke, B: hattın yüksekliği topa baskıya göre).
