// Dönüş vektörleri: gerçek maçta taşıyıcı ve yakın rakipler; Turn.omega, arc, simulate (zarlı/zarsız), choose.
const path = require('path'), fs = require('fs');
const { M, kurMac } = require('./yukle')(path.resolve(process.argv[2]));
const C = self.AlanCore, T = C.Turn, F = new Float64Array(1), U = new BigUint64Array(F.buffer), b = v => { if (v === null || v === undefined) return null; if (typeof v === 'boolean') return v; F[0] = v; return U[0].toString(16); };
const m = M.kafaAc(kurMac(JSON.parse(fs.readFileSync(process.argv[3])).yeni));
const ser = p => ({ id: p.id, team: p.team, bek: p.role === 'Bekçi', a: Object.fromEntries(Object.entries(p.a || {}).filter(([k, v]) => typeof v === 'number').map(([k, v]) => [k, b(v)])), x: b(p.x), y: b(p.y), vx: b(p.vx || 0), vy: b(p.vy || 0), den: b(p.den), en: b(p.en) });
let s = 31; const lcg = () => { s = (s * 1103515245 + 12345) % 2147483648; return s / 2147483648; };
const out = [];
while (m.tick < 4000 && out.length < 300) {
  M.step(m); const h = m.holder; if (!h || m.tick % 4) continue;
  const opps = m.ps.filter(q => q.team !== h.team && Math.hypot(q.x - h.x, q.y - h.y) < 9);
  const a0 = lcg() * 6.28 - 3.14, a1 = lcg() * 6.28 - 3.14;
  const rr = []; const rnd = () => { const v = lcg(); rr.push(b(v)); return v; };
  const sims = [['dolaş', 1], ['dolaş', -1], ['fiske', 1], ['fiske', -1]].map(([mode, dir]) => { const z = T.simulate(h, a0, a1, { mode, dir }, opps, null); return [z.res, b(z.t), b(z.keep)]; });
  const rr2 = []; const rnd2 = () => { const v = lcg(); rr2.push(b(v)); return v; };
  const zs = T.simulate(h, a0, a1, { mode: lcg() < .5 ? 'dolaş' : 'fiske', dir: lcg() < .5 ? 1 : -1 }, opps, rnd2);
  const ch = T.choose(h, a0, a1, opps, rnd);
  out.push({ h: ser(h), opps: opps.map(ser), a0: b(a0), a1: b(a1), om: [b(T.omega(h)), b(T.omega(h, true))], arc: b(T.arc(a0, a1, 1)), sims, rnd: rr, ch: [ch.mode, ch.dir, b(ch.keep), b(ch.cost)], rnd2: rr2, zs: [zs.res, b(zs.t), b(zs.keep)] });
}
fs.writeFileSync(process.argv[4], JSON.stringify(out)); console.log('durum', out.length);
