# Yerleşim (shape.js) — taşıma notu

Kaynak: `kaynak/shape.js` (sade, 3642 satır) → `motor/src/sekil/`. Sınama: `motor/tests/sekil.rs` +
`sina/sekil-vektor.js` (vektör: `node sina/sekil-vektor.js ../alan-v5 <kayit.json> sina/sekil-vektor.json`;
üretici, shape.js'in iç işlevlerini — yolKesis, presKesis, tasHamle, yolBag, koyHedef, coverSpots, laneOpen,
yolT — yükleme sırasında AlanShape'e ekler).

## Dosyalar

| Dosya | İçerik (JS) |
|---|---|
| `mod.rs` | cl, dirOf, ownX, oppX, hsh, spd, **vSp**, ALAN_SPRE (EN2_SPRE, EN_S, EN2_SB, EN2_TS), TEPKI, **tepki**, lagOf, yerS/yerM/yerH/yerF/niyet/geriK/kurtul |
| `sabit.rs` | **S** (+ S.W, prHat, pressR), **GR**, BLOK, GEN, **DUR_A**, GW/GH/GCY/GKX/GKY |
| `varlik.rs` | `Varlik` (dünya varlığı), `Beden`, `hayal`, `Mc` (bağlam), `SekilDurum`, `Sh` (m.sh), `KpC` |
| `gec.rs` | **gec**, **gecTab**, gecTabV, **gecOf**, gecY, gecYk, gecK, VMQ, IVK; match.js `ivme`'nin `_iv`li hâli |
| `gor.rs` | **bak**, **seen**, **gorulen**, **gercek** |
| `ucus.rs` | **flightRead**, **alici**, **aliciDirenc**, kendiTopu, menzilP, menzilO, okuT (vurusOku her beden için) |
| `yer.rs` | **gside**, **slotBase**, slotBase0, **durusDef**, **durusW**, **bekci**, koyHedef, tasHamle, yolBag, yolKesis, presKesis(T), ALAN_KOSU_TP |
| `pres.rs` | macPres, presLim, **ilkPres** |
| `takim.rs` | laneOpen, yolP, yerV, teamBest, **tehlike**, coverSpots, yolT, kurO, teamV (+kpOf), coverBoth, **kapatHat**, **metrics** |
| `konum.rs`, `konum/*` | **position** (kovala; `savunma.rs` pres/süpürücü/adam; `esles.rs` bölge, UA, yedek, takım; `hucum.rs` güvence, kanat, boşluk araması, yol kes, Bekçi, yol ver, sınırlar; `yardim.rs` bindCak, oppIn, laneIn, ileri, perms) |
| `gecici_dis.rs` | yabancı işlevler (`todo!()`) |

Dışa açık adlar (window.AlanShape): aliciDirenc GR bak gorulen gercek ilkPres tehlike gside vSp S DUR_A
durusDef durusW slotBase position metrics seen flightRead bekci alici gec gecTab gecOf tepki kapatHat — hepsi `pub`.

## Tasarım kararları

- **Bağlam `Mc`**: `m` (Mac), `ps` (m.ps), `ball` (`TopT`, temas.rs), `ek` (`SekilDurum`: yerleşimin kendi durumu),
  `k` (`Kural`: maçın sprint katı = JS sarmalanmış `ALAN_SPR`), `d` (`KafaD` sabitleri), `lite` (m._lite), `rnd` (m.r).
  Model `Mac`'te `ps` ve gerçek top yok; birleştirmede `Mc` maçın yapısından kurulmalı.
- **Varlık**: JS'te dünyalar karışık nesneler (oyuncu, `seen` hayaleti `{a,x,y,R,D,team,role,press,ref,gvx,gvy}`,
  kopya `{...p,x,y,vx,vy}`). Rust'ta `Varlik`: konum/hız kendinde, beden `Beden::Ps(pid)` ya da
  `Beden::Ayri(Arc<Oyuncu>)`. Hayaletin bedeni (`hayal`) barsız (`en: None`) ve kimliksiz (`id: NaN`) — JS'te
  hayaletin vSp/tepki'si gerçek oyuncununkinden farklıdır (bar yok, sprint evresi yok), aynısı korunur.
  `r` = JS `o.ref`, `kim` = nesne kimliği. Varış tablosu bedeni `r0()` = JS `o.ref || o`.
- **m.sh**: model `JMSh` taslağı (alanlar Option değil, `fr.first` Pid) yetmiyor ve ortak yazılan alan; kendi
  `sekil::Sh`'imi kullandım (`SekilDurum.sh`). Birleştirmede `m.sh` bu tip olmalı (match.js:1213 sıfırlar,
  kafa `sh.gs`/`sh.fr`/`sh.frB` okur).
- **JS modül değişkenleri / m alanları** `SekilDurum`'da: `m._sn/_snT` (görüş önbelleği: `sn`, `snT`), `_kpC`,
  `_tbC`, `m._kayL`, `m._acikK`, `p._bkCik` (modelde yok), `b._pc` (hayali maçta topun yolu).
  Not: JS `_tbC` modül genelidir (anahtar tik|p.id|H.id): gerçek maç ile kafanın hayali maçı aynı anahtara
  denk gelirse JS'te değer sızabilir; Rust'ta her `SekilDurum` kendi `_tbC`'sini tutar (bilinçli fark).
- **GEC/GTB önbellekleri**: değer yalnız anahtardan hesaplanır (anahtar JS ile aynı: IVK, round(10r), round(4c),
  VMQ(vm); GTB'de z), sıradan bağımsız. İş parçacığı başına (`thread_local`) memo. JS'te `AlanMatch.ivme` yoksa
  önbelleksiz `{g: 1/.18}` döner; gerçek maçta hep var: Rust hep ivmeli hesaplar. Hesabın `ivme`'si match.js
  ivme'nin `_iv` verilmiş, sprint evresiz hâli (saf; `gec.rs:ivme_iv`).
- **ALAN_SPRE** (core.js enerji) yerleşimde yeniden yazıldı (`mod.rs`), `ALAN_SPR` = `Kural::spr`.
  Enerji modülü taşınınca oraya bağlanabilir.
- **ALAN_KOSU_TP** (match.js) saf olduğu için `yer.rs`'te; `ALAN_KOSU_PR` maç durumuna bağlı: gecici.
- **Ölü dallar taşınmadı** (bayrak varsayılanlarıyla hiç çalışmayan): `!ONIS()` dalları (gs.oy ile yolu kesme,
  `_cikT`, `_sutT`, `dd._cb`/coverBoth çağrısı), `ALAN_SAV_OLCU`, `ALAN_ATAMA_YENI`, `!KUYU()`, `!UA` (UA hep
  Map) dalları: "ortak atama (eski)", "buluşmaya pres (eski)", eski isler ataması (`bedel`), `!EN2()` pres
  değiş tokuşu, `ALAN_IZ` izi, `stunD` (hiç kurulmuyor), `YON()` (görüş açısı kapalı), kullanılmayan ara
  değerler (`kosU`, `qf`, `kaleye`, `mNear`, `ePr`, `nFree`'li g). `coverBoth`/`teamBest` dışa açık değil ve
  bugünkü maçta çağrılmaz; bütünlük için taşındı.
- position başındaki "kopya oyuncu referansını onar" döngüsü (markRef/tutQ/tbPrev/_atQ): Rust'ta referanslar
  Pid, gerek yok.
- `gercek(o)`: kopyalar zaten oyuncu sırasına bağlı: kimlik işlevi.
- `Dd().dalga(pre, derin, sn, p)` (paralel tartı) sıralı döngüyle aynı: `derin` sırayla çağrılır.
- `D._mac._prT` → `c.m._prT` (kafanın gördüğü maç; hayali maçta JS gerçek maçınkini okuyor olabilir — kuşkulu).
- `tl(...)` null → 0 (kullanıldığı her karşılaştırma/aritmetikte JS ile aynı sonuç).
- `tacEff` dönüşünde `plan`, `enPresDalga` hiç kurulmuyor (match.js): sabit yok/0; `_kp` → `TacE.kp`.
- `prV` adaylar için bir kez hesaplanıp PRS.pv'de yeniden kullanılır (JS iki kez hesaplar; saf olduğu varsayıldı).
- `p._winSeen !== m.winT`: Option karşılaştırması (JS'te undefined !== null; yalnız kontrada, winT null iken fark eder).
- kapatHat'ın hayaleti `{x,y,a,team,role,ref}`: R yok (8), D yok (1), press yok.

## gecici_dis.rs (beklenen anlam; JS kaynağı dosyanın başında)

AlanDecide: `threat(team,x,y)` tehlike; `frenT(p,x,y,R,T,v)` durma süresi; `freeAt(w,team,x,y)` boşluk;
`coreSide(h)` Çekirdeğin yönü; `govdeV(h)` vuruş hızı; `savurPlan(...)` → planın `T`'si; `topK(h)` topla koşu
katı; `duelP(q,h,ps)`, `girThr(q,h,ps)` ikili; `ucusT(L,arrive,al)` pas varışı; `laneOk(w,team,A,B,arr,lag)` /
`laneOk0` (arr, lag verilmeden); `bekYol`; `yerDeger`; `recvCh`; `keepP(h,w,dir,T)`; `runOn`; `space`, `front`,
`dagin`; `kayipYer(...)` → (x,y)→değer işlevi; sabitler `PNC_GORUS`=48, `DU_fakeT`=9, `DU_oyalaR`=26,
`DU_engage`=2.4.
AlanMatch: `tl(p,ev,k)` taktik rolü (yoksa None); `kpAkt(m,t)` kontrpresçilerin id'leri; `tacEff(m,t)` etkin
taktik (`TacE{t, kp}`); `macEff_macK(m,t)` = `macEff(m,t,{}).macK`; `kuyuSafe(x,y)` (Kuyu açıkken hep None);
`ALAN_KOSU_PR(p)` sprint profili `{Ts,k}`; `slowT(p)` (= `p.slowT || 0`, modelde alan yok).
core.js enerji: `ALAN_EN3_DOL(p,sahip,nd,yerK)`, `ALAN_EN3_KUL(p,e,bizde,bx,by)`.

## model.rs'te değiştirdiklerim

- `Oyuncu.kapPrev`: `Option<JsDeger>` → `Option<Pid>` (kapatma işinin adamı; yalnız yerleşim yazar).
- `Oyuncu._otB`: `Option<JP_otB>` → `Option<Pid>` (bindirmenin taşıyıcısı; yalnız var/yok okunur).
- `JMTacEDurusHucEE` / `JMTacEDurusSavEE` (boş üretilmişti): `f: Option<f64>`, `y: Option<f64>` eklendi
  (duran top dizilişi; `m.tac` maçındır, ama slotBase bunları okumadan çalışamaz).

Modelde olmayan ama kullanılan alanlar: `p.slowT` (gecici), `p._bkCik` (SekilDurum), `m._lite` (Mc.lite),
`m.r` (Mc.rnd), `m.ps`/`m.ball` (Mc).

## Bit bit aynılık

(sınama sonucu aşağıda)
