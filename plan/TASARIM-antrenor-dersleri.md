# Tasarım planı · Antrenör dersleri (oyuncuya tekrar üzerinden öğretmek)

> Durum: fikir aşaması, kod yok. 16 Ekim 2026. Fikir menajerin; bu not onu motora nasıl oturtacağımızı yazar.

## Fikir

Menajer maçın tekrarını izlerken durdurur, bir oyuncuyu seçer ve tahtada gösterir: "burada durmalıydın", "bu koşuyu yapmalıydın", "bu hattı kapatmalıydın". Söz ya da yazıyla değil, tahta üzerinde sürükleme ve işaretle. Oyuncu bunu öğrenir, benzer durumlarda uygular. Antrenmanlar ve hazırlık maçları, bu dersleri öğretmenin ve pekiştirmenin yeri olur.

## İlke: ders bir kural değil, bir değerdir

Motor hareketi kuralla değil değerle seçiyor; ders de buna uymalı.
- Ders yeni bir "şu durumda şunu yap" kuralı olmaz. Oyuncunun zaten yaptığı tartıya giren bir terim olur: benzer durumda, antrenörün gösterdiği yere ek değer.
- Ders oyuncunun kendi değer hesabını ezmez. Oyuncu dersi ne kadar iyi kavradıysa ve ne kadar disiplinliyse o kadar uyar.
- Yanlış ders de öğretilebilir. Oyuncu onu da uygular; menajerin bilgisi bu şekilde gerçekten fark yaratır.

## Ders türleri

| Tür | Tahtada | Motorda nereye girer | Zorluk |
|---|---|---|---|
| Konum | Oyuncunun hayaletini sürükle: "burada dur" | Takım şekli: aday noktaların tartısı (`shape.js` boşluk/destek ve savunma yerleri) | Kolay |
| Koşu | Ok çiz: "bu koşuyu yap" | Takım şekli: koşu adayları ve koşu değeri | Kolay–orta |
| Hat kapatma | Bir pas hattını işaretle: "bunu kapat" | Savunma: pres ve jokey yön seçimi (tehlike ağırlıkları) | Orta |
| Seçim | Bir seçeneği işaretle: "burada Mert'e değil Cansu'ya" | Top sahibinin kararı (`decide.js` seçenek değerleri) | Zor |

İlk sürüm yalnız **konum** dersi.

## Dersin kaydı: durum, mutlak nokta değil

Bir ders saha üzerindeki mutlak bir noktayı değil, o anki durumu ve oyuncunun o duruma göre yerini saklar. Böylece "kendi köşemizde baskı altındaki bekin yanına gel" dersi sahanın öbür kanadında da anlam taşır.

Durum anahtarı (ilk sürüm, küçük tutulur):
- **Evre:** hücum, savunma, geçiş; top havada mı, kimde.
- **Top:** yeri (takımın kendi yönüne göre derinlik ve yan), sahibi ya da alacak oyuncunun rolü.
- **Baskı:** topa en yakın rakibin uzaklığı ya da sahibinin topu tutma ihtimali (`keepP`).
- **Oyuncunun yeri:** topa göre (göreli vektör), kendi ev yerine göre.

Dersin hedefi de göreli saklanır: topa (ya da top sahibine) göre vektör, gerekirse duvara kenetli.

Kayıt örneği:

```json
{ "oyuncu": "Mert", "tur": "konum", "t": 4380, "mac": "909623",
  "durum": { "evre": "hucum-havada", "topDerin": 0.12, "topYan": 0.85, "sahipRol": "Bek", "baski": 4.8 },
  "hedef": { "topaGore": [8, -6] }, "agirlik": 1 }
```

## Motorda uygulama

- Oyuncu yer seçerken (`position()` içindeki aday tartısı) kendi derslerine bakar.
- Her ders için durum benzerliği hesaplanır: anahtarların ağırlıklı uzaklığı, yumuşak çekirdekle (benzerlik 0–1).
- Benzerlik yüksekse dersin hedefi aday listesine eklenir ve şu terim değere katılır:

  `ders değeri = benzerlik × öğrenme düzeyi × ders katsayısı × yakınlık(aday, hedef)`

- Ev bağı (leash) dersin hedefine doğru gevşer: menajerin elle verdiği duruş yeri gibi.
- Ders sayısı bir oyuncu için yüzlerle sınırlı tutulur. Her tartıda tarama ucuzdur. Gerekirse durum anahtarına göre kovalara bölünür.

## Öğrenme

- **Kavrama hızı:** Okuma ve Yerleşim. Yüksek olan dersi daha az tekrarla öğrenir.
- **Genelleme:** Okuma yüksekse benzerlik çekirdeği geniştir; ders benzer ama farklı durumlara da taşınır. Düşükse yalnız neredeyse aynı durumlarda uygulanır.
- **Disiplin / serbestlik:** Oyuncu kendi değer hesabı dersten açıkça daha iyi bir yer gördüğünde dersi delip delmeyeceği. Talimattaki "serbestlik" ile birleştirilebilir.
- **Tekrar ve unutma:** Her antrenman ya da hazırlık maçında ders tekrarlanırsa öğrenme düzeyi artar. Uzun süre tekrar edilmeyen ders zayıflar.
- **Çelişen dersler:** Aynı durumda farklı hedefler varsa daha yeni ve daha çok tekrarlanan baskın olur. Arayüz çelişkiyi menajere gösterir.

## Antrenman ve hazırlık maçları

- **Antrenman:** Seçilen dersler kurulmuş sahnelerde (an altyapısı) tekrarlanır. Oyuncu dersi uygulayabildikçe öğrenme düzeyi artar.
- **Hazırlık maçı:** Gerçek maç akışında dersin ne sıklıkla ve ne kadar doğru uygulandığı ölçülür. Menajer yeni dersler çıkarır.
- **Haftalık yük:** Bir oyuncuya bir haftada öğretilebilecek ders sayısının sınırı olabilir; antrenman bir kaynak hâline gelir.

## Arayüz akışı

1. Tekrar ekranında durdur, oyuncuya dokun.
2. Oyuncunun hayaleti görünür. Sürükleyip doğru yere bırak (koşu için ok çiz).
3. "Ders olarak kaydet". Durum anahtarı otomatik çıkarılır, menajere kısaca gösterilir: "Hücumda, top kendi köşemizde, bek baskı altındayken."
4. **Geri bildirim:** Sistem aynı maçta ve kayıtlı anlarda benzer durumları bulur ve "Mert bu derste artık şuraya gidecek" diye gösterir. Menajer her birini onaylar ya da düzeltir; düzeltme yeni ders olur. Bu döngü hem oyuncunun neyi anladığını gösterir hem yanlış genellemeyi yakalar.
5. Oyuncu kartında ders listesi: öğrenme düzeyi, son tekrar, kaç kez uygulandı.

## Belirlenimcilik ve hız

- Dersler maçın girdisidir (kadro ve taktik gibi). Aynı tohum ve aynı dersler aynı maçı verir; tekrar ve anlar bozulmaz.
- Ders değiştiğinde maç yeniden hesaplanır (taktik değişikliği gibi).
- Tarama, oyuncu başına ders sayısıyla sınırlı. Kafadaki maçlarda (lite) dersler uygulanır mı, yoksa yalnız gerçek maçta mı? Hız ölçülerek karar verilir.

## Riskler

- **Aşırı ezber:** Tek örnekten yanlış genelleme. İlacı: iyi bir benzerlik ölçüsü ve geri bildirim döngüsü.
- **Ölçek:** Ders terimi, değer birimiyle yazılmazsa (bugünkü ev bağı sorunu gibi) her şeyi ezer ya da hiçbir şeyi değiştirmez. Ders katsayısı değer birimiyle kalibre edilmeli.
- **Kapsam:** Seçim dersleri top sahibinin karar koduna dokunur; en sona bırakılmalı.
- **Arayüz yükü:** iPad'de hassas sürükleme ve çizim. Tahta bileşeni (duruş tahtası) yeniden kullanılabilir.

## İlk sürüm (MVP)

1. Yalnız konum dersi, hücumda ve savunmada.
2. Durum anahtarı: evre, top derinliği ve yanı, sahibin rolü, baskı.
3. `shape.js` aday tartısına ders terimi ve aday ekleme; ev bağının gevşemesi.
4. Tekrar ekranında hayalet sürükleme ve kayıt (kalıcı depolama: maç kaydının yanına).
5. Geri bildirim: aynı maçta benzer anları bulup oyuncunun yeni yerini gösterme.

**İlk deneme:** 909623, 00:11–00:15. Mert'i baskı altındaki Deniz'in yanına sürükle. Aynı anda ve benzer anlarda (kendi yarımızda baskı altındaki bek) Mert'in yeri ve sonuç nasıl değişiyor, ölçülür.

## Açık sorular

- Öğrenme özelliği ayrı mı olsun (ör. "Öğrenme"), yoksa Okuma ve Yerleşim'den mi türetilsin?
- Dersler oyuncuya mı ait (kulüp değiştirince gider), takıma mı (sisteme), yoksa ikisine birden mi?
- Rakip takımların da dersleri olsun mu (yapay zekâ menajerler kendi derslerini üretir mi)?
- Antrenman bir zaman kaynağı mı (haftalık takvim), yoksa sınırsız mı?
