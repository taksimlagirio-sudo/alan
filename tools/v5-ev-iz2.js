// Duruş ve serbestlik: iki kanat (i 4, 6) son bölge duruşunda çizgide (y 3 / 47). Kanatların serbestliği SERB ile verilir.
// Top Liman'da ve son bölgedeyken: kanatların ortadan uzaklığı (ev, seçilen hedef, gerçek konum); ayrıca top son bölgede en az 1 sn kaldıktan sonraki anlar (oyuncu yerine varmış olsun).
// Kullanım: SERB=0 node tools/v5-ev-iz2.js <motor> <senaryo> [LEN=2700]
const { kur } = require('./kurulum'); const [dir, sS, lS] = process.argv.slice(2), sc = +sS, LEN = +lS || 2700, SERB = process.env.SERB;
const { m, adim, g } = kur(dir, sc, 0); m.tac[0] = { ...m.tac[0], durus: { huc: [null, null, [null, null, null, { y: 3 }, null, { y: 47 }]] } };
for (const p of m.ps) if (p.team === 0 && (p.i === 4 || p.i === 6) && SERB != null) p.tl = { ...(p.tl || {}), huc: { ...((p.tl && p.tl.huc) || {}), serbest: +SERB } };
const S = g.AlanShape, a = y => Math.abs(y - 25), Z = () => ({ ev: 0, tgt: 0, act: 0, n: 0 }), R = { hepsi: Z(), yerlesik: Z(), orta: Z() }; let sonT = -1;
while (m.tick < LEN && !m.over) { adim(); const h = m.holder; if (!h || h.team !== 0) { sonT = -1; continue; } const son = h.x > 67; if (son && sonT < 0) sonT = m.tick; if (!son) sonT = -1;
  const Bs = son ? [R.hepsi].concat(m.tick - sonT >= 60 ? [R.yerlesik] : []) : h.x > 33 ? [R.orta] : []; if (!Bs.length) continue;
  for (const p of m.ps) { if (p.team !== 0 || (p.i !== 4 && p.i !== 6) || p === h) continue; const ev = S.slotBase(m, p, { x: h.x, y: h.y }, true);
    for (const B of Bs) { B.ev += a(ev.y); B.tgt += a(p.ty ?? p.y); B.act += a(p.y); B.n++; } } }
console.log(JSON.stringify({ sc, serb: SERB ?? null, ...Object.fromEntries(Object.entries(R).map(([k, B]) => [k, { n: B.n, ev: +(B.ev / Math.max(1, B.n)).toFixed(1), hedef: +(B.tgt / Math.max(1, B.n)).toFixed(1), gercek: +(B.act / Math.max(1, B.n)).toFixed(1) }])) }));
