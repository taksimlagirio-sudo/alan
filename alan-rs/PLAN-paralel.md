# Plan: sitede kararları web işçilerine bölmek (6 Kasım)

Mimari oturumundan davranış oturumuna. Durum: **öneri; önce Adım 0 ve Adım 2'nin kapıları.** Kısaca: bugünkü
Rust motorunda paralel kafa neredeyse hiç kazandırmıyor (4 çekirdekte −%6). Nedeni wasm ya da tarayıcı değil, kafanın
yapısı. Siteye işçi eklemeden önce motor tarafında bölüşülebilir işin payı büyütülmeli; o olmazsa site işi yapılmamalı.

## 1. Daha önce ne oldu

| Ne zaman | Motor | Ölçüm |
|---|---|---|
| 22–23 Ekim | JS paralel kafa (her işçi maçın tamamını oynar, yalnız dalgalar bölüşülür) | Dalga dışı iş toplamın %40'ı → %16'ya indirilince 2/3/4 işçi ×1,64 / ×2,04 / ×2,27 (280734, 900 tik). Tarayıcıda 2 işçiyle kare sayısı 1261 → 2101 |
| 25–30 Ekim | JS | Aynılık hataları: kayıp ızgarası hücresi ilk soranın bağlamıyla dolunca 1 ve 4 işçinin maçı ayrışıyordu (`f99bf5b`, `76b4652`). Düzeltildi; sonra paralel varsayılan kapatıldı (`31b6dc1`) |
| 4 Kasım | Rust (`85aeabd`, `ALAN_ISCI=n`) | Dalgalar 4 işçiyle birebir aynı; kazanç küçük (MIMARI.md 2.5: "dalgalar ~%10") |
| 6 Kasım (bugün) | Rust, main | Aşağıda |

## 2. Bugünkü ölçüm (m00, 400 tik, `alan-kos`, en iyi koşu; makinede 4 çekirdek)

| | 1 işçi | 2 işçi | 4 işçi |
|---|---|---|---|
| Bugünkü motor | 13,68 ms/tik | 13,38 | 12,85 (**−%6**) |
| `ALAN_PR_TAZE_IZ` (prensip döngüsü de bölüşülür, önbellekleri taze) | 16,02 | — | 13,72 |

- Bölüşülen dalgalar (`ALAN_DALGA_IZ`): 400 tikte 55 dalga, toplam 1,30 sn. Bu, 6,1 sn'nin **%21'i**. 4 işçiyle tavan
  ~%15; paketleme ve bekleşme bedeliyle ölçülen %6.
- Sürenin en büyük parçası prensip tartısı (`prensip_degerle`, profilde ~%45–50) ve bölüşülmüyor (`PR_BOL = false`).
- Prensip döngüsünü taze önbellekle bölüşmek (`ALAN_PR_TAZE_IZ`) tek çekirdekte 13,68 → 16,02 yavaşlatıyor. 4 işçiyle
  13,72'ye iniyor, yani bugünkü tek çekirdekten hızlı değil.

## 3. Neden işe yaramıyor

1. **Bölüşülebilen iş dar.** JS'te dalga dışı iş %16'ya indirilmişti; Rust'ta bölüşülen iş yalnız %21.
2. **Prensip tartısının önbellekleri sıraya bağlı.** `kafa2_tarti.rs:1146` notu: PW/PE/PV/PD önbellekleri yuvarlanmış
   anahtarla ilk soranın sonucunu verir (JS gibi). İşler başka sırayla ya da başka işçide yapılınca değerler değişir;
   ters sıra sınaması 289. tikte ayrışıyor. Bu yüzden prensip döngüsü bölüşülemiyor.
3. **İşler önbelleği paylaşarak ucuzluyor.** Tek çekirdekte bir işin doldurduğu önbellek sonrakini hızlandırıyor
   (taze önbellek +%17 yavaş). Ayrı işçiler bunu paylaşamaz; bölüşmenin kazancının çoğunu bu kayıp yiyor.
4. **Her işçi dalga dışındaki her şeyi yeniden oynar** (JS ve Rust modeli aynı). Dalga dışı pay büyük kaldıkça kazanç
   tavanı düşük kalır.
5. **Paketleme bedeli:** sonuçlar bincode ile paketlenip açılıyor (`paket.rs`); dalga başına ortalama 13 iş, küçük işlerde
   bedel kazancı geçiyor.

## 4. Plan

### Adım 0: hedefi netleştir (davranış oturumu, ölçüm)
Sitede hesap oynatmaya yetişiyor mu? Bilgisayarda ve telefonda, tohumlu maçla (`canli.html?basla=1&tohum=N`) maçın
1 sn'sinin hesap süresi ölçülmeli. Hesap oynatmanın önündeyse paralel kullanıcıya bir şey kazandırmaz; yalnız ileri
sarma, yeniden oynatma ve taktik değişikliğinden sonraki bekleme hızlanır. **Kapı:** belirgin bir bekleme yoksa
plan burada durur.

### Adım 1: prensip önbelleklerini sıradan bağımsız yapmak (mimari oturumu, motor)
Kayıp ızgarasındaki düzeltmenin aynısı (`f99bf5b`: değer, ilk soranın noktasında değil hücre merkezinde): PW/PE/PV/PD'nin
değeri anahtarın temsil ettiği noktada hesaplanır. **Davranış değişikliği:** bayrak arkasında, ayrı commit, maç
istatistikleriyle (`sina/istatistik.js`) ölçülür, sonra varsayılan yapılıp `sina/ayni.sh --yaz`. Tek çekirdekte hız
kaybı olmamalı.

### Adım 2: prensip döngüsünü dalgaya almak ve Rust'ta ölçmek (mimari oturumu)
`PR_BOL` açılır; `ters` sınaması ve 1/2/4 işçi birebirliği (`ALAN_ISCI`, 20 maç × 2700 tik). Önbellek paylaşımı kaybı
burada ölçülür: yardımcı işçi kendi önbelleğiyle başlar. Gerekirse tik ömürlü önbellekler (ör. `Onbellek::tahmin`)
işçiler arasında paketle taşınır ya da işler önbelleği paylaşacak biçimde gruplanır.
**Kapı:** Rust'ta 4 işçiyle tek çekirdeğe göre **en az ×1,5** yoksa Adım 3'e geçilmez (site işi tarayıcıda daha
pahalı: işçiler arası kopyalar ve `Atomics.wait`).

### Adım 3: wasm bölüştürücü (davranış oturumu, site; motor tarafı mimari oturumuyla)
- **Model:** JS'teki gibi N web işçisi, her biri aynı maçı oynar; dalgalar bölüşülür (Rust `paralel::yerel`'in
  karşılığı). Site zaten çapraz köken yalıtımlı (`266daef`), `SharedArrayBuffer` kullanılabilir.
- **Motor tarafı:** wasm'a `Dagitici` uygulaması. JS'ten içe aktarılan iki işlev (iş/sonuç paketini panoya yaz, panodan
  oku); pano `SharedArrayBuffer` üstünde, bekleme `Atomics.wait` ile (işçilerde izinli). `paralel::hazir` wasm'da
  bugün sabit `false` (boyut için, MIMARI.md 3.9); paralel derlemede bu kalkar (wasm +~100 KB) ya da ayrı bir wasm
  derlenir.
- **Site tarafı:** `?paralel=N` / `localStorage 'alan-paralel'`, varsayılan kapalı; hesap göstergesinde çekirdek sayısı
  (`942246c`'deki gibi).
- **Doğrulama:** 1/2/4 işçi kareleri bit bit aynı (tohumlu, `sina/isci-yeni-sina.js` genişletilir); hız bilgisayarda
  ve telefonda.

## 5. Kim ne yapar

| Adım | Oturum | Dokunduğu yer |
|---|---|---|
| 0 | davranış | ölçüm, site |
| 1–2 | mimari | `kafa2_tarti` önbellekleri, `paralel`, `paket` |
| 3 | davranış (site) + mimari (wasm `Dagitici`) | `alan-v5/rust-isci.js`, `wasm/src`, `motor/src/paralel.rs` |

Ölçüm komutları: `ALAN_ISCI=n alan-kos <kurulum> 400 ozet` (hız), `ALAN_DALGA_IZ=1` (dalgaların payı),
`ALAN_PR_TAZE_IZ=1` (prensip döngüsü taze önbellekle bölüşülür; davranışı değiştirir, yalnız ölçüm için).
