# Görev: SAVUNMA (baskı, markaj, blok)

Analizci oturumdan. Bulgular maç 217468'den (Liman 3–0 Kale Duvarı, güncel main motoru, 9000 tik).
Sen değişikliği yaparsın, analizci yeniden ölçer. Hücum tarafı ayrı bir oturumda (`GOREV-hucum.md`).

## Kurulum ve ölçüm

- Kayıt: `alan-rs/analiz/mac-217468.json`.
  - Liman = takım 0, sağa hücum. Taktiği: pres=1, şiddet=1, blok Yüksek, tetikler Zayıf kontrol / Arkası dönük / Kanatta / Geri pas / Uzun pas havada.
  - Kale Duvarı = takım 1, yapay zekâ, alçak blok, pres=0.
- Ölçüm: `alan-rs/analiz/olc.sh [çıktı]`. Değişiklikten önce ve sonra çalıştır.
- İz kancaları:
  - `ALAN_UA_IZ=<tik>`: eşleşme ataması (işler, ağırlıklar, oyuncu × iş tablosu).
  - Baskı adayı değerleri için `savunma.rs pres_sec` içinde `srt` geçici olarak yazdırılabilir.
- Kurallar `alan-rs/AJAN.md` dosyasında. Davranış değiştiren her iş bir deney bayrağı arkasında, ayrı commit'te olsun. `cargo test --release` yeşil kalmalı.
- Dosyaların: `sekil/konum/savunma.rs`, `sekil/konum/esles.rs`, `sekil/pres.rs`, `sekil/yer.rs` (yalnız savunma dalı), `mac/kuyu.rs`, `mac/enerji.rs`.
- Dokunmayacakların: `kafa/*` ve `sekil/konum/hucum.rs` (hücum oturumunun). Sahiplik haritasını (`kafa2_zemin.rs`) hücum oturumu değiştiriyor.

## Başvuru sayıları (şu anki main)

| Ölçü | Liman savunurken | Kale Duvarı savunurken |
|---|---|---|
| Taşıyıcıya en yakın savunmacı 15+ br | %40 (1467 / 3643 tik) | %33 |
| Hiç kimsenin işi pres/kovala/kapatma değil | %75 | %54 |
| 5 sn+ baskısız dönem | 627–1469 (28 sn), 3064–3755 (23 sn), 3833–4097 (8.8 sn) | |

## S1. Baskı seçiliyor ama atamada siliniyor (en öncelikli)

**Görülen:**
- Burç 00:20–00:48 arasında kendi kale dibinde (118, 20.4) 28 sn topla duruyor. Kaan (97.8, 21.7) 27 sn, Tuna (95.7, 36.2) 26 sn hareketsiz "bölge" işinde; ikisi de 20 br uzakta.
- Tetiklerin hepsi var: önce Sur→Burç geri pası, Burç kanatta, arkası dönük.
- 01:41–02:05 arasında kaleci Sur topla kalesinden 30–35 br dışarıda. Kaan 12 br'de "bölge"de, Ece 12 sn hareketsiz; kale boş.

**Bulunan neden:**
- `savunma.rs:444 pres_sec` baskıcıyı seçiyor. Tik 700'de Tuna P = 1.00, v = 0.403; Kaan P = 1.00, v = 0.403. Sur dışarıdayken (3400, 3600) adayların değeri 0.38–0.41.
- Ardından eşleşme (`esles.rs`) taşıyıcı işinin ağırlığını `w0 = t_p.h × K` ile kuruyor (`esles.rs:190`). `t_p.h`, kafanın taşıyıcıya biçtiği "o olmasa" değeri.
- Kafanın değer haritası kendi yarı sahada düz (hücum görevi H1); bu yüzden `t_p.h` = 0.
- Oyuncu başına değer de `max(hm_get(id)·K, J.w) × PP` ile hesaplanıyor; o da 0.
- `ALAN_UA_IZ=700` kaydı: J5 ve J6 (Burç, car 0/1) için w = 0. Altı oyuncunun satırında da 0.
- Atama kimseye taşıyıcı işi vermeyince `yeni_p` boş kalıyor ve baskıcılar siliniyor (`esles.rs:477`). Kaan ile Tuna "bölge"ye düşüyor.

**Tasarım açığı:**
- Baskının değeri yalnızca "rakibin değerini engellemek". "Topu burada kazanırsak biz ne kazanırız" yok. Bu kazanç tam da rakip kendi kale dibinde, kaleci dışarıdayken en büyük.
- Taktikteki pres/blok/tetik ayarları bu ağırlık pozitif olmadıkça hiç işlemiyor.
- Hücum oturumu haritayı düzeltse bile baskı değeri rakibin değerine bağlı kalacak. Kendi kazancımızın da hesaba girmesi gerekiyor.

**Başarı ölçüsü:**
- Liman (pres=1, blok Yüksek) rakip kendi yarısında topu tutarken 2–3 sn içinde baskı kursun.
- "Hiç kimse pres işinde değil" oranı ve 5 sn+ baskısız dönemler belirgin düşsün.
- Kale Duvarı (pres=0, alçak blok) davranışı değişmesin. Baskı tercihi taktiğe göre ayrışmalı.

## S2. Markaj git-gel (atamada süreklilik yok)

**Görülen:**
- 627–1469 arasında Mert 28 sn'de 62 kez iş değiştiriyor: her 6–8 tikte Ok (y≈43) ↔ Hendek (y≈16) ↔ Mancınık (y≈29) ↔ "arkada alan". Hedefi 27 br sıçrıyor. Üç rakip o sırada hareketsiz.
- Cansu her ~7 tikte "bölge (36, 9)" ↔ "markaj Hendek (6, 16)" arasında gidip geliyor; hedefi 30 br sıçrıyor.
- Maç boyu 12 br'den büyük hedef sıçraması: Mert 209, Cansu 165, Ece 153. 20 tik içinde eski hedefe dönüş: Mert 54, Cansu 38, Ece 34. Her oyuncuda dakikada ~90–100 iş değişimi.

**Bulunan neden:** `ALAN_UA_IZ=700` tablosunda değerler birbirine çok yakın:

| İş | Deniz | Mert | Cansu |
|---|---|---|---|
| arkada · Hendek | .160 | .141 | .164 |
| arkada · Mancınık | .052 | .052 | .046 |

- Atama her tik baştan çözülüyor (`dal_sinir`). Önceki işe tek ayrıcalık tepki süresini çıkarmak (`esles.rs:346`).
- Atama dışında kalan oyuncu `yedek`'te slot tabanına "bölge" ile dönüyor.

**Tasarım açığı:** Histerezis ya da iş değiştirme maliyeti yok. Ufak kaymalar atamayı çeviriyor ve oyuncu iki işi de yapamıyor.

## S3. Savunmacılar hücumcuyu kale çizgisine kadar izliyor

**Görülen:** Kale Duvarı'nın üç hücumcusu Liman kalesine 5–10 br mesafede dururken Deniz, Mert ve Cansu da oraya iniyor ("arkada · X", "Kuyu alanından çıkıyor"). Deniz 00:25 ve 00:37'de Kuyu alanına girip çıkmak zorunda kalıyor.

**Neden (kısmen):** Hücum dizilişinin ön ucu `slot_base0` ile rakibin en geri adamına bağlanıyor (hücum görevi H4a). Savunma bu adamları izleyince en geri çizgi daha da iniyor; ikisi birbirini besliyor.

**Senin tarafın:** Savunmanın en geri çizgisi için bir taban yok. Liman'ın blok Yüksek olduğu hâlde top 110 br uzaktayken savunma kale çizgisinde.

**İncelenmedi:** `savunma.rs` süpürücü/`arkada` işleri ve `slot_base0` savunma dalı (`back = cl(min(BLOK, bd−6), 9.6, 74.4)`).

## S4. Kuyu alanında sönme

**Görülen:**
- Savunmacılar kendi Kuyu alanında süre doldurup sönüyor: Kale 4, Burç 3, Mancınık 1, Siper 1, Mert 1 kez. Sönük süre: Siper 36 tik, Burç 16 tik.
- 04:44'te (8549) Siper, Tuna'nın şutu sürerken sönük.
- Deniz iki kez (769, 1111) "Kuyu alanından çıkıyor" ile markajından kopuyor.

**İncelenmedi:** Neden alana giriyorlar (`mac/kuyu.rs kuyu_saat`, atama hedeflerinin `kuyu_var` ile süre cezası alması).

## S5. Şutta ve golde savunma

**Görülen:**
- **2694:** Kaan'ın şutunda top daha Bekçi'ye varmadan Hendek "önde bekle" (38, 23)'e, Burç "arkada Cansu"ya dönüyor; seken topu beklemiyorlar.
- **3. gol (8668):** Şut anında Hendek "kapatma · Ece" ile hedefi (107, 13.7)'ye, şutçudan uzağa koşuyor. Mancınık "bölge"de, barı .04.
- **2. gol (6078):** Cansu 19 br'de 16 tik topla duruyor; en yakın rakip 8.4 br. Siper (bar .02) ve Kale (bar .16) neredeyse boş.

**İncelenmedi.**

## S6. Toplu bitkinlik

**Görülen:** 03:50–04:10 arasında Liman'ın 6 dış oyuncusunun hepsi aynı anda %1–9 barda (7501: Deniz .08, Mert .07, Ece .03, Cansu .05, Kaan .01, Tuna .07); 12 "Barı boş" olayı. Bu sırada Liman ~40 sn kendi yarısında savunuyordu (Kale Duvarı topta, 6213–7375). Kale Duvarı'nın en net şutu (7177, v = .69) tam bu sırada geliyor.

**İncelenmedi:** Baskı/kovalama sırasında bar harcaması ve dinlenmenin takım içinde dağılımı (`enDinlen .75`, `enPresDalga 0`).

## Bitince

Commit'leri, deney bayraklarını ve `olc.sh` öncesi/sonrası çıktılarını analizci oturuma bildir. Analizci aynı maçı ve başka tohumları yeniden ölçer.
