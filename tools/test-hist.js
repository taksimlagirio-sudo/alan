// Hafıza testi: her gerçek tikten sonra oyuncunun geçmişindeki son kayıt, bir önceki gerçek konumu olmalı (hayali maç yazmamalı).
// Kullanım: node tools/test-hist.js <tools/load.js tam yolu> <motor klasörü>
const { load } = require(process.argv[2]); const g = load(process.argv[3], 1), M = g.AlanMatch, D = g.AlanDecide;
const m = M.createMatch(1, { len: 1500 }); let checked = 0, bad = 0, maxErr = 0, ex = null;
// gerçek tik: position() geçmişe şu anki konumu ekler; sonraki gerçek tike kadar geçmişin son kaydı, oyuncunun bir önceki gerçek konumu olmalı
let prev = null;
while (m.tick < 1500) {
  const before = m.ps.map(p => ({ x: p.x, y: p.y }));
  M.step(m);
  for (let i = 0; i < m.ps.length; i++) { const p = m.ps[i], h = p.hist; if (!h || h.length < 2) continue; const last = h[h.length - 1]; checked++; const err = Math.hypot(last.x - before[i].x, last.y - before[i].y); if (err > 1e-9) { bad++; if (err > maxErr) { maxErr = err; ex = { tick: m.tick, name: p.name, err: err.toFixed(2) }; } } }
}
console.log(JSON.stringify({ checked, bad, pct: (bad / checked * 100).toFixed(1) + '%', maxErr: maxErr.toFixed(2), ex }));
