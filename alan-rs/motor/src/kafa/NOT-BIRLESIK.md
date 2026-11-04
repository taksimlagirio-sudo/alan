# Kafa birleştirme notu (kafa1 + kafa2)

NOT1.md (kafa1) ve NOT2.md (kafa2) taşıma notlarıdır; bu not birleştirmeden sonraki durumu anlatır.
Ayrı yazılmış yerlerde bu dosya geçerlidir.

## Tek `Kafa` (`kafa1_baglam.rs`)

`kafa2_baglam::Kafa` artık `pub use super::kafa1_baglam::Kafa`. `Kafa1Durum` ve `gecici_dis2.rs` silindi.

| Alan | Anlamı |
|---|---|
| `D: KafaD` | JS `AlanDecide.D`. Tek kaynak `kafa1_param::d_js()`; `kafa2_baglam::kafa_d()` onu çağırır (maç `mac::gecici_dis::kafa_d` bunu kullanıyor). İki tanım alan alan aynıydı. |
| `dis: Arc<dyn Dis>` | Dış dünya (maç, yerleşim, enerji). |
| `SAH_F: Option<Arc<Sahiplik>>` | JS `SAH_F`. decide0 kurar, sonra eski değerine döndürür; kafa1'in options'ı `oku(t, x, y)` ile okur. |
| `keepP_son`, `kesisK_son: Cell<f64>` | JS `keepP.son`, `kesisK.son`. kafa2 `get()`/`set()` ile okur ve korur. |
| `ucus`, `suty`, `ge`, `ge_ic`, `rnb`, `sht_ek` | kafa1'in önbellekleri. |
| `mac: Option<MacGor>` | JS `D._mac`, kafa2'nin okuduğu görünüm (tick, len, tac, ps, holder). Maç döngüsü kararın önünde doldurur. |
| `prT: Option<kafa2_baglam::PrT>` | JS `m._prT` (kafa2 yazar). `PrT::d1()` bunu kafa1'in `Dis::prT` biçimine çevirir. |
| `k2: Kafa2Durum` | kafa2'nin önbellekleri ve bayrakları (PWM/PWT, hold_c, BSK, IMKB, IMKK, IT, IMK_OFS, IMK_INAN_T=60, `_inLook`…). PWT=-1e9. `SAH_F` buradan `Kafa`'ya taşındı. |
| `srcC_kc: Option<RefCell<KOnbellek>>` | JS `srcC._kc`, kararın K önbelleği. decide0 her kararda yenisini kurar, karar bitince eskisini geri koyar. |

`Kafa::yeni(dis)` iki tarafın başlangıç değerlerini birlikte kurar.

Adı çakışan yöntemler: `tick()` = JS `D._tick || 0` ve `tick0()` = JS `D._tick ?? 0`, kafa1'in anlamıyla.
kafa2'nin eski `tick()`'i (Option) yerine `k.D._tick`, eski `tick0()`'ı yerine `k.tick()` yazıldı.

Nokta tipi tek: `kafa1_js::Nk` = `model::Nokta` (`pub use`).

## gecici_dis2 → `kafa2_bag.rs`

kafa2'nin dosyaları bu modülü `dis` adıyla çağırır. "GEÇİCİ KOPYA"lar silindi, her işlev gerçeğini
çağırıyor:

- **kafa1 yöntemleri**: `k.X(..)`. İmza farkları burada uyduruluyor:
  - `respond`: `Vec<Option<Nk>>` → dünya (`Some` ise `{...p, x, y}`).
  - `imagine`/`kenarIm`: `Ucus` → `JODet`.
  - `ucusSure`/`bankSure`: `JOLaunch` → `Nk`.
  - `surSure`: `spr` JS doğruluğuyla.
  - `chargeRate`: `ham = false`.
  - `duelP`: `zorK = None`.
  - `enW`: Cow → Arc; değişmediyse aynı Arc.
  - `enGec` → `enGec_en`.
  - `options`: tekS/tekC yalnız `h.id >= 0` (TOPLA) iken `Some`; tekC adayların `P`'si.
  - `applyError`: `ot = 0`, JS `undefined || D._ot || 1`.
  - `chasePlan`/`recvPoint`: `YolNokta` → `Nk`.
  - `secim`: sıra numarası → seçenek.
  - `SavPlan`, `SarjTut`, `BosOy`, `SPT` kafa1'in tipleri.
- **K'nin kipi**: JS'te `launchFor`, `ucusSure`, `bankSure`, `bankFor` decide0'ın içinde hep `srcC` ile çağrılır;
  burada da `k.srcC_kc` ile çağrılıyor. `imagine`/`kenarIm` düz `src` ile çağrılır, `Kc = None`.
  Eski kafanın kontra pası `launchFor0`'ı kullanır (düz dünya, `Kc = None`).
- **Maç / yerleşim**: `k.dis.X(..)` üzerinden gidiyor:
  - Doğrudan karşılığı olanlar: ALAN_SPR, ALAN_SPRE, ALAN_KOSU_T/M, gecTab/gecOf/gec, gside, bekci (m = null), ivme, IK, S().rcvBuf.
  - `ALAN_TEPKI` → `k.TEPKI` (yerleşimin `tepki_s`'iyle; eskiden `TepkiS::default()` idi).
  - `AS_vSp` → `k.vS`, `AM_spd` → `k.spd`.
- kafa1'de karşılığı olmayan küçük JS yardımcıları burada gerçeğiyle duruyor: dokEk, teK, dokKar.
  Sabit bayraklar da burada: KY, EN2, Z5, depoOn, holdF hep `true`.
- `vurTaban`: çekirdeğin core.js:410 formülü. kafa1'in `top_yap`'ındakiyle aynı.

## gecici_dis1'deki kafa2 işlevleri

Kafa2'nin gerçeklerine bağlandı:

- `predictWorld`: her oyuncu için `Some(yer)`. JS'te `Pw[i]` hep bir nesnedir; ucusFormul `kj2t`'yi her rakip için kurar.
- `bekSav`: artık `Op` döner.
- `bosluk`, `sutEngel`, `yerDeger`.
- `riskOf`: kafa2'nin `Tac`'ıyla.
- `sonraDeger`: yeni taşındı (`kafa2_eski::sonraDeger`, decide.js:5639).
- `dalgaKos_var` silindi: kullanılmıyordu.

`sonraDeger` için ek bilgi:

- Yalnız eski kafanın fake dünyasında koşar; bugünkü maçta erişilmez (`!PR`). Sınanmadı.
- `hc` dünyada id ve takımla bulunur (JS'te aynı nesnedir).
- `kon` verilmez.

Bunun gereği olarak kafa1'de şu imzalar değişti:

- `fakeSecenek(h, src, ch, tac: Option<&Tac>, opts, kon: &mut dyn FnMut(&mut Kafa, f64, f64) -> Option<f64>)`.
  - `kon` kafa2'nin KF'sidir (kontraKur). Eski kafada eskiSonra'nın son parametresi oldu; eskiden kullanılmayan `_loss` parametresinin yerine geçti.
  - `tac.tempo ?? .5`.

**`&mut self` olanlar** (kafa2'nin önbelleklerine ve D'nin geçici değişimine dokunan yolda, bkz. ileriTab):
keepP, kesisK, girEsik, girThr, girEk, Vend, runOn, imagine, ucusFormul. kenarIm ve fakeSecenek zaten
öyleydi. ucusFormul D'yi ve dış dünyayı yerel kopyadan okur: `KafaD` her çağrıda klonlanıyor (küçük bir
maliyet; JS'te D paylaşılır, ucusFormul içinde değişmiyor).

## Dis'e eklenenler

Hepsinin varsayılan gövdesi `todo!()`; var olan yöntemlere dokunulmadı.

| Yöntem | JS kaynağı | Kim çağırır |
|---|---|---|
| `bekciFm(p, B, fm: &Fm) -> Nk` | shape.js bekci(p, B, m), maç görünümü verili | bekSav, bekDuelP, sutTab, imkan, ileriKur |
| `slotBase(fm: &Fm, p, B, hucum) -> Nk` | shape.js slotBase(m, p, B, hucum) | ileriKur |
| `S_prHat(blok) -> Option<f64>` | shape.js S.prHat[blok] | ileriKur |
| `tl(p, ev, k) -> Option<f64>` | match.js:939 | predictWorld (adam adama) |
| `sprK(p) -> f64` | match.js:311 | bekSav |
| `hayaliMac(k, tohum, src, hi, ch, tac) -> HayaliMac`, `hayaliAdim(k, m)` | createMatch + kopya + step | yalnız holdWorld (eski kafa, ölü) |

`Fm { tac: &[JMTacE], ps: &[Op], holder: Option<&Oyuncu> }` ve `HayaliMac { ps, holder, ch }` gecici_dis1'de.
Maç tarafında gerçeklemenin örneği `tests/kafa2_karar.rs`'teki `tdis::TDis`'tedir: `sekil::yer::bekci` +
`BekM`, `sekil::yer::slotBase`, `sekil::sabit::prHat`, `mac::kurulum::tl`, `mac::enerji::sprK`.

`Fm.tac` boş verilebilir (bekDuelP, bekSav: JS `{tac: [], ...}`). JS'te o zaman `m.tac[t]` tanımsızdır ve
slotBase'in duran top kayması atlanır. MacDis kendi maçının taktiğini kullanacaksa bu farka dikkat edilmeli.

## Kuşkulu noktalar ve maçın yapması gerekenler

- **İki `PrT`**: kafa2 `k.prT` yazar (JS `D._mac._prT`). kafa1'in girEsik'i ise `dis.prT()` okur. JS'te ikisi
  aynı nesnedir. Maç `k.prT.as_ref().map(|p| p.d1())`'i MacDis'e vermeli; MacDis'te `prT` zaten var
  (`m.mac.prT_k`).
  `D._mac` de iki yerden görülüyor: kafa2 `k.mac`'i, kafa1 `dis.mac_var()`/`mac_ch()`/`mac_holder()`'ı okuyor.
  Maç ikisini tutarlı kurmalı.
- `h._sfNiyet`: tartı sırasında `k.k2.sfNiyet`te. Maçın ALAN_SPR'si bunu okumalı (NOT2).
- `AS_bekci(None)` yerleşimden `None` gelirse oyuncunun yeri döner (kafa1'in runSut'ı da öyle yapıyor).
  `AS_gecTab` → `Option`; yoksa gecOf 0 sayılır.
- `menzil(p, 0.35)`: kafa2 sabit IK.yavas kullanıyor, kafa1'in `k.menzil` maçın IK'sını. JS'te ikisi de 0.35.
- Taşınmayan JS: zincirKur, surSonra, bekTart, lookAhead (NOT2'deki gibi ölü).

## Sınamalar

- `cargo test --release --no-fail-fast`: hepsi geçiyor.
  - kafa1.rs bit bit aynı (&mut değişikliğinden sonra da).
  - kafa2_saf.rs bit bit aynı. `SINAMA_SPR` yerine sınamanın kendi `SinamaDis`'i var: SPR JS'ten, SPRE
    EN2_SPRE formülüyle (eskiden gecici_dis2'nin kopyasıydı, sınamaya taşındı).
- `cargo test --release --test kafa2_karar -- --ignored`: dış dünya, vektörün dünyasından kurulan bir
  `Mac`'in `MacDis`'i ve yeni yöntemler için `TDis`.
  - `kafa2_dunyalar_js_ile`:
    - predictWorld: 40 çağrının hepsi bit bit aynı.
    - bekKay: 60 çağrının 55'i aynı. 5'i (t36, aynı Bekçi) yalnız koşu boyuyla ayrılıyor: yön aynı, yani
      bekci aynı. Oran 1,1035; JS'in ALAN_SPRE'si ~1,214, bizimki 1,1. Maçın o anki sprint durumu
      vektörde yok (`_sprIst` vb.), kafa farkı değil.
  - `kafa2_imkan_sahiplik_js_ile` ve `kafa2_decide_js_ile`: **koşamıyor**. Yerleşimin `sekil::yer::bekci`'si
    (m verili yol) `sekil::gecici_dis::govdeV`'e uğruyor; o kafa işlevi orada henüz `todo!()`
    (sekil → kafa bağı yapılmamış).
  - Bu bağlandıktan sonra da decide'ın bit bit tutması maçın vektörde olmayan durumuna bağlı: sprint
    evresi, kayıp, top uçuşu.
