const { kur } = require(process.cwd() + '/tools/kurulum'); const sc = +process.argv[2];
const { m, adim, g } = kur('hiz30/z42', sc, 0); const DUR = { huc: [null, null, [null, null, null, { y: 3 }, null, { y: 47 }]] }; m.tac[0] = { ...m.tac[0], durus: DUR };
const S = g.AlanShape, R = { son: { ev0: 0, ev: 0, tgt: 0, act: 0, n: 0, el: 0 }, orta: { ev0: 0, ev: 0, tgt: 0, act: 0, n: 0 } };
while (m.tick < 2700 && !m.over) { adim(); const h = m.holder; if (!h || h.team !== 0) continue; const x = h.x; const B = x > 67 ? R.son : x > 33 ? R.orta : null; if (!B) continue;
  for (const p of m.ps) { if (p.team !== 0 || (p.i !== 4 && p.i !== 6) || p === h) continue; const E = { x: h.x, y: h.y }; const ev = S.slotBase(m, p, E, true); const sv = m.tac[0].durus; m.tac[0].durus = null; const ev0 = S.slotBase(m, p, E, true); m.tac[0].durus = sv;
    const a = y => Math.abs(y - 25); B.ev0 += a(ev0.y); B.ev += a(ev.y); B.tgt += p.ty != null ? a(p.ty) : a(p.y); B.act += a(p.y); B.n++; if (ev.el) B.el += ev.el; } }
for (const [k, B] of Object.entries(R)) console.log(`sc${sc} ${k}: n ${B.n} · ortadan uzaklık: varsayılan ev ${(B.ev0 / B.n).toFixed(1)} → duruşlu ev ${(B.ev / B.n).toFixed(1)} · seçilen hedef ${(B.tgt / B.n).toFixed(1)} · gerçek konum ${(B.act / B.n).toFixed(1)}` + (B.el != null ? ` · el payı ort ${(B.el / Math.max(1, B.n)).toFixed(2)}` : ''));
