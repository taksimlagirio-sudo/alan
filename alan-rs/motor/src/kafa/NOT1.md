# Kafa1 notu (decide.js 1–4716)

## Yapı

- `Kafa` (`kafa1_baglam.rs`): JS kapanışının değişkenleri. `D: KafaD` (JS `AlanDecide.D`), `dis: Arc<dyn Dis>`
  (dış dünya), `SAH_F` (kafa2'nin sahiplik haritası; birleştirmede kafa2'nin tipine bağlanacak), `keepP_son` /
  `kesisK_son` (JS `keepP.son`, `kesisK.son`; Cell), önbellekler (`UCUS`, `SUTY`, `GE`/`GEk`, `RNB`; RefCell).
  Gizli küresel yok. Bütün işlevler `impl Kafa` yöntemi, adları JS'teki gibi, `pub`.
- Dünyalar `&[Op]` (`Op = Arc<Oyuncu>`, kafa2 ile ortak, `kafa1_js.rs`). Kimlik (`p === h`) başvuru eşitliği
  (`ayni`, `ptr::eq`): dünyadan alınan oyuncu kendisiyle eşit, kopya (JS `Object.create`, `{...p}`) değil.
  ucusFormul içinde kimlik sırayla (Bekçi savrulunca dünya kopyalanır).
- K'nin kipi: JS `pack(src)` / `src._kc` → `Kc = Option<&RefCell<KOnbellek>>` (yalnız predict/stepBall kullanan
  işlevlerde: ucusFormul, imagine, launchFor, bankFor, ucusSure, bankSure, siddetFor). kafa2 `srcC`'yi buna verir.
- Dosyalar: `kafa1_param` (DU, D, SAV, GON, UC3/UC5…), `kafa1_tehlike` (threat, oppAt, holdVal, pressAt, freeAt,
  zoneR, recvCh, build, space, front, Vend, runOn, runSut, tercihEk, dagin, V, V0, tekF, laneOk, respond,
  kuyuD), `kafa1_sut` (sutT, sutKapali, sutHedef, acikAci, shotCell, shotTab, shotQ, sutUC, TRF…), `kafa1_sut_tablo`
  (shot-table.js ALAN_SHT_VE, üretildi), `kafa1_savur` (savurPlan, onPlan, rakipKovala, savurYol, savurEngel, caOf,
  govdeV, vurE, sutV, vmaxOf, sutGuc), `kafa1_hata` (hataF, applyError, applyErrorSav, pasK, pasKv, SUT_TABAN, gonEk,
  pasEk, sogK), `kafa1_ucus` (ucusFormul, imagine, kenarIm, reach*, solveV, siddetFor, launchFor, solveTo, ucusYol,
  ucusT, ucusSure, bankSure, bankFor, effFric), `kafa1_varis` (sekYaris, frenT, kacanAl, akHiz, alHiz, kosYer,
  kosHiz, bekCikar, rbAt, chasePlan, recvPoint; touchCap/ctrlP/vurusOku'nun başvuru alan kopyası), `kafa1_ikili`
  (keepP, kesisK, presRol, duelP, govdeVurP, coreSide, coreAng, trapped, girEsik, girThr, girEk, commitThr,
  oyalaEk, siddetQ, enIkiliEk, kWof, yanilT, topK, donS, donT, cevK, dolanma, fakeOkunmaz, ikDen, ikHiz),
  `kafa1_enerji` (enMuhasebe/enMuh2, enW, enGec_en, sarjBd, sarjMal, sarjVerim, sarjKarar, sarjTut, chargeRate,
  sprKafa, surSure, SPK, SPRS, sprKop, itT, itV, itKf…), `kafa1_secim` (secim, secimN, secimW, secimZ, sigY, sHash,
  tartiK, ufukK, kenarAkil), `kafa1_fake` (fakeSecenek, fakeP, fakeT, fakeHedef, fakeIlgili, alP).
- `options` (2808–3491) bu ajanda DEĞİL: koordinatör kendisi taşıyor (`kafa1_secenek.rs`, ayrı ağaç).

## Taşınmayanlar (bugünkü davranışta ölü)

- `simOne`, `ucusMac`, `firstTouch`, `looseWin`, `yuru`, `kopyaDurum`: `imagine` bugün yalnız `ucusFormul`'dür;
  `ucusMac`'e yalnız `gorevYap` (karar dışı iş) ulaşır, onu kuran yer yok.
- `gorevYap`, `gorevOyna`, `kuyrukBosalt`, `imSonuc`, `sonucYap`, `altZar`, `dAnlik`: yalnız `sonF()` kapalıyken
  (fakeSecenek'in eski yolu) ya da `isler`'li imagine sonucunda; bugün ikisi de yok. kafa2'nin `kuyrukBosalt` /
  `imSonuc(im0)` çağrıları (`im0.isler` hep yok) atlanabilir.
- `shotCell`'in hesap yolu (AlanMatch.createMatch ile ölçüm): tablo (ALAN_SHT_VE, sürüm "v5-22 · vuruş enerjisi ·
  direk 0.3") motorla uyumlu ve shotTab'ın istediği her hücreyi (d 0–19, a 0–7, c 0–6) içeriyor; tablo dışı → 0.
- `build`'in useCurve/useValTab dalları (ikisi kapalı), `enMuhasebe`'nin Enerji 1 dalı (EN2 açık), `duelP`'nin
  ikili kafası kapalı dalı taşındı ama `respondEski`, `along` taşınmadı (kullanılmıyor).
- Bayraklar sabitlendi: `ALAN_YET.acik(n)` hepsi açık, `tekU`, `KUYU`, `EN2/EN3/SUREKLI`, `ALAN_PRENSIP`,
  `ALAN_KACIR`, `SAL0`, `sonF`, `holdF`, `VE`, `SHT_SAVUR` doğru.

## gecici_dis1.rs

`trait Dis`: dış modüllerin işlevleri (`todo!()` varsayılan gövdeli; maç modülü gerçekler, sınama
`tests/kafa1.rs`'teki `KayitDis` kayıttan verir). Anlamları dosyada her yöntemin başında (JS dosya:satır):
- enerji/sprint: `ALAN_SPRE`, `ALAN_SPR` (maçın sprint evresiyle; `Kural::spr` de bu), `ALAN_SPR_HK`,
  `ALAN_SPR_FAZ` (sürekli koşu kopyası: patT 1, frenT 4, dolT 0, after 1, patK 1), `ALAN_KOSU_T/M`
  (ALAN_KOSU_PR'nin {Ts, k} profiliyle), `ALAN_EN3_KUL/SGIR/DOL/TEPE`, `ALAN_EN2_TS/SB/SD`, `ALAN_EN_S`,
  `ALAN_EN2`, `ALAN_EN` (sabitler), `ALAN_DENGE_KAYIP`, `ALAN_DOKUNUS` (bugün tanımsız: None),
  `ALAN_YOL_VER_TEPKI`, `ALAN_TASIYICI_SONRA_YAVAS` (tanımsız: false).
- yerleşim: `tepki_s` (ALAN_TEPKI'nin S parametreleri), `S`, `GR`, `GR_yasHata`, `gecTab` (`refsiz`: JS `ref: null`
  kopyası), `gecOf`, `gec`, `gside`, `bekci`, `aliciDirenc`, `tehlike`, `ilkPres` (qv.tx/ty'yi yazar),
  `AS_ilkPres_var`.
- maç: `IK`, `ivme`, `kutle(p, p===h)`, `direnc(p, p===h)`, `omuzK`, `durtFark`, `sendS`, `ikEn`, `BEKLE_BOZ`;
  `D._mac`'ten okunanlar: `mac_var`, `mac_ch`, `mac_holder` (id, team, x, y), `tac_enHarcama`, `tac_siddet`
  ((siddet, yz, macEff.macK ?? 1)), `tacEff`, `sh_gs`, `prT` (m._prT, `hm.k` kümesi `hmk`).
- algı kopyaları: `algi_e` (`__e`), `algi_yas` (`__yas`): modeldeki Oyuncu'da bu alanlar yok (yalnız seen
  kopyalarında): dış dünya verir.

Kafa2'nin işlevleri (serbest, `todo!()`; birleştirmede `self.X`): `predictWorld` (yalnız yerleri döner:
`Vec<Option<Nk>>`), `bekSav`, `bosluk`, `sutEngel`, `yerDeger`, `riskOf`, `sonraDeger`, `dalgaKos_var`
(kullanılmıyor: fake dalgası sırayla koşar, sonuç sıradan bağımsız).

## Model değişiklikleri (model.rs)

- `KafaD`: `_sut: Option<JsDeger>` (taktik sayısı ya da "güçlü"/"plase"), `_savT/_savW/_savU/_savCa/_sYon/_gPr/
  _enWD/_enTm/_tekL: Option<f64>`, `_kenar/_itme/_gon: Option<bool>`, `_kanca: Option<JD_kanca>` (yeni yapı
  {ca, w0, s}); eklendi: `_noFake`, `_inLook` (bool). (kafa2 de D'yi yazar: çakışırsa birleştirmede.)
- `JODet`: eklendi `T`, `sonra`, `c` (Option; fake seçeneği). `Secenek`: eklendi `show: Option<JOShow>`.

## Kuşkulu / bilinçli kararlar

- `topK`, `keepP`: `_dkP`, `slowT` modelde yok (çalışırken görülmemiş): tanımsız sayıldı (0 / yok).
- `coreSide`: `h.tx` modelde hep sayı; JS'te tx'siz sanal oyuncu (ör. `{x, y, team}`) bu dalı atlar, Rust'ta
  Oyuncu::default'un tx'i 0'dır. Kafa1'de böyle sanal taşıyıcıya coreSide çağrılmıyor.
- `respond`, `ucusFormul`: `markRef`, `tutQ`, `_kalH` Pid; dünyada aynı sırada olduğu varsayıldı (`src[pid]`).
- `Math.tan` (sutHedef): sabit-mat'ta yok, JS de yerel; Rust `f64::tan` (platformlar arası bit kuşkusu küçük).
- `toFixed` (enMuh2): tam ondalık açılımla, yarıda büyüğe (`to_fixed`).
- Önbellekler: JS'te değeri girdisinden belirlenen kimlik anahtarlı önbellekler (gtab GTC, sutKapali SKW, space
  SPC, front FRC, siddetQ SDQ) taşınmadı: her çağrı yeniden hesaplar (aynı sonuç; hız sonra). `GE`/`GEk` (girEsik)
  ve `RNB` (runSut) JS'teki gibi ilk hesaplayanın değerini tutar (anahtar konumu yuvarlar): korundu.
  `ge_vk_koy`: GEk'e değer koyma kancası (sınamada yerDeger kafa2'nin olduğu için).
- touchCap/ctrlP/vurusOku: temas.rs'teki sıralı (pi) sürümün başvuru alan kopyası (`kap_al`, `ctrl_p_al`,
  `vurus_oku_al`): kafa hızı değişmiş/sanal oyuncuyla ve dünyada olmayan vuranla çağırır.
- `enGec`: JS q'yu yerinde değiştirip geri alır; Rust'ta `enGec_en` yalnız yeni barı verir, çağıran uygular.
- `secimZ`: `o.dok` modelde tutar/durum hep sayı: dok varsa doğrudan o kullanılır (JS'te gönder'in dok.gol dalı).
- fakeSecenek'in dalgası (dalgaKos) sırayla koşar.

## Sınama (tests/kafa1.rs + sina/kafa1-vektor.js)

Bkz. aşağıdaki sonuç tablosu (sınama sayısı / fark).
