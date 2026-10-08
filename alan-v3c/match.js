(function () {
  // Katman 4 (basit) · Yerleşim + maç döngüsü. Talimata sadık, kusursuz değil.
  const C = () => window.AlanCore, Dd = () => window.AlanDecide, W = 100, H = 50;
  const TAC0 = { blok: 'Orta', pres: 1, arkada: 1, onde: 0, genislik: 'Normal', risk: .5, tempo: .5 };
  const BLOK = { 'Düşük': 22, 'Orta': 34, 'Yüksek': 50 }, GEN = { 'Dar': 28, 'Normal': 38, 'Geniş': 46 };
  const SLOTS = [{ role: 'Bekçi', d: 0, l: 0 }, { role: 'Bek', d: .2, l: -.5 }, { role: 'Bek', d: .2, l: .5 }, { role: 'Orta', d: .45, l: -.35 }, { role: 'Orta', d: .45, l: .35 }, { role: 'Kanat', d: .75, l: -.85 }, { role: 'Uç', d: .8, l: .25 }];
  const NAMES = [['Aras', 'Deniz', 'Ece', 'Kaan', 'Mert', 'Selin', 'Tuna'], ['Bora', 'Cem', 'Duru', 'Ilgaz', 'Nehir', 'Rüzgâr', 'Yaz']];
  const cl = (v, a, b) => v < a ? a : v > b ? b : v, dirOf = t => t === 0 ? 1 : -1, ownX = t => t === 0 ? 0 : W;
  function rng(seed) { let a = seed >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
  const spd = p => .17 + (p.a.hiz ?? 10) * .006;
  function createMatch(seed, o) {
    o = o || {}; const m = { r: rng(seed || 1), seed, tick: 0, len: o.len || 7200, score: [0, 0], ps: [], ball: null, holder: null, ch: .3, decT: 0, over: false, events: [], fl: null,
      tac: [0, 1].map(t => ({ ...TAC0, ...((o.tac && o.tac[t]) || {}) })), st: { duel: [0, 0], duelW: [0, 0], pass: [0, 0], passOk: [0, 0], shot: [0, 0], steal: [0, 0], poss: [0, 0], bank: [0, 0], lead: [0, 0], loose: [0, 0] } };
    for (const t of [0, 1]) SLOTS.forEach((s, i) => { const a = { okuma: 10, aktarim: 10, tutus: 10, kesme: 10, yogunluk: 10, hiz: 10, cesaret: 10, surme: 10, ...((o.attrs && o.attrs[t]) || {}) }; m.ps.push({ id: t * 10 + i, team: t, i, role: s.role, slot: s, name: NAMES[t][i], a, R: s.role === 'Bekçi' ? 7 : 8, D: s.role === 'Bekçi' ? 1.2 : 1, x: 50, y: 25, vx: 0, vy: 0, tx: 50, ty: 25, cd: 0 }); });
    kickoff(m, 0); return m;
  }
  function ev(m, type, team, text) { const s = Math.floor(m.tick / 60); m.events.unshift({ t: `${String(Math.floor(s / 60)).padStart(2, '0')}:${String(s % 60).padStart(2, '0')}`, type, team, text }); if (m.events.length > 120) m.events.pop(); }
  function kickoff(m, t) {
    for (const p of m.ps) { const s = p.slot, x0 = ownX(p.team), dir = dirOf(p.team), d = s.role === 'Bekçi' ? 3 : 10 + s.d * 32; p.x = x0 + dir * d; p.y = 25 + s.l * 16; p.vx = p.vy = 0; p.tx = p.x; p.ty = p.y; }
    const h = m.ps.find(p => p.team === t && p.i === 3); h.x = 50; h.y = 25; m.holder = h; m.ball = null; m.ch = .3; m.decT = 30; m.fl = null;
  }
  // takımın şekli: savunmada kompakt ve blok yüksekliğinde, hücumda uzun ve genişlik talimatında
  function slotPos(m, p, bx, by, att) {
    const t = p.team, tac = m.tac[t], s = p.slot, x0 = ownX(t), dir = dirOf(t), bd = (bx - x0) * dir;
    if (s.role === 'Bekçi') { const gx = x0, gy = 25, dx = bx - gx, dy = by - gy, L = Math.hypot(dx, dy) || 1, r = Math.min(6, L * .22); return { x: gx + dx / L * r, y: gy + dy / L * r }; }
    let back, len, wid, sh;
    if (att) { back = cl(bd - 18, 12, 62); len = 48; wid = GEN[tac.genislik] || 38; sh = .15; }
    else { back = cl(Math.min(BLOK[tac.blok] || 34, bd - 4), 9, 62); len = 24; wid = 26; sh = .4; }
    if (!att) { const front = bd - 7; if (back + len > front) len = Math.max(6, front - back); }
    let depth = back + (s.d - .2) / .6 * len; if (!att) depth = Math.min(depth, Math.max(back, bd - 7));
    return { x: cl(x0 + dir * depth, 2, 98), y: cl(25 + s.l * wid / 2 + (by - 25) * sh, 2, 48) };
  }
  function laneOpp(m, t, a, b) { let mx = 0; for (let i = 1; i <= 4; i++) { const x = a.x + (b.x - a.x) * i / 5, y = a.y + (b.y - a.y) * i / 5; mx = Math.max(mx, Dd().oppAt(m.ps, t, x, y)); } return mx; }
  // topsuz hücumcu: slotunun yakınında açık, pas hattı görünen, ileri değerli ve arkadaşlardan ayrık nokta
  function offBall(m, p, base, h) {
    const D = Dd(), t = p.team, dir = dirOf(t), ok = p.a.okuma ?? 10, cands = [base];
    for (let k = 0; k < 8; k++) { const a = k * Math.PI / 4; for (const r of [5, 9]) cands.push({ x: base.x + Math.cos(a) * r, y: base.y + Math.sin(a) * r }); }
    if (p.slot.d >= .7) { const lastD = m.ps.filter(q => q.team !== t && q.role !== 'Bekçi').reduce((mx, q) => Math.max(mx, (q.x - ownX(t)) * dir), 0); cands.push({ x: ownX(t) + dir * (lastD + 5), y: p.y }, { x: ownX(t) + dir * (lastD + 5), y: base.y }); }
    let best = base, bv = -1e9;
    for (const c of cands) {
      if (c.x < 2 || c.x > 98 || c.y < 2 || c.y > 48) continue;
      const open = Math.exp(-D.oppAt(m.ps, t, c.x, c.y) * 2), lane = h ? Math.exp(-laneOpp(m, t, h, c) * 1.5) : 1, fwd = D.threat(t, c.x, c.y);
      let crowd = 0; for (const q of m.ps) if (q.team === t && q !== p && q.role !== 'Bekçi') crowd += Math.exp(-((q.tx - c.x) ** 2 + (q.ty - c.y) ** 2) / 36);
      const leash = Math.hypot(c.x - base.x, c.y - base.y) / 12, v = .5 * open + .6 * lane + 1.2 * fwd - .5 * crowd - .4 * leash * leash + (m.r() - .5) * (20 - ok) * .02;
      if (v > bv) { bv = v; best = c; }
    }
    return best;
  }
  function position(m) {
    const ps = m.ps, h = m.holder, b = m.ball, bx = h ? h.x : b.x, by = h ? h.y : b.y, attT = h ? h.team : b.team;
    // boşta / uçan Çekirdek: her takımdan en yakın 1-2 kişi kovalar; alıcı koşar
    const chase = new Set();
    if (!h) { const tgt = m.fl && m.fl.end ? m.fl.end : { x: b.x, y: b.y }; for (const t of [0, 1]) { const c = ps.filter(p => p.team === t && p.role !== 'Bekçi').sort((u, v) => Math.hypot(u.x - tgt.x, u.y - tgt.y) - Math.hypot(v.x - tgt.x, v.y - tgt.y)); const n = b.done === 'durdu' || b.defl ? 2 : (t === b.team ? 0 : 1); c.slice(0, n).forEach(p => { chase.add(p); p.tx = tgt.x; p.ty = tgt.y; p.sprint = true; }); } if (m.fl && m.fl.q && !chase.has(m.fl.q)) { chase.add(m.fl.q); const q = m.fl.q; q.tx = m.fl.to.x; q.ty = m.fl.to.y; q.sprint = true; } }
    for (const t of [0, 1]) {
      const att = t === attT, tac = m.tac[t], mine = ps.filter(p => p.team === t);
      let press = new Set(), cover = new Set(); const marked = new Set();
      if (!att) {
        const field = mine.filter(p => p.role !== 'Bekçi').sort((u, v) => u.slot.d - v.slot.d);
        field.slice(0, tac.arkada).forEach(p => cover.add(p));
        if (h) mine.filter(p => p.role !== 'Bekçi' && !cover.has(p)).sort((u, v) => Math.hypot(u.x - h.x, u.y - h.y) - Math.hypot(v.x - h.x, v.y - h.y)).slice(0, tac.pres).forEach(p => press.add(p));
      }
      for (const p of mine) {
        if (p === h || chase.has(p)) continue; p.sprint = false; const base = slotPos(m, p, bx, by, att);
        if (p.role === 'Bekçi') { p.tx = base.x; p.ty = base.y; continue; }
        if (att) { if ((m.tick + p.id) % 15 === 0 || p.otx == null) { const c = offBall(m, p, base, h); p.otx = c.x; p.oty = c.y; } p.tx = p.otx; p.ty = p.oty; continue; }
        const gx = ownX(t), opps = ps.filter(q => q.team !== t && q.role !== 'Bekçi');
        if (press.has(p)) { const dx = gx - h.x, dy = 25 - h.y, L = Math.hypot(dx, dy) || 1, dd = press.size > 1 && [...press].indexOf(p) > 0 ? 3.2 : 2.2, side = [...press].indexOf(p) === 1 ? 1 : [...press].indexOf(p) === 2 ? -1 : 0, nx = -dy / L, ny = dx / L; p.tx = h.x + dx / L * dd + nx * side * 2.5; p.ty = h.y + dy / L * dd + ny * side * 2.5; p.sprint = Math.hypot(p.x - p.tx, p.y - p.ty) > 2; continue; }
        if (cover.has(p)) { const a = opps.reduce((u, v) => Math.abs(v.x - gx) < Math.abs(u.x - gx) ? v : u), dx = gx - a.x, dy = 25 - a.y, L = Math.hypot(dx, dy) || 1; p.tx = a.x + dx / L * 4; p.ty = a.y + dy / L * 4; if (Math.abs(p.tx - gx) > Math.abs(base.x - gx)) { p.tx = base.x; } continue; }
        // bölge: slotunu tutar, bölgesindeki rakibin Kuyu tarafına geçer, top–Kuyu hattına biraz kayar
        let tx = base.x, ty = base.y; const near = opps.filter(q => q !== h && !marked.has(q) && Math.hypot(q.x - base.x, q.y - base.y) < 9).sort((u, v) => Math.hypot(u.x - base.x, u.y - base.y) - Math.hypot(v.x - base.x, v.y - base.y))[0];
        if (near) { marked.add(near); const gxx = (gx + bx) / 2, gyy = (25 + by) / 2, dx = gxx - near.x, dy = gyy - near.y, L = Math.hypot(dx, dy) || 1; tx = near.x + dx / L * 3; ty = near.y + dy / L * 3; const off = Math.hypot(tx - base.x, ty - base.y); if (off > 7) { tx = base.x + (tx - base.x) * 7 / off; ty = base.y + (ty - base.y) * 7 / off; } }
        else { const dx = gx - bx, dy = 25 - by, L = Math.hypot(dx, dy) || 1, u = ((tx - bx) * dx + (ty - by) * dy) / L, px = bx + dx / L * u, py = by + dy / L * u; tx += (px - tx) * .3; ty += (py - ty) * .3; }
        p.tx = tx; p.ty = ty;
      }
    }
  }
  function move(m) {
    for (const p of m.ps) {
      const dx = p.tx - p.x, dy = p.ty - p.y, L = Math.hypot(dx, dy), v = spd(p) * (p.sprint ? 1.25 : 1) * (p === m.holder ? .85 : 1) * (p.noTouch ? .3 : 1), want = L < .3 ? 0 : Math.min(v, L * .25);
      const wx = L ? dx / L * want : 0, wy = L ? dy / L * want : 0; p.vx += (wx - p.vx) * .18; p.vy += (wy - p.vy) * .18; p.x = cl(p.x + p.vx, .8, W - .8); p.y = cl(p.y + p.vy, .8, H - .8);
    }
  }
  function launch(m, h, o) {
    const D = Dd(), K = C(), v = D.applyError(h, o.launch, m.r, o.ot || 0), t = h.team; const tk = o.ot ? ' · tek dokunuş' : ''; if (o.ot) { m.st.ot = m.st.ot || [0, 0]; m.st.ot[t]++; if (o.kind !== 'gönder' && o.kind !== 'aşırt' && o.kind !== 'kenardan') ev(m, 'pas', t, `${h.name} tek dokunuşla → ${o.q ? o.q.name : ''} · şarj %${Math.round(m.ch * 100)}`); }
    m.ball = K.makeBall({ x: h.x, y: h.y, vx: v.vx, vy: v.vy, air: v.air || 0, team: t, ch: m.ch, from: h, recv: o.q || null }); if (o.kind === 'aşırt') { m.st.lob = m.st.lob || [0, 0]; m.st.lob[t]++; ev(m, 'pas', t, `${h.name} aşırttı → ${o.q.name}${tk}`); } m.holder = null;
    const pr = K.predict(m.ball, m.ps, null, 600); m.fl = { t0: m.tick, kind: o.kind, q: o.q || null, to: o.to, end: { x: pr.end.x, y: pr.end.y }, shot: o.shot };
    if (o.kind === 'gönder') { m.st.shot[t]++; m.ball.shotPk = o.det ? o.det.Pk : null; if (!m.lite) { const c = m.st.cal || (m.st.cal = { n: 0, pk: 0, g: 0 }); if (m.ball.shotPk != null) { c.n++; c.pk += m.ball.shotPk; } } ev(m, 'gonder', t, `${h.name} ${o.shot} gönderdi${tk} · şarj %${Math.round(m.ch * 100)}`); }
    else { m.st.pass[t]++; if (o.kind === 'kenardan') { m.st.bank[t]++; ev(m, 'pas', t, `${h.name} kenardan sektirdi → ${o.q.name}${tk}`); } if (o.kind === 'önüne') m.st.lead[t]++; if (o.why === 'kombinasyon') { m.st.combo = m.st.combo || [0, 0]; m.st.combo[t]++; } }
  }
  const D0 = () => window.AlanDecide.D && window.AlanDecide.freeAt ? Object.assign({ freeAt: window.AlanDecide.freeAt }, window.AlanDecide.D) : null;
  function take(m, p, how) {
    m._inSp = m.ball ? Math.hypot(m.ball.vx, m.ball.vy) : 0;
    const prev = m.ball ? m.ball.team : null, fl = m.fl; m.holder = p; m.ball = null; m.decT = Math.max(2, Math.round(9 - (p.a.okuma ?? 10) * .4 + m.r() * 3)); m.fl = null; p.driveTo = null; // karşılamadan sonra karar: Okuma'sı yüksek oyuncu Çekirdek gelmeden bakmıştır, hemen oynar
    if (prev === p.team) { if (fl && fl.kind !== 'gönder') { m.st.passOk[p.team]++;
      // Tek dokunuş: alıcı Çekirdeği durdurmadan yönlendirebilir (pas ya da gönderme). Şarj hiç azalmaz, ama isabet gelen hıza ve Aktarım'a göre düşer.
      // Kontrol ederse şarjın bir kısmı gider (boşta karşılayan daha çok korur). Hangisinin iyi olduğuna oyuncu kendi karar verir.
      const D = Dd(), inSp = m._inSp || 0, chFull = m.ch, chCtrl = D.recvCh(m.ps, p.team, p.x, p.y, chFull), ot = D.otFactor(p, inSp);
      let one = null; if (!m._lite && (p.a.okuma ?? 10) >= D.D.otOk) { const r = D.decideOT(p, m.ps, chFull, m.r, m.tac[p.team], ot), rc = D.decide(p, m.ps, chCtrl, m.r, m.tac[p.team]); const ctrlBest = rc.opts.length ? rc.opts[0].v : 0; if (r && r.v > ctrlBest + D.D.otMargin) one = r; }
      if (one) { m.ch = chFull; one.ot = ot; launch(m, p, one); return; }
      m.ch = chCtrl; } }
    else { m.st.steal[p.team]++; m.ch = .3; ev(m, 'kesme', p.team, how || `${p.name} Çekirdeği aldı`); }
  }
  function step(m) {
    if (m.over) return; const K = C(), D = Dd(), ps = m.ps; m.tick++;
    for (const p of ps) p.noTouch = p.stun > m.tick;
    if (window.AlanShape) window.AlanShape.position(m); else position(m); move(m); if (window.AlanShape) { const sw = window.AlanShape.metrics(m).swarm; m.st.swarm = (m.st.swarm || 0) + sw; m.st.swN = (m.st.swN || 0) + 1; }
    const h = m.holder;
    if (h) {
      m.st.poss[h.team]++;
      const pr = D.oppAt(ps, h.team, h.x, h.y), still = cl(1 - Math.hypot(h.vx, h.vy) / .2, 0, 1);
      m.ch = Math.min(1, m.ch + D.chargeRate(h, ps, still, m.ch));
      // ikili mücadele: yakındaki savunmacı girip girmemeye kendisi karar verir. Tutarsa alır (ya da Çekirdek boşa çıkar), tutmazsa geçilir.
      for (const q of ps) {
        if (q.team === h.team || q.role === 'Bekçi' || q.noTouch || q.cd > m.tick) continue; if (Math.hypot(q.x - h.x, q.y - h.y) > D.DU.engage) continue;
        const P = D.duelP(q, h, ps), est = P + (m.r() - .5) * (20 - (q.a.okuma ?? 10)) * .03; q.cd = m.tick + D.DU.cd;
        if (est < D.commitThr(q, h, ps)) continue;
        m.st.duel[q.team]++;
        if (m.r() < P) { m.st.duelW[q.team]++; if (m.r() < .6) { take(m, q, `İkili · ${q.name}, ${h.name}'den Çekirdeği aldı`); return; } const a = m.r() * 6.28; m.ball = K.makeBall({ x: h.x, y: h.y, vx: Math.cos(a) * .5, vy: Math.sin(a) * .5, team: h.team, ch: m.ch, from: q, defl: 1 }); m.holder = null; m.fl = null; m.st.loose[h.team]++; ev(m, 'kesme', q.team, `${q.name} dokundu, Çekirdek boşta`); return; }
        q.stun = m.tick + Math.round(D.DU.stun - (q.a.okuma ?? 10) * D.DU.stunOk); q.noTouch = true; ev(m, 'pas', h.team, `${h.name}, ${q.name}'i geçti`);
      }
      if (--m.decT <= 0) {
        const r = D.decide(h, ps, m.ch, m.r, m.tac[h.team]), o = r.best; m.lastDec = { name: h.name, team: h.team, x: h.x, y: h.y, opts: r.opts.slice(0, 4).map(o => ({ kind: o.kind + (o.shot ? ' ' + o.shot : ''), to: o.to, q: o.q ? o.q.name : '', P: o.det ? (o.kind === 'gönder' ? o.det.Pk : o.det.P) : null, v: o.v })) }; m.decT = Math.round((5 + (1 - m.tac[h.team].tempo) * 8 + m.r() * 3) * (m._lite ? 2 : 1)); // tutarken ve sürerken düşünmeye devam eder (tempo sıklığı belirler); kafadaki oyunda daha seyrek (hız)
        if (o.kind === 'sür') { h.driveTo = { x: o.to.x, y: o.to.y }; h.tx = o.to.x; h.ty = o.to.y; h.drive = true; }
        else if (o.kind === 'tut') { h.driveTo = null; h.tx = h.x; h.ty = h.y; if (o.why === 'çek' && h.lastWhy !== 'çek') ev(m, 'pas', h.team, `${h.name} rakibi üstüne çekiyor${o.det.lookTo ? ' · ' + o.det.lookTo.name + ' açılacak' : ''}`); }
        else launch(m, h, o);
        h.lastWhy = o.why || o.kind;
      }
      // sürüş kararı bir sonraki karara kadar sürer (eskiden tek tik sürüp taşıyıcı duruyordu)
      if (h === m.holder && !h.drive) { if (h.driveTo && Math.hypot(h.driveTo.x - h.x, h.driveTo.y - h.y) > .8) { h.tx = h.driveTo.x; h.ty = h.driveTo.y; } else { h.driveTo = null; h.tx = h.x; h.ty = h.y; } }
      return;
    }
    const b = m.ball;
    if (b.done === 'durdu') { let best = null, bd = 1e9; for (const p of ps) { const d = Math.hypot(p.x - b.x, p.y - b.y); if (d < bd) { bd = d; best = p; } } if (bd < K.Q.reach && m.r() < K.ctrlP(b, best, ps)) take(m, best, `${best.name} boştaki Çekirdeği aldı`); }
    else {
      const r = K.stepAll(b, ps, null, null, m.r);
      if (r && r.took) { const fl = m.fl; take(m, r.p, fl && fl.kind === 'gönder' && r.p.role === 'Bekçi' ? `${r.p.name} göndermeyi tuttu` : `${r.p.name} aktarımı kesti`); return; }
      if (r && !r.took && m.fl) { m.fl.end = { x: b.x, y: b.y }; }
      if (b.done && String(b.done).startsWith('kuyu')) { const sc = b.done === 'kuyu-sag' ? 0 : 1; m.score[sc]++; if (b.shotPk != null && m.st.cal) m.st.cal.g++; ev(m, 'sayi', sc, `SAYI · ${b.from ? b.from.name : ''} · ${m.score[0]}–${m.score[1]}`); kickoff(m, 1 - sc); }
      else if (b.done === 'durdu' && m.fl) { m.fl.end = { x: b.x, y: b.y }; }
    }
    for (const p of ps) p.drive = false;
    if (m.tick >= m.len) m.over = true;
  }
  function run(m) { while (!m.over) step(m); return m; }
  window.AlanMatch = { TAC0, BLOK, GEN, createMatch, step, run, position, move, slotPos };
})();
