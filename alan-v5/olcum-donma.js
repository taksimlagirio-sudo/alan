// Donma ölçümü (Claude Code'un ölçüsüyle aynı tanım): taşıyıcı topu 2 sn'den (40 tik) uzun tuttuğu her tikte
// duran hücumcu (6 saha oyuncusundan taşıyıcı hariç), 1 sn'den uzun duran hücumcu, duran savunmacı, açık pas yolu (laneOk > 0,5).
// "Durmak": hız < 0,03. Kullanım: await olcDonma(M, D, K, seed, rakipIndex, tikSiniri, msSiniri)
function olcDonma(M, D, K, seed, ri, maxT, maxMs) {
  const R = K.RAKIP[ri % K.RAKIP.length], ids = ['aras', 'deniz', 'mert', 'ece', 'cansu', 'kaan', 'tuna'];
  const tacL = { sistem: 'Alan', pres: 1, arkada: 1, blok: 'Orta', genislik: 'Normal', tempo: .5, risk: .5, kazaninca: 'Dengeli' };
  const m = M.createMatch(seed, { tac: [tacL, { ...R.tac, yz: true }], len: 5400, diz: [{ sav: '2-2-2', huc: '2-1-3' }, R.diz] });
  const oy = [ids.map(id => { const p = K.KADRO.find(k => k.id === id); return { ad: p.ad, a: p.a }; }), R.oyuncular.map(([ad, a]) => ({ ad, a }))];
  for (const t of [0, 1]) for (const p of m.ps) if (p.team === t) { const o = oy[t][p.i]; if (o) { p.name = o.ad; p.a = { ...p.a, ...o.a }; } }
  M.assignRoles(m, [[{ h: 'Kurucu', s: 'Bölge' }, { h: 'Serbest', s: 'Bölge' }, { h: 'Serbest', s: 'Bölge' }, { h: 'Kanat', s: 'Bölge' }, { h: 'Pivot', s: 'Bölge' }, { h: 'Serbest', s: 'Bölge' }], R.roller]);
  let ph = null, hT = 0, n = 0, dA = 0, dA1 = 0, dD = 0, ac = 0, tot = 0, pas = 0, lp = null; const still = new Map(), T0 = performance.now();
  while (performance.now() - T0 < maxMs && m.tick < maxT && !m.over) {
    M.step(m); const h = m.holder; if (h !== ph) { ph = h; hT = m.tick; }
    for (const p of m.ps) { const v = Math.hypot(p.vx || 0, p.vy || 0); if (v < .03) { if (!still.has(p)) still.set(p, m.tick); } else still.delete(p); }
    if (m._pas && m._pas !== lp) { lp = m._pas; pas++; }
    if (!h || h.role === 'Bekçi') continue; tot++; if (m.tick - hT <= 40) continue; n++;
    const A = m.ps.filter(p => p.team === h.team && p !== h && p.role !== 'Bekçi'), Df = m.ps.filter(p => p.team !== h.team && p.role !== 'Bekçi');
    dA += A.filter(p => still.has(p)).length; dA1 += A.filter(p => still.has(p) && m.tick - still.get(p) > 20).length; dD += Df.filter(p => still.has(p)).length; ac += A.filter(q => D.laneOk(m.ps, h.team, h, q) > .5).length;
  }
  return { tik: m.tick, ms: +((performance.now() - T0) / m.tick).toFixed(1), tot, n, dA, dA1, dD, ac, pas, skor: m.score.join('-') };
}
if (typeof window !== 'undefined') window.olcDonma = olcDonma;
