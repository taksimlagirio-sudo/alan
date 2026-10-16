# ALAN · Design için brif: antrenör dersleri prototipi (ilk sürüm: konum dersi)

> 16 Ekim 2026. Fikrin tamamı: `TASARIM-antrenor-dersleri.md`. Bu brif yalnız **inceleme prototipini** anlatır.
> Prototipi kendi motor kopyan üzerine kur; kullanıcı onda inceleyecek. Siteye girerken motor tarafını Claude Code kendi bayrağı ve ölçümüyle taşır (`CALISMA-DUZENI.md`). Bu yüzden motor değişikliklerini **ayrı ve dar** tut, pakette ayrı bir diff ve kısa bir not olarak ver.

## 0 · Kullanıcı ne görmek istiyor

1. **Ders vermek nasıl hissettiriyor:** Tekrarı durdur, oyuncuya dokun, hayaletini sürükle, kaydet. Dokunmatikte (iPad) rahat mı?
2. **Ders işe yarıyor mu:** Aynı an derssiz ve dersli oynatılınca oyuncu gerçekten gösterilen yere gidiyor mu, sonuç değişiyor mu?
3. **Ders nereye taşınıyor:** Aynı maçta benzer anlarda ders uygulanıyor mu, alakasız anlarda davranış bozulmadan kalıyor mu?

Öğrenme hızı, tekrar, unutma, haftalık yük, koşu / hat / seçim dersleri bu prototipte **yok**.

## 1 · İlk deneme (kabulün ana sahnesi)

Maç 909623 (Efe → Volkan anının maçı), **00:11–00:15**. Deniz (bek) kendi köşemizde baskı altında. Menajer Mert'i Deniz'in yanına sürükler. Beklenen: aynı anda Mert oraya gider; maçtaki benzer anlarda (kendi yarımızda, baskı altındaki bek) de gider.
Saat → tik: 60 tik = 1 sn (7200 tik = 120 sn). 00:11 ≈ tik 660.

## 2 · Ekran akışı (canlı sayfa üzerinde)

1. **Durdur ve dokun.** Maç ya da an oynarken durdur. Bir oyuncuya dokununca bugünkü görev kartı açılır (görev göstergesi korunur); kartta yeni bir **"Ders ver"** düğmesi.
2. **Hayaleti sürükle.** Oyuncunun yarı saydam hayaleti çıkar, parmakla sürüklenir. Bırakınca hedef noktası ve oyuncudan hedefe ince bir çizgi. Hassasiyet için sürüklerken hayalet parmağın biraz üstünde dursun (parmak örtmesin).
3. **Durum özeti.** Kaydetmeden önce motorun çıkardığı durum düz cümleyle gösterilir: "Hücumda · top kendi köşemizde (sağ) · top Deniz'de · baskı: rakip 4,8 br". Menajer okur, kaydeder.
4. **Önizleme (asıl an).** Kaydedince aynı an iki kez oynar: derssiz ve dersli. Sahada ikisi üst üste, oyuncunun iki izi iki renkle (eski yol soluk, yeni yol belirgin). Altta kısa sonuç: "derssiz: top kaybı 00:13 · dersli: Deniz → Mert pası".
5. **Benzer anlar.** Aynı maçta dersin uygulandığı anlar liste hâlinde: saat, benzerlik (0–1), oyuncunun derssiz ve dersli yeri arasındaki fark (br). Birine dokununca o an önizleme gibi açılır. Her biri için **onayla / bu durumda değil** (ikincisi o anı dersin dışında bırakan bir "karşı örnek" olarak kaydedilir; prototipte yalnız listeden düşürmek yeterli).
6. **Oyuncu kartında ders listesi.** Her ders: durum özeti, öğrenme düzeyi, kaç anda uygulandı. Silme.
7. **İnceleme kaydırıcıları (yalnız prototipte, ayrı bir küçük panelde):** öğrenme düzeyi (0–1), ders katsayısı, benzerlik genişliği. Kullanıcı ölçeği elle bulsun; Claude Code sonra ölçümle oturtur.

Kayıt: prototipte tarayıcıda (localStorage) yeter; maç tohumu ve oyuncu adıyla.

## 3 · Dersin kaydı

```json
{ "id": "d1", "oyuncu": "Mert", "tur": "konum", "mac": "909623", "t": 660,
  "durum": { "evre": "hucum", "topHavada": false, "topDerin": 0.12, "topYan": 0.85,
             "sahipSlot": { "d": 0.15, "l": 0.9 }, "baski": 4.8,
             "kendi": [-3, 9] },
  "hedef": { "topaGore": [6, -4] }, "ogrenme": 1, "aynali": true }
```

- **Hepsi takımın yönüne göre:** `topDerin` 0 = kendi kale çizgimiz, 1 = rakibin. Tersi yöne oynayan takım için x çevrilir. Vektörler de (`kendi`, `topaGore`) takımın hücum yönü +x olacak şekilde saklanır.
- **`topYan`:** 0 bir taç çizgisi, 1 öbürü. **`aynali: true`** ise ders sahanın öbür kanadında da geçerli: durum karşılaştırılırken `topYan` 0,5 etrafında aynalanır, hedefin y'si de aynalanır.
- **`sahipSlot`:** rol yok (rolsüz sistem); top sahibinin dizilişteki yeri (`slot.d`, `slot.l`). Top havadaysa alacak oyuncunun.
- **`baski`:** top sahibine en yakın rakibin uzaklığı (br). `keepP` daha doğru ama pahalı; prototipte uzaklık yeter.
- **`kendi`:** dersi alan oyuncunun topa göre yeri, kayıt anında.
- **`hedef.topaGore`:** gösterilen yerin topa göre vektörü. Uygularken o anki topun yerine eklenir, saha içine kırpılır.

## 4 · Benzerlik

- `evre` farklıysa 0. Top havada / ayakta farkı ağırlıkla girer (aynı değilse benzerlik yarıya iner).
- Diğerleri ağırlıklı uzaklık: `d² = (ΔtopDerin/0,08)² + (ΔtopYan/0,12)² + (ΔsahipSlot/0,25)² + (Δbaski/3)² + (|Δkendi|/8)²`
- `benzerlik = exp(-d² / (2·σ²))`, `σ = 0,6 + Görüş × 0,04` (Görüş 10 → 1,0; 20 → 1,4). Görüş yüksek oyuncu dersi daha uzak durumlara taşır.
- **Eşik:** benzerlik < 0,25 ise ders hiç girmez (alakasız anlarda davranış birebir aynı kalsın).
- Bir oyuncunun birden çok dersi varsa yalnız en benzeri uygulanır (çelişen dersler prototipte yok).

> Not: tasarım notu kavrama için "Okuma ve Yerleşim" diyor; Okuma artık Tepki / Görüş / Yer alma olarak bölündü, Yerleşim yalnız Bekçi'nin. Prototipte **genelleme = Görüş**, **kavrama = öğrenme kaydırıcısı** (Yer alma'ya bağlamayı sonra konuşuruz).

## 5 · Motorda (dar tut, tek bayrak)

Hepsi `self.ALAN_DERS` bayrağının arkasında. **Bayrak kapalıyken ya da ders yokken maç bit bit aynı kalmalı** (kabulün ilk maddesi). Zar (`m.r()`) dersler için hiç çağrılmasın; yoksa belirlenimcilik bozulur ve aynı tohum başka maç olur.

**Hücumda** (`shape.js` `position()`, hücum aday tartısı, bugün satır ~240–256):
- Oyuncunun uygulanan dersi varsa hedef noktası aday listesine eklenir (`cands.push({ x, y, ders: 1 })`).
- Hem ilk elemede (`pre`) hem asıl tartıda (`v`) her adaya şu terim eklenir:
  `ders = benzerlik × ogrenme × K × exp(-|aday − hedef|² / (2·4²)) × (1,5 − serbestlik)`
  Serbestlik talimatı disiplin yerine geçer: serbest oyuncu dersi daha kolay deler.
- Ev bağı hedefe doğru gevşer: `lsh` ev yeri (`bs`) yerine ev ile hedefin arasındaki noktadan ölçülür, `benzerlik × ogrenme` oranında hedefe yakın.
- Dikkat: koşu planı (`p.otx`) top el değiştirene ya da `every` tik dolana kadar yeniden tartılmıyor. Ders ancak bir sonraki tartıda devreye girer; önizlemede bu gecikme görünürse not et, Claude Code çözer.
- Kafadaki maçta (`m._lite`) ders uygulanmaz (prototip). Görev adı: ders kazanırsa `p.job = 'ders · ' + kısa özet`.

**Savunmada** (aynı dosya, bölge görevi, bugün satır ~185–186):
- Yalnız görev **bölge** iken: bölge hedefi (`tx, ty`) dersin hedefine doğru `w = benzerlik × ogrenme × K_s` (en çok 0,8) oranında çekilir. Pres, kapatma, markaj, kovalama, geri koşu görevlerinde ders girmez.

**Ölçek:** `K` teamV ile aynı birimde. Başlangıç: tam öğrenme ve benzerlik 1'de, kayıt anında oyuncunun hedefe gitmesini sağlayan en küçük değerin yaklaşık 1,5 katı. Kaydırıcıyla oynanabilir kalsın; iz kaydına (`ALAN_IZ`) `ders` sütunu ekle ki terimin öbür terimlere göre büyüklüğü görülsün.

**İşçi mesajları** (`mac-isci.js`): dersler maçın girdisi, taktik gibi. `yeni`, `an` ve `degistir` mesajlarına `dersler: [...]` alanı. Önizleme için aynı anı iki kez oynatmak `an` mesajıyla zaten mümkün (`canli.html?an=ID` altyapısı: anın tam kopyası `grafOku` ile kurulur); ikinci oynatmada `dersler` dolu gider. Benzer anlar için işçi maç boyunca dersin benzerliğinin eşiği geçtiği tikleri `olay` olarak yollasın: `{ tip: 'ders', id, tick, benzerlik }`.

## 6 · Kabul

1. **Değişmezlik:** bayrak kapalı ve ders yokken 909623 ve en az iki maç daha (41, 63) eski motorla bit bit aynı.
2. **Asıl an:** tam öğrenmede Mert, kayıt anından en geç 1 sn sonra hedefin 3 br yakınında.
3. **Taşınma:** aynı maçta dersin uygulandığı anların listesi; her birinde derssiz / dersli yer farkı.
4. **Alakasızlık:** dersin uygulanmadığı anlarda (benzerlik eşiğin altında) bütün oyuncuların yeri derssiz maçla aynı.
5. **Hız:** tik süresi artışı %5'in altında (ders taraması ucuz olmalı).
6. **Dokunmatik:** iPad'de sürükleme ve kaydetme tek elle yapılabiliyor.

## 7 · Teslim

- Sayfa dosyaları (canlı sayfa ve gerekirse küçük bir ders paneli).
- Motor değişikliklerinin **ayrı diff'i** (`plan/ders-motor.diff` gibi) ve bir `DESIGN-NOT-antrenor-dersleri.md`: neyi nereye eklediğin, `K` için bulduğun değer, ilk denemenin sonucu (Mert'in derssiz / dersli yeri, sonuç), gördüğün tuhaflıklar.
- Paketteki `alan-v5/*.js` siteye doğrudan girmez; Claude Code diff'i okuyup bugünkü motora bayrakla taşır.
