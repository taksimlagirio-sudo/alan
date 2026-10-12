// Sayfanın kurduğu maç (varsayılan Liman – Demir Blok), işçideki kurulumla. Herkesin hareketsiz kaldığı anları ve yavaş adımları arar.
const path = require('path'), fs = require('fs'); const [dir, sS] = process.argv.slice(2), seed = +sS;
const { load } = require(path.resolve(__dirname, '../tools/load')); const g = load(dir, seed); (0, eval)(fs.readFileSync(path.join(dir, 'kadro.js'), 'utf8'));
const M = g.AlanMatch, K = g.ALAN_KADRO, R = K.RAKIP.find(r => r.ad === 'Demir Blok'), ids = ['aras', 'deniz', 'mert', 'ece', 'cansu', 'kaan', 'tuna'];
const c = { tac: [{ sistem: 'Alan', pres: 1, arkada: 1, blok: 'Orta', genislik: 'Normal', tempo: .5, risk: .5, kazaninca: 'Dengeli' }, R.tac], diz: [{ sav: '2-2-2', huc: '2-1-3' }, R.diz], oyuncular: [ids.map(id => { const p = K.KADRO.find(k => k.id === id); return { ad: p.ad, a: p.a }; }), R.oyuncular.map(([ad, a]) => ({ ad, a }))], roller: [null, R.roller] };
const m = M.createMatch(seed, { tac: c.tac, len: 5400, diz: c.diz });
for (const t of [0, 1]) { const A = c.oyuncular[t]; for (const p of m.ps) if (p.team === t && A[p.i]) { p.name = A[p.i].ad; p.a = { ...p.a, ...A[p.i].a }; } }
M.assignRoles(m, c.roller); for (const p of m.ps) { const r = c.roller[p.team] && c.roller[p.team][p.i - 1]; if (r && r.eg) p.eg = r.eg; } for (const t of [0, 1]) if (c.diz[t]) M.dizUygula(m, t, c.diz[t]);
let still = 0, maxStill = 0, stillAt = -1, maxStep = 0, stepAt = -1, slow = 0; const prev = m.ps.map(p => [p.x, p.y]); const t0 = Date.now(), olay = [];
while (!m.over && m.tick < 5400) { const a = Date.now(); M.step(m); const d = Date.now() - a; if (d > maxStep) { maxStep = d; stepAt = m.tick; } if (d > 500) slow++;
  let mv = 0; m.ps.forEach((p, k) => { mv = Math.max(mv, Math.hypot(p.x - prev[k][0], p.y - prev[k][1])); prev[k] = [p.x, p.y]; });
  if (mv < 0.01) { still++; if (still > maxStill) { maxStill = still; stillAt = m.tick; } } else still = 0; }
console.log(`tohum ${seed} · skor ${m.score} · süre ${((Date.now() - t0) / 1000).toFixed(0)} sn · en uzun hareketsiz ${maxStill} tik (tik ${stillAt}) · en yavaş adım ${maxStep} ms (tik ${stepAt}) · 0,5 sn üstü adım ${slow}`);
