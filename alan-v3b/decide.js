(function () {
  // Katman 3 · Taşıyıcının kararı: her seçeneği kafada yeni fizikle simüle eder, Okuma'ya göre kusurlu görür.
  const C = () => window.AlanCore, NOF = [];
  const DU = { base: .3, skill: .03, cage: .14, thrCover: .3, thrLast: .5, stun: 22, stunOk: .5, engage: 2.4, cd: 14 };
  const D = {
    simMax: 180, errOk: .05, errPos: 3,  // algı hatası: rakipler (20-Okuma)×errOk×errPos birim sapmayla görülür
    aimErr: .035, v0Err: .04,            // Aktarım hatası: açı ve hız
    arrive: .3, arriveSpace: .08, vmax: 3, shotV: 2.6, plaseV: 1.7,
    riskBase: 1, threatK: 22, sideK: .35, b0: .06, b1: .28, recvKeepT: 8, shotThru: .15, disc: .985, otErr: 1.2, otOk: 6, otMargin: .02, lobV: 1.05, lobLand: .88, lobMin: 12, lobMax: 50, lobErr: 1.8, spcBallV: 1, useValTab: false, useCurve: false, useSpace: true, look: true, lookR: 200, lookN: 3, lookT: 110, lookMargin: .1, laneLag: .4, aimPow: .45, aimPlase: .75, arriveFirm: .8, vK: .35, combo: .9, chV: .6, pierceV: .85, chg0: .0035, zoneMin: .45, zoneD: 32, chRecv: .55,
    trials: 2, nFull: 9, nSpots: 4, nFullLook: 3, nMates: 6, growMax: 8, shotRange: 34   // kafada: rakip ve alıcı Çekirdeğe doğru hareket eder (menzil zamanla büyür)
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
      const rc = lane && lane.loose ? Math.max(0, 6 - (p.a.okuma ?? 10) * .25) : Math.max(0, 14 - (p.a.okuma ?? 10) * .5 - (t0 || 0) + (lane ? lane.lag : 0)), v = sp(p); let g = null;
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
  // Alıcısız Çekirdek (gönderme) maçta da boştaki Çekirdek gibi okunur: rakiplerin tepkisi kısa
  function simOne(K, h, b, seen, q, depth, to, cache) {
    // kovalama planı alanları yok sayan hızlı bir yörüngeyle yapılır; Çekirdeğin kendisi gerçek alanlarla, temasa kadar adım adım ilerler
    let hit = null; const qq = q ? seen.find(p => p.id === q.id) : null, sp = p => (.17 + (p.a.hiz ?? 10) * .006) * 1.25, pts0 = K.predict(b, seen, null, D.simMax).pts, plan = chasePlan(pts0, seen.filter(p => p.team !== h.team), 0, { lag: (20 - (h.a.okuma ?? 10)) * D.laneLag, loose: !q }), qg = qq ? recvPoint(pts0, qq) : null, drift = seen.filter(p => p.team !== h.team && !plan.has(p) && p.role !== 'Bekçi' && (p.vx || p.vy)), bks = seen.filter(p => p.team !== h.team && p.role === 'Bekçi' && !plan.has(p));
    for (let t = 0; t < D.simMax && !b.done; t++) {
      if (t <= 30) for (const p of drift) { p.x += p.vx; p.y += p.vy; }
      // Bekçi maçtaki kuralla Çekirdeği izler: her an Çekirdek–Kuyu hattındaki yerine doğru, ataletle (maçtaki hareket kuralının aynısı)
      for (const p of bks) { const gx = p.team === 0 ? 0 : 100, dx = b.x - gx, dy = b.y - 25, L = Math.hypot(dx, dy) || 1, r = Math.max(2.5, Math.min(9, L * (.14 + (p.a.okuma ?? 10) * .004))), tx = gx + dx / L * r, ty = 25 + dy / L * r, ex = tx - p.x, ey = ty - p.y, el = Math.hypot(ex, ey), want = el < .3 ? 0 : Math.min(.17 + (p.a.hiz ?? 10) * .006, el * .25); p.vx = (p.vx || 0) + ((el ? ex / el * want : 0) - (p.vx || 0)) * .18; p.vy = (p.vy || 0) + ((el ? ey / el * want : 0) - (p.vy || 0)) * .18; p.x += p.vx; p.y += p.vy; }
      // tepki süresi dolana kadar her oyuncu o anki hızıyla gitmeye devam eder (Bekçi dahil); sonra hedefine koşar
      for (const [p, g] of plan) { if (t <= g.rc) { if (t <= 30) { p.x += p.vx || 0; p.y += p.vy || 0; } continue; } const dx = g.x - p.x, dy = g.y - p.y, L = Math.hypot(dx, dy); if (L > .2) { const v = Math.min(sp(p), L); p.x += dx / L * v; p.y += dy / L * v; } }
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
      // Algı hatası rakip gövdelerinde her yöne; Bekçi'de sadece Kuyu–Çekirdek hattı boyunca (Bekçi'nin hep o hatta durduğunu herkes bilir, belirsiz olan ne kadar önde olduğudur)
      const seen = src.map(p => { if (p === h) return p; if (p.team === h.team) return { ...p }; const e1 = (rnd() - .5) * err * D.errPos, e2 = (rnd() - .5) * err * D.errPos; if (p.role === 'Bekçi') { const gx = p.team === 0 ? 0 : 100, dx = b0.x - gx, dy = b0.y - 25, L = Math.hypot(dx, dy) || 1; return { ...p, x: p.x + dx / L * e1, y: p.y + dy / L * e1 }; } return { ...p, x: p.x + e1, y: p.y + e2 }; });
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
  function launchFor(h, to, arrive, src, ch) { const L = Math.hypot(to.x - h.x, to.y - h.y) || 1, dx = (to.x - h.x) / L, dy = (to.y - h.y) / L; if (reachSpeed(h, dx, dy, L, D.vmax, src, ch) < arrive) return { vx: dx * D.vmax, vy: dy * D.vmax, ok: false }; let lo = arrive, hi = D.vmax; for (let i = 0; i < 7; i++) { const mid = (lo + hi) / 2; if (reachSpeed(h, dx, dy, L, mid, src, ch) < arrive) lo = mid; else hi = mid; } return { vx: dx * hi, vy: dy * hi, ok: true }; }
  function solveTo(h, to, arrive, src, ch) { const L = Math.hypot(to.x - h.x, to.y - h.y) || 1, dx = (to.x - h.x) / L, dy = (to.y - h.y) / L; if (reachSpeed(h, dx, dy, L, D.vmax, src, ch) < arrive) return null; let lo = arrive, hi = D.vmax; for (let i = 0; i < 7; i++) { const mid = (lo + hi) / 2; if (reachSpeed(h, dx, dy, L, mid, src, ch) < arrive) lo = mid; else hi = mid; } return hi; }
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
      const dq = Math.hypot(q.x - h.x, q.y - h.y); if (dq >= D.lobMin) out.push({ kind: 'aşırt', to: { x: q.x, y: q.y }, q });
    }
    // Boşluğa pas: sabit mesafe yok. Sahada, bir arkadaşın savunmadan önce (Çekirdek oraya vardığında) yetişebileceği noktalar kafada taranır; en tehlikelileri (oradan topla oynamanın değeri) hedef olur.
    // Aynı nokta için yerden (önüne) ve havadan (aşırtma) seçenek üretilir; hangisinin tuttuğunu kafadaki deneme söyler.
    { const pts = [], mt = src.filter(p => p.team === team && p !== h && p.role !== 'Bekçi'); for (const m of mt) for (const r of [6, 10, 15, 21]) for (let k = 0; k < 10; k++) { const a = k * Math.PI / 5; pts.push({ x: m.x + Math.cos(a) * r, y: m.y + Math.sin(a) * r }); }
      const sp = p => .17 + (p.a.hiz ?? 10) * .006, cand = [];
      for (const p of pts) { if (p.x < 2 || p.x > 98 || p.y < 2 || p.y > 48) continue; const L = Math.hypot(p.x - h.x, p.y - h.y); if (L < 8 || L > D.lobMax) continue; const tb = L / 1.1; let bq = null, ta = 1e9, td = 1e9; for (const o of src) { if (o === h) continue; const t = Math.hypot(o.x - p.x, o.y - p.y) / (sp(o) * 1.25); if (o.team === team) { if (o.role !== 'Bekçi' && t < ta) { ta = t; bq = o; } } else { const tt = t + Math.max(0, 14 - (o.a.okuma ?? 10) * .5); if (tt < td) td = tt; } } const arr = Math.max(ta, tb); if (!bq || arr > td - 2) continue; const own = 1 / (1 + Math.exp((arr - td) / 5)); cand.push({ p, q: bq, own, s: own * V(src, team, p.x, p.y, ch) }); }
      // iki aşama (hız): önce ucuz değerle sırala, en iyi birkaçını "açtığı" ölçüyle (Vend) yeniden tart
      if (D.useSpace && !D._inLook) { cand.sort((a, b) => b.s - a.s); for (const c of cand.slice(0, 10)) c.s = c.own * Vend(src, team, c.p.x, c.p.y, ch); }
      cand.sort((a, b) => b.s - a.s); const used = []; for (const c of cand) { if (used.length >= D.nSpots) break; if (used.some(u => Math.hypot(u.p.x - c.p.x, u.p.y - c.p.y) < 5)) continue; used.push(c); out.push({ kind: 'önüne', to: c.p, q: c.q, arrive: D.arriveSpace, spot: 1 }); if (Math.hypot(c.p.x - h.x, c.p.y - h.y) >= D.lobMin) out.push({ kind: 'aşırt', to: c.p, q: c.q, spot: 1 }); } }
    const gd = Math.hypot(gx - h.x, 25 - h.y);
    if (gd < D.shotRange) { for (const sd of [-1, 1]) { out.push({ kind: 'gönder', shot: 'güçlü', to: { x: gx, y: 25 + sd * K.MOUTH * D.aimPow }, v: D.shotV }); out.push({ kind: 'gönder', shot: 'plase', to: { x: gx, y: 25 + sd * K.MOUTH * D.aimPlase }, v: D.plaseV }); } }
    const fx = gx - h.x, fy = 25 - h.y, fn = Math.hypot(fx, fy) || 1;
    for (const a of [0, .7, -.7]) { const ca = Math.cos(a), sa = Math.sin(a), dx = (fx * ca - fy * sa) / fn, dy = (fx * sa + fy * ca) / fn, to = { x: h.x + dx * 6, y: h.y + dy * 6 }; if (to.x < 2 || to.x > 98 || to.y < 2 || to.y > 48) continue; out.push({ kind: 'sür', to, dir: { x: dx, y: dy } }); }
    out.push({ kind: 'tut' });
    return out;
  }
  // TEK ÖLÇÜ. Her seçenek aynı birimle tartılır: "bu hamlenin sonunda bu takımın sayı atma ihtimali, eksi kayıpta rakibe verdiği ihtimal".
  // V: Çekirdek bu noktada, bu şarjla bizdeyken sayıya ulaşma ihtimali = ya buradan gönderirim (gerçek fizikle ölçülmüş gönderme tablosu × hatta gövde yok), ya da kurmaya devam ederim (bölgenin taban değeri).
  // Bekçi o noktaya göre yerleşmiş varsayılır (tablo zaten öyle ölçülmüştür).
  // Kurmaya devam etmenin değeri ölçülmüş tablodan (value-table.js): bu mesafede, savunma bu kadar açıkken (boşluk) hücumların sayıyla bitme oranı. Tablo yoksa eski taban sayılar.
  function build(src, team, x, y) { const Cv = window.ALAN_CURVE; if (D.useCurve && Cv) { const dd = Math.hypot((team === 0 ? 100 : 0) - x, 25 - y); return 1 / (1 + Math.exp(-(Cv.a + Cv.b * space(src, team, x, y) + Cv.c * dd / 100))); } const T = D.useValTab ? window.ALAN_VAL : null, gx = team === 0 ? 100 : 0, d = Math.hypot(gx - x, 25 - y); if (!T) return D.b0 + D.b1 * threat(team, x, y); const s = space(src, team, x, y), g = s < .1 ? 0 : s < .25 ? 1 : 2, f = Math.min(T.nb - 1, d / 8), i0 = Math.floor(f), i1 = Math.min(T.nb - 1, i0 + 1), t = f - i0; return T.cells[i0 * T.ng + g] * (1 - t) + T.cells[i1 * T.ng + g] * t; }
  // Boşluk: savunmanın ne kadar açık olduğu. Kuyu'nun önündeki noktalarda (yarıçap 6–30), bir hücumcunun oraya bir savunmacıdan önce varıp varamayacağı × oradan göndermenin (sadece Bekçi'ye karşı) değeri. En iyi nokta.
  // Savunma topa göre kaydıkça (Okuma gecikmesiyle) bu değer büyür, yerleşince küçülür. Çekirdeği hareket ettirmek, hat kırmak, rakibi üstüne çekmek bu sayıyı büyütür; bu yüzden kural yazmaya gerek yok.
  const SPC = new WeakMap(), SPT = []; for (const r of [6, 10, 14, 18, 22, 26, 30]) for (let k = -3; k <= 3; k++) SPT.push([r, k * .37]);
  // Çekirdeği tutan oyuncu boşluğa varan "arkadaş" sayılmaz: tutmak boşluk yaratmaz, kendi şansı zaten gönderme (shotQ) ve sürme seçeneklerinde ölçülür. Boşluk, Çekirdeği alacak arkadaşların varabileceği yerdir.
  // Boşluk topa göre ölçülür: o noktaya hem bir hücumcu hem Çekirdek (paslarla, ortalama D.spcBallV hızla), savunmadan önce varabilir mi × oradan gönderme değeri.
  // Kontrada uzaktaki Çekirdek bile boşluk sayılır: savunmacılar kendi Kuyu'larından uzaktaysa geri dönmeleri Çekirdeğin oraya gitmesinden uzun sürer.
  function space(src, team, bx, by) { let c = SPC.get(src); const key = team + ':' + Math.round(bx) + ',' + Math.round(by); if (c && c[key] != null) return c[key]; if (!c) { c = {}; SPC.set(src, c); } const gx = team === 0 ? 100 : 0, dir = team === 0 ? -1 : 1, B = { x: bx, y: by }; let best = 0;
    for (const [r, an] of SPT) { const x = gx + dir * r * Math.cos(an), y = 25 + r * Math.sin(an); if (y < 1 || y > 49) continue; let ta = 1e9, td = 1e9; for (const p of src) { const t = Math.hypot(p.x - x, p.y - y) / (.17 + (p.a.hiz ?? 10) * .006); if (p.team === team) { if (p.role !== 'Bekçi' && t < ta && Math.hypot(p.x - bx, p.y - by) > 1.5) ta = t; } else { const tt = t + 6 - (p.a.okuma ?? 10) * .3; if (tt < td) td = tt; } } const tb = Math.hypot(bx - x, by - y) / D.spcBallV, own = 1 / (1 + Math.exp((Math.max(ta, tb) - td) / 6)); if (own * shotTab(x, y, team, .4) <= best) continue; const v = own * shotQ(src, team, x, y, .4); if (v > best) best = v; } // gönderme değeri gövdeler dahil (shotQ): savunmacının kapattığı hat boşluk değildir
    c[key] = best; return best; }
  // Önündeki boşluk: Çekirdeğin 8–20 birim önündeki noktalardan, bir arkadaşın ve Çekirdeğin (paslarla) savunmadan önce varabildiği en değerli bölge. Bölgenin değeri buradaki taban değerle aynı ölçü.
  // Savunma kuralları belli olduğu için bu hesap kafada yapılır: ileriye bakış hamleyi oynatır, savunma kurallarıyla kayar, sonunda bu iki boşluk ölçülür.
  // Önündeki boşluk sahanın sabit noktalarında ölçülür (Çekirdeğe göre değil): Çekirdeği sürmek ölçülen bölgeyi kendisiyle birlikte taşıyıp bedava kazanç gibi görünmesin.
  const FRT = []; for (let x = 36; x <= 92; x += 8) for (let y = 6; y <= 44; y += 7.6) FRT.push([x, y]);
  const FRC = new WeakMap();
  function front(src, team, bx, by) { let c = FRC.get(src); const key = team + ':' + Math.round(bx) + ',' + Math.round(by); if (c && c[key] != null) return c[key]; if (!c) { c = {}; FRC.set(src, c); } let best = 0; for (const [fx, y] of FRT) { const x = team === 0 ? fx : 100 - fx; let ta = 1e9, td = 1e9; for (const p of src) { const t = Math.hypot(p.x - x, p.y - y) / (.17 + (p.a.hiz ?? 10) * .006); if (p.team === team) { if (p.role !== 'Bekçi' && t < ta && Math.hypot(p.x - bx, p.y - by) > 1.5) ta = t; } else { const tt = t + 6 - (p.a.okuma ?? 10) * .3; if (tt < td) td = tt; } } const tb = Math.hypot(bx - x, by - y) / D.spcBallV, own = 1 / (1 + Math.exp((Math.max(ta, tb) - td) / 6)), v = own * (D.b0 + D.b1 * threat(team, x, y)); if (v > best) best = v; } c[key] = best; return best; }
  // Boşluk, ancak bir hamle dizisinin SONUNDA değerlendirilir (ileriye bakışın bitişi): "şu an tutarsam arkadaşın boşluğu bende kalır" diye sayılırsa tutmak hep en iyi görünür.
  // Hemen yapılacak hamlelerin değeri ise gerçekleşmiş sonuçla (gönderme, pas, kayıp) ölçülür; boşluğu kullanmak için o pası atmak gerekir.
  function Vend(src, team, x, y, ch) { return Math.max(shotQ(src, team, x, y, ch), build(src, team, x, y), space(src, team, x, y), front(src, team, x, y)); }
  // Sürmenin önündeki boşluk: taşıyıcı (sürerek, daha yavaş) önündeki noktalara savunmadan önce varabilir mi × o bölgenin değeri. Sadece sürme seçeneğinde sayılır: tutmak bu boşluğu kazandırmaz.
  function runOn(w, team, h, at) { const dir = team === 0 ? 1 : -1, vC = (.17 + (h.a.hiz ?? 10) * .006) * .85; let best = 0; for (const f of [6, 12, 18]) for (const s of [-8, 0, 8]) { const x = at.x + dir * f, y = at.y + s; if (x < 2 || x > 98 || y < 2 || y > 48) continue; const ta = Math.hypot(x - at.x, y - at.y) / vC; let td = 1e9; for (const p of w) { if (p.team === team || p.role === 'Bekçi') continue; td = Math.min(td, Math.hypot(p.x - x, p.y - y) / (.17 + (p.a.hiz ?? 10) * .006) + 6 - (p.a.okuma ?? 10) * .3); } const own = 1 / (1 + Math.exp((ta - td) / 6)), v = own * Math.max(shotQ(w, team, x, y, .4), D.b0 + D.b1 * threat(team, x, y)); if (v > best) best = v; } return best; }
  function V(src, team, x, y, ch) { return false ? Math.max(shotQ(src, team, x, y, ch), build(src, team, x, y), space(src, team, x, y), front(src, team, x, y)) : Math.max(shotQ(src, team, x, y, ch), build(src, team, x, y)); }
  // Savunmanın cevabı: Çekirdek "at" noktasına T tik sonra varır (sürerek ya da pasla). O noktaya en yakın iki savunmacı tepki süresinden sonra onun Kuyu tarafına koşar, yetişebildiği kadar.
  // Sürüş göz önünde olur, çabuk okunur; pası okumak daha uzun sürer (Okuma).
  // Kaba hat açıklığı (ön eleme için): A'dan B'ye düz pasın yolundaki her rakip, Çekirdek yanından geçerken uzanıp yetişebilir mi
  function laneOk(src, team, A, B) { const L = Math.hypot(B.x - A.x, B.y - A.y) || 1, ux = (B.x - A.x) / L, uy = (B.y - A.y) / L; let open = 1; for (const o of src) { if (o.team === team || o.role === 'Bekçi' || o.noTouch) continue; const al = (o.x - A.x) * ux + (o.y - A.y) * uy; if (al <= 0 || al >= L + 1) continue; const pe = Math.abs(-(o.x - A.x) * uy + (o.y - A.y) * ux), R = 1.6 + Math.max(0, al / 1.1 - (14 - (o.a.okuma ?? 10) * .5)) * (.17 + (o.a.hiz ?? 10) * .006) * 1.25; open *= 1 - 1 / (1 + Math.exp((pe - R) / .6)); } return open; }
  function respond(src, team, at, T, kind) { const gx = team === 0 ? 100 : 0, gd = Math.hypot(gx - at.x, 25 - at.y) || 1, g = { x: at.x + (gx - at.x) / gd * 2.2, y: at.y + (25 - at.y) / gd * 2.2 }; const ds = src.filter(p => p.team !== team && p.role !== 'Bekçi' && !p.noTouch).sort((u, v) => Math.hypot(u.x - at.x, u.y - at.y) - Math.hypot(v.x - at.x, v.y - at.y)).slice(0, 2); return src.map(p => { if (!ds.includes(p)) return p; const ok = p.a.okuma ?? 10, rc = kind === 'drive' ? Math.max(0, 8 - ok * .3) : Math.max(0, 14 - ok * .5), run = Math.max(0, T - rc) * (.17 + (p.a.hiz ?? 10) * .006) * 1.25, dx = g.x - p.x, dy = g.y - p.y, L = Math.hypot(dx, dy); if (L <= run) return { ...p, x: g.x, y: g.y }; return { ...p, x: p.x + dx / L * run, y: p.y + dy / L * run }; }); }
  // Buradan iyi bir gönderme ihtimali: Bekçi'nin uzanma süresine göre ağzın ne kadarı açık × Kuyu'ya hat (gövdeler; şarj alan direncini deler) × isabet
  // Gönderme tablosu: sadece Bekçi varken (Çekirdek–Kuyu hattında yerleşmiş) bu mesafe, açı ve şarjla yapılan göndermenin gerçek fizikle girme oranı.
  // Tahmin değil ölçüm: her hücre ilk istendiğinde gerçek vuruşlarla (güçlü/plase, iki köşe, Aktarım 10 hatası) bir kez oynanır ve saklanır.
  // Önceden ölçülmüş tablo (shot-table.js) varsa ve Kuyu ölçüsü aynıysa onu kullanır; yoksa hücreleri ilk istendiğinde ölçer.
  const SHT = new Map(); { const T0 = window.ALAN_SHT; if (T0 && T0.mouth === C().MOUTH && T0.kr === C().KR) for (const [k, v] of T0.cells) SHT.set(k, v); }
  function shotCell(di, ai, ci) {
    // Hücre, taşıyıcının kafasındaki gönderme denemesiyle aynı yoldan ölçülür (imagine: Aktarım hatası, Bekçi'nin tepkisi ve uzanması dahil). Böylece "buradan gönderirsem" ile "şimdi gönderirsem" aynı sayıyı verir.
    const key = di * 10000 + ai * 100 + ci; if (SHT.has(key)) return SHT.get(key);
    const K = C(), dist = 3 + di * 2, ang = ai * .2, ch = .1 + ci * .15, x = 100 - Math.cos(ang) * dist, y = 25 + Math.sin(ang) * dist, h = { id: -1, x, y, team: 0, a: { aktarim: 10, okuma: 20 }, R: 8, D: 1, name: 't' };
    const L = Math.hypot(100 - x, 25 - y) || 1, r = Math.min(6, L * .22), bk = { id: -2, x: 100 - (100 - x) / L * r, y: 25 + (y - 25) / L * r, team: 1, role: 'Bekçi', a: { okuma: 10, hiz: 10, tutus: 10, kesme: 10, yogunluk: 10 }, R: 7, D: 1.2, name: 'b' };
    let s = 12345 + key; const rnd = () => { s = (s * 1664525 + 1013904223) >>> 0; return s / 4294967296; };
    let best = 0;
    for (const [v, aim] of [[D.shotV, D.aimPow], [D.plaseV, D.aimPlase]]) { let pk = 0; for (const sd of [-1, 1]) { const ty = 25 + sd * K.MOUTH * aim, dx = 100 - x, dy = ty - y, l = Math.hypot(dx, dy) || 1, v0 = v * K.chgMul(ch), im = imagine(h, [h, bk], { x, y, vx: dx / l * v0, vy: dy / l * v0, ch }, rnd, 6); pk += im.Pk / 2; } best = Math.max(best, pk); }
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
    const fwdOf = o => (o.to.x - h.x) * (team === 0 ? 1 : -1), famOf = o => o.kind === 'aşırt' ? 'aşırt' : o.q ? (fwdOf(o) < -2 ? 'geri' : 'pas') : o.kind;
    const pick = [], fam = new Map(); for (const o of opts.filter(o => o.v != null && o.kind !== 'gönder').sort((a, b) => b.v - a.v)) { const k = famOf(o); const c = fam.get(k) || 0; if (c >= (k === 'pas' ? 2 : 1)) continue; fam.set(k, c + 1); pick.push(o); }
    { const sup = opts.filter(o => o.v != null && o.q && o.next != null && !pick.includes(o)).sort((a, b) => b.next - a.next)[0]; if (sup) pick.push(sup); } // destek pası: açtığı seçenek en büyük olan, kafada mutlaka denenir
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
          const hc = map.get(h); m.holder = hc; m.ball = null; m.ch = ch; m.decT = 1; m.fl = null; m.tick = 0; m._lite = true; // kafadaki oyun: topsuz oyuncular hedeflerini yeniden seçmez, tek dokunuş düşünülmez (hız için)
          // ilk hamleyi zorla: maç kararı modül nesnesinden (window.AlanDecide.decide) çağırır; parametre nesnesi D'den değil
          const AD = window.AlanDecide, od = AD.decide; let first = true;
          AD.decide = (hh, ps, c2, r2, t2) => { if (first && hh === hc) { first = false; const oo = { ...o, q: o.q ? map.get(o.q) : null }; return { best: oo, opts: [oo] }; } return od(hh, ps, c2, r2, t2); };
          let res = null;
          try { for (let t = 0; t < D.lookT; t++) { M.step(m); if (m.score[team] > 0) { res = 1; break; } if (m.score[1 - team] > 0) { res = -1; break; } if (m.holder && m.holder.team !== team) { res = -((D.useSpace ? Vend : V)(m.ps, 1 - team, m.holder.x, m.holder.y, .3)); break; } } } catch (e) { res = 0; }
          finally { AD.decide = od; }
          if (res == null) { const bx = m.holder ? m.holder.x : m.ball ? m.ball.x : h.x, by = m.holder ? m.holder.y : m.ball ? m.ball.y : h.y; res = m.holder ? (D.useSpace ? Vend : V)(m.ps, team, bx, by, m.ch) : 0; }
          sum += res;
        }
        o.look = sum / n; o.v = (1 - w) * o.v + w * o.look;
      }
      for (const o of opts) if (o.v != null && o.look == null && o.kind !== 'gönder') { const same = pick.filter(p => famOf(p) === famOf(o)); if (same.length) o.v = Math.min(o.v, ...same.map(p => p.v)); }
    } finally { D._inLook = false; }
  }
  // Tek dokunuş hatası: gelen Çekirdek ne kadar hızlıysa ve Aktarım ne kadar düşükse yönlendirmek o kadar zor
  function otFactor(p, inSp) { return 1 + D.otErr * Math.min(1, inSp / .8) * Math.max(.4, 1 - ((p.a.aktarim ?? 10) - 10) * .06); }
  // Tek dokunuş kararı: sadece yönlendirme seçenekleri (pas, önüne, aşırtma, kenardan, gönderme), hata çarpanı kafadaki denemelere de uygulanır
  function decideOT(h, src, ch, rnd, tac, ot) { D._ot = ot; try { const r = decide(h, src, ch, rnd, tac); const o = r.opts.find(o => o.kind !== 'sür' && o.kind !== 'tut' && o.launch); return o || null; } finally { D._ot = 0; } }
  function decide(h, src, ch, rnd, tac) {
    const K = C(), team = h.team, opts = options(h, src, ch), risk = D.riskBase * (1 - ((h.a.cesaret ?? 10) - 10) * .03) * (tac && tac.risk != null ? 1.4 - tac.risk * .8 : 1), T = Math.round(10 + (1 - (tac && tac.tempo != null ? tac.tempo : .5)) * 14 + 3);
    const Vo = (w, x, y) => V(w, 1 - team, x, y, .3), WD = predictWorld(src, team, h, T + 8), direct = new Map(), srcC = Object.assign(src.slice(), { _kc: new Map() });
    // Alıcının değeri: Çekirdeği o dünyada karşıladıktan sonra da elinde tutabilir mi (yanındaki markajcının ikili mücadelesi)? Tutarsa oradan V, kaybederse rakibe oradan V.
    const recvVal = (w, q, at, c) => { const r = { ...q, x: at.x, y: at.y }, kA = keepP(r, w, null, D.recvKeepT); return kA * V(w, team, at.x, at.y, c) - (1 - kA) * Vo(w, at.x, at.y) * risk; };
    const lossAt = (x, y) => Vo(src, x, y) * risk;
    // Ön eleme da "açtığı" değeri görür (Vend: boşluk ve önündeki boşluk dahil): yoksa orta sahada değer düz olduğu için açık alana pas daha kafada denenmeden elenir.
  // Ön eleme (hız): her pas türü ve gönderme önce kaba bir ölçüyle (hat açık mı × varış noktasının değeri) sıralanır; sadece en umutlu birkaçı kafada gerçekten oynatılır.
    { const inL = D._inLook, nP = inL ? D.nFullLook : D.nFull, nS = inL ? 1 : 2; const mates = src.filter(p => p.team === team && p !== h && p.role !== 'Bekçi'), nextOf = (q, at) => { let b = 0; for (const r of mates) { if (r === q) continue; const v = laneOk(src, team, at, r) * V(src, team, r.x, r.y, ch); if (v > b) b = v; } return b; };
      const rough = o => { const at = o.kind === 'önüne' || o.kind === 'aşırt' ? o.to : o.q || o.to; if (o.kind === 'gönder') return laneOk(src, team, h, at) * shotQ(src, team, h.x, h.y, ch) * (o.shot === 'plase' ? 1 : 1.02); const l = o.kind === 'aşırt' ? .85 : laneOk(src, team, h, at); o.next = o.q ? D.combo * nextOf(o.q, at) : 0; return l * Math.max((D.useSpace && !D._inLook ? Vend : V)(src, team, at.x, at.y, ch), o.next); };
      // bir pas, kendi değeriyle değil açtığı seçenekle de umutlu sayılır (alıcının oradan yapabileceği en iyi pas, kaba)
      const pl = opts.filter(o => o.q).map(o => [o, rough(o)]).sort((a, b) => b[1] - a[1]), sh = opts.filter(o => o.kind === 'gönder').map(o => [o, rough(o) + (o.to.y - 25) * (src.find(p => p.role === 'Bekçi' && p.team !== team)?.y > 25 ? -1 : 1) * 1e-3]).sort((a, b) => b[1] - a[1]);
      const drop = new Set([...pl.slice(nP), ...sh.slice(nS)].map(x => x[0])); for (let i = opts.length - 1; i >= 0; i--) if (drop.has(opts[i])) opts.splice(i, 1); }
    for (const o of opts) {
      if (o.kind === 'tut') { const P = keepP(h, src, null, T), c2 = Math.min(1, ch + chargeRate(h, src, 1, ch) * T), w = respond(src, team, h, T, 'drive'); o.v = P * V(w, team, h.x, h.y, c2) - (1 - P) * lossAt(h.x, h.y); o.det = { P, ch: c2 }; }
      else if (o.kind === 'sür') { const P = keepP(h, src, o.dir, T), c2 = Math.min(1, ch + chargeRate(h, src, 0, ch) * T), w = respond(WD, team, o.to, T, 'drive'); o.v = P * Math.max(V(w, team, o.to.x, o.to.y, c2), D.useSpace ? runOn(w, team, h, o.to) : 0) - (1 - P) * lossAt(o.to.x, o.to.y); o.det = { P, ch: c2 }; }
      else if (o.kind === 'gönder') { const dx = o.to.x - h.x, dy = o.to.y - h.y, L = Math.hypot(dx, dy) || 1, v0 = o.v * K.chgMul(ch) * (1 + ((h.a.aktarim ?? 10) - 10) * .02); o.launch = { vx: dx / L * v0, vy: dy / L * v0 }; const im = imagine(h, src, { x: h.x, y: h.y, ...o.launch, ch }, rnd, D._inLook ? 2 : 6); o.v = im.Pk + im.P * V(src, team, im.end.x, im.end.y, .3) - im.Pl * lossAt(im.end.x, im.end.y); o.det = im; }
      else { const lf = o.kind === 'aşırt' ? lobFor(h, o.to) : launchFor(h, o.to, o.arrive, srcC, ch); if (!lf.ok) continue; o.launch = lf; const im = imagine(h, src, { x: h.x, y: h.y, vx: lf.vx, vy: lf.vy, air: lf.air || 0, ch }, rnd, D._inLook ? 1 : D.trials, o.q, o.kind === 'önüne' || o.kind === 'aşırt' ? o.to : null), at = o.kind === 'önüne' || o.kind === 'aşırt' ? o.to : o.q, tf = Math.hypot(at.x - h.x, at.y - h.y) / Math.max(.5, Math.hypot(lf.vx, lf.vy) * .7) + 4, w = respond(WD, team, at, tf, 'pass'); o.v = im.Pk + im.P * recvVal(w, o.q, at, recvCh(w, team, at.x, at.y, ch)) - im.Pl * lossAt(im.end.x, im.end.y); o.det = im; if (o.kind === 'pas') direct.set(o.q, im.P); }
    }
    // kenardan sektirme: sadece doğrudan hat zayıfsa çözüm olarak düşünülür (Okuma 10+)
    if ((h.a.okuma ?? 10) >= 10) for (const [q, Pd] of direct) {
      if (Pd >= .6) continue;
      for (const wy of [0, 50]) { const qy2 = wy === 0 ? -q.y : 100 - q.y, tt = (wy - h.y) / (qy2 - h.y); if (tt <= .15 || tt >= .85) continue; const B = { x: h.x + (q.x - h.x) * tt, y: wy === 0 ? .8 : 49.2 }, lf = bankFor(h, B, q, D.arrive, srcC, ch); if (!lf.ok) continue;
        const im = imagine(h, src, { x: h.x, y: h.y, vx: lf.vx, vy: lf.vy, ch }, rnd, D.trials, q), o = { kind: 'kenardan', to: B, q, launch: lf, det: im }, tf = (Math.hypot(B.x - h.x, B.y - h.y) + Math.hypot(q.x - B.x, q.y - B.y)) / Math.max(.5, Math.hypot(lf.vx, lf.vy) * .6) + 4, w = respond(WD, team, q, tf, 'pass'); o.v = im.P * recvVal(w, q, q, Math.max(.15, recvCh(w, team, q.x, q.y, ch) - .05)) - im.Pl * lossAt(im.end.x, im.end.y); opts.push(o); }
    }
    // İleriye bakış (kısa oyun): Kuyu önünde, en umutlu birkaç farklı hamleyi kafada 1–1,5 saniye ileri oynatır. İki takım da aynı beyinle oynar: savunmacı cevabını verir, arkadaş açısını alır.
    // Sonuç: sayı = 1, kayıp = rakibin oradan topla oynama değeri kadar eksi, devam = o anki topla oynama değeri. Okuma ne kadar güvendiğini (ağırlık) ve kaç kez denediğini belirler.
    // Her kararda kafada oynatır (kararsızlık şartı yok). Eski not: Sadece kararsız kaldığında ileri bakar: en iyi iki farklı hamle türü birbirine yakınsa (fark D.lookMargin'dan küçük)
    if (D.look && !D._inLook && h.slot && window.AlanMatch && Math.hypot((team === 0 ? 100 : 0) - h.x, 25 - h.y) < D.lookR) { const fam = new Map(); for (const o of opts) { if (o.v == null) continue; const k = o.kind === 'gönder' ? 'gönder' : o.kind === 'aşırt' ? 'aşırt' : o.q ? ((o.to.x - h.x) * (team === 0 ? 1 : -1) < -2 ? 'geri' : 'pas') : o.kind; if (!fam.has(k) || fam.get(k) < o.v) fam.set(k, o.v); } const fv = [...fam.values()].sort((a, b) => b - a); if (fv.length > 1) lookAhead(h, src, ch, tac, opts, rnd); }
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
  window.AlanDecide = { D, Vend, space, front, SHT, shotCell, otFactor, decideOT, lobFor, shotQ, freeAt, pressAt, recvCh, zoneR, DU, predictWorld, secondStep, duelP, commitThr, V, chargeRate, keepP, chasePlan, threat, oppAt, holdVal, imagine, launchFor, bankFor, applyError, options, decide, realize };
})();
