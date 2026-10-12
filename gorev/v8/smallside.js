(function () {
  // Küçük saha · 4'e 4 + Bekçi, dört yanı kapalı alan, sağ kenarda Kuyu. Bire birin mekanikleri + pas, önüne pas, kenardan pas, gönderme, tek dokunuş (tekte pas, tekte gönderme), savunma sistemleri.
  // Taşıyıcı seçeneklerini kafasında oynatır; kafadaki oynama gerçek oynamayla aynı adım fonksiyonunu kullanır (rnd null = beklenen değer).
  const C = () => window.AlanCore, AR = { x0: 30, x1: 70, y0: 0, y1: 24 }, G = { x: 70, y: 12, h: 2.75 }, LINE = null;
  const cl = (v, a, b) => v < a ? a : v > b ? b : v, wrap = a => { while (a > Math.PI) a -= 2 * Math.PI; while (a < -Math.PI) a += 2 * Math.PI; return a; }, dist = (a, b) => Math.hypot(a.x - b.x, a.y - b.y);
  const SYS = ['Alan', 'Adam adama', 'Çift pres', 'Kenara sıkıştır'];
  const spd = p => .17 + (p.a.hiz ?? 10) * .006, react = p => 14 - (p.a.okuma ?? 10) * .5, isK = p => p.role === 'Bekçi';
  function create(cfg) {
    const aq = cfg.atk ?? 10, dq = cfg.def ?? 10, wide = cfg.shape !== 'Dar';
    const A = [[34, 12], [43, wide ? 4 : 8], [43, wide ? 20 : 16], [52, 12]], D = [[45, 9], [45, 15], [55, 8], [55, 16]], ps = [];
    A.forEach(([x, y], i) => ps.push({ id: i, team: 0, x, y, vx: 0, vy: 0, home: { x, y }, R: 8, D: 1, role: 'Orta', ca: 0, cd: 0, a: { surme: aq, okuma: aq, tutus: aq, aktarim: aq, kesme: 10, hiz: cfg.atkHiz ?? 10 } }));
    D.forEach(([x, y], i) => ps.push({ id: 4 + i, team: 1, x, y, vx: 0, vy: 0, home: { x, y }, R: 8, D: 1, role: 'Orta', cd: 0, a: { kesme: dq, okuma: dq, tutus: 10, surme: 10, hiz: cfg.defHiz ?? 10 } }));
    ps.push({ id: 8, team: 1, x: G.x - 1.4, y: G.y, vx: 0, vy: 0, home: { x: G.x - 1.4, y: G.y }, R: 7, D: 1.2, role: 'Bekçi', cd: 0, a: { kesme: 10, okuma: dq, tutus: dq, surme: 10, hiz: 10 } });
    const SC = { 'Ver-kaç': { a: [[48, 5], [50.5, 9.5], [44, 16], [60, 15]], d: [[51.2, 6], [53, 11], [60, 11.5], [57, 19]] }, 'Üçüncü adam': { a: [[45, 12], [49.5, 16.5], [44, 5], [54, 8]], d: [[47.8, 12], [51.3, 17], [58, 10], [58, 16]] }, 'Tekte gönderme': { a: [[52, 19], [46, 12], [56, 5], [60.5, 11]], d: [[54.5, 18.5], [50, 12], [58, 8], [62.3, 12]] } }[cfg.scn];
    if (SC) { SC.a.forEach(([x, y], i) => { Object.assign(ps[i], { x, y, home: { x, y } }); }); SC.d.forEach(([x, y], i) => { Object.assign(ps[4 + i], { x, y, home: { x, y } }); }); }
    const used = new Set(); ps.filter(p => p.team === 1 && !isK(p)).map(d => ps.filter(a => a.team === 0).map(a => [d, a, dist(d, a)])).flat().sort((u, v) => u[2] - v[2]).forEach(([d, a]) => { if (d.man == null && !used.has(a.id)) { d.man = a.id; used.add(a.id); } });
    return { cfg, ps, h: ps[0], ch: .3, b: null, act: { k: 'kalkan' }, t: 0, keep: 1, res: null, kickT: -99, looseT: -99, recv: null, passT: null, then: null, kicker: null, route: null, after: false, how: null, nPass: 0, nTek: 0, nShot: 0, nTekShot: 0, shotTek: false };
  }
  function clone(st) { const ps = st.ps.map(p => ({ ...p })), m = q => q ? ps[q.id] : null; return { ...st, ps, h: m(st.h), recv: m(st.recv), kicker: m(st.kicker), chaser: m(st.chaser), then: st.then ? { ...st.then, r: m(st.then.r), T: st.then.T } : null, b: st.b ? { ...st.b } : null, route: st.route ? st.route.slice() : null }; }
  function core(st) { const B = C().Q.body, h = st.h; return h ? { x: h.x + Math.cos(h.ca) * B, y: h.y + Math.sin(h.ca) * B } : { x: st.b.x, y: st.b.y }; }
  function walls(b) { const r = .4, e = .85; /* bilardo gibi: kenar Çekirdeğin hızının ancak %15'ini alır */ if (b.x < AR.x0 + r) { b.x = AR.x0 + r; b.vx = Math.abs(b.vx) * e; b.vy *= .9; } if (b.x > AR.x1 - r) { if (Math.abs(b.y - G.y) < G.h - .3 && b.vx > 0) { b.goal = true; b.done = 'kuyu'; return; } b.x = AR.x1 - r; b.vx = -Math.abs(b.vx) * e; b.vy *= .9; } if (b.y < AR.y0 + r) { b.y = AR.y0 + r; b.vy = Math.abs(b.vy) * e; b.vx *= .9; } if (b.y > AR.y1 - r) { b.y = AR.y1 - r; b.vy = -Math.abs(b.vy) * e; b.vx *= .9; } }
  function ballStep(b, ps) { if (b.goal) return; C().stepBall(b, ps); walls(b); }
  // Vuruş: hedefe (kenardan sekecekse kenara göre aynalanmış noktaya) nişan alınır; hız, Çekirdek hedefe istenen hızla varacak şekilde çözülür.
  function kickV(o, T, v, ps, ch) { ps = ps || []; ch = ch ?? .3; const K = C(), aim = T.my != null ? { x: T.x, y: 2 * T.my - T.y } : T, dx = aim.x - o.x, dy = aim.y - o.y, L = Math.hypot(dx, dy) || 1, ux = dx / L, uy = dy / L;
    const arr = v0 => { const b = K.makeBall({ x: o.x, y: o.y, vx: ux * v0, vy: uy * v0, team: ps.team ?? 0, ch }); let best = 1e9, sp = 0; for (let i = 0; i < 220 && !b.done; i++) { ballStep(b, ps); if (b.goal) return Math.hypot(b.vx, b.vy); const d = Math.hypot(b.x - T.x, b.y - T.y); if (d < best) { best = d; sp = Math.hypot(b.vx, b.vy); } else if (d > best + 1) break; } return best < 1.3 ? sp : (b.done ? 0 : 9); };
    let lo = .1, hi = 2.8; for (let i = 0; i < 9; i++) { const m = (lo + hi) / 2; if (arr(m) < v) lo = m; else hi = m; } return { vx: ux * hi, vy: uy * hi, L }; }
  // İsabet: nişan açısı hatası (Aktarım/Sürme), mesafeyle büyüyen sapma. Kafadaki oynama sapmayı yaşamaz ama hesaba katar.
  const acc = (sd, L) => 1 / (1 + (sd * L / 1.8) ** 2);
  function aimErr(st, vel, sd, rnd) { if (rnd) { const an = (rnd() + rnd() - 1) * sd * 1.7, cs = Math.cos(an), sn = Math.sin(an); return { vx: vel.vx * cs - vel.vy * sn, vy: vel.vx * sn + vel.vy * cs }; } st.keep *= acc(sd, vel.L); return vel; }
  function nearestOpp(st, p) { let best = null, bd = 1e9; for (const q of st.ps) if (q.team !== p.team && !isK(q)) { const d = dist(q, p); if (d < bd) { bd = d; best = q; } } return best; }
  function loose(st, vel, team) { const o = core(st); st.b = C().makeBall({ x: o.x, y: o.y, vx: vel.vx, vy: vel.vy, team, ch: st.ch }); st.b.t = 9; }
  function start(st, A, rnd) {
    st.act = A; const h = st.h, nd = nearestOpp(st, h), cx = v => cl(v, AR.x0 + 1, AR.x1 - 1), cy = v => cl(v, AR.y0 + 1, AR.y1 - 1);
    if (A.k === 'kalkan' || A.k === 'sür') return;
    let T, v, sk;
    if (A.k === 'fiske') { T = { x: cx(nd.x + 3.5), y: cy(nd.y + A.side * 2.6) }; v = .3; sk = h.a.surme; st.route = [{ x: cx(nd.x + .3), y: cy(nd.y - A.side * 2.8) }]; st.recv = null; }
    else if (A.k === 'kenar') { const top = A.wall === 'üst', dw = top ? nd.y - AR.y0 : AR.y1 - nd.y; T = { x: cx(nd.x + 4), y: top ? AR.y0 + Math.max(1.2, dw * .35) : AR.y1 - Math.max(1.2, dw * .35), my: top ? AR.y0 : AR.y1 }; v = .35; sk = h.a.surme; st.route = [{ x: cx(nd.x + .3), y: cy(nd.y + (top ? 2.8 : -2.8)) }]; st.recv = null; }
    else if (A.k === 'gönder') { T = { x: G.x, y: G.y + A.side * G.h * .6 }; v = 1.1; sk = h.a.aktarim; st.route = null; st.recv = null; st.nShot++; st.shotTek = false; }
    else { const q = A.q; T = A.T ? { ...A.T } : A.k === 'önüne' ? { x: cx(q.x + 3.5), y: q.y } : { x: q.x, y: q.y }; if (A.k === 'kenardan') T.my = A.wall === 'üst' ? AR.y0 : AR.y1; v = A.v || (A.k === 'önüne' ? .3 : A.then ? .6 : .5); sk = h.a.aktarim; st.route = null; st.recv = q; st.passT = { x: T.x, y: T.y }; st.then = A.then ? { ...A.then } : null; st.nPass++; st.bkPend = A.k === 'kenardan'; if (st.bkPend) st.nBank = (st.nBank || 0) + 1; if (st.then && st.then.r === h) { const rp = { x: cx(nd.x + 2.6), y: cy(nd.y + (h.y < nd.y ? -2.4 : 2.4)) }; st.route = [rp]; st.then.T = rp; } }
    const fps = Object.assign(st.ps.filter(p => p !== h), { team: h.team }); if (A.k === 'gönder') v *= C().chgMul(st.ch); const vel = aimErr(st, kickV(core(st), T, v, fps, st.ch), .01 + (20 - sk) * .006, rnd); loose(st, vel, h.team);
    st.kicker = h; st.h = null; st.kickT = st.t; st.looseT = st.t;
  }
  // Tek dokunuş: alıcı Çekirdeği durdurmadan hedefe yönlendirir. Kontrol süresi yok, presçi yetişemez; bedeli isabet: dönüş açısı büyüdükçe ve Çekirdek hızlı geldikçe sapma ve ıska artar (Aktarım azaltır).
  function redirect(st, p, rnd) {
    const th = st.then, b = st.b, vin = Math.hypot(b.vx, b.vy), ak = p.a.aktarim ?? 10, T = th.k === 'tek' ? (th.T || { x: cl(th.r.x + 1.5, AR.x0 + 1, AR.x1 - 1), y: th.r.y }) : { x: G.x, y: G.y + th.side * G.h * .6 };
    const tin = Math.atan2(b.vy, b.vx), tout = Math.atan2(T.y - p.y, T.x - p.x), turn = Math.abs(wrap(tout - tin)), sd = .02 + (20 - ak) * .006 + turn * .05 + Math.max(0, vin - .6) * .06, miss = cl(.04 + turn * .08 + Math.max(0, vin - .9) * .2 - (ak - 10) * .01, .02, .6);
    st.then = null; st.kicker = p; st.kickT = st.t; st.looseT = st.t;
    if (rnd ? rnd() < miss : false) { const an = tin + Math.PI + (rnd() - .5) * 2.4, s2 = vin * .45; b.vx = Math.cos(an) * s2; b.vy = Math.sin(an) * s2; st.recv = null; return; }
    if (!rnd) st.keep *= 1 - miss;
    const vel = kickV({ x: b.x, y: b.y }, T, th.k === 'tek' ? .5 : 1.2 * C().chgMul(b.ch), Object.assign(st.ps.filter(q => q !== p), { team: p.team }), b.ch); vel.vx = vel.vx + b.vx * .2; vel.vy = vel.vy + b.vy * .2; vel.L = dist(p, T);
    const v2 = aimErr(st, vel, sd, rnd); b.vx = v2.vx; b.vy = v2.vy; b.done = null;
    if (th.k === 'tek') { st.recv = th.r; st.passT = T; st.nTek++; } else { st.recv = null; st.nShot++; st.nTekShot++; st.shotTek = true; }
  }
  // Savunma sistemi: kim basar, nasıl basar, diğerleri neyi tutar. Bekçi Kuyu'nun ağzında, Çekirdek ile Kuyu arasında durur.
  function defTargets(st) {
    const ps = st.ps, D = ps.filter(p => p.team === 1 && !isK(p)), A = ps.filter(p => p.team === 0), sys = st.cfg.sys, h = st.h && st.h.team === 0 ? st.h : null, cp = st.h || st.b, out = new Map();
    const byDist = D.slice().sort((u, v) => dist(u, cp) - dist(v, cp)), goalSide = (m, k) => { const gx = G.x - m.x, gy = G.y - m.y, gl = Math.hypot(gx, gy) || 1; return { x: m.x + gx / gl * k, y: m.y + gy / gl * k + (cp.y - m.y) * .1 }; };
    const pressPt = (d, mode) => { const base = Math.atan2(d.y - h.y, d.x - h.x), side = wrap(h.ca - base) > 0 ? 1 : -1; if (mode === 'kes') { const gx = G.x - h.x, gy = G.y - h.y, gl = Math.hypot(gx, gy) || 1; return { x: h.x + gx / gl * 2.2, y: h.y + gy / gl * 2.2 }; } if (mode === 'iç') { if (dist(d, h) < 2.4) return core(st); const s = h.y < 12 ? 1 : -1; return { x: h.x + 1.2, y: h.y + s * 1.6 }; } return { x: h.x + Math.cos(base + side * 1.25) * 1.9, y: h.y + Math.sin(base + side * 1.25) * 1.9 }; };
    let pr = [];
    if (!st.h) { if (st.t - st.looseT >= react(byDist[0])) pr = byDist.slice(0, 2).map(d => [d, 'top']); }
    else if (!h) pr = [];
    else if (sys === 'Adam adama') pr = [[D.find(d => d.man === h.id) || byDist[0], 'yan']];
    else if (sys === 'Çift pres') pr = [[byDist[0], 'yan'], [byDist[1], 'kes']];
    else if (sys === 'Kenara sıkıştır') pr = [[byDist[0], 'iç']];
    else pr = [[byDist[0], 'yan']];
    const pset = new Set(pr.map(x => x[0]));
    for (const [d, mo] of pr) out.set(d.id, { ...(mo === 'top' ? { x: st.b.x, y: st.b.y } : pressPt(d, mo)), press: true, job: mo === 'top' ? 'Çekirdeğe' : mo === 'kes' ? 'önünü kesiyor' : mo === 'iç' ? 'kenara itiyor' : 'basıyor' });
    let fr = D.filter(d => !pset.has(d));
    if (sys === 'Adam adama') { for (const d of fr) out.set(d.id, { ...goalSide(ps[d.man], 1.6), job: '' }); }
    else {
      if (sys === 'Kenara sıkıştır' && h && fr.length) { const wp = { x: Math.min(AR.x1 - 2, h.x + 5), y: h.y < 12 ? 2.5 : AR.y1 - 2.5 }, w = fr.slice().sort((u, v) => dist(u, wp) - dist(v, wp))[0]; out.set(w.id, { ...wp, job: 'tuzakta bekliyor' }); fr = fr.filter(d => d !== w); }
      const tg = A.filter(a => a !== st.h).sort((u, v) => dist(u, G) - dist(v, G)), used = new Set();
      for (const d of fr) { let best = null, bd = 1e9; for (const a of tg) { if (used.has(a)) continue; const q = dist(d, a) + dist(a, G) * .3; if (q < bd) { bd = q; best = a; } } if (best) { used.add(best); const g = goalSide(best, 1.8), pull = sys === 'Alan' ? .3 : .15; out.set(d.id, { x: g.x + (cp.x - g.x) * pull * .5, y: g.y + (cp.y - g.y) * pull, job: '' }); } else out.set(d.id, { x: Math.min(AR.x1 - 3, cp.x + 8), y: 12, job: '' }); }
    }
    const k = ps.find(isK), gy = cl(G.y + (cp.y - G.y) * .35, G.y - G.h + .6, G.y + G.h - .6); out.set(k.id, { x: G.x - 1.4, y: gy, job: '' });
    return out;
  }
  // Topsuz hücumcu: takımla birlikte ilerleyen yerinin çevresinde en boş, pas hattı açık, Kuyu'ya yakın noktayı seçer.
  function supportPt(st, p) {
    const cp = st.h || st.b, ax = cl(p.home.x + (cp.x - 34) * .8, AR.x0 + 2, AR.x1 - 3), ay = p.home.y, D = st.ps.filter(q => q.team === 1 && !isK(q)); let best = null, bv = -1e9;
    for (let k = -1; k < 8; k++) { const pt = k < 0 ? { x: ax, y: ay } : { x: cl(ax + Math.cos(k * .785) * 3, AR.x0 + 1.5, AR.x1 - 2), y: cl(ay + Math.sin(k * .785) * 3, 1.5, AR.y1 - 1.5) };
      let fd = 6, ln = 3; for (const d of D) { fd = Math.min(fd, dist(d, pt)); if (st.h) { const sx = pt.x - cp.x, sy = pt.y - cp.y, L2 = sx * sx + sy * sy || 1, tt = cl(((d.x - cp.x) * sx + (d.y - cp.y) * sy) / L2, 0, 1); ln = Math.min(ln, Math.hypot(cp.x + sx * tt - d.x, cp.y + sy * tt - d.y)); } }
      const v = fd * .3 + ln * .4 - dist(pt, G) * .06; if (v > bv) { bv = v; best = pt; } }
    return best;
  }
  // Karşılama noktası: Çekirdeğin (alanlar ve kenarlarla) gideceği yolda bu oyuncunun ilk yetişebildiği yer. Çekirdek sapınca alıcı da yolunu düzeltir.
  function interceptPt(st, p) { const b = { ...st.b }, v = spd(p) * 1.25; let last = { x: b.x, y: b.y }; for (let i = 1; i <= 90 && !b.done; i++) { ballStep(b, st.ps); last = { x: b.x, y: b.y, i }; if (i % 2 === 0 && Math.max(0, dist(p, b) - C().Q.reach * .7) <= v * i) return last; } return last; }
  function moveTo(p, g, v, k) { const gx = g.x - p.x, gy = g.y - p.y, gl = Math.hypot(gx, gy); if (gl > .3) { p.vx += (gx / gl * Math.min(v, gl) - p.vx) * k; p.vy += (gy / gl * Math.min(v, gl) - p.vy) * k; } else { p.vx *= .7; p.vy *= .7; } }
  function step(st, rnd) {
    const K = C(), T = K.Turn, Q = K.Q, t = st.t, ps = st.ps, h = st.h;
    if (h) { const nd = nearestOpp(st, h), la = 2 + (h.a.okuma ?? 10) * .4, aw = Math.atan2(h.y - (nd.y + nd.vy * la), h.x - (nd.x + nd.vx * la)), sC = spd(h) * ((h.slowT || 0) > t ? .55 : 1) * (dist(nd, h) < 1.9 ? .8 : 1), A = h.team === 0 ? st.act : { k: 'sür', dir: Math.PI };
      let vx, vy, ta; if (A.k === 'kalkan') { vx = Math.cos(aw) * sC * .3; vy = Math.sin(aw) * sC * .3; ta = aw; } else { vx = Math.cos(A.dir) * sC * .85; vy = Math.sin(A.dir) * sC * .85; ta = A.dir + cl(wrap(aw - A.dir), -1, 1) * .7; }
      h.vx += (vx - h.vx) * .25; h.vy += (vy - h.vy) * .25;
      const w = T.omega(h), d = wrap(ta - h.ca); if (Math.abs(d) > .02) { const pa = wrap(aw + Math.PI), m1 = wrap(h.ca + d / 2), m2 = wrap(h.ca + (d - Math.sign(d) * 2 * Math.PI) / 2), dir = Math.abs(wrap(m1 - pa)) < .9 && Math.abs(wrap(m2 - pa)) > Math.abs(wrap(m1 - pa)) ? -Math.sign(d) : Math.sign(d); h.ca = wrap(h.ca + dir * (dir === Math.sign(d) ? Math.min(w, Math.abs(d)) : w)); } }
    if (h) { let pr = 0; for (const o of ps) if (o.team !== h.team) pr += K.infl(o, h.x, h.y); const still = cl(1 - Math.hypot(h.vx, h.vy) / .2, 0, 1); st.ch = Math.min(1, st.ch + .0035 * (.4 + .6 * still) * (.25 + .75 * Math.exp(-pr * 2)) * (1 - st.ch * .5) * (1 + ((h.a.tutus ?? 10) - 10) * .05)); }
    if (!h && st.b && !st.recv && (t % 6 === 0 || !st.chaser)) { let best = null, bt = 1e9; for (const p of ps) { if (p.team !== 0 || (p === st.kicker && st.route && st.route.length)) continue; const ip = interceptPt(st, p), tt = ip.i ?? 99; if (tt < bt) { bt = tt; best = p; } } st.chaser = best; }
    const atkTeam = h ? h.team : (st.b ? st.b.team : 0), dT = atkTeam === 0 ? defTargets(st) : null;
    for (const p of ps) { if (p === h) continue; let g, v = spd(p), k = .3;
      if (atkTeam === 0) { if (p.team === 1) { const o = dT.get(p.id); g = o; p.job = o.job; v *= o.press ? 1.2 : 1.1; k = .35; }
        else if (!h && p === st.recv) { if (!p.ip || (t + p.id) % 6 === 0) p.ip = interceptPt(st, p); g = p.ip; v *= 1.25; p.job = 'pası alıyor'; }
        else if (!h && !st.recv && p === st.chaser) { if (!p.ip || (t + p.id) % 6 === 0) p.ip = interceptPt(st, p); g = p.ip; v *= 1.25; }
        else if (!h && p === st.kicker && st.route) { if (st.route.length && dist(p, st.route[0]) < 1) st.route.shift(); g = st.route.length ? st.route[0] : st.b; v *= 1.25; }
        else if (!h && st.then && st.then.k === 'tek' && p === st.then.r) { g = { x: cl(p.x + 2, AR.x0 + 1, AR.x1 - 2), y: p.y }; v *= 1.2; }
        else { if (p.sp == null || (t + p.id * 4) % 15 === 0) p.sp = supportPt(st, p); g = p.sp; } }
      else { const cp = core(st), near = ps.filter(q => q.team === p.team && q !== h && !isK(q)).sort((u, v2) => dist(u, cp) - dist(v2, cp)).slice(0, 2); g = p.team === 0 && near.includes(p) ? cp : p; v *= 1.2; }
      moveTo(p, g, v, k); }
    for (const p of ps) { p.x = cl(p.x + p.vx, AR.x0 + .9, AR.x1 - .9); p.y = cl(p.y + p.vy, AR.y0 + .9, AR.y1 - .9); }
    for (let i = 0; i < ps.length; i++) for (let j = i + 1; j < ps.length; j++) { const a = ps[i], b = ps[j], dd = dist(a, b) || 1; if (dd < 1.6) { const push = (1.6 - dd) / 2, ux = (a.x - b.x) / dd, uy = (a.y - b.y) / dd; a.x += ux * push; a.y += uy * push; b.x -= ux * push; b.y -= uy * push; } }
    if (h) { const B = Q.body, r = .4, kx = h.x + Math.cos(h.ca) * B, ky = h.y + Math.sin(h.ca) * B; if (ky < AR.y0 + r) h.y += AR.y0 + r - ky; if (ky > AR.y1 - r) h.y -= ky - (AR.y1 - r); if (kx < AR.x0 + r) h.x += AR.x0 + r - kx; if (kx > AR.x1 - r) h.x -= kx - (AR.x1 - r); }
    if (h) { const o = core(st); for (const p of ps) { if (p.team === h.team || t < (p.cd || 0)) continue; if (dist(o, p) < Q.reach && !T.shielded(h, p, o.x, o.y)) { p.cd = t + 12; h.slowT = t + 18; h.vx *= .45; h.vy *= .45; const P = cl(.26 + ((p.a.kesme ?? 10) - (h.a.surme ?? 10)) * .035 + Math.hypot(h.vx, h.vy) * .4, .05, .9); if (rnd ? rnd() < P : false) { take(st, p, 'uzanıp aldı'); return endStep(st); } if (!rnd) st.keep *= 1 - P; } } }
    else { const b = st.b, ox = b.x, oy = b.y; if (!b.done) ballStep(b, ps); if (b.goal) return endStep(st); const sx = b.x - ox, sy = b.y - oy, sl = sx * sx + sy * sy, dd = q => { const tt = sl > 0 ? cl(((q.x - ox) * sx + (q.y - oy) * sy) / sl, 0, 1) : 1; return Math.hypot(ox + sx * tt - q.x, oy + sy * tt - q.y); };
      const reach = p => isK(p) ? Q.bekBody + Math.min(Q.bekDive, Math.max(0, (t - st.kickT) - (Q.bekReact - (p.a.okuma ?? 10) * Q.bekReactOk)) * Q.bekDiveV) : p.team === b.team ? Q.reach : .9 + Math.min(Q.reach - .9, Math.max(0, (t - st.kickT) - (6 - (p.a.okuma ?? 10) * .25)) * .12); /* rakip, son vuruştan hemen sonra yalnızca gövdesine çarpan Çekirdeği keser; uzanmak için tepki süresi gerekir. Hızlı ve tekte oynanan pas bu yüzden presçinin yanından geçebilir. */
      const cand = ps.filter(p => t >= (p.cd || 0) && !(p === st.kicker && t - st.kickT < 5) && dd(p) < reach(p)).sort((u, v) => dd(u) - dd(v));
      for (const p of cand) { p.cd = t + 6; const mine = p.team === b.team;
        if (mine && p === st.recv && st.then) { redirect(st, p, rnd); break; }
        const P = K.ctrlP(b, p, ps);
        if (rnd) { if (rnd() < P) { if (mine) gain(st, p); else take(st, p, isK(p) ? 'Bekçi tuttu' : st.recv ? 'pası kesti' : 'boştaki Çekirdeği kaptı'); break; } const an = Math.atan2(-b.vy, -b.vx) + (rnd() - .5) * 2, s2 = Math.hypot(b.vx, b.vy) * .45; b.vx = Math.cos(an) * s2; b.vy = Math.sin(an) * s2; b.done = null; st.recv = null; st.then = null; st.route = null; st.chaser = null; }
        else { if (mine) { st.keep *= P + (1 - P) * .5; gain(st, p); break; } st.keep *= 1 - P; b.vx = -b.vx * .45; b.vy = -b.vy * .45; b.done = null; st.recv = null; st.then = null; st.route = null; st.chaser = null; break; } } } /* kafada da: tutamayan rakip Çekirdeği sektirir; Çekirdek yoluna devam etmez (eskiden ediyordu, gönderme bu yüzden fazla değerli görünüyordu) */
    return endStep(st);
  }
  function gain(st, p) { const keepCh = st.b ? st.b.ch : st.ch, own = st.b ? st.b.team === p.team : (st.h && st.h.team === p.team); if (st.bkPend && p.team === 0 && p !== st.kicker) st.nBankOk = (st.nBankOk || 0) + 1; st.bkPend = false; const o = st.b || core(st); st.h = p; p.ca = Math.atan2(o.y - p.y, o.x - p.x); st.b = null; st.recv = null; st.route = null; st.kicker = null; st.then = null; st.chaser = null; st.act = { k: 'sür', dir: p.team === 0 ? 0 : Math.PI }; st.need = true; st.ch = own ? Math.max(.15, keepCh * .7) : .3; }
  function take(st, p, how) { st.bkPend = false; if (st.after) { gain(st, p); st.res = 'geri kazandı'; return; } gain(st, p); st.res = isK(p) ? 'Bekçi aldı' : 'kaybetti'; st.how = how; }
  function endStep(st) { if (!st.res && st.b && st.b.goal) { if (st.b.team === 0 && !st.after) { st.res = 'SAYI'; st.how = st.shotTek ? 'tekte gönderme' : 'gönderme'; } else st.res = st.res || 'süre'; } st.t++; return st; }
  function cands(st) {
    const h = st.h, nd = nearestOpp(st, h), one = st.cfg.one !== 'kapalı', out = [{ k: 'kalkan' }]; for (const d of [0, .6, -.6, 1.2, -1.2, Math.PI]) out.push({ k: 'sür', dir: d });
    if (dist(nd, h) < 7 && nd.x > h.x - 1) { out.push({ k: 'fiske', side: 1 }, { k: 'fiske', side: -1 }); if (h.y < 6 && nd.y < 7) out.push({ k: 'kenar', wall: 'üst' }); if (h.y > AR.y1 - 6 && nd.y > AR.y1 - 7) out.push({ k: 'kenar', wall: 'alt' }); }
    if (dist(h, G) < 24) out.push({ k: 'gönder', side: 1 }, { k: 'gönder', side: -1 });
    const mates = st.ps.filter(q => q.team === 0 && q !== h);
    for (const q of mates) { out.push({ k: 'pas', q }); if (q.x + 3.5 < AR.x1 - 1) out.push({ k: 'önüne', q }); if (dist(h, q) > 5) { const B = []; for (const [wall, wy] of [['üst', AR.y0], ['alt', AR.y1]]) for (const T of [{ x: q.x, y: q.y }, { x: cl(q.x + 3.5, AR.x0 + 1, AR.x1 - 1.5), y: q.y }, { x: cl(q.x + 2.5, AR.x0 + 1, AR.x1 - 1.5), y: cl(q.y + (wy ? 2 : -2), 1, AR.y1 - 1) }]) for (const v of [.5, .9]) { const A = { k: 'kenardan', q, wall, T, v }; A.m = laneMargin(st, A); B.push(A); } B.sort((a, b) => b.m - a.m); for (const A of B.slice(0, 2)) if (A.m > -.5) out.push(A); }
      if (one) { for (const r of mates.concat([h])) if (r !== q) out.push({ k: 'pas', q, then: { k: 'tek', r } }); if (dist(q, G) < 20) out.push({ k: 'pas', q, then: { k: 'şut', side: q.y < G.y ? 1 : -1 } }); } }
    return out;
  }
  // Hızlı hat kontrolü (kafada, oyuncusuz yörünge): her rakip yörüngenin her noktasına, Çekirdek oraya varmadan, tepkisini ve koşusunu hesaba katarak uzanabilir mi? En küçük pay döner.
  function laneMargin(st, A) { const o = core(st), T = { ...A.T, my: A.wall === 'üst' ? AR.y0 : AR.y1 }, fps = Object.assign(st.ps.filter(p => p !== st.h), { team: 0 }), vel = kickV(o, T, A.v, fps, st.ch), b = C().makeBall({ x: o.x, y: o.y, vx: vel.vx, vy: vel.vy, team: 0, ch: st.ch }); let m = 9;
    for (let t = 1; t < 200 && !b.done; t++) { ballStep(b, fps); if (Math.hypot(b.x - T.x, b.y - T.y) < 1.2) break; if (t % 2) continue; for (const d of st.ps) { if (d.team !== 1) continue; const rt = Math.max(0, t - react(d)), R = (isK(d) ? 1.4 : .9 + Math.min(.7, Math.max(0, t - 6 + (d.a.okuma ?? 10) * .25) * .12)) + rt * spd(d) * 1.2; m = Math.min(m, Math.hypot(b.x - d.x, b.y - d.y) - R); } }
    return m; }
  const thr = o => Math.exp(-dist(o, G) / 12);
  function value(st, A, rnd) {
    const s = clone(st), ok = s.h.a.okuma ?? 10, err = Math.max(0, 16 - ok) * .15; for (const p of s.ps) if (p.team === 1) { p.x += (rnd() - .5) * err; p.y += (rnd() - .5) * err; }
    s.keep = 1; if (A.q) A = { ...A, q: s.ps[A.q.id], then: A.then ? { ...A.then, r: A.then.r ? s.ps[A.then.r.id] : null } : null }; const t0 = thr(core(s)); start(s, A, null);
    for (let i = 0; i < 60 && !s.res; i++) { if (s.h && s.need) { s.need = false; s.act = { k: 'sür', dir: 0 }; } step(s, null); }
    if (s.res === 'SAYI') return 3 * s.keep; if (s.res) return 0;
    let own; if (s.h) own = s.h.team === 0 ? 1 : 0; else { let da = 1e9, dd = 1e9; for (const p of s.ps) { if (isK(p)) continue; const d = dist(p, s.b); if (p.team === 0) da = Math.min(da, d); else dd = Math.min(dd, d); } own = 1 / (1 + Math.exp((da - dd) / 1.5)); }
    let free = 6; if (s.h) for (const p of s.ps) if (p.team === 1 && !isK(p)) free = Math.min(free, dist(p, s.h));
    return s.keep * own * (1 + 1.5 * (thr(core(s)) - t0) + .03 * free);
  }
  function decide(st, rnd) { const ok = st.h.a.okuma ?? 10, C2 = cands(st).map(A => ({ A, v: value(st, A, rnd) + (rnd() - .5) * Math.max(0, 16 - ok) * .03 })); C2.sort((a, b) => b.v - a.v); return C2[0].A; }
  const NM = { kalkan: 'gövdeyle koruyor', sür: 'sürüyor', fiske: 'fiske çalımı', kenar: 'kenar çalımı', pas: 'pas', önüne: 'önüne pas', kenardan: 'kenardan pas', gönder: 'gönderme', 'pas→tek pas': 'pas → tekte pas', 'pas→tek gönderme': 'pas → tekte gönderme', 'ver-kaç': 'ver-kaç (tekte geri)' };
  const keyOf = A => A.then ? (A.then.k === 'tek' ? (A.then.r && A.q && A.then.r.id === A._h ? 'ver-kaç' : 'pas→tek pas') : 'pas→tek gönderme') : A.k;
  function run(cfg, rnd, rec) {
    const st = create(cfg), fr = [], used = {}; let nd = 0;
    const push = () => { const o = core(st); fr.push({ p: st.ps.map(p => [Math.round(p.x * 100) / 100, Math.round(p.y * 100) / 100, p.team === 1 && p.job ? p.job : '']), c: [o.x, o.y], ch: st.h ? st.ch : st.b ? st.b.ch : st.ch, h: st.h ? st.h.id : -1, loose: !st.h, lab: st.h && st.h.team === 0 ? NM[keyOf(st.act)] : st.h ? 'turuncu taşıyor' : st.then ? (st.then.k === 'tek' ? 'pas yolda · tekte pas gelecek' : 'pas yolda · tekte gönderme gelecek') : st.recv ? 'pas yolda' : 'Çekirdek boşta', after: st.after }); };
    while (!st.res && st.t < 900) { if (st.h && st.h.team === 0 && (st.t >= nd || st.need)) { st.need = false; const A = decide(st, rnd); A._h = st.h.id; const k = keyOf(A); used[k] = (used[k] || 0) + 1; start(st, A, rnd); nd = st.t + 12; } step(st, rnd); if (rec) push(); }
    const out = { res: st.res || 'süre doldu', t: st.t, how: st.how, used, nPass: st.nPass, meta: st.ps.map(p => ({ team: p.team, R: p.R, D: p.D, a: p.a, role: p.role })), nTek: st.nTek, nBank: st.nBank || 0, nBankOk: st.nBankOk || 0, nShot: st.nShot, nTekShot: st.nTekShot, fr, takeF: st.res === 'kaybetti' ? fr.length - 1 : null };
    if (st.res === 'kaybetti' && rec) { st.res = null; st.after = true; const t1 = st.t + 150; while (!st.res && st.t < t1) { step(st, rnd); push(); } if (st.res === 'geri kazandı') out.after = 'mavi geri kazandı'; }
    else if (rec) for (let i = 0; i < 40; i++) push();
    return out;
  }
  window.AlanSmall = { AR, G, LINE, SYS, NM, create, step, run };
})();
