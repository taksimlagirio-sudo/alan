# Kafa 2 (decide.js 4717–8890) — taşıma notu

## Ne taşındı (dosya → JS işlevleri)

| Dosya | İşlevler |
|---|---|
| `kafa2_baglam.rs` | Kapanışın durumu: `Kafa` (D, `mac` = D._mac, `prT` = m._prT, `k2: Kafa2Durum` önbellekler, `k1` yer tutucu), `Dunya = Vec<Arc<Oyuncu>>`, `Tac` (etkin taktik), `MacGor`, `PrT`, `kafa_d()` (D sabitleri), `d_alt_bosalt`, JS anlam yardımcıları (`veya` = `a \|\| b`, `kxy` = `{...p, x, y}` …) |
| `kafa2_dunya.rs` | holdWorld, bekSav, bekKay, pwKos, predictWorld, bekYerW, geriCekil, paylastir; `markRef` / `tutQ` çözümü (`ref_bul`) |
| `kafa2_imkan.rs` | sutTab (+ ALAN_IMK_SUT_VE tablosu `kafa2_imk_sut.bin`), imkBol, imkKov, imkOrt, imkSap/imkInan/imkTau/imkSg, imkan, imkUzak, sutYol, sutS, sutVakit, tasima, sutEngel, bskTab, bosluk |
| `kafa2_zemin.rs` | gam, kayipF (`KayipF::deger` = lossAt), ileriKur, ileriTab, ileriDeger, ilerleme, zemin, sahiplikKur (`Sahiplik::oku` = SAH_F), zeminS, yerDeger |
| `kafa2_tarti.rs` | bekYol, pDunya, zeminH, surD, pencere, enIyiPas, nodeV, durumD, prGam, prensipDegerle, prensipDegerle0 (m._prT dahil; `Cx` = JS cx) |
| `kafa2_karar.rs` | riskOf, otFactor, decideOT, kayipYer, decide, decide0, decide0K (budama, tartG: tut / sür / gönder / pas-önüne-kenardan, dalgalar, enerji muhasebesi bağı, kenardan sekme bankHedef/bankBir/kancaK, kenardan kendine, çalım, Bekçi süresi, dok "diger"), dalgaKos, realize |
| `kafa2_eski.rs` | Eski kafa (`!PR`): kontraKur, zinPay, alAt, bekDuelP, secondStep, tercih, amac, tekSut (tek dokunuşla şut), eskiSonra (kontra / risk / maç zamanı / tercih / amac / fake) |
| `gecici_dis2.rs` | Yabancı işlevler (aşağıda) |

Dışa açık (maç döngüsü için, JS adlarıyla, `pub`): `decide(k, hi, src, ch, rnd, tac) -> Karar { best, opts, T }`,
`decideOT`, `otFactor`, `realize`, `kayipYer`, `predictWorld`, `bekKay`, `bekSav`, `bekYerW`, `bekYol`, `yerDeger`,
`ilerleme`, `sahiplikKur`, `imkan`, `sutTab`, `surD`, `bosluk`, `dalgaKos`, `secondStep`. `freeAt`, `pressAt`,
`respond` kafa1'in (sade kaynakta 3714–3960); kafa2 onları gecici_dis2'den çağırır.

Kullanıcının listesinden kafa1 aralığına düşenler (taşımadım, gecici_dis2'de): imagine, respond, options,
fake (fakeSecenek), şarj (sarjTut, chargeRate), enerji muhasebesi (enMuhasebe, enW, enGec), freeAt, pressAt.
imkan ve sutTab ise kafa2 aralığında (6147, 6270): taşındı.

## Çağrı biçimi

- `h` → `hi: Pid` (dünyadaki sırası). Seçeneğin `q`'su da kararın dünyasındaki sıra (JS `o.q` nesnesi).
  Maç `gor(m, h)` ile kendi dünyasını veriyor: sırası `m.ps` ile aynı olmalı (`gercek()` geri eşler).
- Dünya `&[Arc<Oyuncu>]`: değişmeyen oyuncu aynı `Arc` (JS'teki nesne kimliği: `Rw[i] !== p`, `b2 !== w[i]`).
- `p.markRef` / `p.tutQ` (`Pid`) maç görünümünün (`k.mac.ps`) sırası sayıldı; yoksa kararın dünyası.
  JS gibi önce dünyada id/team ile aranır.
- Maç döngüsü karardan önce `k.mac = Some(MacGor { tick, len, tac, ps, holder })` doldurmalı (JS `D._mac`);
  `k.prT` kararın ardından `m._prT` olarak okunur (shape.js:1359 kullanıyor).
- `rnd`: `&mut dyn FnMut() -> f64` (maçın zarı).

## gecici_dis2.rs: beklenen anlamlar

GEÇİCİ KOPYA (JS'in birebir kopyası; birleştirmede gerçeğiyle değiştirilmeli):
ALAN_Y, ALAN_OZ, ALAN_HZ (EN3: ham Hız), ALAN_SPR (yalnız çekirdek; **maç match.js:2168'de sprint
evresiyle sarar** — `_sf`, `_sfNiyet`, ALAN_SPR_FAZ; sınamalar JS değerini `SINAMA_SPR` ile verir), ALAN_SPRE
(EN2_SPRE, refL 20), ALAN_TEPKI (oyuncu::tepki), threat, pasMax, kuyuD, KY/EN2/Z5/depoOn/holdF (hep açık),
chKayip, alP, zoneR, pressAt, freeAt, tercihEk, dokEk, teK, dokKar, coreSide, coreAng, SPT, kenarAkil, vurTaban.

todo!() (kafa1, decide.js 1–4716): V, Vend, runOn, laneOk(src, team, A, B, arr, lag), keepP(h, src, dir, T)
— yan çıktısı `keepP.son` → `k.k1.keepP_son` (kafa2 okur ve sutVakit'te korur) —, recvCh, shotQ, sutKapali,
respond (değişen oyuncular yeni `Arc`), oppAt, ucusT(L, arrive, al), ucusSure, bankSure, launchFor, bankFor,
imagine / kenarIm (ucusFormul; kuyruk yok, sonuç `JODet`), savurPlan (`SavPlan {k, T, wRel, uSh, caR, A, s}`),
caOf, vurE, sutGuc, sutV, gonEk, sutHedef (sahte oyuncuyla da çağrılır: id −9), surSure(h, L, dir, spr),
sprKop, sarjTut, chargeRate, topK, donS, duelP, enW, enGec (q'nun barının dünyadaki değeri; çağıran q'yu
kopyalar), enMuhasebe, options (koordinatörün imzası: `options(&self, h, src, ch) -> Secenekler { out, tekH,
tekS, tekC }`), fakeSecenek, bosOy, applyError, chasePlan (rakip dizisinin sırasıyla), recvPoint, secim,
SPK (ALAN_SPR_FAZ.ayir), ikKafa (AlanMatch.IK var mı).
Yerleşim (shape.js): AS_bekci(p, B, fm) → Nokta, AS_slotBase(fm, p, B, hucum), AS_gside, AS_gecTab /
AS_gecOf / AS_gec, AS_vSp, AS_S_prHat(blok), AS_S_rcvBuf. `Fm {tac, ps, holder}` = JS `{tac, ps, holder}`;
kısmi holder (`{team, x, y, id: -1}`) sahte oyuncuyla verilir.
Maç (match.js): AM_tl(p, "sav", "adam"), AM_ivme(q, wx, wy, vmax) (q.vx/vy yazar), AM_spd, AM_sprK,
ALAN_KOSU_T / ALAN_KOSU_M (sprint evresiyle koşu), AM_hayali + AM_step (holdWorld'ün hayali maçı).

## Modelde değiştirdiklerim (model.rs)

- `Secenek`: noktalar `Option<Nokta>` (yeni ortak `Nokta`), `spr: Option<f64>`, `duvar: bool`, `dok: Option<Dok>`
  (sıralı sözlük + `sonra` yazısı), `_pw: Option<Arc<Dunya>>`; tartının yazdığı alanlar eklendi (kE, bekci,
  asan, _at, _w, _T, _wE, _en*, look, next, hw `SecHw`, amac `SecAmac`, sonra, geri, ust, cek, why, riskCay,
  zaman, yerlesme, drk, calim, wp, wp1).
- `JODet`: tüm alanlar `Option` (P, ch, Pk, Pl, end*, Ps, Psek, T, Psim, lookTo).
- `KafaD`: `_tekL, _savT/_savW/_savU/_savCa, _gPr, _enWD, _enTm, _sYon: Option<f64>`, `_sut: Option<JsDeger>`
  (sayı ya da şut türü), `_kanca: Option<DKanca>` (yeni), `_itme/_gon: Option<bool>`.
- JS'te D'de olup modelde olmayan `_inLook, _noHold, _noFake, _dalgada, _zin, _mac` → `Kafa2Durum` / `Kafa.mac`.

## Kuşkulu / eksik

- **ALAN_PRENSIP sabit açık sayıldı** (JS'te `null == self.ALAN_PRENSIP && (= true)`; sayfa kapatmazsa).
  O zaman `PR = !_inLook`; `_inLook` yalnız holdWorld ve lookAhead'de açılır, ikisine de bugün hiçbir yol yok
  (holdF() hep true, D.look false). Yani `!PR` dalları (kontraKur, sonDunya'nın hayali dünyası, recvVal,
  sonPay, sekPay, tekSut, tercih, amac, fake, kontra/risk/macZ) **bugünkü maçta ölü**: taşındı ama sınanamaz.
  Taşınmayanlar (hiçbir kipte erişilmez): zincirKur (`!PR && !_inLook`), sonraDeger, surSonra, bekTart,
  lookAhead/reuseLook/saveLook, `!ALAN_PRENSIP` bloğu, dirK/sabK (sabit 0).
- Kapsam ölçümü (V8, 1500 tik, 909623): ölü bulunanlar holdWorld, secondStep, look*, amac, tercih, bekDuelP,
  bosluk, zincirKur, kontraKur, zinPay, sonraDeger, paylastir, realize; decide0K'nın `!PR` blokları.
- `enGec` geçici `q.en` değişikliği: JS q nesnesini yerinde değiştirir; burada q'nun kopyası tartının
  bağlamına (`cx.src`, `cx.h`) konur. Önceden kurulmuş dünyalardaki aynı nesne etkilenmez → nadir bit farkı olabilir.
- `h._sfNiyet` (tartı sırasında taşıyanın sprint niyeti, match.js ALAN_SPR / kosuPr okur): `k.k2.sfNiyet =
  (id, team, niyet, itK)` olarak tutulur; maç modülünün ALAN_SPR / ALAN_KOSU_* bunu okumalı.
- imkOrt: `p.a`'nın sayısal alanlarının ortalaması; toplama sırası JPA alan sırası (JS anahtar sırası
  varsayıldı). Yeni biçim kayıtlarda `p.a`'da başka anahtar varsa farklı olur.
- pDunya anahtarı `toFixed(2)` ile; Rust `{:.2}` yalnız tam ortadaki değerlerde ayrı yuvarlar (yalnız önbellek isabeti).
- ileriKur'un sade oyuncu kopyasında JS `tx/ty` tanımsız → NaN (aritmetik ve karşılaştırmada aynı davranır).
- imkan'ın `iz` (döküm) çıktısı, imkParmak/imkDenet (şut tablosu parmak izi uyarısı: işlev kaynak metninin
  karması; D.imkEski hiçbir yerde okunmuyor) taşınmadı. ALAN_IMK_* / ALAN_PW_IZ / ALAN_PAS_IZ / ALAN_DUR_IZ
  ayarları tanımsız varsayıldı (varsayılan dallar).
- D._kuyruk (iş kuyruğu): imagine = ucusFormul, kuyruğa iş koymuyor → kuyruksuz.
- `menzil(p, Q)` → `temas::menzil(p, 0.35)` (IK.yavas varsayılanı).
- dalgaKos paralel değil (ALAN_PARALEL yok); her tartı D'nin `_` alanlarının girişteki kopyasıyla.
- realize: makeBall'un `vur` tabanı vurTaban; temasın Kural'ı CekirdekKural.

## Sınamalar

- `motor/tests/kafa2_saf.rs` + `sina/kafa2-vektor.js` (decide.js kopyasına iç adlar dışa açılarak; gerçek maç
  909623, 35 an): imkSap/imkTau/imkSg/imkInan, ALAN_SPRE/ALAN_TEPKI, threat/pressAt/freeAt/kuyuD, sutEngel,
  geriCekil, paylastir, sutTab (şutçusuz, Bekçi yeri verili; tablo + gövde yolu), riskOf, prGam.
  Sonuç: **hepsi bit bit aynı** (oyuncu 1960, nokta 1680, sutEngel 420, geriCekil 7840, paylastir 3920,
  sutTab 280, riskOf/prGam 420 değer; imkân eğrileri 400 t). sutTab'da imkOrt'un toplama sırası da tuttu.
  Not: imkKov'un önbelleği `D._mac.tick` ile sıfırlanır; `D._mac` yoksa hiç sıfırlanmaz (JS de öyle).
- `motor/tests/kafa2_karar.rs` (`#[ignore]`) + `sina/kafa2-karar-vektor.js`: gerçek maçta predictWorld, bekKay,
  imkan, sahiplikKur ve decide çağrıları (iç çağrılar dahil; decide.js kopyasında sarmal) girdi/çıktı/zar ile.
  Birleştirmeden sonra `cargo test --release -- --ignored kafa2`. Oyuncunun kafanın okuduğu alanları
  (job, markRef, tutQ, _kv, otx, slot, tl, eg, _sf…) yazılır; `_sf`/sprint evresi maç modülüne bağlı.
- Vektör üretimi: `node sina/kafa2-vektor.js <sade-dizin> <kayit.json> sina/kafa2-vektor.json`,
  `node sina/kafa2-karar-vektor.js <sade-dizin> <kayit.json> sina/kafa2-karar-vektor.json [tik]`.
  Sade dizin: alan-v5'in kopyası, üstüne `kaynak/*.js`. Şut tablosu: `node sina/kafa2-imk-uret.js ../alan-v5`.
