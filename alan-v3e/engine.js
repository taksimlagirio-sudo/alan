(function () {
  // ALAN v3e · tam saha 7'ye 7. Küçük saha motorunun bütün mekanikleri (gövde, dönüş, fiske, kenar çalımı, pres planları, kenardan pas, tek dokunuş, tepki süreli pas kesme, şarj, alanlar)
  // + v3d'nin taktik ayarları (sistem, pres, arkada kalan, blok, genişlik, tempo, risk, kazanınca). Taşıyıcı seçeneklerini kafasında oynatır; kafadaki oynama gerçek oynamayla aynı adım fonksiyonunu kullanır (rnd null = beklenen değer).
  const C = () => window.AlanCore, W = 100, H = 50;
  const cl = (v, a, b) => v < a ? a : v > b ? b : v, wrap = a => { while (a > Math.PI) a -= 2 * Math.PI; while (a < -Math.PI) a += 2 * Math.PI; return a; }, dist = (a, b) => Math.hypot(a.x - b.x, a.y - b.y);
  const dirOf = t => t === 0 ? 1 : -1, goalOf = t => ({ x: t === 0 ? W : 0, y: H / 2 }), ownGoal = t => goalOf(1 - t), depth = (t, x) => t === 0 ? x : W - x, fromDepth = (t, d) => t === 0 ? d : W - d;
  const spd = p => .17 + (p.a.hiz ?? 10) * .006, react = p => 14 - (p.a.okuma ?? 10) * .5, isK = p => p.role === 'Bekçi';
  const TAC0 = { sistem: 'Alan', pres: 1, arkada: 1, blok: 'Orta', genislik: 'Normal', tempo: .5, risk: .5, kazaninca: 'Dengeli', tek: 'açık' };
  const BLOK = { 'Düşük': 16, 'Orta': 28, 'Yüksek': 44 }, PRESS = { 'Düşük': 38, 'Orta': 58, 'Yüksek': 100 }, GEN = { 'Dar': 26, 'Normal': 36, 'Geniş': 46 };
  const SLOTS = [{ d: .1, l: -.45 }, { d: .1, l: .45 }, { d: .45, l: -.8 }, { d: .45, l: 0 }, { d: .45, l: .8 }, { d: .85, l: 0 }];
  function create(cfg) {
    const ps = []; let id = 0;
    for (const t of [0, 1]) { const q = (cfg.q && cfg.q[t]) ?? 10, hz = (cfg.hiz && cfg.hiz[t]) ?? 10;
      ps.push({ id: id++, team: t, role: 'Bekçi', slot: null, x: 0, y: 0, vx: 0, vy: 0, R: 7, D: 1.2, ca: 0, cd: 0, a: { surme: 10, okuma: q, tutus: q, aktarim: q, kesme: 10, hiz: 10 } });
      SLOTS.forEach((s, i) => ps.push({ id: id++, team: t, role: 'Oyuncu', i, slot: s, x: 0, y: 0, vx: 0, vy: 0, R: 8, D: 1, ca: 0, cd: 0, a: { surme: q, okuma: q, tutus: q, aktarim: q, kesme: q, hiz: hz } })); }
    const st = { cfg, tac: [0, 1].map(t => ({ ...TAC0, ...((cfg.tac && cfg.tac[t]) || {}) })), ps, h: null, b: null, ch: .3, act: { k: 'kalkan' }, t: 0, len: cfg.len || 2700, score: [0, 0], keep: 1, res: null, kickT: -99, looseT: -99, recv: null, passT: null, then: null, kicker: null, route: null, chaser: null, winT: -999, winTeam: -1, nd: 0,
      S: { poss: [0, 0], shot: [0, 0], pass: [0, 0], passOk: [0, 0], tek: [0, 0], tekShot: [0, 0], bank: [0, 0], bankOk: [0, 0], win: [0, 0], gTek: [0, 0], gKontra: [0, 0], gSabit: [0, 0] } };
    for (const t of [0, 1]) { const D = ps.filter(p => p.team === 1 - t && !isK(p)); const used = new Set(); ps.filter(p => p.team === t && !isK(p)).map(d => D.map(a => [d, a, Math.abs(a.slot.d - (1 - d.slot.d)) * 30 + Math.abs(a.slot.l + d.slot.l) * 10])).flat().sort((u, v) => u[2] - v[2]).forEach(([d, a]) => { if (d.man == null && !used.has(a.id)) { d.man = a.id; used.add(a.id); } }); }
    kickoff(st, 0); return st;
  }
  function kickoff(st, t) { for (const p of st.ps) { const tm = p.team; if (isK(p)) { p.x = fromDepth(tm, 2.5); p.y = 25; } else { p.x = fromDepth(tm, 8 + p.slot.d * 34); p.y = 25 + p.slot.l * 15; } p.vx = p.vy = 0; p.cd = 0; p.sp = null; p.ip = null; }
    const h = st.ps.find(p => p.team === t && p.i === 3); h.x = 50 - dirOf(t) * 1.2; h.y = 25; h.ca = t === 0 ? 0 : Math.PI; st.h = h; st.b = null; st.ch = .3; st.recv = st.then = st.kicker = st.route = st.chaser = null; st.act = { k: 'kalkan' }; st.nd = st.t + 20; st.need = false; }
  function clone(st) { const ps = st.ps.map(p => ({ ...p })), m = q => q ? ps[q.id] : null; return { ...st, ps, h: m(st.h), recv: m(st.recv), kicker: m(st.kicker), chaser: m(st.chaser), then: st.then ? { ...st.then, r: m(st.then.r) } : null, b: st.b ? { ...st.b } : null, route: st.route ? st.route.slice() : null, score: st.score.slice(), S: null, rollout: true }; }
  function core(st) { const B = C().Q.body, h = st.h; return h ? { x: h.x + Math.cos(h.ca) * B, y: h.y + Math.sin(h.ca) * B } : { x: st.b.x, y: st.b.y }; }
  const ballStep = (b, ps) => C().stepBall(b, ps);
  const goalIn = b => b.done && String(b.done).startsWith('kuyu') ? (b.done === 'kuyu-sag' ? 0 : 1) : -1;
  function kickV(o, T, v, ps, ch, team) { const K = C(), aim = T.my != null ? { x: T.x, y: 2 * T.my - T.y } : T, dx = aim.x - o.x, dy = aim.y - o.y, L = Math.hypot(dx, dy) || 1, ux = dx / L, uy = dy / L;
    const arr = v0 => { const b = K.makeBall({ x: o.x, y: o.y, vx: ux * v0, vy: uy * v0, team, ch }); let best = 1e9, sp = 0; for (let i = 0; i < 220 && !b.done; i++) { const vx = b.vx, vy = b.vy; ballStep(b, ps); if (goalIn(b) >= 0) return Math.hypot(vx, vy); const d = Math.hypot(b.x - T.x, b.y - T.y); if (d < best) { best = d; sp = Math.hypot(b.vx, b.vy); } else if (d > best + 1) break; } return best < 1.3 ? sp : (b.done ? 0 : 9); };
    let lo = .1, hi = 2.8; for (let i = 0; i < 9; i++) { const m = (lo + hi) / 2; if (arr(m) < v) lo = m; else hi = m; } return { vx: ux * hi, vy: uy * hi, L }; }
  const acc = (sd, L) => 1 / (1 + (sd * L / 1.8) ** 2);
  function aimErr(st, vel, sd, rnd) { if (rnd) { const an = (rnd() + rnd() - 1) * sd * 1.7, cs = Math.cos(an), sn = Math.sin(an); return { vx: vel.vx * cs - vel.vy * sn, vy: vel.vx * sn + vel.vy * cs }; } st.keep *= acc(sd, vel.L); return vel; }
  function nearestOpp(st, p) { let best = null, bd = 1e9; for (const q of st.ps) if (q.team !== p.team && !isK(q)) { const d = dist(q, p); if (d < bd) { bd = d; best = q; } } return best; }
  const others = (st, p) => Object.assign(st.ps.filter(q => q !== p), { team: p.team });
  function loose(st, vel, team) { const o = core(st); st.b = C().makeBall({ x: o.x, y: o.y, vx: vel.vx, vy: vel.vy, team, ch: st.ch }); st.b.t = 9; }
  function start(st, A, rnd) {
    st.act = A; const h = st.h, g = dirOf(h.team), G = goalOf(h.team), nd = nearestOpp(st, h), cx = v => cl(v, 1, W - 1), cy = v => cl(v, 1, H - 1), M = C().MOUTH;
    if (A.k === 'kalkan' || A.k === 'sür') return;
    let T, v, sk;
    if (A.k === 'fiske') { T = { x: cx(nd.x + g * 3.5), y: cy(nd.y + A.side * 2.6) }; v = .3; sk = h.a.surme; st.route = [{ x: cx(nd.x + g * .3), y: cy(nd.y - A.side * 2.8) }]; st.recv = null; }
    else if (A.k === 'kenar') { const top = A.wall === 'üst', dw = top ? nd.y : H - nd.y; T = { x: cx(nd.x + g * 4), y: top ? Math.max(1.2, dw * .35) : H - Math.max(1.2, dw * .35), my: top ? 0 : H }; v = .35; sk = h.a.surme; st.route = [{ x: cx(nd.x + g * .3), y: cy(nd.y + (top ? 2.8 : -2.8)) }]; st.recv = null; }
    else if (A.k === 'gönder') { T = { x: G.x, y: G.y + A.side * M * .6 }; v = 1.1 * C().chgMul(st.ch); sk = h.a.aktarim; st.route = null; st.recv = null; if (st.S) st.S.shot[h.team]++; st.shotTek = false; }
    else { const q = A.q; T = A.T ? { ...A.T } : A.k === 'önüne' ? { x: cx(q.x + g * 3.5), y: q.y } : { x: q.x, y: q.y }; if (A.k === 'kenardan') T.my = A.wall === 'üst' ? 0 : H; v = A.v || (A.k === 'önüne' ? .3 : A.then ? .6 : .5); sk = h.a.aktarim; st.route = null; st.recv = q; st.passT = { x: T.x, y: T.y }; st.then = A.then ? { ...A.then } : null;
      if (st.S) { st.S.pass[h.team]++; if (A.k === 'kenardan') st.S.bank[h.team]++; } st.pend = { team: h.team, bank: A.k === 'kenardan' };
      if (st.then && st.then.r === h) { const rp = { x: cx(nd.x + g * 2.6), y: cy(nd.y + (h.y < nd.y ? -2.4 : 2.4)) }; st.route = [rp]; st.then.T = rp; } }
    const vel = aimErr(st, kickV(core(st), T, v, others(st, h), st.ch, h.team), .01 + (20 - sk) * .006, rnd); loose(st, vel, h.team);
    st.kicker = h; st.h = null; st.kickT = st.t; st.looseT = st.t;
  }
  function redirect(st, p, rnd) {
    const th = st.then, b = st.b, vin = Math.hypot(b.vx, b.vy), ak = p.a.aktarim ?? 10, G = goalOf(p.team), M = C().MOUTH, T = th.k === 'tek' ? (th.T || { x: cl(th.r.x + dirOf(p.team) * 1.5, 1, W - 1), y: th.r.y }) : { x: G.x, y: G.y + th.side * M * .6 };
    const tin = Math.atan2(b.vy, b.vx), tout = Math.atan2(T.y - p.y, T.x - p.x), turn = Math.abs(wrap(tout - tin)), sd = .02 + (20 - ak) * .006 + turn * .05 + Math.max(0, vin - .6) * .06, miss = cl(.04 + turn * .08 + Math.max(0, vin - .9) * .2 - (ak - 10) * .01, .02, .6);
    st.then = null; st.kicker = p; st.kickT = st.t; st.looseT = st.t;
    if (rnd ? rnd() < miss : false) { const an = tin + Math.PI + (rnd() - .5) * 2.4, s2 = vin * .45; b.vx = Math.cos(an) * s2; b.vy = Math.sin(an) * s2; st.recv = null; return; }
    if (!rnd) st.keep *= 1 - miss;
    const vel = kickV({ x: b.x, y: b.y }, T, th.k === 'tek' ? .5 : 1.2 * C().chgMul(b.ch), others(st, p), b.ch, p.team); vel.vx += b.vx * .2; vel.vy += b.vy * .2; vel.L = dist(p, T);
    const v2 = aimErr(st, vel, sd, rnd); b.vx = v2.vx; b.vy = v2.vy; b.done = null;
    if (th.k === 'tek') { st.recv = th.r; st.passT = T; if (st.S) st.S.tek[p.team]++; } else { st.recv = null; if (st.S) { st.S.shot[p.team]++; st.S.tekShot[p.team]++; } st.shotTek = true; }
  }
  // Savunan takım: sistem (kim, nasıl basar), pres sayısı ve çizgisi, blok yüksekliği; Bekçi ağızda.
  function defTargets(st, dt) {
    const ps = st.ps, tac = st.tac[dt], D = ps.filter(p => p.team === dt && !isK(p)), A = ps.filter(p => p.team !== dt && !isK(p)), h = st.h && st.h.team !== dt ? st.h : null, cp = st.h || st.b, OG = ownGoal(dt), out = new Map();
    const byDist = D.slice().sort((u, v) => dist(u, cp) - dist(v, cp)), goalSide = (m, k) => { const gx = OG.x - m.x, gy = OG.y - m.y, gl = Math.hypot(gx, gy) || 1; return { x: m.x + gx / gl * k, y: m.y + gy / gl * k + (cp.y - m.y) * .1 }; };
    const pressPt = (d, mode) => { const base = Math.atan2(d.y - h.y, d.x - h.x), side = wrap(h.ca - base) > 0 ? 1 : -1; if (mode === 'kes') { const gx = OG.x - h.x, gy = OG.y - h.y, gl = Math.hypot(gx, gy) || 1; return { x: h.x + gx / gl * 2.2, y: h.y + gy / gl * 2.2 }; } if (mode === 'iç') { if (dist(d, h) < 2.4) return core(st); const s = h.y < 25 ? 1 : -1; return { x: h.x - dirOf(dt) * 1.2, y: h.y + s * 1.6 }; } const ss = mode === 'yan2' ? -side : side; return { x: h.x + Math.cos(base + ss * 1.25) * 1.9, y: h.y + Math.sin(base + ss * 1.25) * 1.9 }; };
    let pr = [];
    if (!st.h) { if (st.t - st.looseT >= react(byDist[0])) pr = byDist.slice(0, 2).map(d => [d, 'top']); }
    else if (h) { const inZone = depth(dt, cp.x) < PRESS[tac.blok] || dist(byDist[0], h) < 4, n = inZone ? tac.pres : 0;
      if (tac.sistem === 'Adam adama' && n > 0) { const m = D.find(d => d.man === h.id) || byDist[0]; pr = [[m, 'yan']]; byDist.filter(d => d !== m).slice(0, n - 1).forEach((d, i) => pr.push([d, i ? 'yan2' : 'kes'])); }
      else byDist.slice(0, n).forEach((d, i) => pr.push([d, i === 0 ? (tac.sistem === 'Kenara sıkıştır' ? 'iç' : 'yan') : i === 1 ? 'kes' : 'yan2'])); }
    const pset = new Set(pr.map(x => x[0]));
    for (const [d, mo] of pr) out.set(d.id, { ...(mo === 'top' ? { x: st.b.x, y: st.b.y } : pressPt(d, mo)), press: true, job: mo === 'top' ? 'Çekirdeğe' : 'pres' });
    let fr = D.filter(d => !pset.has(d));
    const bd = depth(dt, cp.x), back = cl(Math.min(BLOK[tac.blok], bd - 5), 7, 70), ly = (cp.y - 25) * .35;
    if (tac.sistem === 'Kenara sıkıştır' && h && fr.length && pr.length) { const wp = { x: cl(h.x - dirOf(dt) * 5, 2, W - 2), y: h.y < 25 ? 3 : H - 3 }, w = fr.slice().sort((u, v) => dist(u, wp) - dist(v, wp))[0]; out.set(w.id, { ...wp, job: 'tuzak' }); fr = fr.filter(d => d !== w); }
    if (tac.sistem === 'Adam adama') { for (const d of fr) { const m = ps[d.man]; out.set(d.id, { ...goalSide(m, 1.6), job: '' }); } }
    else { const used = new Set(); for (const d of fr) { const an = { x: fromDepth(dt, back + d.slot.d * 22), y: cl(25 + d.slot.l * 13 + ly, 2, H - 2) }; let best = null, bq = 1e9; for (const a of A) { if (a === st.h || used.has(a)) continue; const q = dist(a, an); if (q < 8 && q < bq) { bq = q; best = a; } } if (best) { used.add(best); const g = goalSide(best, 1.8); out.set(d.id, { x: (g.x + an.x) / 2 * .4 + g.x * .6, y: (g.y + an.y) / 2 * .4 + g.y * .6, job: '' }); } else out.set(d.id, { ...an, job: '' }); } }
    const k = ps.find(p => p.team === dt && isK(p)), gy = cl(25 + (cp.y - 25) * .35, 25 - C().MOUTH + .6, 25 + C().MOUTH - .6); out.set(k.id, { x: fromDepth(dt, 1.6 + Math.min(3, Math.max(0, 30 - bd) * .08)), y: gy, job: '' });
    return out;
  }
  // Hücum eden takım: dizilişteki yerinin çevresinde en boş, pas hattı açık, Kuyu'ya yakın nokta. Arkada kalanlar güvence; Kontra'da öndekiler savunmanın arkasına koşar.
  function attTargets(st, at) {
    const ps = st.ps, tac = st.tac[at], cp = st.h || st.b, out = new Map(), F = ps.filter(p => p.team === at && !isK(p) && p !== st.h), O = ps.filter(p => p.team !== at && !isK(p)), G = goalOf(at), bd = depth(at, cp.x), back = cl(bd - 30, 8, 58);
    const kon = tac.kazaninca === 'Kontra' && st.winTeam === at && st.t - st.winT < 180, guards = new Set(F.slice().sort((u, v) => u.slot.d - v.slot.d).slice(0, tac.arkada)), lastO = Math.max(...O.map(o => depth(at, o.x)));
    const runners = kon ? new Set(F.filter(p => !guards.has(p)).sort((u, v) => v.slot.d - u.slot.d).slice(0, 3)) : new Set();
    for (const p of F) {
      if (guards.has(p)) { const opF = O.slice().sort((u, v) => depth(at, u.x) - depth(at, v.x))[0]; out.set(p.id, { x: fromDepth(at, cl(Math.min(bd - 12, depth(at, opF.x) - 3), 6, 60)), y: cl(25 + p.slot.l * 10, 3, H - 3), job: 'güvence' }); continue; }
      const anD = runners.has(p) ? Math.min(96, lastO + 4) : back + p.slot.d * 44, an = { x: fromDepth(at, cl(anD, 4, 96)), y: cl(25 + p.slot.l * GEN[tac.genislik] / 2, 2, H - 2) };
      if (p.sp && (st.t + p.id * 3) % 15 !== 0) { out.set(p.id, p.sp); continue; }
      let best = an, bv = -1e9; for (let k = -1; k < 8; k++) { const r = k < 4 ? 3.5 : 7, pt = k < 0 ? an : { x: cl(an.x + Math.cos(k * 1.57 + (k > 3 ? .785 : 0)) * r, 2, W - 2), y: cl(an.y + Math.sin(k * 1.57 + (k > 3 ? .785 : 0)) * r, 2, H - 2) };
        let fd = 7, ln = 3; for (const d of O) { fd = Math.min(fd, dist(d, pt)); if (st.h) { const sx = pt.x - cp.x, sy = pt.y - cp.y, L2 = sx * sx + sy * sy || 1, tt = cl(((d.x - cp.x) * sx + (d.y - cp.y) * sy) / L2, 0, 1); ln = Math.min(ln, Math.hypot(cp.x + sx * tt - d.x, cp.y + sy * tt - d.y)); } }
        let crowd = 0; for (const q of F) if (q !== p && q.sp) crowd += Math.exp(-((q.sp.x - pt.x) ** 2 + (q.sp.y - pt.y) ** 2) / 30);
        const v = fd * .25 + ln * .35 - dist(pt, G) * .04 - dist(pt, an) * .08 - crowd * .8 + (runners.has(p) ? -Math.max(0, lastO - depth(at, pt.x)) * .2 : 0); if (v > bv) { bv = v; best = pt; } }
      p.sp = { ...best, job: runners.has(p) ? 'kontra koşusu' : '' }; out.set(p.id, p.sp); }
    const k = ps.find(p => p.team === at && isK(p)); if (k !== st.h) out.set(k.id, { x: fromDepth(at, 3), y: 25, job: '' });
    return out;
  }
  function interceptPt(st, p) { const b = { ...st.b }, v = spd(p) * 1.25; let last = { x: b.x, y: b.y, i: 99 }; for (let i = 1; i <= 80 && !b.done; i++) { ballStep(b, st.ps); last = { x: b.x, y: b.y, i }; if (i % 2 === 0 && Math.max(0, dist(p, b) - C().Q.reach * .7) <= v * i) return last; } return last; }
  function moveTo(p, g, v, k) { const gx = g.x - p.x, gy = g.y - p.y, gl = Math.hypot(gx, gy); if (gl > .3) { p.vx += (gx / gl * Math.min(v, gl) - p.vx) * k; p.vy += (gy / gl * Math.min(v, gl) - p.vy) * k; } else { p.vx *= .7; p.vy *= .7; } }
  function step(st, rnd) {
    const K = C(), T = K.Turn, Q = K.Q, t = st.t, ps = st.ps, h = st.h;
    if (h) { const nd = nearestOpp(st, h), g0 = h.team === 0 ? 0 : Math.PI, la = 2 + (h.a.okuma ?? 10) * .4, aw = Math.atan2(h.y - (nd.y + nd.vy * la), h.x - (nd.x + nd.vx * la)), sC = spd(h) * ((h.slowT || 0) > t ? .55 : 1) * (dist(nd, h) < 1.9 ? .8 : 1), A = st.act;
      let vx, vy, ta; if (A.k === 'kalkan' || isK(h)) { vx = Math.cos(aw) * sC * .3; vy = Math.sin(aw) * sC * .3; ta = aw; } else { const dd = A.dir + g0; vx = Math.cos(dd) * sC * .85; vy = Math.sin(dd) * sC * .85; ta = dd + cl(wrap(aw - dd), -1, 1) * .7; }
      h.vx += (vx - h.vx) * .25; h.vy += (vy - h.vy) * .25;
      const w = T.omega(h), d = wrap(ta - h.ca); if (Math.abs(d) > .02) { const pa = wrap(aw + Math.PI), m1 = wrap(h.ca + d / 2), m2 = wrap(h.ca + (d - Math.sign(d) * 2 * Math.PI) / 2), dir = Math.abs(wrap(m1 - pa)) < .9 && Math.abs(wrap(m2 - pa)) > Math.abs(wrap(m1 - pa)) ? -Math.sign(d) : Math.sign(d); h.ca = wrap(h.ca + dir * (dir === Math.sign(d) ? Math.min(w, Math.abs(d)) : w)); }
      let pr = 0; for (const o of ps) if (o.team !== h.team) pr += K.infl(o, h.x, h.y); const still = cl(1 - Math.hypot(h.vx, h.vy) / .2, 0, 1); st.ch = Math.min(1, st.ch + .0035 * (.4 + .6 * still) * (.25 + .75 * Math.exp(-pr * 2)) * (1 - st.ch * .5) * (1 + ((h.a.tutus ?? 10) - 10) * .05)); }
    const at = h ? h.team : st.b.team, dt = 1 - at;
    if (!h && st.b && !st.recv && (t % 6 === 0 || !st.chaser)) { let best = null, bt = 1e9; for (const p of ps) { if (p.team !== at || isK(p) || (p === st.kicker && st.route && st.route.length)) continue; const ip = interceptPt(st, p); if (ip.i < bt) { bt = ip.i; best = p; } } st.chaser = best; }
    const dT = defTargets(st, dt), aT = attTargets(st, at);
    for (const p of ps) { if (p === h) continue; let g, v = spd(p), k = .3;
      if (p.team === dt) { const o = dT.get(p.id); g = o; p.job = o.job; v *= o.press ? 1.2 : 1.1; k = .35; }
      else if (!h && p === st.recv) { if (!p.ip || (t + p.id) % 6 === 0) p.ip = interceptPt(st, p); g = p.ip; v *= 1.25; p.job = 'pası alıyor'; }
      else if (!h && p === st.kicker && st.route) { if (st.route.length && dist(p, st.route[0]) < 1) st.route.shift(); g = st.route.length ? st.route[0] : st.b; v *= 1.25; }
      else if (!h && !st.recv && p === st.chaser) { if (!p.ip || (t + p.id) % 6 === 0) p.ip = interceptPt(st, p); g = p.ip; v *= 1.25; }
      else if (!h && st.then && st.then.k === 'tek' && p === st.then.r) { g = { x: cl(p.x + dirOf(at) * 2, 1, W - 1), y: p.y }; v *= 1.2; }
      else { g = aT.get(p.id) || p; p.job = g.job || ''; if (g.job === 'kontra koşusu') v *= 1.25; }
      moveTo(p, g, v, k); }
    for (const p of ps) { p.x = cl(p.x + p.vx, .9, W - .9); p.y = cl(p.y + p.vy, .9, H - .9); if (!isK(p)) for (const gx of [0, W]) { const dd = Math.hypot(p.x - gx, p.y - 25), R0 = C().KR + .8; if (dd < R0) { p.x = gx + (p.x - gx) / dd * R0; p.y = 25 + (p.y - 25) / dd * R0; } } }
    for (let i = 0; i < ps.length; i++) for (let j = i + 1; j < ps.length; j++) { const a = ps[i], b = ps[j], dx = a.x - b.x; if (dx > 1.6 || dx < -1.6) continue; const dd = Math.hypot(dx, a.y - b.y) || 1; if (dd < 1.6) { const push = (1.6 - dd) / 2, ux = dx / dd, uy = (a.y - b.y) / dd; a.x += ux * push; a.y += uy * push; b.x -= ux * push; b.y -= uy * push; } }
    if (h) { const B = Q.body, r = .4, kx = h.x + Math.cos(h.ca) * B, ky = h.y + Math.sin(h.ca) * B; if (ky < r) h.y += r - ky; if (ky > H - r) h.y -= ky - (H - r); if (kx < r) h.x += r - kx; if (kx > W - r) h.x -= kx - (W - r); }
    if (h) { const o = core(st); for (const p of ps) { if (p.team === h.team || t < (p.cd || 0)) continue; if (dist(o, p) < Q.reach && !T.shielded(h, p, o.x, o.y)) { p.cd = t + 12; h.slowT = t + 18; h.vx *= .45; h.vy *= .45; const P = cl(.26 + ((p.a.kesme ?? 10) - (h.a.surme ?? 10)) * .035 + Math.hypot(h.vx, h.vy) * .4, .05, .9); if (rnd ? rnd() < P : false) { take(st, p); return end(st); } if (!rnd) st.keep *= 1 - P; } } }
    else { const b = st.b, ox = b.x, oy = b.y; if (!b.done || b.done === 'durdu') { if (!b.done) ballStep(b, ps); } const gi = goalIn(b); if (gi >= 0) { goal(st, gi); return end(st); }
      const sx = b.x - ox, sy = b.y - oy, sl = sx * sx + sy * sy, dd = q => { const tt = sl > 0 ? cl(((q.x - ox) * sx + (q.y - oy) * sy) / sl, 0, 1) : 1; return Math.hypot(ox + sx * tt - q.x, oy + sy * tt - q.y); };
      const reach = p => isK(p) ? Q.bekBody + Math.min(Q.bekDive, Math.max(0, (t - st.kickT) - (Q.bekReact - (p.a.okuma ?? 10) * Q.bekReactOk)) * Q.bekDiveV) : p.team === b.team ? Q.reach : .9 + Math.min(Q.reach - .9, Math.max(0, (t - st.kickT) - (6 - (p.a.okuma ?? 10) * .25)) * .12);
      const cand = ps.filter(p => t >= (p.cd || 0) && !(p === st.kicker && t - st.kickT < 5) && dd(p) < reach(p)).sort((u, v) => dd(u) - dd(v));
      for (const p of cand) { p.cd = t + 6; const mine = p.team === b.team;
        if (mine && p === st.recv && st.then) { redirect(st, p, rnd); break; }
        const P = K.ctrlP(b, p, ps);
        if (rnd) { if (rnd() < P) { if (mine) gain(st, p); else take(st, p); break; } const an = Math.atan2(-b.vy, -b.vx) + (rnd() - .5) * 2, s2 = Math.hypot(b.vx, b.vy) * .45; b.vx = Math.cos(an) * s2; b.vy = Math.sin(an) * s2; b.done = null; st.recv = st.then = st.route = st.chaser = null; }
        else { if (mine) { st.keep *= P + (1 - P) * .5; gain(st, p); break; } st.keep *= 1 - P; b.vx = -b.vx * .45; b.vy = -b.vy * .45; b.done = null; st.recv = st.then = st.route = st.chaser = null; break; } } }
    return end(st);
  }
  function gain(st, p) { const keepCh = st.b ? st.b.ch : st.ch, o = st.b || core(st); if (st.S && st.pend && st.pend.team === p.team && p !== st.kicker) { st.S.passOk[p.team]++; if (st.pend.bank) st.S.bankOk[p.team]++; } st.pend = null; st.h = p; p.ca = Math.atan2(o.y - p.y, o.x - p.x); st.b = null; st.recv = st.route = st.kicker = st.then = st.chaser = null; st.act = { k: 'sür', dir: 0 }; st.need = true; st.ch = Math.max(.15, keepCh * .7); }
  function take(st, p) { const o = st.b || core(st); st.pend = null; if (st.rollout) { st.res = 'kayıp'; st.lossAt = { x: o.x, y: o.y }; return; } st.h = p; p.ca = Math.atan2(o.y - p.y, o.x - p.x); st.b = null; st.recv = st.route = st.kicker = st.then = st.chaser = null; st.act = { k: 'sür', dir: 0 }; st.need = true; st.ch = .3; st.winT = st.t; st.winTeam = p.team; if (st.S) st.S.win[p.team]++; }
  function goal(st, team) { if (st.rollout) { st.res = 'sayı' + team; return; } st.score[team]++; if (st.S) { if (st.shotTek) st.S.gTek[team]++; if (st.winTeam === team && st.t - st.winT < 360) st.S.gKontra[team]++; else st.S.gSabit[team]++; } st.events && st.events.push({ t: st.t, team }); kickoff(st, 1 - team); }
  function end(st) { if (st.S && st.h) st.S.poss[st.h.team]++; st.t++; return st; }
  function laneMargin(st, A) { const o = core(st), T = { ...A.T, my: A.wall === 'üst' ? 0 : H }, team = st.h.team, fps = others(st, st.h), vel = kickV(o, T, A.v, fps, st.ch, team), b = C().makeBall({ x: o.x, y: o.y, vx: vel.vx, vy: vel.vy, team, ch: st.ch }); let m = 9;
    for (let t = 1; t < 200 && !b.done; t++) { ballStep(b, fps); if (Math.hypot(b.x - T.x, b.y - T.y) < 1.2) break; if (t % 2) continue; for (const d of st.ps) { if (d.team === team) continue; const rt = Math.max(0, t - react(d)), R = (isK(d) ? 1.4 : .9 + Math.min(.7, Math.max(0, t - 6 + (d.a.okuma ?? 10) * .25) * .12)) + rt * spd(d) * 1.2; m = Math.min(m, Math.hypot(b.x - d.x, b.y - d.y) - R); } }
    return m; }
  function cands(st) {
    const h = st.h, team = h.team, g = dirOf(team), G = goalOf(team), nd = nearestOpp(st, h), tac = st.tac[team], out = [{ k: 'kalkan' }];
    if (!isK(h)) { for (const d of [0, .6, -.6, 1.2, -1.2, Math.PI]) out.push({ k: 'sür', dir: d });
      if (dist(nd, h) < 7 && (nd.x - h.x) * g > -1) { out.push({ k: 'fiske', side: 1 }, { k: 'fiske', side: -1 }); if (h.y < 6 && nd.y < 7) out.push({ k: 'kenar', wall: 'üst' }); if (h.y > H - 6 && nd.y > H - 7) out.push({ k: 'kenar', wall: 'alt' }); }
      if (dist(h, G) < 28) out.push({ k: 'gönder', side: 1 }, { k: 'gönder', side: -1 }); }
    const mates = st.ps.filter(q => q.team === team && q !== h && !isK(q) && dist(q, h) < 36), P = [];
    for (const q of mates) { const lead = { x: cl(q.x + g * 3.5, 1, W - 1), y: q.y }; for (const A of [{ k: 'pas', q, T: { x: q.x, y: q.y }, v: .5, w: false }, { k: 'önüne', q, T: lead, v: .3 }]) { A.m = laneMargin(st, { ...A, wall: null, T: { ...A.T } }); P.push(A); }
      if (dist(h, q) > 5) for (const [wall, wy] of [['üst', 0], ['alt', H]]) if (Math.abs(h.y - wy) < 20 && Math.abs(q.y - wy) < 20) for (const v of [.5, .9]) { const A = { k: 'kenardan', q, wall, T: { x: q.x, y: q.y }, v }; A.m = laneMargin(st, A); P.push(A); } }
    P.sort((a, b) => b.m - a.m); const keepP = P.filter(A => A.m > -.6).slice(0, 7); out.push(...keepP);
    if (tac.tek !== 'kapalı') { const top = [...new Set(keepP.filter(A => A.k === 'pas').map(A => A.q))].slice(0, 2); for (const q of top) { const rs = mates.concat([h]).filter(r => r !== q).sort((u, v) => dist(u, G) - dist(v, G)).slice(0, 2); for (const r of rs) out.push({ k: 'pas', q, T: { x: q.x, y: q.y }, then: { k: 'tek', r } }); if (dist(q, G) < 22) out.push({ k: 'pas', q, T: { x: q.x, y: q.y }, then: { k: 'şut', side: q.y < 25 ? 1 : -1 } }); } }
    return out;
  }
  const thr = (o, team) => Math.exp(-dist(o, goalOf(team)) / 16);
  function value(st, A, rnd) {
    const team = st.h.team, s = clone(st), ok = s.h.a.okuma ?? 10, err = Math.max(0, 16 - ok) * .15, tac = st.tac[team]; for (const p of s.ps) if (p.team !== team) { p.x += (rnd() - .5) * err; p.y += (rnd() - .5) * err; }
    s.keep = 1; const mp = q => q ? s.ps[q.id] : q; const B = { ...A, q: mp(A.q), then: A.then ? { ...A.then, r: mp(A.then.r) } : null }; const c0 = core(s), t0 = thr(c0, team), d0 = depth(team, c0.x); start(s, B, null);
    for (let i = 0; i < 50 && !s.res; i++) { if (s.h && s.need) { s.need = false; s.act = { k: 'sür', dir: 0 }; } step(s, null); }
    const lossW = .35 * (1.5 - tac.risk);
    if (s.res === 'sayı' + team) return 3 * s.keep; if (s.res && s.res.startsWith('sayı')) return -1;
    if (s.res === 'kayıp') return -lossW * (1 + 2 * thr(s.lossAt, 1 - team));
    let own; if (s.h) own = s.h.team === team ? 1 : 0; else { let da = 1e9, dd = 1e9; for (const p of s.ps) { if (isK(p)) continue; const d = dist(p, s.b); if (p.team === team) da = Math.min(da, d); else dd = Math.min(dd, d); } own = 1 / (1 + Math.exp((da - dd) / 1.5)); }
    let free = 6; if (s.h) for (const p of s.ps) if (p.team !== team && !isK(p)) free = Math.min(free, dist(p, s.h));
    const o = core(s); return s.keep * own * (1 + 1.5 * (thr(o, team) - t0) + .015 * (depth(team, o.x) - d0) + .03 * free) /* ilerleme: Kuyu'dan uzakta da ileri gitmek değerlidir */ - (1 - s.keep * own) * lossW * (1 + 2 * thr(o, 1 - team));
  }
  function decide(st, rnd) { const ok = st.h.a.okuma ?? 10, C2 = cands(st).map(A => ({ A, v: value(st, A, rnd) + (rnd() - .5) * Math.max(0, 16 - ok) * .03 })); C2.sort((a, b) => b.v - a.v); return C2[0].A; }
  function tick(st, rnd) { if (st.h && (st.t >= st.nd || st.need)) { st.need = false; const A = decide(st, rnd); start(st, A, rnd); const tac = st.tac[st.h ? st.h.team : st.b.team], kon = tac.kazaninca === 'Kontra' && st.winTeam === (st.h ? st.h.team : st.b.team) && st.t - st.winT < 180; st.nd = st.t + Math.round(8 + (1 - (kon ? Math.max(.8, tac.tempo) : tac.tempo)) * 10); } step(st, rnd); }
  function play(cfg, rnd) { const st = create(cfg); st.events = []; while (st.t < st.len) tick(st, rnd); return { score: st.score, S: st.S, t: st.t }; }
  window.AlanV3e = { TAC0, create, tick, step, play, core, goalOf };
})();
