# Görev: HÜCUM (taşıyıcı + topsuz hücumcular)

Analizci oturumdan. Bulgular maç 217468'den (Liman 3–0 Kale Duvarı, güncel main motoru, 9000 tik).
Sen değişikliği yaparsın, analizci yeniden ölçer. Savunma tarafı ayrı bir oturumda (`GOREV-savunma.md`).

## Güncel durum (analizci, 6 Kasım ~19:00)

- **Yeni başvuru:** main `d0a344f` ile maç 217468 sonucu 0–5; değeri ≈0 karar 86 / 518 (`analiz/out/izle/claude_wizardly-knuth-1vc0a3-d0a344f/`). Aşağıdaki 313 / 804 ve diğer sayılar eski main `1f0ec1a`'ya ait. Karşılaştırmayı `d0a344f`'ye göre yap.
- **Tek maçla karar verme.** Aynı taktikle beş koşunun skorları 0–5, 3–3, 1–2, 1–2 ve 2–4. `analiz/cok.sh <çıktı> [tohumlar]` aynı taktik ve kadroyla birden fazla tohum oynatıp ortalamayı veriyor; varsayılan 6 tohum. `ALAN_*` bayrakları ortamdan geçer:
  ```bash
  ALAN_SAH_YAKINSA_IZ=1 alan-rs/analiz/cok.sh alan-rs/analiz/out/sahyak
  ```
  Tek tohum için `TOHUM=<n> analiz/olc.sh <çıktı>`.
- **H1, sahyak (`wizardly-knuth` `09be289`):** değeri ≈0 karar 86'dan 18'e indi; kabul. **Yan etki:** 5 sn'den uzun baskısız tutma 2'den 6'ya çıktı. Mert 7792'de tut 0.106, en iyi pas 0.087. Tut haritanın iyimser pas tahmininden değer alıyor.
  - **Analizcinin önerisi (a):** tut = eldeki en iyi seçenek × zaman indirimi. İndirim taktiğin tempo/sabır ayarına bağlansın; buna yalnızca beklenen bir açılmanın (arkadaşın koşusu, rakibin kayması) payı eklensin.
  - **(b), harita ile uçuş simülasyonunun pası farklı değerlemesi:** ayrı iş. Baskı, koşu skoru ve alıcı değeri de haritadan geliyor.
  - **Okuma notu:** "baskısız tutma" sayısında baskı yapmayan takım Kale Duvarı (pres=0). Liman'ın taktiği tempo .25 / sabır .75, yani bir miktar beklemek beklenen davranış. Sorun yalnız hiçbir şey değişmezken beklemek.
- **onsira:** işe yaramadı (86 → 143); kapalı kalsın.
- **H2 düzeltmesi:** "kaleye yakın kararların ~%90'ında şut seçeneği yok" sonucu yanlış ölçüme dayanıyor.
  - Bu ölçü `firsat.py`'den geliyor; `kararlar.txt`'deki seçenek listesi motorda ilk 14 seçenekle kesik (`dongu.rs:1315 take(14)`). `d0a344f`'de kaleye yakın 152 kararın 151'inde liste tam 14'te kesik.
  - Şut üretimini ve savurmada düşmeyi doğrudan izle ölç:
    - `kafa1_secenek.rs` şut bloğunda üretilen `gönder` sayısı.
    - `kafa2_karar.rs:367`'de `savur_plan` None dönen sayı.
    - `mac/kafa_bag.rs kafa_not` içindeki "savurmanın yolu kapalı" reddi.
  - `1f0ec1a`'da 130 kararın 44'ünde bütün şutlar savurmada düşüyordu. savtepki (`797b00d`) bu ölçüyle yeniden değerlendirilmeli.
- **Main'de yeni çıkan (`d0a344f`):**
  - Liman 30–42 br'den ve köşeden şut çekiyor (Ece (92, 1.7), 39 br): 9 şutun 7'si doğrudan rakibe gitti, gol yok.
  - Kale Duvarı'nda hiç açık pas seçeneği olmayan tik oranı %47'den %72'ye çıktı.
- **Aras:** kendi kalesinde 12–15 sn topla bekliyor. Bekçi'nin Kuyu alanındaki süre sınırının dışında olabilir.
- `cargo test --release` her değişiklikte çalıştırılmalı (AJAN.md).
- Analizci saatlik izliyor: motoru değiştiren yeni uç commit'leri kendiliğinden oynatıp `analiz/out/izle/` altına yazıyor. Bayrakla açılan davranış için hangi bayrakların açılacağını analizciye söyle; varsayılan kapalı bayrak izleyicide görünmez.

## Kurulum ve ölçüm

- Kayıt: `alan-rs/analiz/mac-217468.json` (Liman = takım 0, sağa hücum; Kale Duvarı = takım 1, yapay zekâ).
- Ölçüm: `alan-rs/analiz/olc.sh [çıktı]`. Maçı oynatır (`alan-kos … analiz` ve `… kararlar`), aşağıdaki sayıları yazar. Değişiklikten önce ve sonra çalıştır, iki çıktıyı karşılaştır.
- Tek bir anı incelemek için motorun iz kancaları:
  - `ALAN_SURV_IZ=1`: her seçeneğin değeri ve bileşenleri (TUTV, SURV, PASV, GONV, SAHIP).
  - `ALAN_C3_IZ=<tik>`: sürme adayları ve sahiplik değerleri.
  - `ALAN_SAH_IZ=x,y`: sahiplik haritasının o hücresi.
  - `ALAN_UCUS_SUT_IZ=<ad>`: şutun uçuş hesabı.
- Kurallar `alan-rs/AJAN.md` dosyasında. Davranış değiştiren her iş bir deney bayrağı arkasında, ayrı commit'te olsun. `cargo test --release` yeşil kalmalı.
- Dosyaların: `kafa/*` (seçenek, tartı, karar, zemin, uçuş, savur), `sekil/konum/hucum.rs`, `sekil/yer.rs` (yalnız hücum dalı).
- Dokunmayacakların: `sekil/konum/savunma.rs`, `esles.rs`, `sekil/pres.rs` (savunma oturumunun).

## Başvuru sayıları (şu anki main)

| Ölçü | Liman | Kale Duvarı |
|---|---|---|
| Değeri ≈0 taşıyıcı kararı | 313 / 804 (Burç 145, Sur 112, Mert 28) | |
| Pas / kenardan (300 sn) | 16 / 10 | 13 / 10 |
| Takım boyu ort / maks (topta) | 66 / 93 | 78 / 115 |
| Hiç açık pas seçeneği olmayan tik | %31 | %47 |
| Taşıyıcı kaleye 35 br içindeyken kutuda (kaleye <20) kimse yok | %39 | %14 |
| …kutuda açık arkadaş yok | %88 | %89 |
| Kaleye 26 br içi kararlarda bütün şutların savurmada düştüğü karar | 44 / 130 | |

## H1. Donma: taşıyıcı topla kıpırdamıyor (en öncelikli)

**Görülen:**
- Burç 00:20–00:48 (tik 627–1469) kendi kale dibinde (118, 20.4) 28 sn duruyor, 117 kez "tut" seçiyor.
- Sur 01:41–02:05 arasında topu kaleden 35.8 br dışarı sürüp (89.9, 49.4)'te ~17 sn bekliyor.
- Mert 00:57–01:09 kendi köşesinde (11.4, 56) 12 sn duruyor.
- Bu kararlarda en iyi seçenek "tut" = 0; geri kalan her şey ≤ 0.

**Bulunan neden:**
1. **Değer kendi yarıya ulaşmıyor.** Sahiplik haritası (`kafa2_zemin.rs: sahiplik_kur`) şut değerini yalnızca rakip kaleye 60 br içinde üretiyor (`kafa2_imkan.rs:488 imk_uzak`). Bu değeri yalnızca `SAH_ADIM = 3` adım yayıyor (6 br ızgara). `ALAN_SAH_IZ=114,18` kaydında Kale Duvarı için N = −0.001; komşu hücrelerin hepsi ≈ 0.
2. **Ön eleme mutlak işaret istiyor.** Pas adayları (`kafa1_secenek.rs:367`) ve sürme adayları (`:504`) `s = own × sah > 0` koşulundan geçmek zorunda. Harita 0'ın biraz altında olunca hepsi tartılmadan eleniyor. `ALAN_C3_IZ=640`: Burç'un 24 sürme adayının hepsinde sah < 0. Tik 640'ta yalnızca 3 "tut" ve 20 uzun "kenardan" kalıyor.
3. **"tut" tam 0.** Değeri `P × durum − (1−P) × kayıp` (`kafa2_tarti.rs:1296`). Baskı yok, P = 1; harita düz, durum = 0; sonuç 0. Uzun paslarda doğrudan varma Pd = 0, hepsi eksi.
4. **Kayıp bedeli işlemiyor.** V0 = 0 olduğu için (`kafa2_tarti.rs:1365`) bedel de 0.

**Tasarım açığı:**
- Topa sahip olmanın değeri şut bölgesinden 3 hamle ötede yok.
- Tempo ve sabır ayarları beklemeye bir bedel koymuyor.
- Ön eleme "durmaktan iyi mi?" diye değil, "pozitif mi?" diye soruyor.
- Kaleci de bu yüzden kalesinden çıkıp sahada duruyor.

**Başarı ölçüsü:** Değeri ≈0 karar sayısı ve 5 sn'den uzun baskısız donma sayısı belirgin düşsün. Pas sayısı artsın, kayıp oranı patlamasın.

**Not:** Savunmanın baskı değeri de bu haritadan türüyor (savunma görevi S1). Harita düzelince orası da değişir. Harita değişikliğini sen yap; savunma oturumu baskıyı haritadan bağımsız kılmaya çalışıyor.

## H2. Seçilen şut çıkmıyor; yakın baskıda şut hiç üretilmiyor

**Görülen:**
- 01:40'ta (3004–3030) Kaan kaleye 8.7 br, tam orta; Bekçi 7 br yanda.
- 3006 ve 3007'de "gönder plase" (v = 0.70) seçiliyor ama top çıkmıyor.
- Sonraki kararlarda şut seçeneği hiç yok. Kaan geri kayıyor, sendeliyor, Sur topu alıyor.

**Bulunan neden:**
- Maç savurmayı reddediyor: "savurmanın yolu kapalı" (`mac/hareket.rs:449`, iz: `kafa_bag::kafa_not`).
- Kafa kararı algılanan dünyayla (`dongu.rs:1231 gor`, tepki gecikmeli) veriyor. Maç ise savurmayı gerçek konumlarla deniyor. Hendek o arada uzanıp savurma yoluna girmiş.
- Reddedilen şut bir sonraki tikte yeniden seçiliyor.
- Sonra kafanın kendi savurma denetimi de (`kafa2_karar.rs:367`, `savur_plan` None ise `o.v = None`) bütün şutları düşürüyor.
- Savurma yolu, 10 br içindeki her rakibin koşup yetişmesine bakıyor (`kafa1_savur.rs:286 savur_yol`).

Ölçüm: kaleye 26 br içindeki 130 kararın 44'ünde üretilen bütün şutlar savurmada düştü. Aynısı Mancınık'ta da oldu (6426–6427).

**Tasarım açığı:**
- Kafa ile maç aynı eylemi farklı dünyalarla deniyor.
- Reddedilen eylem tekrar seçilebiliyor.
- Savurma tek şut yolu olduğu için yakın markajda bitiricilik çöküyor.

## H3. Hatta duran adama şut (uçuş hesabında tünelleme)

**Görülen:** 04:38'de (8351 kararı, 8357 şutu) Cansu 21.5 br'den şut çekiyor. Kale tam hatta, 7 br önünde duruyor. Kafa golü Pk = 0.486 görüyor. Top ilk tikte Kale'den sekiyor, hızının %82'sini koruyor (5.31 → −4.36) ve 95 br yuvarlanıp Liman'ın kendi kale ağzına (x = 4.6) kadar geliyor.

**Bulunan neden:** Kafa topun yolunu tik başına bir noktada örnekliyor (`kafa1_ucus.rs` ~820–840). `ALAN_UCUS_SUT_IZ=Cansu` kaydında:
- 1. tikte top (105.2, 23.6), 2. tikte (110.4, 24.0).
- Kale (108.4, 23.0)'da, yani iki örnek noktanın tam arasında.
- Gövde kontrolü (`dd < BODY_R`, `:832`) yalnızca örnek noktalarda yapılıyor. Kale 1. noktada 3.2 br uzakta, yalnızca F = 0.16 ile "uzanabilir" sayılıyor; 2. noktada hiç bakılmıyor.
- Maçta ise çarpışma sürekli hesaplanıyor.

**Tasarım açığı:**
- Hızlı topta (tik başına gövde çapından fazla yol) aradaki gövdeler görülmüyor.
- Gövdeden sekme neredeyse esnek. Bu fizik bir karar mı? İstenmiyorsa kafanın sekme hesabıyla birlikte düşünülmeli.

## H4. Topsuz hücumcular

### H4a. Takım ikiye bölünüyor: dizilişin ön ucu rakibin en geri adamına bağlı

**Görülen:**
- Burç 28 sn dururken Hendek (7.3, 13.6), Mancınık (6–12, 25–31) ve Ok (7–11, 45–47) Liman kale çizgisine 5–10 br mesafede duruyor; top 110 br geride.
- Kale Duvarı topta iken Ok %56, Hendek %50, Mancınık %46 oranında taşıyıcıdan 60+ br uzakta.
- Mancınık zamanın %78'inde kaleye 20 br içinde, %49'unda 3 br'lik sıkı markajda, %0 açık.
- Ok topta iken bir seferde 19.8 sn hareketsiz.

**Bulunan neden:** `sekil/yer.rs:51–58 slot_base0`, hücum dalı:
- Arka uç topa göre: `back = cl(bd − 21.6 − 9.6·(1−risk), 12, 74.4)`.
- Ön uç rakibe göre: `front = min(112.8, max(back + 28.8, last + 1))`. Buradaki `last`, rakibin kendi kalemize en uzak dış oyuncusu.
- Burç x = 118'de, Deniz x ≈ 9'dayken back = 12, front ≈ 112. Takım boyu 100 br oluyor, ön slotlar x ≈ 8'e düşüyor.
- Liman savunmacıları bu adamları "arkada · X" ile kale çizgisine kadar izliyor. İki takım birbirini kaleye itiyor.
- Taşıyıcıyla öndekiler arasındaki mesafe pas menzilini (`pass_max_f = 80`) aşıyor; kalan tek bağlantı uzun kenardan pas. Bu H1'i besliyor.
- Kale Duvarı'nın kontra koşucularında ip katsayısı 0.01 (`hucum.rs leash_k`, `self.win`).

**Tasarım açığı:**
- Dizilişin boyuna üst sınır yok.
- Ön uç topun yerinden değil, rakip savunmanın derinliğinden belirleniyor.
- Hücumcu "kaleye yakınlık × rakipten önce varma" (`run_v`) için kaleye gömülüyor. Pas alabilir olmak skorda yeterince yer almıyor.

### H4b. Liman ceza sahasına girmiyor; destek geride kalıyor

**Görülen:**
- Taşıyıcı kaleye 35 br içindeyken tiklerin %39'unda kutuda (kaleye <20) hiç Liman oyuncusu yok, %88'inde kutuda açık arkadaş yok.
- Sıklıkla 3–5 oyuncu topun 10+ br gerisinde kalıyor.
- Üç gol de Cansu'nun 15–19 br'den bireysel şutu. Kutuya koşu ya da pasla bitirme yok.
- Kanat Kaan zamanın %39'unda kaleye 20 br içinde ama %25'inde sıkı markajda, yalnızca %20'sinde açık.

**İncelenmedi:** `hucum.rs hucumcu0` aday skoru (`run − ry + tv − leash·lsh² + kb`). Alçak blok kutuyu doldurunca `run_v` düşük kalıyor olabilir. Tuna'nın (Uç) kutuda durma oranı %1.

### H4c. Taşıyıcı dururken arkadaşlar uzaklaşıyor

**Görülen:**
- 00:57–01:09 arasında Mert köşede 12 sn topla duruyor.
- 5 arkadaşı önce "kontra koşusu", sonra "boşluk" ile 80–100 br ileri koşup orada bekliyor (Kaan hedefi (106, 24), Tuna (101, 58)).
- Taşıyıcı dururken 3+ arkadaşın kontra koşusunda olduğu 80 tik var.
- Destek için geri gelen kimse yok. Liman topta iken taşıyıcıya 15 br içinde arkadaş olmayan tik oranı %36.

**İncelenmedi:** Kontra koşusunun bitiş koşulu. Taşıyıcı ilerlemiyorsa geri gelme kuralı yok gibi.

### H4d. Deniz (Bek) hücumda hep geride

**Görülen:** Liman topta iken Deniz zamanın %71'inde hareketsiz, %81'inde "güvence" işinde, taşıyıcıya ortalama 56.6 br. Ok'un (önde bekle) 3–5 br yanında, Aras'ın önünde duruyor. Rakibin `onde=1` ayarı için bir oyuncu kalması mantıklı olabilir; ama hiç ilerlemiyor.

## H5. Pas kararları (daha düşük öncelik)

- **5893:** Ece kale çizgisinden (119, 12) öbür kanattaki Tuna'ya (112, 55) 43 br çapraz pas atıyor (v = −0.0007). Bir tik sonra Siper kesiyor.
- **5801:** Ece kendine pası köşeye (115, 5.5) atıyor ve çizgiye sıkışıyor.
- **4814–4904:** Tuna sağ köşede (109, 59) 90 tik boyunca topu 5 kez kaybedip geri alıyor.
- **Kenardan paslar:** Kale Duvarı'nın 10 kenardan pasının 5'i rakibe gidiyor. Sur'un iki uzun açılışı (4579, 8749) rakipte bitiyor.
- **Geri yöne koşma:** Rakip kaleye 28 br içinde topla geri yöne koşma süreleri: Ece 88 tik, Kaan 72, Tuna 71, Ok 56, Hendek 53.

## Bitince

Commit'leri, deney bayraklarını ve `olc.sh` öncesi/sonrası çıktılarını analizci oturuma bildir. Analizci aynı maçı ve başka tohumları yeniden ölçer.
