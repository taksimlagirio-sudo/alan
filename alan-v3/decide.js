(function () {
  // Katman 3 · Taşıyıcının kararı: her seçeneği kafada yeni fizikle simüle eder, Okuma'ya göre kusurlu görür.
  const C = () => window.AlanCore, NOF = [];
  const DU = { base: .3, skill: .03, cage: .14, thrCover: .3, thrLast: .5, stun: 22, stunOk: .5, engage: 2.4, cd: 14 };
  const D = {
    simMax: 180, errOk: .05, errPos: 3,  // algı hatası: rakipler (20-Okuma)×errOk×errPos birim sapmayla görülür
    aimErr: .035, v0Err: .04,            // Aktarım hatası: açı ve hız
    arrive: .3, arriveSpace: .08, vmax: 3, shotV: 2.6, plaseV: 1.7,
    riskBase: 1, threatK: 22, sideK: .35, vBase: .14, vBuild: .12, vShot: .8, shotThru: .15, disc: .985, otErr: 1.2, otOk: 6, otMargin: .02, lobV: 1.05, lobLand: .88, lobMin: 12, lobMax: 50, lobErr: 1.8, look: false, lookR: 36, lookN: 3, lookT: 75, laneLag: .4, aimPow: .45, aimPlase: .75, arriveFirm: .8, vK: .35, combo: .9, chV: .6, pierceV: .85, chg0: .0035, zoneMin: .45, zoneD: 32, chRecv: .55,
    trials: 2, nMates: 4, growMax: 8, shotRange: 34   // kafada: rakip ve alıcı Çekirdeğe doğru hareket eder (menzil zamanla büyür)
  };
  function threat(team, x, y) { const gx = team === 0 ? 100 : 0, d = Math.hypot(x - gx, y - 25); return Math.exp(-d / D.threatK) * (1 - D.sideK * Math.abs(y - 25) / 25); }
  function oppAt(src, team, x, y) { let s = 0; for (const p of src) if (p.team !== team) s += C().infl(p, x, y); return s; }
  function holdVal(src, team, x, y) { return threat(team, x, y) * (.4 + .6 * Math.exp(-oppAt(src, team, x, y) * 2)); }
  function applyError(h, v, rnd, ot) { const ak = h.a.aktarim ?? 10, lk = (v.air ? D.lobErr : 1) * (ot || D._ot || 1), ang = (rnd() + rnd() - 1) * (20 - ak) * D.aimErr / 10 * lk, k = 1 + (rnd() + rnd() - 1) * (20 - ak) * D.v0Err / 10 * lk, c = Math.cos(ang), s = Math.sin(ang); return { vx: (v.vx * c - v.vy * s) * k, vy: (v.vx * s + v.vy * c) * k, air: v.air || 0 }; }
  function firstTouch(b, src, ox, oy, team, q) {
    if (b.air > C().Q.airTouch) return null;
    const Q = C().Q, sx = b.x - ox, sy = b.y - oy, sl = sx * sx + sy * sy; let best = null, bd = 1e9;
    for (const p of src) {
      if (p.noTouch || (p === b.from && b.t < 8)) continue; if (q && p.team === team && p !== q) continue;
      const t0 = sl > 0 ? ((p.x - ox) * sx + (p.y - oy) * sy) / sl : 1, tt = Math.max(0, Math.min(1, t0)), dd = Math.hypot(ox + sx * tt - p.x, oy + sy * tt - p.y);
      const R = (p.role === 'Bekçi' ? C().bekR(b, p) : Q.reach + (((p.a && p.a.yogunluk) ?? 10) - 10) * .04) * (t0 < 0 ? Q.awayK : 1);
      if (dd < R && dd < bd) { bd = dd; best = p; }
    }
    return best;
  }
  // Kimin kovalayacağı (Katman 4 ile aynı kural): yörüngede her oyuncu tepki gecikmesinden sonra en erken nereye varır; takımın en erkeni ve "ben de yetişirim" payındakiler gider
  function chasePlan(pts, ps, t0, lane) {
    const arr = [], sp = p => (.17 + (p.a.hiz ?? 10) * .006) * 1.25;
    for (const p of ps) {
      if (p.role === 'Bekçi' && Math.hypot(p.x - (p.team === 0 ? 0 : 100), p.y - 25) > 12) continue;
      const rc = Math.max(0, 14 - (p.a.okuma ?? 10) * .5 - (t0 || 0) + (lane ? lane.lag : 0)), v = sp(p); let g = null;
      for (let i = 0; i < pts.length; i += 2) { const q = pts[i]; if (q.air > C().Q.airTouch) continue; const tr = rc + Math.max(0, Math.hypot(q.x - p.x, q.y - p.y) - 1.4) / v; if (tr <= i) { g = { t: i, x: q.x, y: q.y, rc }; break; } }
      if (!g) { const q = pts[pts.length - 1]; g = { t: Math.max(pts.length, rc + Math.hypot(q.x - p.x, q.y - p.y) / v), x: q.x, y: q.y, rc }; }
      arr.push([p, g, g.t < pts.length]);
    }
    const out = new Map(); for (const tm of [0, 1]) { const mine = arr.filter(([p]) => p.team === tm); if (!mine.length) continue; const best = Math.min(...mine.map(([, g]) => g.t)); for (const [p, g, hit] of mine) if (g.t <= best + (20 - (p.a.okuma ?? 10)) * .6 + 1 || (lane && hit)) out.set(p, g); }
    return out;
  }
  // Alıcı tepkisiz (pası bekliyor): yörüngede en erken yetişebildiği nokta
  function recvPoint(pts, p) { const v = (.17 + (p.a.hiz ?? 10) * .006) * 1.25; for (let i = 0; i < pts.length; i += 2) { const q = pts[i]; if (q.air > C().Q.airTouch) continue; if (Math.max(0, Math.hypot(q.x - p.x, q.y - p.y) - 1.4) / v <= i) return { x: q.x, y: q.y }; } const q = pts[pts.length - 1]; return { x: q.x, y: q.y }; }
  // Tek hayali yörünge; rakip gövdesine çarpınca beklenen değer: tutar / delip geçer (yörünge sürer) / seker
  // Kötü karşılamadan sonra Çekirdek dibe düşer: kim daha yakınsa o kapar (sabit bir oran değil, o anki kalabalık)
  function looseWin(seen, h, x, y) { let A = .2, Dn = .2; for (const p of seen) { if (p === h || p.role === 'Bekçi') continue; const w = Math.exp(-Math.hypot(p.x - x, p.y - y) / 2); if (p.team === h.team) A += w; else Dn += w; } return A / (A + Dn); }
  function simOne(K, h, b, seen, q, depth, to, cache) {
    // kovalama planı alanları yok sayan hızlı bir yörüngeyle yapılır; Çekirdeğin kendisi gerçek alanlarla, temasa kadar adım adım ilerler
    let hit = null; const qq = q ? seen.find(p => p.id === q.id) : null, sp = p => (.17 + (p.a.hiz ?? 10) * .006) * 1.25, pts0 = K.predict(b, seen, null, D.simMax).pts, plan = chasePlan(pts0, seen.filter(p => p.team !== h.team), 0, { lag: (20 - (h.a.okuma ?? 10)) * D.laneLag }), qg = qq ? recvPoint(pts0, qq) : null, drift = seen.filter(p => p.team !== h.team && !plan.has(p) && (p.vx || p.vy));
    for (let t = 0; t < D.simMax && !b.done; t++) {
      if (t <= 30) for (const p of drift) { p.x += p.vx; p.y += p.vy; }
      for (const [p, g] of plan) { if (t <= g.rc) continue; const dx = g.x - p.x, dy = g.y - p.y, L = Math.hypot(dx, dy); if (L > .2) { const v = Math.min(sp(p), L); p.x += dx / L * v; p.y += dy / L * v; } }
      const qt = qg || to; if (qq && qt) { const dx = qt.x - qq.x, dy = qt.y - qq.y, L = Math.hypot(dx, dy); if (L > .2) { const v = Math.min(sp(qq), L); qq.x += dx / L * v; qq.y += dy / L * v; } }
      const ox = b.x, oy = b.y; K.stepBall(b, seen); if (b.done) break; const p = firstTouch(b, seen, ox, oy, h.team, qq || q); if (p) { hit = p; break; }
    }
    const X = { x: b.x, y: b.y };
    if (hit) {
      const P = K.ctrlP(b, hit, seen);
      if (hit.team === h.team) return { m: P + (1 - P) * .4, o: (1 - P) * .6, k: 0, ...X };
      const T = (1 - P) * K.thruP(b, hit), rest = 1 - P - T; let R = { m: 0, o: 0, k: 0 };
      if (T > .02 && depth < 2) { const b2 = K.makeBall({ x: b.x, y: b.y, vx: b.vx * K.Q.thruKeep, vy: b.vy * K.Q.thruKeep, team: b.team, ch: b.ch, from: b.from, recv: b.recv }); b2.e = b.e; b2.alive = b.alive; b2.cds = new Map([[hit, 1e9]]); R = simOne(K, h, b2, seen, q, depth + 1, to); }
      const gxk = h.team === 0 ? 100 : 0, nearG = Math.hypot(b.x - gxk, b.y - 25) < 16; let rb = .4; if (nearG) { let A = .3, Dn = .3; for (const p of seen) { if (p === h) continue; const d = Math.hypot(p.x - (gxk - (h.team === 0 ? 7 : -7)), p.y - 25); if (d < 12) { if (p.team === h.team) A++; else Dn++; } } rb = A / (A + Dn); }
      return { m: T * R.m + rest * rb, o: P + T * R.o + rest * (1 - rb), k: T * R.k, ...X };
    }
    if (b.done && String(b.done).startsWith('kuyu')) return ((b.done === 'kuyu-sag') === (h.team === 0)) ? { m: 0, o: 0, k: 1, ...X } : { m: 0, o: 1, k: 0, ...X };
    let tm = 1e9, tO = 1e9; for (const p of seen) { if (p === h) continue; const tt = Math.hypot(p.x - b.x, p.y - b.y) / (.17 + (p.a.hiz ?? 10) * .006); if (p.team === h.team) tm = Math.min(tm, tt); else tO = Math.min(tO, tt); }
    const sv = 1 / (1 + Math.exp((tm - tO) / 12)); return { m: sv, o: 1 - sv, k: 0, ...X };
  }
  // Kafada simülasyon. Dönüş: P (arkadaş alır), Pk (Kuyu), Pl (rakip alır), end
  function imagine(h, src, b0, rnd, trials, q, to) {
    const K = C(), cache = {}; trials = trials || D.trials; let mate = 0, kuyu = 0, opp = 0, ex = 0, ey = 0; const err = (20 - (h.a.okuma ?? 10)) * D.errOk;
    for (let k = 0; k < trials; k++) {
      const v = applyError(h, b0, rnd), b = K.makeBall({ x: b0.x, y: b0.y, vx: v.vx, vy: v.vy, air: v.air || 0, team: h.team, ch: b0.ch, from: h, recv: q || null });
      const seen = src.map(p => p === h ? p : p.team === h.team ? { ...p } : { ...p, x: p.x + (rnd() - .5) * err * D.errPos, y: p.y + (rnd() - .5) * err * D.errPos });
      const R = simOne(K, h, b, seen, q, 0, to, cache); mate += R.m; kuyu += R.k; opp += R.o; ex += R.x; ey += R.y;
    }
    return { P: mate / trials, Pk: kuyu / trials, Pl: opp / trials, end: { x: ex / trials, y: ey / trials } };
  }
  // Hız seçimi: düz hatta, yoldaki alanların ortalama direnciyle; hedefe `va` hızıyla varacak başlangıç hızı
  function effFric(src, team, ch, pts) { const P = C().P; let k = 0; for (const p of pts) k += C().K(src, p.x, p.y, team); k /= pts.length; return k < 0 ? P.fric + -k * P.oppDrag * (1 - P.pierce * ch) : P.fric * (1 - P.ownFlow * k); }
  function reachDist(v0, va, fr) { const roll = C().P.roll; let v = v0, d = 0; for (let i = 0; i < 900 && v > va; i++) { v = Math.max(0, v * (1 - fr) - roll); d += v; } return d; }
  function solveV(L, va, fr) { let lo = va, hi = D.vmax; if (reachDist(hi, va, fr) < L) return { v0: hi, ok: false }; for (let i = 0; i < 16; i++) { const mid = (lo + hi) / 2; if (reachDist(mid, va, fr) < L) lo = mid; else hi = mid; } return { v0: hi, ok: true }; }
  const along = (a, b, n) => { const o = []; for (let i = 1; i <= n; i++) o.push({ x: a.x + (b.x - a.x) * i / (n + 1), y: a.y + (b.y - a.y) * i / (n + 1) }); return o; };
  // Hız seçimi: Çekirdeği kafada gerçek alanlarla (o anki oyuncular) yuvarlar; hedefe `va` hızıyla varacak başlangıç hızını arar
  function reachSpeed(h, dx, dy, L, v0, src, ch) { const K = C(), b = K.makeBall({ x: h.x, y: h.y, vx: dx * v0, vy: dy * v0, team: h.team, ch, from: h }); for (let t = 0; t < 400 && !b.done; t++) { K.stepBall(b, src); if (Math.hypot(b.x - h.x, b.y - h.y) >= L) return Math.hypot(b.vx, b.vy); } return -1; }
  // Aşırtma: sabit yatay hız, hedefin biraz önüne iner (inişten sonra yuvarlanır). Havada kalma süresi mesafeye bağlı.
  function lobFor(h, to) { const L = Math.hypot(to.x - h.x, to.y - h.y) || 1, v = D.lobV, air = Math.max(4, Math.round(L * D.lobLand / v)); return { vx: (to.x - h.x) / L * v, vy: (to.y - h.y) / L * v, air, ok: L <= D.lobMax }; }
  function launchFor(h, to, arrive, src, ch) { const L = Math.hypot(to.x - h.x, to.y - h.y) || 1, dx = (to.x - h.x) / L, dy = (to.y - h.y) / L; if (reachSpeed(h, dx, dy, L, D.vmax, src, ch) < arrive) return { vx: dx * D.vmax, vy: dy * D.vmax, ok: false }; let lo = arrive, hi = D.vmax; for (let i = 0; i < 10; i++) { const mid = (lo + hi) / 2; if (reachSpeed(h, dx, dy, L, mid, src, ch) < arrive) lo = mid; else hi = mid; } return { vx: dx * hi, vy: dy * hi, ok: true }; }
  function solveTo(h, to, arrive, src, ch) { const L = Math.hypot(to.x - h.x, to.y - h.y) || 1, dx = (to.x - h.x) / L, dy = (to.y - h.y) / L; if (reachSpeed(h, dx, dy, L, D.vmax, src, ch) < arrive) return null; let lo = arrive, hi = D.vmax; for (let i = 0; i < 10; i++) { const mid = (lo + hi) / 2; if (reachSpeed(h, dx, dy, L, mid, src, ch) < arrive) lo = mid; else hi = mid; } return hi; }
  function bankFor(h, B, q, arrive, src, ch) { const P = C().P, v2 = solveTo({ x: B.x, y: B.y, team: h.team }, q, arrive, src, ch); if (v2 == null) return { vx: 0, vy: 0, ok: false }; const vB = v2 / P.rest; if (vB > D.vmax) return { vx: 0, vy: 0, ok: false }; const v1 = solveTo(h, B, vB, src, ch); const l1 = Math.hypot(B.x - h.x, B.y - h.y) || 1; if (v1 == null) return { vx: (B.x - h.x) / l1 * D.vmax, vy: (B.y - h.y) / l1 * D.vmax, ok: false }; return { vx: (B.x - h.x) / l1 * v1, vy: (B.y - h.y) / l1 * v1, ok: true }; }
  function options(h, src, ch) {
    const K = C(), team = h.team, gx = team === 0 ? 100 : 0, out = [];
    // ön eleme: arkadaşları kaba hat açıklığı × değerle sıralar, en umutlu 4'ünü ayrıntılı düşünür
    const lane = (a, b) => { let m = 0; for (let i = 1; i <= 4; i++) m = Math.max(m, oppAt(src, team, a.x + (b.x - a.x) * i / 5, a.y + (b.y - a.y) * i / 5)); return m; };
    const mates = src.filter(q => q.team === team && q !== h && q.role !== 'Bekçi' && Math.hypot(q.x - h.x, q.y - h.y) >= 4 && Math.hypot(q.x - h.x, q.y - h.y) <= 55).map(q => ({ q, s: Math.exp(-lane(h, q) * 2) * (threat(team, q.x, q.y) + .05) })).sort((a, b) => b.s - a.s).slice(0, D.nMates).map(o => o.q);
    for (const q of mates) {
      out.push({ kind: 'pas', to: { x: q.x, y: q.y }, q, arrive: D.arrive });
      // hat dar ise sert pas: rakibe tepki süresi bırakmaz, ama alıcı için tutması zor (karşılama kuralı hızı cezalandırır)
      if (lane(h, q) > .12) out.push({ kind: 'pas', firm: true, to: { x: q.x, y: q.y }, q, arrive: D.arriveFirm });
      const ux = gx - q.x, uy = 25 - q.y, un = Math.hypot(ux, uy) || 1;
      for (const L of [6, 12]) { const to = { x: q.x + ux / un * L, y: q.y + uy / un * L }; if (to.x < 2 || to.x > 98 || to.y < 2 || to.y > 48) continue; if (holdVal(src, team, to.x, to.y) > holdVal(src, team, q.x, q.y) + .03) out.push({ kind: 'önüne', to, q, arrive: D.arriveSpace, L }); }
      // aşırtma: aradaki gövdelerin ve alanların üstünden; uzak arkadaşa, ayağına ya da biraz önüne
      const dq = Math.hypot(q.x - h.x, q.y - h.y); if (dq >= D.lobMin) for (const L of [0, 4]) { const to = { x: q.x + ux / un * L, y: q.y + uy / un * L }; if (to.x < 2 || to.x > 98 || to.y < 2 || to.y > 48) continue; out.push({ kind: 'aşırt', to, q }); }
    }
    const gd = Math.hypot(gx - h.x, 25 - h.y);
    if (gd < D.shotRange) { for (const sd of [-1, 1]) { out.push({ kind: 'gönder', shot: 'güçlü', to: { x: gx, y: 25 + sd * K.MOUTH * D.aimPow }, v: D.shotV }); out.push({ kind: 'gönder', shot: 'plase', to: { x: gx, y: 25 + sd * K.MOUTH * D.aimPlase }, v: D.plaseV }); } }
    const fx = gx - h.x, fy = 25 - h.y, fn = Math.hypot(fx, fy) || 1;
    for (const a of [0, .7, -.7]) { const ca = Math.cos(a), sa = Math.sin(a), dx = (fx * ca - fy * sa) / fn, dy = (fx * sa + fy * ca) / fn, to = { x: h.x + dx * 6, y: h.y + dy * 6 }; if (to.x < 2 || to.x > 98 || to.y < 2 || to.y > 48) continue; out.push({ kind: 'sür', to, dir: { x: dx, y: dy } }); }
    out.push({ kind: 'tut' });
    return out;
  }
  // Değer: o noktadan Çekirdekle sonunda sayıya ulaşma ihtimali. Şarj, Kuyu'ya giden yoldaki rakip direncini deler; aynı yerde şarjlı Çekirdek daha değerlidir.
  // Bir noktanın değeri, Çekirdek oradayken: Bekçi o zamana kadar o noktaya göre yerleşmiş olur (Çekirdek–Kuyu hattında). Şu anki yeri, başka bir taşıyıcıya göre yerleşmiş hâli değil.
  function keeperFor(src, team, x, y) { const gx = team === 0 ? 100 : 0; return src.map(p => { if (p.role !== 'Bekçi' || p.team === team) return p; const dx = x - gx, dy = y - 25, L = Math.hypot(dx, dy) || 1, r = Math.min(6, L * .22); return { ...p, x: gx + dx / L * r, y: 25 + dy / L * r }; }); }
  // Savunmanın cevabı: Çekirdek "at" noktasına vardığında (T tik sonra), o noktaya en yakın iki savunmacı tepki süresinden sonra Kuyu tarafına, taşıyıcının önüne koşmuş olur.
  // Sürmenin ve pasın değeri bu cevaptan sonraki dünyada ölçülür; "önüm boş" ancak savunma oraya yetişemiyorsa gerçekten boştur.
  function respond(src, team, at, T, react) { const gx = team === 0 ? 100 : 0, gd = Math.hypot(gx - at.x, 25 - at.y) || 1, g = { x: at.x + (gx - at.x) / gd * 2.2, y: at.y + (25 - at.y) / gd * 2.2 }; const ds = src.filter(p => p.team !== team && p.role !== 'Bekçi' && !p.noTouch).sort((u, v) => Math.hypot(u.x - at.x, u.y - at.y) - Math.hypot(v.x - at.x, v.y - at.y)).slice(0, 2); return src.map(p => { if (!ds.includes(p)) return p; const rc = react != null ? react : 14 - (p.a.okuma ?? 10) * .5, run = Math.max(0, T - rc) * (.17 + (p.a.hiz ?? 10) * .006) * 1.25, dx = g.x - p.x, dy = g.y - p.y, L = Math.hypot(dx, dy); if (L <= run) return { ...p, x: g.x, y: g.y }; return { ...p, x: p.x + dx / L * run, y: p.y + dy / L * run }; }); }
  // Sürmeye savunmanın cevabı: sürüş yavaştır, yakındaki iki savunmacı T tik içinde tepki verip taşıyıcının yeni yerinin önüne (Kuyu tarafına) geçer, yetişebildiği kadar.
  // Pas hızlıdır: ona cevap kafadaki uçuşun içinde (araya girme) zaten var. Bu yüzden bu cevap sadece sürmeye uygulanır.
  function respond(src, team, at, T) { const gx = team === 0 ? 100 : 0, gd = Math.hypot(gx - at.x, 25 - at.y) || 1, g = { x: at.x + (gx - at.x) / gd * 2.2, y: at.y + (25 - at.y) / gd * 2.2 }; const ds = src.filter(p => p.team !== team && p.role !== 'Bekçi' && !p.noTouch).sort((u, v) => Math.hypot(u.x - at.x, u.y - at.y) - Math.hypot(v.x - at.x, v.y - at.y)).slice(0, 2); return src.map(p => { if (!ds.includes(p)) return p; const rc = Math.max(0, 8 - (p.a.okuma ?? 10) * .3), run = Math.max(0, T - rc) * (.17 + (p.a.hiz ?? 10) * .006) * 1.25, dx = g.x - p.x, dy = g.y - p.y, L = Math.hypot(dx, dy); if (L <= run) return { ...p, x: g.x, y: g.y }; return { ...p, x: p.x + dx / L * run, y: p.y + dy / L * run }; }); }
  function V(src, team, x, y, ch) { return D.vBase + D.vBuild * threat(team, x, y) + D.vShot * shotQ(keeperFor(src, team, x, y), team, x, y, ch); }
  // Buradan iyi bir gönderme ihtimali: Bekçi'nin uzanma süresine göre ağzın ne kadarı açık × Kuyu'ya hat (gövdeler; şarj alan direncini deler) × isabet
  // Gönderme tablosu: sadece Bekçi varken (Çekirdek–Kuyu hattında yerleşmiş) bu mesafe, açı ve şarjla yapılan göndermenin gerçek fizikle girme oranı.
  // Tahmin değil ölçüm: her hücre ilk istendiğinde gerçek vuruşlarla (güçlü/plase, iki köşe, Aktarım 10 hatası) bir kez oynanır ve saklanır.
  const SHT = new Map();
  function shotCell(di, ai, ci) {
    const key = di * 10000 + ai * 100 + ci; if (SHT.has(key)) return SHT.get(key);
    const K = C(), dist = 3 + di * 2, ang = ai * .2, ch = .1 + ci * .15, x = 100 - Math.cos(ang) * dist, y = 25 + Math.sin(ang) * dist, h = { x, y, team: 0, a: { aktarim: 10 }, R: 8, D: 1, name: 't' };
    const L = Math.hypot(100 - x, 25 - y) || 1, r = Math.min(6, L * .22), bk = { x: 100 - (100 - x) / L * r, y: 25 + (y - 25) / L * r, team: 1, role: 'Bekçi', a: { okuma: 10, hiz: 10, tutus: 10, kesme: 10, yogunluk: 10 }, R: 7, D: 1.2, name: 'b' };
    let s = 12345 + key; const rnd = () => { s = (s * 1664525 + 1013904223) >>> 0; return s / 4294967296; };
    let best = 0;
    for (const [v, aim] of [[D.shotV, D.aimPow], [D.plaseV, D.aimPlase]]) { let ok = 0, n = 0; for (const sd of [-1, 1]) for (let k = 0; k < 8; k++) { const tx = 100, ty = 25 + sd * K.MOUTH * aim, dx = tx - x, dy = ty - y, l = Math.hypot(dx, dy) || 1, v0 = v * K.chgMul(ch), e = applyError(h, { vx: dx / l * v0, vy: dy / l * v0 }, rnd), b = K.makeBall({ x, y, vx: e.vx, vy: e.vy, team: 0, ch, from: h }); for (let t = 0; t < 300 && !b.done; t++) { const q = K.stepAll(b, [h, bk], null, null, rnd); if (q && q.took) break; } if (b.done === 'kuyu-sag') ok++; n++; } best = Math.max(best, ok / n); }
    SHT.set(key, best); return best;
  }
  function shotTab(x, y, team, ch) { const gx = team === 0 ? 100 : 0, dist = Math.hypot(gx - x, 25 - y), ang = Math.atan2(Math.abs(25 - y), Math.abs(gx - x)); if (dist > 41) return 0; const fd = Math.max(0, (dist - 3) / 2), fc = Math.max(0, Math.min(6, (ch - .1) / .15)), ai = Math.min(7, Math.round(ang / .2)); const d0 = Math.floor(fd), c0 = Math.floor(fc), td = fd - d0, tc = fc - c0, g = (a, b) => shotCell(Math.min(19, a), ai, Math.min(6, b)); return (g(d0, c0) * (1 - td) + g(d0 + 1, c0) * td) * (1 - tc) + (g(d0, c0 + 1) * (1 - td) + g(d0 + 1, c0 + 1) * td) * tc; }
  // Buradan iyi bir gönderme ihtimali = gönderme tablosu (Bekçi + mesafe + açı + şarj, gerçek fizik) × Kuyu'ya giden hatta gövde yok (savunmacılar yetişemiyor)
  function shotQ(src, team, x, y, ch) { const Q = C().Q, gx = team === 0 ? 100 : 0, d = Math.hypot(gx - x, 25 - y) || 1; let pass = 1; const ux = (gx - x) / d, uy = (25 - y) / d; for (const p of src) { if (p.team === team || p.role === 'Bekçi') continue; const al = (p.x - x) * ux + (p.y - y) * uy; if (al <= 0 || al >= d) continue; const pe = Math.abs(-(p.x - x) * uy + (p.y - y) * ux), reachT = Math.max(0, al / 2.4 - (14 - (p.a.okuma ?? 10) * .5)) * (.17 + (p.a.hiz ?? 10) * .006) * 1.25, R = Q.reach + reachT; if (pe < R + .8) pass *= D.shotThru; } return shotTab(x, y, team, ch) * pass; }
  // Şarj kazanımı (maç da bunu kullanır): durarak ve boş alanda daha hızlı
  // Boşta olmak bölgeye göre değişir: rakibin Kuyu'suna yaklaştıkça bir rakibin "baskı" sayılması için çok daha yakın olması gerekir (orada boş kalmak zaten zordur).
  // Rakibin etki yarıçapı Kuyu'dan uzakta tam, Kuyu önünde D.zoneMin katına iner.
  function zoneR(team, x, y) { const gx = team === 0 ? 100 : 0, d = Math.hypot(gx - x, 25 - y); return D.zoneMin + (1 - D.zoneMin) * Math.min(1, Math.max(0, (d - 8) / D.zoneD)); }
  function pressAt(src, team, x, y) { const z = zoneR(team, x, y); let s = 0; for (const p of src) { if (p.team === team || p.noTouch) continue; const d = Math.hypot(p.x - x, p.y - y) / z, R = (p.R || 8) * .55; s += (p.D || 1) * Math.exp(-(d * d) / (R * R)); } return s; }
  function freeAt(src, team, x, y) { return Math.exp(-pressAt(src, team, x, y) * 1.6); }
  // Pasla gelen Çekirdeğin şarjı: pas şarjı düşürür, ama boşta karşılayan oyuncu Çekirdeği rahatça yerleştirir (boşluğu kadar şarj korunur)
  function recvCh(src, team, x, y, ch) { return Math.max(.15, ch * .7, D.chRecv * freeAt(src, team, x, y)); }
  function chargeRate(h, src, still, ch) { const pr = pressAt(src, h.team, h.x, h.y); return D.chg0 * (.4 + .6 * still) * (.25 + .75 * Math.exp(-pr * 2)) * (1 - ch * .5) * (1 + ((h.a.tutus ?? 10) - 10) * .05); }
  // T tik boyunca Çekirdeği elde tutabilme ihtimali: yakındaki her rakip ikili mücadeleye girebilir (maçtaki kural)
  function keepP(h, src, dir, T) { let keep = 1; const vC = (.17 + (h.a.hiz ?? 10) * .006) * .85; for (const q of src) { if (q.team === h.team || q.role === 'Bekçi' || q.noTouch) continue; const dx = h.x - q.x, dy = h.y - q.y, d = Math.hypot(dx, dy) || 1e-6, vO = (.17 + (q.a.hiz ?? 10) * .006) * 1.25, away = dir ? (dir.x * dx + dir.y * dy) / d : 0, reachT = Math.max(0, d - DU.engage) / Math.max(.03, vO - vC * away); if (reachT > T) continue; const P = duelP(q, h, src), tries = 1 + Math.floor((T - reachT) / DU.cd); if (P < commitThr(q, h, src)) continue; keep *= Math.pow(1 - P, tries); } return keep; }
  // İkili mücadele. Savunmacı girerse: başarı ihtimali Kesme-Sürme farkı ve taşıyıcının etrafındaki kafes (başka rakipler) ile belirlenir.
  function duelP(q, h, src) { let cage = 0; for (const o of src) if (o !== q && o.team === q.team && o.role !== 'Bekçi' && !o.noTouch && Math.hypot(o.x - h.x, o.y - h.y) < 3.2) cage++; return Math.min(.85, Math.max(.08, DU.base + ((q.a.kesme ?? 10) - (h.a.surme ?? 10)) * DU.skill + cage * DU.cage)); }
  // Girme kararı: savunmacı başarıyı kendi okumasıyla tahmin eder; arkasında güvence varsa daha kolay girer, son adamsa sabırlıdır.
  function commitThr(q, h, src) { const gx = q.team === 0 ? 0 : 100; let cover = false; for (const o of src) if (o !== q && o.team === q.team && o.role !== 'Bekçi' && Math.abs(o.x - gx) < Math.abs(h.x - gx) - 1 && Math.hypot(o.x - h.x, o.y - h.y) < 14) { cover = true; break; } return cover ? DU.thrCover : DU.thrLast; }
  // Kafada ileriye bakış: T tik sonra dünya nasıl olur? Çekirdeğe en yakın 2 rakip ona koşar, diğerleri kayar; arkadaşlar hedeflerine yürür.
  function predictWorld(src, team, B, T) {
    const near = new Set(src.filter(p => p.team !== team && p.role !== 'Bekçi').sort((u, v) => Math.hypot(u.x - B.x, u.y - B.y) - Math.hypot(v.x - B.x, v.y - B.y)).slice(0, 2));
    return src.map(p => { if (p.role === 'Bekçi') return p; const sp = .17 + (p.a.hiz ?? 10) * .006; let gx, gy, maxD;
      if (p.team !== team) { maxD = sp * 1.15 * Math.max(0, T - (14 - (p.a.okuma ?? 10) * .5)); if (near.has(p)) { const gxk = p.team === 0 ? 0 : 100, dx = gxk - B.x, dy = 25 - B.y, L = Math.hypot(dx, dy) || 1; gx = B.x + dx / L * 2; gy = B.y + dy / L * 2; } else { gx = p.x + (B.x - p.x) * .25; gy = p.y + (B.y - p.y) * .5; } }
      else { if (p.tx == null || (p.x === B.x && p.y === B.y)) return p; maxD = sp * T; gx = p.tx; gy = p.ty; }
      const dx = gx - p.x, dy = gy - p.y, d = Math.hypot(dx, dy); if (d < .01) return p; const k = Math.min(1, maxD / d); return { ...p, x: p.x + dx * k, y: p.y + dy * k, vx: 0, vy: 0, _o: p }; });
  }
  // İkinci adım: Çekirdek bu oyuncudayken (tahmini dünyada) en iyi sonraki pas
  function secondStep(world, carrier, ch, rnd) {
    const team = carrier.team, lane = (a, b) => { let m = 0; for (let i = 1; i <= 4; i++) m = Math.max(m, oppAt(world, team, a.x + (b.x - a.x) * i / 5, a.y + (b.y - a.y) * i / 5)); return m; };
    const cand = world.filter(q => q.team === team && q !== carrier && q.role !== 'Bekçi' && Math.hypot(q.x - carrier.x, q.y - carrier.y) >= 4 && Math.hypot(q.x - carrier.x, q.y - carrier.y) <= 50).map(q => ({ q, s: Math.exp(-lane(carrier, q) * 2) * V(world, team, q.x, q.y, ch) })).sort((a, b) => b.s - a.s).slice(0, 2);
    let best = 0, who = null;
    for (const { q } of cand) { const lf = launchFor(carrier, q, D.arrive, world, ch); if (!lf.ok) continue; const im = imagine(carrier, world, { x: carrier.x, y: carrier.y, vx: lf.vx, vy: lf.vy, ch }, rnd, 1, q, null), v = im.P * V(world, team, q.x, q.y, Math.max(.15, ch * .7)); if (v > best) { best = v; who = q._o || q; } }
    return { v: best, who };
  }
  // Gönderme ileriye bakılmaz: sonucu anında belli, kafadaki fizikle doğrudan ölçülür.
  // Her seçenek aynı gelecek örnekleriyle (aynı algı hatası, aynı zar) oynanır: seçenekler arasındaki fark gürültüden değil hamleden gelir.
  function lookAhead(h, src, ch, tac, opts, rnd) {
    const M = window.AlanMatch, team = h.team, okH = h.a.okuma ?? 10, w = Math.max(0, Math.min(1, (okH - 4) / 8)), n = okH >= 13 ? 5 : 4, seeds = Array.from({ length: n }, () => Math.floor(rnd() * 1e9)); if (w <= 0) return;
    const pick = [], fam = new Map(); for (const o of opts.filter(o => o.v != null && o.kind !== 'gönder').sort((a, b) => b.v - a.v)) { const k = o.q ? 'pas' : o.kind; const c = fam.get(k) || 0; if (c >= (k === 'pas' ? 2 : 1)) continue; fam.set(k, c + 1); pick.push(o); }
    const lookedBest = () => Math.max(...pick.map(o => o.v)); // bakılmayan seçenekler: her ailenin en iyisi bakıldığı için, daha düşük sezgisel değerleri bakılmış en iyiyi geçemez
    const err = (20 - okH) * D.errOk * D.errPos;
    D._inLook = true;
    try {
      for (const o of pick) {
        let sum = 0;
        for (let k = 0; k < n; k++) {
          let ss = seeds[k]; const rk = () => { ss = (ss * 1664525 + 1013904223) >>> 0; return ss / 4294967296; }; const m = M.createMatch(seeds[k], {}), map = new Map();
          m.ps = src.map(p => { const c = { ...p, a: p.a, slot: p.slot }; if (p.team !== team && p !== h) { c.x += (rk() - .5) * err; c.y += (rk() - .5) * err; } map.set(p, c); return c; });
          if (tac) m.tac[team] = { ...m.tac[team], ...tac };
          const hc = map.get(h); m.holder = hc; m.ball = null; m.ch = ch; m.decT = 1; m.fl = null; m.tick = 0;
          const od = D.decide; let first = true;
          D.decide = (hh, ps, c2, r2, t2) => { if (first && hh === hc) { first = false; const oo = { ...o, q: o.q ? map.get(o.q) : null }; return { best: oo, opts: [oo] }; } return od(hh, ps, c2, r2, t2); };
          let res = null;
          try { for (let t = 0; t < D.lookT; t++) { M.step(m); if (m.score[team] > 0) { res = 1; break; } if (m.score[1 - team] > 0) { res = -1; break; } if (m.holder && m.holder.team !== team) { res = -(V(m.ps, 1 - team, m.holder.x, m.holder.y, .3)); break; } } } catch (e) { res = 0; }
          finally { D.decide = od; }
          if (res == null) { const bx = m.holder ? m.holder.x : m.ball ? m.ball.x : h.x, by = m.holder ? m.holder.y : m.ball ? m.ball.y : h.y; res = m.holder ? V(m.ps, team, bx, by, m.ch) : .5 * V(m.ps, team, bx, by, .3); }
          sum += res;
        }
        o.look = sum / n; o.v = (1 - w) * o.v + w * o.look;
      }
      for (const o of opts) if (o.v != null && o.look == null && o.kind !== 'gönder') { const same = pick.filter(p => (p.q ? 'pas' : p.kind) === (o.q ? 'pas' : o.kind)); if (same.length) o.v = Math.min(o.v, ...same.map(p => p.v)); }
    } finally { D._inLook = false; }
  }
  // Tek dokunuş hatası: gelen Çekirdek ne kadar hızlıysa ve Aktarım ne kadar düşükse yönlendirmek o kadar zor
  function otFactor(p, inSp) { return 1 + D.otErr * Math.min(1, inSp / .8) * Math.max(.4, 1 - ((p.a.aktarim ?? 10) - 10) * .06); }
  // Tek dokunuş kararı: sadece yönlendirme seçenekleri (pas, önüne, aşırtma, kenardan, gönderme), hata çarpanı kafadaki denemelere de uygulanır
  function decideOT(h, src, ch, rnd, tac, ot) { D._ot = ot; try { const r = decide(h, src, ch, rnd, tac); const o = r.opts.find(o => o.kind !== 'sür' && o.kind !== 'tut' && o.launch); return o || null; } finally { D._ot = 0; } }
  function decide(h, src, ch, rnd, tac) {
    const K = C(), team = h.team, opts = options(h, src, ch), risk = D.riskBase * (1 - ((h.a.cesaret ?? 10) - 10) * .03) * (tac && tac.risk != null ? 1.4 - tac.risk * .8 : 1), T = Math.round(10 + (1 - (tac && tac.tempo != null ? tac.tempo : .5)) * 14 + 3);
    const vOpp = (x, y) => V(src, 1 - team, x, y, .3), chP = Math.max(.15, ch * .7), direct = new Map(), WD = predictWorld(src, team, h, T + 8);
    for (const o of opts) {
      if (o.kind === 'tut') { const P = keepP(h, src, null, T), c2 = Math.min(1, ch + chargeRate(h, src, 1, ch) * T); o.v = P * V(src, team, h.x, h.y, c2) - (1 - P) * vOpp(h.x, h.y) * risk; o.det = { P, ch: c2 }; }
      else if (o.kind === 'sür') { const P = keepP(h, src, o.dir, T), c2 = Math.min(1, ch + chargeRate(h, src, 0, ch) * T); o.v = P * V(respond(WD, team, o.to, T), team, o.to.x, o.to.y, c2) - (1 - P) * vOpp(o.to.x, o.to.y) * risk; o.det = { P, ch: c2 }; }
      else if (o.kind === 'gönder') { const dx = o.to.x - h.x, dy = o.to.y - h.y, L = Math.hypot(dx, dy) || 1, v0 = o.v * K.chgMul(ch) * (1 + ((h.a.aktarim ?? 10) - 10) * .02); o.launch = { vx: dx / L * v0, vy: dy / L * v0 }; const im = imagine(h, src, { x: h.x, y: h.y, ...o.launch, ch }, rnd, 3); o.v = im.Pk + Math.max(0, 1 - im.Pk - im.Pl) * V(src, team, im.end.x, im.end.y, .3) - im.Pl * vOpp(im.end.x, im.end.y) * risk; o.det = im; }
      else { const lf = o.kind === 'aşırt' ? lobFor(h, o.to) : launchFor(h, o.to, o.arrive, src, ch); if (!lf.ok) continue; o.launch = lf; const im = imagine(h, src, { x: h.x, y: h.y, vx: lf.vx, vy: lf.vy, air: lf.air || 0, ch }, rnd, D.trials, o.q, o.kind === 'önüne' || o.kind === 'aşırt' ? o.to : null); const at = o.kind === 'önüne' || o.kind === 'aşırt' ? o.to : o.q; o.v = im.P * V(WD, team, at.x, at.y, recvCh(WD, team, at.x, at.y, ch)) - im.Pl * vOpp(im.end.x, im.end.y) * risk; o.det = im; if (o.kind === 'pas') direct.set(o.q, im.P); }
    }
    // kenardan sektirme: sadece doğrudan hat zayıfsa çözüm olarak düşünülür (Okuma 10+)
    if ((h.a.okuma ?? 10) >= 10) for (const [q, Pd] of direct) {
      if (Pd >= .6) continue;
      for (const wy of [0, 50]) { const qy2 = wy === 0 ? -q.y : 100 - q.y, tt = (wy - h.y) / (qy2 - h.y); if (tt <= .15 || tt >= .85) continue; const B = { x: h.x + (q.x - h.x) * tt, y: wy === 0 ? .8 : 49.2 }, lf = bankFor(h, B, q, D.arrive, src, ch); if (!lf.ok) continue;
        const im = imagine(h, src, { x: h.x, y: h.y, vx: lf.vx, vy: lf.vy, ch }, rnd, D.trials, q), o = { kind: 'kenardan', to: B, q, launch: lf, det: im }; o.v = im.P * V(src, team, q.x, q.y, Math.max(.15, chP - .05)) - im.Pl * vOpp(im.end.x, im.end.y) * risk; opts.push(o); }
    }
    // ileriye bakış (Okuma ne kadar derin baktığını belirler): tutarsam rakipler gelince ne açılır? Pas verirsem alıcı oradan kime verir?
    const w2 = Math.max(0, Math.min(1, ((h.a.okuma ?? 10) - 6) / 10));
    if (w2 > 0) {
      const tut = opts.find(o => o.kind === 'tut');
      if (tut) { const wd = predictWorld(src, team, h, T), me = wd[src.indexOf(h)], c2 = tut.det.ch, nx = secondStep(wd, me, c2, rnd), here2 = V(src, team, h.x, h.y, c2), P = tut.det.P; tut.det.look = nx.v; tut.det.lookTo = nx.who; tut.v = P * ((1 - w2) * here2 + w2 * Math.max(here2, D.combo * nx.v)) - (1 - P) * vOpp(h.x, h.y) * risk; tut.why = nx.v * D.combo > here2 ? 'çek' : 'bekle'; }
      for (const o of opts.filter(o => o.q && o.v != null && o.det && o.det.end).sort((a, b) => b.v - a.v).slice(0, 3)) {
        const R = o.kind === 'önüne' ? o.to : { x: o.q.x, y: o.q.y }, tf = Math.hypot(R.x - h.x, R.y - h.y) / 1.2 + 6, wd = predictWorld(src, team, R, tf), qi = src.indexOf(o.q), rc = { ...(wd[qi]._o || wd[qi]), x: R.x, y: R.y }; wd[qi] = rc;
        const nx = secondStep(wd, rc, chP, rnd), vR = V(wd, team, R.x, R.y, chP), after = (1 - w2) * vR + w2 * Math.max(vR, D.combo * nx.v); o.v = o.det.P * after - o.det.Pl * vOpp(o.det.end.x, o.det.end.y) * risk; o.det.look = nx.v; o.det.lookTo = nx.who; if (D.combo * nx.v > vR) o.why = 'kombinasyon';
      }
    }
    // gelecekteki bir fırsat şimdiki kadar kesin değil: savunma bu sürede yerleşir, Bekçi açıyı kapatır. Gönderme "şimdi"dir; tutmak, sürmek ve pas sonradan gelen bir değerdir.
    for (const o of opts) { if (o.v == null || o.v <= 0 || o.kind === 'gönder') continue; const tw = o.kind === 'tut' || o.kind === 'sür' ? T : Math.hypot(o.to.x - h.x, o.to.y - h.y) / 1.2 + 6; o.v *= Math.pow(D.disc, tw); }
    // İleriye bakış (kısa oyun): Kuyu önünde, en umutlu birkaç farklı hamleyi kafada 1–1,5 saniye ileri oynatır. İki takım da aynı beyinle oynar: savunmacı cevabını verir, arkadaş açısını alır.
    // Sonuç: sayı = 1, kayıp = rakibin oradan topla oynama değeri kadar eksi, devam = o anki topla oynama değeri. Okuma ne kadar güvendiğini (ağırlık) ve kaç kez denediğini belirler.
    if (D.look && !D._inLook && h.slot && window.AlanMatch && Math.hypot((team === 0 ? 100 : 0) - h.x, 25 - h.y) < D.lookR) lookAhead(h, src, ch, tac, opts, rnd);
    const ok = opts.filter(o => o.v != null).sort((a, b) => b.v - a.v);
    return { best: ok[0], opts: ok };
  }
  function realize(h, o, src, ch, rnd) {
    src = src.filter(() => true);
    const K = C(); if (!o.launch) return null; const v = applyError(h, o.launch, rnd), b = K.makeBall({ x: h.x, y: h.y, vx: v.vx, vy: v.vy, air: v.air || 0, team: h.team, ch, from: null, recv: null }); let res = null;
    const w = src.map(p => ({ ...p })), q = o.q ? w[src.indexOf(o.q)] : null, hw = w[src.indexOf(h)], sp = p => (.17 + (p.a.hiz ?? 10) * .006) * 1.25;
    b.from = hw; b.recv = q;
    for (let t = 0; t < 900 && !b.done; t++) {
      // gerçek dünya: alıcı hedefe koşar, rakipler Okuma'ya bağlı gecikmeyle Çekirdeğe koşar
      const ch2 = (b.t === 0 ? (b._plan = chasePlan(K.predict(b, w, null, 200).pts, w.filter(p => p.team !== h.team), 0)) : b._plan);
      for (const p of w) { if (p.role === 'Bekçi' || p.id === h.id) continue; let tx = null, ty = null; if (p === q) { const g = o.kind === 'önüne' ? o.to : (b._rp || (b._rp = recvPoint(K.predict(b, w, null, 200).pts, q))); tx = g.x; ty = g.y; } else if (ch2.has(p) && t > ch2.get(p).rc) { tx = ch2.get(p).x; ty = ch2.get(p).y; } if (tx == null) continue; const dx = tx - p.x, dy = ty - p.y, L = Math.hypot(dx, dy); if (L > .2) { const v = Math.min(sp(p), L); p.x += dx / L * v; p.y += dy / L * v; } }
      const r = K.stepAll(b, w, null, null, rnd); if (r && r.took) { res = r.p.team === h.team ? 'mate' : 'opp'; break; }
    }
    if (!res && (b.done === 'durdu' || !b.done)) { b.vx = b.vy = 0; for (let t = 0; t < 200 && !res; t++) { for (const p of w) { if (p.role === 'Bekçi' || p.id === h.id) continue; const near = p === q || (p.team !== h.team && t > 14 - (p.a.okuma ?? 10) * .5 && Math.hypot(p.x - b.x, p.y - b.y) < 25); if (!near) continue; const dx = b.x - p.x, dy = b.y - p.y, L = Math.hypot(dx, dy); if (L < K.Q.reach) { res = p.team === h.team ? 'mate' : 'opp'; break; } const v = Math.min(sp(p), L); p.x += dx / L * v; p.y += dy / L * v; } } }
    if (!res) res = b.done && String(b.done).startsWith('kuyu') ? (((b.done === 'kuyu-sag') === (h.team === 0)) ? 'kuyu' : 'kendi-kuyu') : 'loose';
    return { res, b };
  }
  window.AlanDecide = { D, otFactor, decideOT, lobFor, shotQ, freeAt, pressAt, recvCh, zoneR, DU, predictWorld, secondStep, duelP, commitThr, V, chargeRate, keepP, chasePlan, threat, oppAt, holdVal, imagine, launchFor, bankFor, applyError, options, decide, realize };
})();
