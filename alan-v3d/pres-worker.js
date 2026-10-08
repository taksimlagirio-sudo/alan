// Pres pozisyonları: Mavi geriden kuruyor, Turuncu yüksek blokla basıyor. Presin doğruluğunu, presten kaçışı ve presin etkisini ölçer.
self.window = self;
const V = '?v=' + (self.location.search.match(/v=(\w+)/) || [, '1'])[1]; importScripts('core.js' + V, 'shot-table.js' + V, 'decide.js' + V, 'match.js' + V, 'shape.js' + V);
const CARDS = {
  p1: { title: 'Tek presçi', why: 'Turuncu yüksek blokta, taşıyıcıya tek kişi basıyor. Beklenen: tek presçi kolay geçilir ya da yanından oynanır; pres Çekirdeği nadiren kazanır ama taşıyıcıyı hızlandırır.', pres: 1, arkada: 1, x: 16, y: 25, role: 'Kurucu' },
  p2: { title: 'İki presçi', why: 'İki kişi basıyor. Beklenen: biri taşıyıcıya, öbürü en yakın pas hattına. Daha çok kazanır, ama arkada bir arkadaş daha boşta kalır.', pres: 2, arkada: 1, x: 16, y: 25, role: 'Kurucu' },
  p3: { title: 'Üç presçi', why: 'Üç kişi basıyor, arkada bir kişi. Beklenen: geride çok sık kazanır; geçilirse arkası bomboş ve kontra yer.', pres: 3, arkada: 1, x: 16, y: 25, role: 'Kurucu' },
  pk: { title: 'Kanatta sıkıştırma', why: 'Çekirdek çizgi kenarında. İki kişi basıyor, çizgi bir duvar gibi. Beklenen: kenardan sektirme, içeri çıkış ya da çizgi boyunca presin arkasına.', pres: 2, arkada: 1, x: 18, y: 5, role: 'Kanat' }
};
const dirX = 1, KX = 100; // Mavi sağa hücum eder, Turuncu x=100'ü savunur
function play(q) {
  const M = self.AlanMatch, SH = self.AlanShape, c = CARDS[q.card], SA = M.STYLES[q.atk], SD = M.STYLES['Hücumcu'];
  const defTac = { ...SD.tac, blok: 'Yüksek', pres: c.pres, arkada: c.arkada, onde: 0 };
  const m = M.createMatch(q.seed, { tac: [SA.tac, defTac], roles: [SA.roles, SD.roles] });
  const h0 = m.ps.find(p => p.team === 0 && p.rh === c.role) || m.ps.find(p => p.team === 0 && p.role !== 'Bekçi');
  h0.x = c.x; h0.y = c.y; m.holder = h0; m.ball = null; m.ch = .3; m.fl = null;
  for (let t = 0; t < 90; t++) { m.decT = 99; m.tick++; SH.position(m); M.move(m); h0.x = c.x; h0.y = c.y; h0.vx = h0.vy = 0; }
  m.tick = 0; m.decT = 1; m.ra = [{}, {}]; m.events = [];
  const lineX = m.ps.filter(p => p.team === 1 && p.role !== 'Bekçi').map(p => p.x).sort((a, b) => a - b).slice(0, 3).reduce((a, b) => a + b, 0) / 3;
  const R = { lineX, res: 'süre', winT: null, winX: null, escT: null, goal: false, nP: [0, 0], gs: [0, 0], dist: [0, 0], free: [0, 0], kinds: {}, out: {}, beat: 0, reach: 0, rOut: {}, rP: 0, rThr: 0, rCommit: 0 };
  const D = self.AlanDecide, eng = D.DU.engage, open = new Map(); // presçi taşıyıcıya ulaştı: o andan sonra ne oldu?
  const frames = [], names = m.ps.map(p => p.name), teams = m.ps.map(p => p.team), rh = m.ps.map(p => p.rh || (p.role === 'Bekçi' ? 'Bekçi' : '')), rs = m.ps.map(p => p.rs || (p.role === 'Bekçi' ? 'Bekçi' : ''));
  let pend = null, prevH = m.holder, endT = null;
  for (let t = 0; t < 900; t++) {
    const hb = m.holder; let pressed = false, nearD = 1e9;
    if (hb && hb.team === 0) for (const p of m.ps) if (p.team === 1 && p.role !== 'Bekçi') nearD = Math.min(nearD, Math.hypot(p.x - hb.x, p.y - hb.y));
    pressed = nearD < 5;
    if (hb && hb.team === 0) for (const p of m.ps) { if (p.team !== 1 || p.role === 'Bekçi' || p.noTouch) continue; const d = Math.hypot(p.x - hb.x, p.y - hb.y); if (d <= eng && !open.has(p)) { open.set(p, { t: m.tick, h: hb, ev: m.events.length, duel: m.st.duel[1] }); R.reach++; R.rP += D.duelP(p, hb, m.ps); R.rThr += D.commitThr(p, hb, m.ps); } }
    const evN = m.events.length;
    try { M.step(m); } catch (e) { R.res = 'hata: ' + e.message; break; }
    const h = m.holder;
    for (const [p, o] of open) { let k = null; const newEv = m.events.slice(0, m.events.length - o.ev).map(e => e.text);
      if (h && h.team === 1) k = h === p ? 'aldı' : 'arkadaşı aldı';
      else if (newEv.some(t => t.includes(p.name + ' dokundu'))) k = 'dokundu, Çekirdek boşta';
      else if (newEv.some(t => t.includes(p.name + "'i geçti"))) k = 'geçildi';
      else if (!h && m.fl && m.fl.kind) k = 'taşıyıcı pası attı';
      else if (h && h !== o.h) k = 'taşıyıcı pası attı';
      else if (m.tick - o.t > 40) k = 'girmedi, bekledi';
      if (k) { if (m.st.duel[1] > o.duel) R.rCommit++; R.rOut[k] = (R.rOut[k] || 0) + 1; open.delete(p); } }
    // taşıyıcı baskı altındayken bıraktı: ne yaptı?
    if (hb && hb.team === 0 && !h && m.fl && pressed) { const k = m.fl.kind; R.kinds[k] = (R.kinds[k] || 0) + 1; pend = k; }
    if (pend && h) { const o = h.team === 0 ? 'tuttu' : 'kesildi'; R.out[pend] = R.out[pend] || [0, 0]; R.out[pend][0]++; if (o === 'tuttu') R.out[pend][1]++; pend = null; }
    // presin doğruluğu: her 5 tikte, Mavi Çekirdekteyken
    if (h && h.team === 0 && t % 5 === 0) {
      const pr = m.ps.filter(p => p.team === 1 && p.press); R.nP[0] += pr.length; R.nP[1]++;
      const first = pr.sort((a, b) => Math.hypot(a.x - h.x, a.y - h.y) - Math.hypot(b.x - h.x, b.y - h.y))[0];
      if (first) { const ax = first.x - h.x, ay = first.y - h.y, gx = KX - h.x, gy = 25 - h.y, L1 = Math.hypot(ax, ay) || 1, L2 = Math.hypot(gx, gy) || 1; R.gs[1]++; if ((ax * gx + ay * gy) / (L1 * L2) > .6) R.gs[0]++; R.dist[0] += L1; R.dist[1]++; }
      let fr = 0; for (const p of m.ps) { if (p.team !== 0 || p === h || p.role === 'Bekçi') continue; let d = 1e9; for (const o of m.ps) if (o.team === 1 && o.role !== 'Bekçi') d = Math.min(d, Math.hypot(o.x - p.x, o.y - p.y)); if (d > 7) fr++; } R.free[0] += fr; R.free[1]++;
    }
    const hd = m.holder, cp = M.corePos(m), bx = cp.x, by = cp.y, ph = hd ? hd.team : (m.ball ? m.ball.team : 0);
    frames.push([bx, by, hd ? m.ps.indexOf(hd) : -1, ph, m.ch, ...m.ps.flatMap(p => [Math.round(p.x * 10) / 10, Math.round(p.y * 10) / 10])]);
    if (R.escT == null && hd && hd.team === 0 && hd.x > lineX + 3) R.escT = m.tick;
    if (m.score[0] > 0) { R.goal = true; R.res = R.winT == null ? 'SAYI' : 'kaybedip geri kazandı, SAYI'; break; }
    if (m.score[1] > 0) { R.res = 'pres → SAYI yedi'; break; }
    if (R.winT == null && hd && hd.team === 1) { R.winT = m.tick; R.winX = hd.x; R.res = hd.role === 'Bekçi' ? 'kaçtı, Bekçi aldı' : R.escT != null ? 'kaçtı, sonra kaybetti' : hd.x > lineX + 3 ? 'uzun top kesildi' : 'pres kazandı'; endT = m.tick + 180; }
    if (R.escT != null && R.winT == null && m.tick - R.escT > 240) { R.res = 'kaçtı'; break; }
    if (endT != null && m.tick >= endT) break;
  }
  if (R.res === 'süre' && R.escT != null) R.res = 'kaçtı';
  R.beat = m.events.filter(e => e.team === 0 && /geçti/.test(e.text)).length;
  return { R, frames, names, teams, rh, rs, events: m.events.slice().reverse().map(e => ({ t: e.t, text: e.text, team: e.team })) };
}
self.onmessage = e => { const q = e.data; try { self.postMessage({ id: q.id, card: q.card, k: q.k, ok: true, r: play(q) }); } catch (err) { self.postMessage({ id: q.id, card: q.card, k: q.k, ok: false, msg: String(err && err.message || err) }); } };
