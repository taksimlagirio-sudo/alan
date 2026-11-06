# mac: maç döngüsü (kaynak/match.js) — taşıma notu

## Dosyalar

| Dosya | İçerik (JS) |
|---|---|
| `mod.rs` | ortak: `MacEk` (taslakta olmayan maç alanları), `Olay`/`ev`, `Fake`, `Perde`, `KenarW`, `HamA` (sıralı `p.a`), JS yardımcıları (`cl`, `dirOf`, `ownX`, `or0`, `gt`, `to_fixed`, `KX/KY`) |
| `tac.rs` | `Tac` (talimat, bütün alanlar `Option`, `ust` = `{...a, ...b}`), TAC0, GECIS_VARS, KP_VARS, geriSay, onRef, gecisBasla, gecisGuncelle, tq0, inWin, kpAkt, kpEff, yzUyum, tacEff, macW, fak, macEff (`MacZ` = macZ kapanışı veri olarak), tacEff0, BLOK, GEN, EVKOD, evreBul, evreUygula |
| `kurulum.rs` | createMatch, SLOTS, NAMES, dizSlots, dizUygula, TLV, PAKET, tl, tlKur, RSLOT, markAta, assignRoles, roleSample, kickoff, slotPos; `KurDis` (yerleşimin slotBase/bekci'si; sınamada kayıttan) |
| `kayit.rs` | mac-isci.js: `Kayit` (kayit-*.json `yeni`), kurUygula, egUy, rastgele, `macKur` (createMatch → rastgele → kurUygula → kafaAc) |
| `enerji.rs` | core.js enerji sabitleri/işlevleri (EN, EN2, EN3, SPR_FAZ, EN_S, EN2_SB/SD/TS, EN3_TEPE/DOL/SGIR, HZ, SPRE), **ALAN_SPR sarması** (`ALAN_SPR(p, &MacKural)`), sprK, sfPk, hkK, kosuPr, ALAN_KOSU_T/TP/M, ALAN_SPR_HK, enSay, verim, enHarca, enerji2, enCal, sertT, alanYz, alanSec, sprintKarar3, ALAN_EN3_KUL, sprintBasla, sfKesin, sprFaz, vK |
| `hareket.rs` | topHiz, ivme, dolan, iliski, perde, otAkil, yanil, fakeBasla, fakeYonet, topaYigilma, korumaBaski, outOfKuyu/kuyuSafe (Kuyu açık: boş), corePos, savurBasla, savurAdim, onHareket, kancaOf, `r#move` (JS move) |
| `ikili.rs` | IK, ikEn, omuzK, durtFark, kutle, direnc, ikiliTemas, govdeAyir (`IkBag` = JS IKT/IKM/IKL), sendS, denge, dengeKayip (ALAN_DENGE_KAYIP), dengeHiz |
| `kuyu.rs` | kuyuCikYon, kuyuCikT, kuyuSaat |
| `vurus.rs` | makeBall (yeni top kimliği), vurTaban, launch, BODY_R/E |
| `kafa_bag.rs` | KS, kafaAc, kafaBitir, kafaBasla, kafaCikis, kafaNot, kafaSapma (yazılar dahil) |
| `dongu.rs` | take, step (notlarla sarılı), _step (alıcının önceden kararı, Bekçi süresi, tepki, kenar çalımı, taşıyıcı: şarj, dönüş/fiske, gövdeyle vurma, omuz/dürtme sonucu, ikiliye giriş, karar; boştaki top: durdu/stepAll/dalış/direk/gövde/sayı), run, amBitir, amYok, gor, BEKLE_BOZ |
| `notlar.rs` | NOT_K, notTik, notSayi, notlar |
| `gecici_dis.rs` | kafa ve yerleşim işlevleri (todo!()), `Karar`, `FakeShow`, `SavurPlan`, `RakipK`, D ve DU sabitleri |

## Küresellerin karşılığı

- `self.ALAN_SPR` (sprint evresine göre sarılı) → `enerji::ALAN_SPR(p, &MacKural)`. `MacKural` = `temas::Kural`
  (`impl Kural for MacKural`): `mh` = JS `self.__mH` (son karar veren taşıyıcı; `m.mac.__mH`'ta), `tick`
  (JS `D._tick || _mt`), `holder`/`flq` (JS `_mM.holder`, `_mM.fl.q`: kosuPr'nin sprKar'ı). `mac_kurali(&m)`.
  Çekirdek temas işlevlerine (contact, stepAll, ctrlP, bekHar, kuyuGirer) bu verilir.
- `self.ALAN_KOSU_T/TP/M/PR`, `ALAN_SPR_HK`, `ALAN_SPRE`, `ALAN_EN3_*`, `ALAN_DENGE_KAYIP` → aynı adlı işlevler.
- `ALAN_SPR_FAZ` alıcısı → `SPR_FAZ` (sürekli koşuda döndürülen değerler: patT 1, frenT 4, dolT 0, after 1, patK 1).
- `ALAN_EN2.pat` alıcısı → `EN2.pat = 0` (ALAN_SUREKLI açık). `ALAN_EN.sprint/dolT` match.js'in değiştirdiği değerlerle.
- `_mt`, `_mH` (move başındaki tik ve taşıyıcı) → `vK(p, ph, mt, k)` parametreleri.
- IKT/IKM/IKL → `ikili::IkBag` (IKM: `IkHedef { st, lite }`).
- `D._tick`, `D._mac` yazılmaz (kafa `m.tick`'i okusun). `D._kanca` → `decideOT`'un `kanca` parametresi.

## Bugünkü bayraklarla sadeleşen (taşınmadı, bilerek)

- EN2/EN3 açık: `enerji` = `enerji2`; `sprintSec` = `sprintKarar3` (sprintDeger, sprintKarar2, kosuHata ölü).
  sprintKarar3'te SPK (sürekli kat) dalı; üç kademeli (tempo/hızlı/sprint) dal ölü.
- ALAN_DOKUNUS tanımsız (dokunuşla sürme kapalı): `dokunus`, `dkHiz`, `_dkb/_dkv/_dkL` yok; move'un dokunuş bloğu yok.
- KUYU açık: `outOfKuyu`, `kuyuSafe` boş; Bekçi süresi dolunca santra rakibe (BEK_SURE dalı ölü).
- `arkGecir` hep false; `b._gec`, `p._iv`, `p._sfNiyet` hiç yazılmıyor → okunmadı.
- `position`, `offBall`, `laneOpp` (yalnız AlanShape yokken): taşınmadı. `slotPos` taşındı (dışa açık).
- `fazK`, `sfTopla`, `STYLES` kullanılmıyor: taşınmadı. `perde` taşındı ama step'te çağrılmıyor (JS'te de).
- `m.lite` (yazım hatası, hiç yok) → launch'ta `!m.lite` her zaman doğru: kalibrasyon sayacı hep tutulur.

## gecici_dis.rs — beklenen anlam

| işlev | anlam |
|---|---|
| `slotBase(m, p, ex, ey, att)` | shape.js: p'nin topa (E) göre yerleşim yeri (x, y) |
| `bekci(m, p, ex, ey)` | shape.js: Bekçi'nin yeri |
| `position(m)`, `bak(m)` | shape.js: topsuzların hedef/iş/sprint/press'i; bakış |
| `metrics_swarm(m)` | shape.js `metrics(m).swarm` |
| `gorulen(m, p)` | p'nin gördüğü dünya (m.ps sırasıyla kopya); `None` = gerçek (JS `G === m.ps`) |
| `gercek(o)` | görülen dünyadaki seçeneği gerçeğe bağla (yerinde; JS dönen değeri kullanılmıyor/aynı nesne) |
| `pressAt`, `oppAt` | baskı ve rakip yoğunluğu (x, y) |
| `threat(team, x, y)` | tehlike |
| `ucusT(d, aF)` | pasın d'ye uçuş süresi |
| `coreSide(m, h)` | Çekirdeğin gövdedeki yönü (ux, uy) |
| `caOf`, `vurE` | taşıyıcının bakış açısı; vuruş enerjisi |
| `savurPlan(...)` | savurma planı (`SavurPlan`; `P` = JS `pl.P || 0`); `None` = yol kapalı |
| `onPlan(...)` | ön hareket planı (`JP_onH`'in s, wS, phi, caR, kal, a, Tsw'si) |
| `fakeT/fakeIlgili/fakeP/fakeHedef` | fake süresi, ilgili mi, aldanma olasılığı, aldananın hedefi |
| `gonEk`, `pasEk` | vuruş hatasının katı |
| `applyError`, `applyErrorSav` | vuruşa hata ekle (m.r'den zar: `&mut Rng` verilir) → (vx, vy) |
| `sarjKarar`, `recvCh`, `otFactor`, `chargeRate`, `sarjMal` | şarj kararı, alınan şarj, tek dokunuş katı, şarj hızı, şarjın enerji bedeli |
| `decide(m, p, g, ch, r, tE)` | karar: `Karar { opts, best: opts içindeki sıra, T }`; p `g` içinde kendi sırasında (alıcının önceden kararında p = hc kopyası, g'de onun yerinde) |
| `decideOT(..., ot, kanca)` | tek dokunuş seçeneği (`None` = yok) |
| `secim(m, h, r, g)` | seçilen seçeneğin r.opts içindeki **sırası** (JS nesne kimliği yerine) |
| `secimN`, `secimW` | seçim gürültüsü / sürme devamının payı |
| `rakipKovala(m, o, bx, by, md)` | fiskede rakibin Çekirdek noktasına (taşıyıcıya göre) yaklaşması |
| `govdeVurP`, `duelP`, `enIkiliEk`, `girEsik` (`None` = JS null), `commitThr` | ikili olasılıkları ve eşikleri |
| `V(ps, team, x, y, ch)` | alan değeri (notlar) |
| `D::*`, `DU::*` | decide.js sabitlerinin kopyası (todo değil) |

Kafa `o.det.lookTo` (bakılan oyuncu) taslakta yok: `dongu::dis_lookTo` şimdilik `None` (yalnız olay yazısı).

## Modelde değiştirdiklerim (model.rs)

- `Mac`: `ball: Option<MacTop>` (yeni `MacTop { tt: temas::TopT, ek: TopEk }`, `Deref` → `Top`, kimlik `tt.t.kimlik`);
  `tac: Vec<mac::Tac>`, `tac0: Option<Vec<Option<mac::Tac>>>`, `diz: Option<Vec<Option<mac::Diz>>>`,
  `evre: Option<Vec<Option<JMEvreE>>>`, `_macC: BTreeMap<String, MacW>`, `_perde: Option<mac::Perde>`,
  `_gelenB: Option<Box<MacTop>>`, `_onHk: Option<JP_onH>`. **Yeni**: `ps: Vec<Oyuncu>`, `r: Rng`,
  `events: Vec<mac::Olay>`, `mac: mac::MacEk` (taslakta yoktu; `ps`/`r`/`events` herkesin okuduğu, createMatch yazar).
- `JMPre.r`, `JMCtl.r` → `gecici_dis::Karar`. `JPSavur.o` → `Box<Secenek>`. `JMFl.to` → `Option`.
- `JMKp.sahip` → `Option<Pid>`; `JMNot.s/actor/team` → `Option`.
- `JMSt.enH` → `Vec<EnSayac>`, `enP` → `BTreeMap<usize, EnSayac>` (sıralı ad→değer; "surme", "geri", "dalış" de var),
  `cal` → `Option`; yeni sayaçlar `fake, fakeOk, ot, combo, direk` (boş = yok).
- `JPTl.huc/sav` → `BTreeMap<String, f64>` (yalnız verilen talimatlar; eksik anahtar `kurulum::tl` ile varsayılana düşer).
- `Oyuncu`: yeni `slowT, kenarW, fkBlock, fkCool, gvH, gvCd` (match yazar, taslakta yoktu).
- `Secenek`: yeni `savW, savU, ot, noErr, why, devam, wp, wp1, show, _kafaId` (taslakta yoktu; kafa da doldurmalı: noErr, why, wp, wp1, show).
- `TopEk`: yeni `_dk0`.
- `Default for Rng` (mac/mod.rs'te; Mac'in Default'u için).
- `motor/Cargo.toml`: serde_json `float_roundtrip` (kayıttaki sayılar 1 ulp şaşıyordu; diziliş bit bit aynı olsun diye).

## Sınamalar (motor/tests/mac_*.rs, vektör: `node sina/mac-vektor.js ../alan-v5 sina/mac-vektor.json <kayit>...`)

Vektör üretici match.js'in bir kopyasını iç işlevleri dışa açarak yükler (yalnız dışa açar). Vektör yoksa sınamalar atlanır.

Bit bit aynı çıkanlar (3 gerçek kayıt + 909623'ün 2400 tikinden anlık durumlar):
- **kayıttan kurulum** (`mac_kur`): createMatch + kurUygula + assignRoles + dizUygula + markAta + kickoff;
  oyuncu yerleri, slot/rslot/slot0/rslot0, tl, rh/rs, markRef, eg, R, D, en, _korT, 101 mekanik, ALAN_OZ;
  yerleşime giden her slotBase/bekci çağrısının girdisi de aynı (çıktısı JS'ten).
- `macW`, `macEff` (macK, macZ), `enerji2` (bir tik; en, D, _Dl, _D0, _yog, _enOnce, _bosT; lite ve değil),
  `ALAN_SPR`, `sprK`, `ALAN_SPRE`, `kosuPr`, `ALAN_KOSU_T/M`, `vK`, `ALAN_EN3_SGIR`, `kuyuCikYon`, `kuyuCikT`,
  `ivme`, `kutle`, `direnc`, `omuzK`, `ikEn`, `durtFark`, `sendS`, `dengeKayip`, `dengeHiz` (taşıyıcı olmayan),
  `govdeAyir` (ikiliTemas dahil), `kafaSapma` (yazılar), `tacEff`/`evreBul`/`gecisGuncelle` (`mac_tac`).

Sınanmayanlar (kafa/yerleşim todo!() olduğundan ya da tam maç gerektirdiğinden): step/take/move'un bütünü, alanSec
(pressAt), sprintKarar3 (threat, ucusT), launch, savur, fake, ikili hamleleri, notlar. Bunlar kafa ve yerleşim
bağlanınca tam maç izi (tik tik) karşılaştırmasıyla sınanmalı.

## Kuşkulu / yaklaşık

- **macEff önbelleği sıraya bağımlı** (JS'te de): `_macC[d:floor(kal/120)]` ilk hesaplandığı tikteki kal ile kalır; macK bu yüzden geçmişe bağlı. Bit bit aynılık için korundu; TASARIM-yapi.md ilkesine aykırı, ileride `macW(d, 120·k)` gibi sıradan bağımsız yapılabilir (oyun değişir).

- `null != b.gvx` → `mac::gv_var(b)` = top bir oyuncuya tutuldu mu (`tt.t.holder`); temas yalnız tutulunca yazıyor.
- kafaBasla'da `d.endM` her zaman var sayıldı (taslakta seçimli değil): `endM.x` sonlu ise alış yeri odur.
- kosuPr'nin `_mM` (son hareket eden maç) yerine o anki maç; kafanın kopyalarında JS farklı maçı görebilir.
- `{...TAC0, ...tac}`'ta açıkça `null` verilen alan JS'te null olur, burada TAC0'ınki kalır (kayıtlarda yok).
- JS nesne kimlikleri: top → `kimlik` sayacı (`m.mac.topSay`), seçenek → `_kafaId`, kararın seçeneği → sıra.
- `m.lastDec.opts` gösterim için kısmen dolduruluyor (hw, bulus, am, en, drk, rc, cek, neden, bak, dok yok).
- `m.events` metinleri JS'teki gibi; `js_num` tam sayıyı "3", diğerlerini Rust `{}` biçimiyle yazar.
