(function () {
  // Beyin (v4). Oyun tek bir beynin dünyasında oynanır: oyuncular kafasızdır, her komutu beyin verir.
  // Dünyanın yasaları (hareket, Çekirdek, alanlar, şarj, temas, ikili mücadelenin zarı, sayı) core.js ve match.js'tedir; beyin onları bilir ve
  // karar verirken kendi dünyasını ileri oynatabilir, ama değiştiremez. Beynin serbest olduğu tek yer komutlardır.
  // Komut noktaları (match.js çağırır): konum (topsuz herkes: yerleşim, pres, kovalama) · ikili (savunmacı girsin mi) · karar (top sahibi) · tekDokunus (alıcı).
  // Refleks katmanı: aşağıdaki varsayılan kurallar (v3g'nin oyuncu kuralları). İleri bakış katmanı bu refleksleri temel alır ve üstüne kurulur.
  const hyp = window.AlanCore.hyp, C = () => window.AlanCore, Dd = () => window.AlanDecide, Mm = () => window.AlanMatch;
  const PASSK = ['pas', 'önüne', 'aşırt', 'kenardan'];
  const R = {
    // topsuz oyuncular: yerleşim, pres, kovalama (Katman 4 kuralları)
    konum(m) { if (window.AlanShape) window.AlanShape.position(m); else Mm().position(m); },
    // ikili mücadele: savunmacı, kendi kestirimine (Okuma) göre girer
    ikili(m, q, h, P, est) { return !(est < Dd().commitThr(q, h, m.ps)); },
    // top sahibinin kararı
    karar(m, h) { const D = Dd(), tE = Mm().tacEff(m, h.team), r = D.decide(h, m.ps, m.ch, m.r, tE); let o = r.best;
      // Karşılamadan önce görülen pas: alıcı Çekirdek gelmeden bakmıştı (plan). Kontrol süresi geçince, aynı hedef hâlâ makulse (tutma ihtimali en fazla 0,15 düşmüşse) onu oynar.
      const pl = h._plan; h._plan = null; if (pl && m.tick - pl.t < 30) { const same = r.opts.find(x => x.kind === pl.kind && x.q === pl.q && x.to && hyp(x.to.x - pl.to.x, x.to.y - pl.to.y) < 4); if (same && same.det && same.det.P >= pl.P - .15 && same.v > 0) o = same; }
      m.lastDec = { name: h.name, team: h.team, x: h.x, y: h.y, opts: r.opts.slice(0, 4).map(o => ({ kind: o.kind + (o.shot ? ' ' + o.shot : ''), to: o.to, q: o.q ? o.q.name : '', P: o.det ? (o.kind === 'gönder' ? o.det.Pk : o.det.P) : null, v: o.v })) };
      return o; },
    // alıcı: tek dokunuşla mı oynasın, kontrol mü etsin
    tekDokunus(m, p, chFull, chCtrl, ot) { const D = Dd(); let one = null;
      if (!m._lite && self.ALAN_OKC(p, 'karar') >= D.D.otOk) { const tE = Mm().tacEff(m, p.team), r = D.decideOT(p, m.ps, chFull, m.r, tE, ot), rc = (() => { /* Kontrol etmenin bedeli zamandır: Çekirdeği durdurup gitmek istediği tarafa döndürene kadar savunma kapanır. */
          let tTurn = 0; const K2 = C(), aim = r && (r.T || r.to); if (aim && p.ca != null) { const ta = Math.atan2(aim.y - p.y, aim.x - p.x); tTurn = K2.Turn.arc(p.ca, ta, K2.Turn.shortDir(p.ca, ta)) / K2.Turn.omega(p); }
          const w = D.D.diagCtrlReal ? m.ps : D.predictWorld(m.ps, p.team, p, D.D.ctrlT + tTurn + 9), rc1 = D.decide(p, w, chCtrl, m.r, tE); for (const o of rc1.opts) if (o.q) o.q = m.ps.find(z => z.id === o.q.id) || o.q; return rc1; })(); const ctrlBest = rc.opts.length ? rc.opts[0].v : 0; const b0 = rc.best; p._plan = b0 && b0.q && b0.to && b0.det && PASSK.includes(b0.kind) ? { kind: b0.kind, q: b0.q, to: { x: b0.to.x, y: b0.to.y }, P: b0.det.P ?? 0, t: m.tick } : null; if (r && r.v > ctrlBest + D.D.otMargin) one = r; }
      return one; }
  };
  // ── İleri bakış katmanı ─────────────────────────────────────────────────────────────────────────────────────────────
  // Beyin, karar anlarında aday komutları kendi dünyasında ileri oynatarak tartar. İleri oynatmada:
  //  · maçın gerçek zarı kullanılmaz (her deneme kendi zarıyla); · içerideki herkes refleks katmanıyla oynar (iç içe ileri bakış yok);
  //  · dünya, komutun verildiği oyuncunun gözüyle görülür (Okuma: rakiplerin yeri ve hızı o kadar hatalı).
  // Seçim, sonuçları iyi olanlar arasından Okuma'ya bağlı bir isabetle yapılır (softmax: Okuma yüksekse en iyiye daha çok ağırlık).
  // Bu katman kapalıyken (A.bak = false) maç v3g ile aynıdır.
  const A = {
    bak: true,          // ileri bakış açık mı
    K: 99, R: 1, RG: 6, /* K: bütün adaylar oynatılır (ön eleme yok) */ /* gönderme: sonucu ya sayı ya kayıp olan, oynaklığı yüksek hamle; tek denemeyle şanslı bir sayı onu en iyi gösterir. Ortalaması 6 denemeyle alınır */         // top sahibi: en umutlu K aday, her biri R deneme
    H: 20, HMAX: 60, HUC: 240,    // ufuk: en az H tik, sonra top birinin eline geçene kadar (en çok HMAX). Ölçüm: 30/90'a göre kalite aynı, süre ~%25 kısa
    savunma: true, SP: 45, SH: 45,   // savunma: her SP tikte pres yoğunluğu (taktiğin ±1'i), SH tik ileri bakarak
    kovala: true,       // pas atıldığında savunan takımda kaç kişinin kovalayacağı
    algi: .05,          // Okuma'nın algı hatası: rakip konumu (20 − Okuma) × algi birim sapar, hızı da orantılı (Okuma 10: ±0,5 birim)
    t0: .005, tOk: .003, // seçim sıcaklığı: t0 + (20 − Okuma) × tOk (değer birimi; Okuma 10: 0,035). Ölçüm: Okuma 16'lık takım Okuma 6'lık takıma 6 maçta 24–9
    icHafif: false,     // ileri oynatmanın içindeki refleks kararlar hafif hesapla (D._inLook: daha az deneme, tutma tablosu yerine kafadaki P)
    hafif: true,        // ileri oynatmada hafif kip (m._lite: kafadaki oyunlarda kullanılan sadeleştirmeler; yaklaşık 2 kat hızlı)
    // v4b eklemeleri
    icKonum: 6, icKarar: 15, // (4) ileri oynatmada top sahibi en erken bu kadar tikte bir yeniden düşünür (refleks kararlar sürenin %75'ini yiyordu)
            // (4) ileri oynatmada topsuz yerleşim her tik değil, bu kadar tikte bir yeniden hesaplanır; arada herkes son talimatını sürdürür (beynin gerçek ritmi yarım saniye)
    kosu: true, KN: 2, KT: 90, // (1) koordinasyon: top sahibinin hamlesiyle birlikte en umutlu KN topsuz koşu da aynı oynatmada tartılır; seçilen koşu en çok KT tik sürer
    golDene: true,      // en iyi gönderme her zaman ayrıca denenir
    ikiliB: true, IR: 3,  // ikili: yakındaki savunmacı girsin mi, beyin iki seçeneği IR'er kez oynatıp savunmacının Okuma'sıyla seçer
    yer: true, YP: 10, YR: 2, YH: 60, YT: 60, YD: 7, // yerleşim: her YP tikte sıradaki bir topsuz oyuncu için 9 konum (refleks hedefi ve onun YD birim çevresi) YR'şer kez YH tik oynatılır; seçilen kayma YT tik sürer
    plan: true, DN: 99, /* bütün sürme ve paslar ikinci adımıyla tartılır (gönderme zaten sonuna kadar oynatılıyor): iki taraf aynı terazide */ YMIN: 20, // v4d · plan: ileri oynatmada kimse düşünmez; beynin komutu (plan) uygulanır, planda olmayan ilk karar anında oynatma durur ve o an tabloyla değerlendirilir. DN: ikinci adımı (alıcının bütün seçenekleri) oynatılacak en iyi ilk adım sayısı. YMIN: karar dışı tartmalarda (yerleşim, pres, kovalama, ikili) top sahibi en az bu kadar tik son hâlini sürdürür
    bek: true, BP: 15, BR: [1.5, 2.5, 4, 6, 9, 13], // Bekçi'nin derinliği: her BP tikte beyin, Kuyu ile tehdit noktası arasındaki hatta birkaç derinlik için oradan gönderilecek Çekirdeği (iki direğe, güçlü ve plase) fizikle oynatır; sayı ihtimali en düşük olanı Bekçi'nin Okuma'sıyla seçer
    uc: 'tablo'         // (2) ufuk sonu değeri: 'tablo' (durum değeri tablosu) | 'kaba' (tablosuz: Çekirdek kimde, Kuyu'ya uzaklık, önündeki rakip sayısı)
  };
  const isP = v => v && typeof v === 'object' && v.team !== undefined && v.a && v.role !== undefined && v.id !== undefined;
  function pack(o) { const seen = new WeakSet(); return JSON.parse(JSON.stringify(o, function (k, v) { if (isP(v)) return { __pid: v.id }; if (v && typeof v === 'object') { if (seen.has(v)) return undefined; seen.add(v); } return v; })); }
  function unpack(o, ps) { if (Array.isArray(o)) return o.map(x => unpack(x, ps)); if (o && typeof o === 'object') { if (o.__pid !== undefined) return ps.find(p => p.id === o.__pid) || null; const r = {}; for (const k in o) r[k] = unpack(o[k], ps); return r; } return o; }
  const desc = o => ({ kind: o.kind, q: o.q ? o.q.id : null, to: o.to ? { x: o.to.x, y: o.to.y } : null, shot: o.shot || null, v: o.v });
  const same = (o, d) => o.kind === d.kind && (o.q ? o.q.id : null) === d.q && (o.shot || null) === d.shot && (!d.to || (o.to && hyp(o.to.x - d.to.x, o.to.y - d.to.y) < .6));
  const ok = (p, ch) => self.ALAN_OKC ? self.ALAN_OKC(p, ch) : ((p.a && p.a.okuma) ?? 10);
  const brng = m => m._br || (m._br = Mm().rng(((m.seed || 1) * 2654435761) ^ 0x5bd1e995)); // beynin kendi zarı (seçimdeki isabet): maçın fizik zarına dokunmaz
  // Uç değer: sayı ±1; top bizde +V, rakipte −V (durum değeri tablosu); boşta 0
  // Kaba uç değer (tablosuz): Kuyu'ya yakınlık × önünde (Kuyu tarafında) kalan rakip saha oyuncusu başına azalma. Elle yazılmış ama tek bir yerde ve sadece "ufuk sonunda durum ne kadar iyi" sorusu için.
  function kaba(c, h) { const gx = h.team === 0 ? 100 : 0, d = hyp(gx - h.x, 25 - h.y), dir = h.team === 0 ? 1 : -1; let n = 0; for (const p of c.ps) if (p.team !== h.team && p.role !== 'Bekçi' && (p.x - h.x) * dir > 0) n++; return .6 * Math.exp(-d / 30) * Math.max(.1, 1 - .12 * n); }
  function endVal(c, team, s0) { const ds = (c.score[team] - s0[team]) - (c.score[1 - team] - s0[1 - team]); if (ds) return Math.sign(ds);
    const h = c.holder, T = window.ALAN_VS; if (h && A.uc === 'kaba') { const v = kaba(c, h); return h.team === team ? v : -v; } if (!h || !T) return 0; const tr = c.winTeam === h.team && (c.tick - (c.winT ?? -1e9)) < 180, v = T.v[window.AlanState.idx(c.ps, h.team, h.x, h.y, tr)] || 0; return h.team === team ? v : -v; }
  // Bir deneme: dünyayı kopyala, komutu uygula, ileri oynat, sonucu takımın gözünden değerlendir
  const IC = {}; // ileri oynatmanın içi de sınırsız: beynin hayal ettiği her an da her seçeneği görür
  function rollout(m0, j) { const M = Mm(), c = M.cloneMatch(m0), s0 = c.score.slice(), D = Dd().D, keep = [D._m, D._tick], kd = {}; for (const k in IC) { kd[k] = D[k]; D[k] = IC[k]; } c._ic = true; c.r.s = j.salt | 0; if (A.hafif) c._lite = true; const il0 = D._inLook; if (A.icHafif) D._inLook = true;
    if (j.sigma) { const e = Mm().rng(j.salt ^ 0x27d4eb2d); for (const p of c.ps) if (p.team !== j.team) { p.x += (e() - .5) * 2 * j.sigma; p.y += (e() - .5) * 2 * j.sigma; p.vx = (p.vx || 0) * (1 + (e() - .5) * j.sigma * .2); p.vy = (p.vy || 0) * (1 + (e() - .5) * j.sigma * .2); } }
    if (A.plan) { c._pm = true; c._pmMin = j.tip === 'karar' || j.tip === 'karar2' ? 0 : c.tick + A.YMIN; }
    if (j.tip === 'karar' || j.tip === 'karar2') { c._zorla = { hid: j.hid, d: j.d, o: j.o }; c.kosu = c.kosu ? c.kosu.slice() : [null, null]; c.kosu[j.team] = j.run ? { ...j.run, until: c.tick + A.KT } : null; }
    else if (j.tip === 'ikili') c._ikZ = { qid: j.qid, eng: j.eng };
    else if (j.tip === 'yer') { c.yer = { ...(c.yer || {}) }; c.yer[j.pid] = { dx: j.dx, dy: j.dy, until: c.tick + A.YT }; }
    else if (j.tip === 'savunma') c.tac[j.team] = { ...c.tac[j.team], pres: j.pres };
    else if (j.tip === 'kovala') { c.kovM = c.kovM ? c.kovM.slice() : [null, null]; c.kovM[j.team] = j.kov; }
    const H = j.H ?? A.H, HM = j.HMAX ?? A.HMAX; for (let t = 0; t < HM || (!c.holder && t < A.HUC); t++) { /* Çekirdek havadaysa sonuç belli olana kadar oynat: yarım kalan an 0 sayılınca uzak gönderme "risksiz" görünüyordu */ if (c._lite || D._inLook) { D._m = c; D._tick = c.tick + 1; } const dT0 = c.decT; M.step(c); if (c.holder && c.decT > dT0 && c.decT < A.icKarar) c.decT = A.icKarar; if (c.over) break; if (c.score[0] !== s0[0] || c.score[1] !== s0[1]) break; if (c._dur) break; if (!c._pm && t >= H && c.holder) break; }
    D._m = keep[0]; D._tick = keep[1]; D._inLook = il0; for (const k in kd) D[k] = kd[k];
    const v1 = endVal(c, j.team, s0); if (j.tip !== 'karar2' || !c._dur || !c.holder || c.holder.team !== j.team || c.over) return v1;
    // ikinci adım: ilk adımın sonundaki gerçek durumda yeni top sahibinin bütün seçenekleri oynatılır; değer, beynin onun yeteneğiyle seçeceği beklenen sonuç
    const h2 = c.holder; c._dur = false; c._pm = false; const c0 = M.cloneMatch(c); c0._cap = { hid: h2.id, opts: null }; c0._ic = true; c0._zorla = null; c0._pm = false; const k0 = [D._m, D._tick]; M.step(c0); D._m = k0[0]; D._tick = k0[1]; const full = c0._cap.full; if (!full || !full.length) return v1;
    const vs = full.map((o, i) => { let n = o.kind === 'gönder' ? A.RG : A.R, s = 0; for (let r = 0; r < n; r++) s += rollout(c, { tip: 'karar', hid: h2.id, o: pack(o), team: j.team, sigma: j.sigma, salt: (j.salt ^ (i * 2654435761) ^ (r * 7919 + 3)) >>> 0 }); return s / n; });
    const T = A.t0 + Math.max(0, 20 - ok(h2, 'karar')) * A.tOk, mx = Math.max(...vs), w = vs.map(v => Math.exp((v - mx) / T)), sw = w.reduce((a, x) => a + x, 0); return vs.reduce((a, v, i) => a + v * w[i] / sw, 0); }
  // Seçim: denemelerin ortalaması, Okuma'ya bağlı sıcaklıkla
  function sec(m, vals, okuma) { const T = A.t0 + Math.max(0, 20 - okuma) * A.tOk, mx = Math.max(...vals), w = vals.map(v => Math.exp((v - mx) / T)), s = w.reduce((a, b) => a + b, 0); let u = brng(m)() * s; for (let k = 0; k < w.length; k++) { u -= w[k]; if (u <= 0) return k; } return w.length - 1; }
  const avg = a => a.reduce((x, y) => x + y, 0) / a.length;
  function kosuAday(m, h) { const D = Dd(), dir = h.team === 0 ? 1 : -1, gx = h.team === 0 ? 100 : 0, KR = C().KR, out = [];
    for (const r of m.ps) { if (r.team !== h.team || r === h || r.role === 'Bekçi') continue; for (const dy of [-8, 0, 8]) { const T = { x: Math.max(3, Math.min(97, r.x + dir * 14)), y: Math.max(3, Math.min(47, r.y + dy + (25 - r.y) * .25)) }; if (hyp(T.x - gx, T.y - 25) < KR + 2) continue;
        const pre = D.laneOk(m.ps, h.team, h, T) * D.V(m.ps, h.team, T.x, T.y, m.ch); out.push({ id: r.id, ad: r.name, x: T.x, y: T.y, pre }); } }
    out.sort((a, b) => b.pre - a.pre); const seen = new Set(), res = []; for (const o of out) { if (seen.has(o.id)) continue; seen.add(o.id); res.push(o); if (res.length >= A.KN) break; } return res; }
  // Konum: refleks yerleşim + beynin seçtiği koşu. İleri oynatmada yerleşim her A.icKonum tikte bir (arada son talimatlar sürer).
  function konum(m) { if (m._ic && A.icKonum > 1 && m._kT != null && m.tick - m._kT < A.icKonum) return; if (m._ic) m._kT = m.tick; R.konum(m);
    if (m.kosu) for (const t of [0, 1]) { const k = m.kosu[t]; if (!k) continue; const p = m.ps.find(z => z.id === k.id), h = m.holder;
      if (m.tick > k.until || (h && h.team !== t) || (h && h.id === k.id) || (!h && m.fl && m.fl.q && m.fl.q.id === k.id) || hyp(p.x - k.x, p.y - k.y) < 1) { m.kosu[t] = null; continue; } /* pas koşucuya atıldıysa koşu biter: koşucu artık Çekirdeğe gider (alıcı kuralı) */ if (h || (m.ball && m.ball.team === t)) { p.tx = k.x; p.ty = k.y; p.sprint = true; p.job = 'beyin koşusu'; } }
    if (A.bek) { if (!m._ic) bekPlan(m); if (m.bekD) for (const t of [0, 1]) { const y = m.bekD[t]; if (!y) continue; const p = m.ps.find(z => z.team === t && z.role === 'Bekçi'); if (!p || p === m.holder || p.job !== 'Bekçi') continue; const gx = t === 0 ? 0 : 100, E = m.E || { x: 50, y: 25 }, L = hyp(E.x - gx, E.y - 25) || 1; p.tx = gx + (E.x - gx) / L * Math.min(y.r, L - 1); p.ty = 25 + (E.y - 25) / L * Math.min(y.r, L - 1); } }
    // beynin seçtiği yerleşim: refleks hedefine göre kayma (refleks oyunu izlemeye devam eder, beyin onun üstüne nereye kayılacağını seçer)
    if (m.yer) for (const id in m.yer) { const y = m.yer[id]; if (m.tick > y.until) { delete m.yer[id]; continue; } const p = m.ps.find(z => z.id === +id); if (!p || p === m.holder || BUSY.test(p.job || '')) continue; p.tx = Math.max(1, Math.min(99, p.tx + y.dx)); p.ty = Math.max(1, Math.min(49, p.ty + y.dy)); } }
  function bekPlan(m) { const D = Dd(), K = C(); m.bekD = m.bekD || [null, null];
    for (const t of [0, 1]) { const bk = m.ps.find(p => p.team === t && p.role === 'Bekçi'); if (!bk) continue; const cur = m.bekD[t]; if (cur && m.tick < cur.until) continue;
      const gx = t === 0 ? 0 : 100, h = m.holder, sh = h && h.team !== t ? h : null, E = sh ? { x: sh.x, y: sh.y } : (m.E || { x: 50, y: 25 }), L = hyp(E.x - gx, E.y - 25) || 1, ux = (E.x - gx) / L, uy = (E.y - 25) / L, ch = sh ? m.ch : .5;
      const shooter = sh ? { ...sh, x: E.x, y: E.y } : { id: -1, x: E.x, y: E.y, team: 1 - t, a: { aktarim: 10, okuma: 10 }, R: 8, D: 1, name: 's' };
      let s0 = (m.tick * 2654435761 + t) >>> 0; const rnd = () => { s0 = (s0 * 1664525 + 1013904223) >>> 0; return s0 / 4294967296; };
      const rs = A.BR.filter(r => r < L - 1.5), vals = rs.map(r => { const kb = { ...bk, x: gx + ux * r, y: 25 + uy * r, vx: 0, vy: 0 }; let pk = 0, n = 0; for (const [v, aim] of [[D.D.shotV, D.D.aimPow], [D.D.plaseV, D.D.aimPlase]]) for (const sd of [-1, 1]) { const ty = 25 + sd * K.MOUTH * aim, gx2 = t === 0 ? 0 : 100, dx = gx2 - E.x, dy = ty - E.y, l = hyp(dx, dy) || 1, v0 = v * K.chgMul(ch); pk += D.imagine(shooter, [shooter, kb], { x: E.x, y: E.y, vx: dx / l * v0, vy: dy / l * v0, ch }, rnd, 3).Pk; n++; } return -pk / n; });
      if (!rs.length) continue; const k = sec(m, vals, ok(bk, 'bekci')); m.bekD[t] = { r: rs[k], until: m.tick + A.BP }; } }
  const BUSY = /alıcı|kovala|pres|sekme|tuzak|ikisini|ver-kaç|geri koşu|Bekçi|koşusu/;
  // Planla: bu tikte beynin tartması gereken bir karar var mı? Varsa denemeleri (iş listesi) ve sonuçla ne yapılacağını döndürür.
  function planla(m) { if (!A.bak || m._ic || m.over) return null; const M = Mm();
    if (m.holder && m.kovM) m.kovM = null; // top birinin elindeyse kovalama ayarı biter
    const h = m.holder;
    // 1 · top sahibinin kararı (bu tik karar verecekse)
    if (h && m.decT <= 1 && !(h.stun > m.tick)) { const c0 = M.cloneMatch(m); c0._cap = { hid: h.id, opts: null }; c0._ic = true; const D = Dd().D, keep = [D._m, D._tick]; M.step(c0); D._m = keep[0]; D._tick = keep[1];
      const opts = c0._cap.opts; if (!opts || !opts.length) return null; const cand = [], full = c0._cap.full;
      for (const d of opts.slice().sort((a, b) => b.v - a.v)) { if (!cand.some(x => same({ kind: x.kind, q: x.q != null ? { id: x.q } : null, shot: x.shot, to: x.to }, d))) cand.push(d); if (cand.length >= A.K) break; } if (A.golDene) { const g = opts.filter(o => o.kind === 'gönder').sort((a, b) => b.v - a.v)[0]; if (g && !cand.some(x => same({ kind: x.kind, q: x.q != null ? { id: x.q } : null, shot: x.shot, to: x.to }, g))) cand.push(g); } /* gönderme her zaman denenir: boş Kuyu'yu refleks değer göremeyebilir, gerçek fizik görür */
      const okuma = ok(h, 'karar'), sigma = Math.max(0, 20 - ok(h, 'algi')) * A.algi, jobs = [];
      // (1) Koordinasyon: hamle + koşu birlikte. Koşu adayları ucuz bir ön elemeyle (koşu noktasının değeri × pas hattı), sonucu oynatma belirler.
      const runs = A.kosu ? kosuAday(m, h) : [], combo = [];
      cand.forEach(d => combo.push({ d, run: null })); cand.slice(0, 2).forEach(d => runs.forEach(run => combo.push({ d, run })));
      const fullOf = d => full.find(o => same(o, d)); combo.forEach((x, k) => { const fo = A.plan ? fullOf(x.d) : null; x.o = fo ? pack(fo) : null; for (let r = 0; r < (x.d.kind === 'gönder' ? A.RG : A.R); r++) jobs.push({ tip: 'karar', hid: h.id, d: x.d, o: x.o, run: x.run, team: h.team, sigma, salt: (m.tick * 40503) ^ (k * 2654435761) ^ (r * 97 + 13) }); });
      const son = (v, plan2) => { const k = sec(m, v, okuma), x = combo[k]; m._zorla = { hid: h.id, d: x.d, full, ct: m.tick }; m.kosu = m.kosu ? m.kosu.slice() : [null, null]; m.kosu[h.team] = x.run ? { ...x.run, until: m.tick + A.KT } : (m.kosu[h.team] && m.kosu[h.team].until > m.tick ? m.kosu[h.team] : null); if (x.run) { m.st.kosu = m.st.kosu || [0, 0]; m.st.kosu[h.team]++; }
        m._beyinSon = { tip: 'karar', tick: m.tick, ad: h.name, aday: combo.map((c, i) => ({ kind: c.d.kind + (c.run ? ' + ' + c.run.ad + ' koşsun' : '') + (plan2 && plan2.has(i) ? ' (2 adım)' : ''), v: +v[i].toFixed(3) })), secilen: k }; };
      return { tip: 'karar', jobs, bitir(res) { let at = 0; const v = combo.map(x => { const n = x.d.kind === 'gönder' ? A.RG : A.R, s = avg(res.slice(at, at + n)); at += n; return s; });
        if (!A.plan || !A.DN) { son(v); return null; }
        // ikinci dalga: en iyi DN ilk adım, alıcının bütün seçenekleriyle derinleştirilir (ilk adım aynı zarla yeniden oynanır, sonundaki gerçek durumdan)
        const top = v.map((x, i) => [x, i]).filter(([, i]) => combo[i].d.kind !== 'gönder').sort((a, b2) => b2[0] - a[0]).slice(0, A.DN).map(([, i]) => i); if (!top.length) { son(v); return null; }
        const jobs2 = top.map(i => { const x = combo[i]; let at2 = 0; for (let q = 0; q < i; q++) at2 += combo[q].d.kind === 'gönder' ? A.RG : A.R; return { tip: 'karar2', hid: h.id, d: x.d, o: x.o, run: x.run, team: h.team, sigma, salt: (m.tick * 40503) ^ (i * 2654435761) ^ (0 * 97 + 13) }; });
        return { tip: 'karar2', jobs: jobs2, bitir(res2) { const v2 = v.slice(); top.forEach((i, q) => { v2[i] = res2[q]; }); son(v2, new Set(top)); return null; } }; } }; }
    // 2 · pas atıldı: savunan takımda kaç kişi kovalasın
    if (A.kovala && !h && m.ball && m.fl && m.fl.t0 === m.tick && m.fl.kind !== 'gönder') { const t = 1 - m.ball.team, S = window.AlanShape && window.AlanShape.S, base = S ? S.marginT : .6, ks = [0, base, base * 2.5];
      const defs = m.ps.filter(p => p.team === t && p.role !== 'Bekçi'), okuma = avg(defs.map(p => ok(p, 'yerlesim'))), sigma = Math.max(0, 20 - okuma) * A.algi, jobs = ks.map((kov, k) => ({ tip: 'kovala', team: t, kov, sigma, H: 0, HMAX: A.HMAX, salt: (m.tick * 69069) ^ (k * 2246822519) ^ 7 }));
      return { tip: 'kovala', jobs, bitir(res) { const k = sec(m, res, okuma); m.kovM = m.kovM ? m.kovM.slice() : [null, null]; m.kovM[t] = ks[k]; } }; }
    // 2b · ikili: menzile giren savunmacı girsin mi
    if (A.ikiliB && h) { const D = Dd(); for (const q of m.ps) { if (q.team === h.team || q.role === 'Bekçi' || q.noTouch || q.cd > m.tick + 1) continue; if (hyp(q.x - h.x, q.y - h.y) > D.DU.engage + .6) continue;
        const okq = ok(q, 'ikili'), sigma = Math.max(0, 20 - ok(q, 'algi')) * A.algi, jobs = []; for (const eng of [true, false]) for (let r = 0; r < A.IR; r++) jobs.push({ tip: 'ikili', qid: q.id, eng, team: q.team, sigma, salt: (m.tick * 2654435761) ^ (q.id * 97) ^ (eng ? 1 : 2) ^ (r * 7919) });
        return { tip: 'ikili', jobs, bitir(res) { const v = [avg(res.slice(0, A.IR)), avg(res.slice(A.IR))], k = sec(m, v, okq); m._ikZ = { qid: q.id, eng: k === 0 }; m._beyinSon = { tip: 'ikili', tick: m.tick, ad: q.name, aday: [{ kind: 'gir', v: +v[0].toFixed(3) }, { kind: 'girme', v: +v[1].toFixed(3) }], secilen: k }; } }; } }
    // 3 · savunma: pres yoğunluğu (taktiğin ±1'i), SP tikte bir
    if (A.savunma && m.tick % A.SP === 0 && (h || m.ball)) { const t = 1 - (h ? h.team : m.ball.team); m.tacB = m.tacB || [{ ...m.tac[0] }, { ...m.tac[1] }]; const b = m.tacB[t].pres ?? 1, ps = [...new Set([Math.max(0, b - 1), b, Math.min(3, b + 1)])];
      const defs = m.ps.filter(p => p.team === t && p.role !== 'Bekçi'), okuma = avg(defs.map(p => ok(p, 'yerlesim'))), sigma = Math.max(0, 20 - okuma) * A.algi, jobs = ps.map((pres, k) => ({ tip: 'savunma', team: t, pres, sigma, H: A.SH, HMAX: A.SH, salt: (m.tick * 1103515245) ^ (k * 3266489917) ^ 11 }));
      return { tip: 'savunma', jobs, bitir(res) { const k = sec(m, res, okuma); m.tac[t] = { ...m.tac[t], pres: ps[k] }; } }; }
    // 4 · yerleşim: sıradaki topsuz oyuncu için konum (takımlar ve oyuncular sırayla)
    if (A.yer && (h || m.ball) && m.tick % A.YP === 3) { m._yI = (m._yI || 0) + 1; const t = m._yI % 2, pl = m.ps.filter(p => p.team === t && p.role !== 'Bekçi' && p !== h); if (pl.length) { const p = pl[Math.floor(m._yI / 2) % pl.length];
        if (!BUSY.test(p.job || '')) { const offs = [[0, 0]]; for (let k = 0; k < 8; k++) offs.push([Math.cos(k * Math.PI / 4) * A.YD, Math.sin(k * Math.PI / 4) * A.YD]); const okp = ok(p, 'yerlesim'), sigma = Math.max(0, 20 - ok(p, 'algi')) * A.algi, jobs = [];
          offs.forEach(([dx, dy], k) => { for (let r = 0; r < A.YR; r++) jobs.push({ tip: 'yer', pid: p.id, dx, dy, team: t, sigma, H: A.YH, HMAX: A.YH, salt: (m.tick * 1597334677) ^ (k * 3812015801) ^ (r * 131) }); });
          return { tip: 'yer', jobs, bitir(res) { const v = offs.map((_, k) => avg(res.slice(k * A.YR, k * A.YR + A.YR))), k = sec(m, v, okp); m.yer = { ...(m.yer || {}) }; m.yer[p.id] = { dx: offs[k][0], dy: offs[k][1], until: m.tick + A.YT }; m._beyinSon = { tip: 'yer', tick: m.tick, ad: p.name, aday: offs.map((o, i) => ({ kind: i ? `kay ${o[0].toFixed(0)},${o[1].toFixed(0)}` : 'refleks yeri', v: +v[i].toFixed(3) })), secilen: k }; } }; } } }
    return null; }
  // Maçı baştan sona (ya da bir tike kadar) beynin yönetiminde oynat. isler: iş listesini çalıştıran işlev (varsayılan: sırayla, bu iş parçacığında)
  function oyna(m, o) { o = o || {}; const M = Mm(), until = o.until ?? m.len; while (!m.over && m.tick < until) { let p = planla(m); while (p) p = p.bitir(p.jobs.map(j => rollout(m, j))) || null; M.step(m); if (o.kare) o.kare(m); } return m; }
  async function oynaAsync(m, o) { o = o || {}; const M = Mm(), until = o.until ?? m.len; while (!m.over && m.tick < until) { let p = planla(m); while (p) p = p.bitir(o.isler ? await o.isler(m, p.jobs) : p.jobs.map(j => rollout(m, j))) || null; M.step(m); if (o.kare) o.kare(m); if (o.ilerle && m.tick % 60 === 0) await o.ilerle(m); } return m; }
  // Komut noktaları: ileri bakışın seçtiği komut varsa onu uygula, yoksa refleks
  function karar(m, h) { const z = m._zorla;
    if (m._pm) { if (z && z.hid === h.id && z.o) { m._zorla = null; h._plan = null; return unpack(z.o, m.ps); } if (m.tick < (m._pmMin || 0)) return { kind: 'tut' }; m._dur = true; return { kind: 'tut' }; } // plan kipi: planın komutu, yoksa dur (o an tabloyla değerlendirilir)
    // Yakalamadaki seçenek aynı tikte hesaplanmıştı (aynı dünya): yeniden hesaplamadan, oyuncuları gerçek dünyadakilerle değiştirerek kullan.
    if (z && z.hid === h.id && z.full && z.ct === m.tick - 1) /* planla adımdan önce, karar adımın içinde (tik bir artmış) */ { const f0 = z.full.find(o => same(o, z.d)); if (f0) { m._zorla = null; h._plan = null; const byId = id => m.ps.find(p => p.id === id); const f = { ...f0, q: f0.q ? byId(f0.q.id) : f0.q };
        m.lastDec = { name: h.name, team: h.team, x: h.x, y: h.y, beyin: true, opts: z.full.slice(0, 4).map(o => ({ kind: o.kind + (o.shot ? ' ' + o.shot : ''), to: o.to, q: o.q ? o.q.name : '', P: o.det ? (o.kind === 'gönder' ? o.det.Pk : o.det.P) : null, v: o.v })) }; return f; } }
    if (z && z.hid === h.id) { m._zorla = null; const D = Dd(), tE = Mm().tacEff(m, h.team), r = D.decide(h, m.ps, m.ch, m.r, tE), f = r.opts.find(o => same(o, z.d)); h._plan = null;
      if (f) { m.lastDec = { name: h.name, team: h.team, x: h.x, y: h.y, beyin: true, opts: r.opts.slice(0, 4).map(o => ({ kind: o.kind + (o.shot ? ' ' + o.shot : ''), to: o.to, q: o.q ? o.q.name : '', P: o.det ? (o.kind === 'gönder' ? o.det.Pk : o.det.P) : null, v: o.v })) }; return f; } return r.best; }
    return R.karar(m, h); }
  // yakalama: refleks kararın aday listesini de saklasın (ileri bakış adayları buradan alır)
  const dec0 = R.karar; R.karar = function (m, h) { if (m._cap && m._cap.hid === h.id && !m._cap.opts) { const D = Dd(), tE = Mm().tacEff(m, h.team), r = D.decide(h, m.ps, m.ch, m.r, tE); const ok2 = r.opts.filter(o => o.v != null); m._cap.opts = ok2.map(desc); m._cap.full = ok2; return r.best; } return dec0(m, h); };
  // İşçilere taşıma: dünyanın durumu yapılandırılmış kopyayla (structuredClone / postMessage) taşınır; tek taşınamayan şey zar üreteçleri (durum sayısıyla taşınır)
  function paketle(m) { const r = m.r, br = m._br; m.r = { __rng: r.s }; if (br) m._br = { __rng: br.s }; try { return structuredClone(m); } finally { m.r = r; if (br) m._br = br; } }
  function ac(o) { const M = Mm(); o.r = M.rng(0, o.r.__rng); if (o._br) o._br = M.rng(0, o._br.__rng); return o; }
  window.AlanBeyin = { A, paketle, ac, refleks: R, planla, rollout, oyna, oynaAsync, endVal, konum, ikili: (m, q, h, P, est) => { const z = m._ikZ; if (z && z.qid === q.id) { m._ikZ = null; return z.eng; } return R.ikili(m, q, h, P, est); }, karar, tekDokunus: (m, p, a, b, c) => m._pm ? null : R.tekDokunus(m, p, a, b, c) };
})();
