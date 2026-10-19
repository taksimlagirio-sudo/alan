# Tasarım planı · Arka hat talimatı ve forvetin cevabı

> Durum: fikir aşaması. Kod yok; yalnız ölçüm için yerel bir deneme dalı var, aşağıda. 19 Ekim 2026. Kullanıcının kararı: ofsayt yok; bunun yerine menajere arka hat seçeneği verilir.

## Neden

Taktik ölçümü 2'nin bulgusu (`lab/TAKTIK-OLCUMU-2.md`): **yüksek blok, hattı yükseltmiyor.**
- Arkadaki savunmacılar (savunma dizilişinin en gerisindekiler) bloktan bağımsız olarak en derindeki rakibin gerisinde kalıyor.
- Top bizdeyken rakibin son adamının yeri: Yüksek blokta %73, Orta blokta %75, Düşük blokta %81.
- Ofsayt olmadığı için bu hep mümkün ve bedelsiz. Arkada boşluk oluşmuyor; planın "yüksek presin bedeli arkada bırakılan alan" fikri sahada neredeyse yok.
- Sonuç: tarzlar arasında bir sıralama var, ama birbirlerine karşı panzehir yok.

Ofsayt bu ikilemi futbolda yaratır. ALAN'da ofsayt yerine **ikilemi menajerin kararına** bırakıyoruz.

## Talimat: arka hat (savunma, oyuncu başına da verilebilir)

| seçenek | ne yapar | kazanç | bedel |
|---|---|---|---|
| **Adamı takip et** (bugünkü davranış, varsayılan) | Arkadaki, en derin rakibin kale tarafında kalır | Arkada boşluk yok | Rakibin forveti hattı geriye iter; takım uzar, hatlar arası açılır |
| **Hattı koru** | Bloğun hattında durur, oradan geriye inmez; top hattın gerisine geçince serbest | Kısa, kompakt takım; pres güçlü | Hattın arkasında bekleyen rakip boştur |
| **Süpürücü** | Hat korunur, ama bir oyuncu hattın birkaç birim gerisinde boşluğu kapatır | Arası: kompakt ve arkası kısmen güvende | Bir kişi eksik pres ya da eksik orta saha |

- **Takım geneli ve oyuncu başına.** Rolsüz sistemle uyumlu: biri stoper gibi adamını takip eder, öbürü hattı korur.
- **Hat yüksekliği bloktan gelir.** Blok artık gerçekten hattın yerini belirler (bugün yalnız orta ve öndekileri taşıyor).
- İleride kayan ayar olarak düşünülebilir: "hat disiplini" 0 (adamla iner) … 1 (hattı korur).

## Hücumun cevabı: forvet nerede bekler

Savunmanın seçimine hücumda forvetin talimatı karşılık verir:
- **Derinde bekle:** son savunmacının gerisinde ya da omzunda kalır, uzun topu bekler.
- **Geri düş (sahte 9):** hatlar arasına iner, pas istasyonu olur; arkasına koşu açar.

| | Forvet derinde bekler | Forvet geri düşer |
|---|---|---|
| **Savunma hattı korur** | **Hücum kazanır:** arkada boş adam, uzun top | **Savunma kazanır:** kompakt blok, pres |
| **Savunma adamı takip eder** | Dengede: savunma geri itilir, hatlar arası açılır | **Hücum kazanır:** stoper adamıyla öne çıkar, arkasına koşu açılır |

Bu, ölçümde eksik olan cevap ilişkisi. Menajer maç içinde rakip forvetin nerede beklediğine bakıp kararını değiştirir; maçı izlemek anlam kazanır.

## Özelliklere iş çıkıyor

Özellik ölçümü 1'de etkisiz ya da tek yönlü çıkanlar için doğal bir iş:
- **Bekçi'nin Yerleşim ve Alan hakimiyeti** (ölçümde etkisiz): hattı koruyan takımda süpürücü kaleci olmak hayati olur.
- **Hız:** hattı koruyan stoperin hızı arkaya koşuyu yakalar. Hız taktiğe bağlı bir değer kazanır (planın "özelliğin değeri taktiğe göre değişmeli" şartı).
- **Görüş ve Tepki:** arkaya koşuyu erken okumak.
- **Forvet tarafı:** derinde bekleyen forvette Hız ve Kontrol; geri düşen forvette Görüş, Pas ve Güç.

## Koşullu talimat

Plandaki koşul sistemine bağlanır. Örnekler:
- "Hattı koru; rakip arkada adam bırakırsa adamı takip et."
- "Öndeyken adamı takip et" (skoru korumak).
- "Rakip forvet geri düşerse hattı koru."

## Yapay zekâ menajer

- Yapay zekâ menajerler bu seçimi bilmeli ve rakibe göre değiştirmeli. Yoksa insan menajer her maç aynı açığı kullanır.
- Tarz kimliği olur: "hattı koruyan, önde basan" bir takımla "adamını takip eden, derinde bekleyen" bir takım sahada farklı görünür ve farklı yenilir.

## Arayüz

- Savunma tahtasında **sürüklenebilir hat çizgisi**: bloğun hattı. Arkadaki oyuncular için üç düğmeli anahtar (adam / hat / süpürücü).
- Hücum tahtasında forvet için "derinde / geri düş" anahtarı.
- Canlı maçta: hattın arkasında boş bekleyen rakip ve açılan hatlar arası boşluk görünür olmalı. Menajer neyi değiştireceğini sahadan okusun.

## Motorda (Claude Code için not)

- Yer: `shape.js` `position()`, savunmadaki "arkada kalan" (`sweep`) dalı. Bugün en derin rakibin kale tarafında (`gside`, `icOmuz`) ya da ondan 2 br geride (`dR − 2`) durur. `slotBase0`'da blok yalnız orta ve öndekileri taşır (`back = min(BLOK, top − 6)`).
- **Hattı koru:** arkadakinin hedefi evinin (blok hattı) gerisine inmez. Top hattın gerisine geçince ya da top sahibi hattın gerisindeyse serbest kalır.
- **Süpürücü:** hattın 4–8 br gerisinde, top ile kale ortası arasındaki çizgide.
- **Forvetin derinde beklemesi:** hücum aday tartısında son savunmacının gerisine ek değer. **Geri düşmesi:** hatlar arası adaylarına ek değer. Bugünkü "hatlar arası" kuralı ve "forvet son çizgiyi iter" (`slotBase0` hücum: `front = last + 1`) ile birlikte düşünülmeli.
- Kafadaki maç da bilmeli: rakibin arka hat seçimini görmeden pas ve koşu kararı yanlış tartılır.
- **Deneme dalı:** `deney/arka-hat` (yalnız yerel, gönderilmedi). İçinde "hattı koru"nun ilk sürümü var: `tac.arkaHat = 'blok'`. Talimat verilmezse maç birebir aynı. Ölçülmedi.

## Kabul (ölçüm)

1. Talimat verilmezse maç birebir aynı.
2. **Hattı koru:** savunmada takım boyu kısalır ve son adam bloğa göre yükselir (bugün Yüksek / Orta / Düşük: %73 / %75 / %81). Arkaya pas payı ve arkaya pastan sonra şut artar.
3. **Cevap tablosu:** dört hücreden ikisinde hücum, birinde savunma belirgin üstün olmalı (xG ve şut farkı, eşleşme başına en az 20 maç).
4. Hız ve Bekçi özelliklerinin değeri, arka hat seçimine göre farklılaşmalı (özellik ölçümünün taktik altında tekrarı).

## Açık sorular

- İkili mi, üçlü mü, kayan ayar mı?
- Forvetin cevabı ayrı bir talimat mı, yoksa mevcut "koşu" ve "serbestlik" talimatlarından mı doğsun?
- Hattın arkasında boş kalan rakibe Bekçi ne zaman çıksın (Bekçi'nin çıkış kuralı main'de yeni değişti, 2127349)?
- Hattı koruyan takımın arkasına atılan topa savunmacılar nasıl döner (dönüş süresi Çeviklik'ten)?
