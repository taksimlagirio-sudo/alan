// rollTo'yu eski yolla (makeBall + stepBall döngüsü) rastgele girdilerde karşılaştırır: sonuçlar bit bit aynı olmalı.
const { load } = require('./load'); const g = load(process.argv[2] || 'alan-v3f', 5), K = g.AlanCore, N = +process.argv[3] || 200000;
const hyp = K.hyp; let r = 12345; const rnd = () => (r = (Math.imul(r, 1664525) + 1013904223) >>> 0) / 4294967296;
const old = (h, dx, dy, L, v0, src, ch) => { const b = K.makeBall({ x: h.x, y: h.y, vx: dx * v0, vy: dy * v0, team: h.team, ch, from: h }); for (let t = 0; t < 400 && !b.done; t++) { K.stepBall(b, src); if (hyp(b.x - h.x, b.y - h.y) >= L) return hyp(b.vx, b.vy); } return -1; };
const mkSrc = () => { const n = 14, a = []; for (let i = 0; i < n; i++) a.push({ x: rnd() * 100, y: rnd() * 50, team: i < 7 ? 0 : 1, R: i % 7 === 0 ? 7 : 8, D: i % 7 === 0 ? 1.2 : 1, ...(rnd() < .2 ? { cone: true, fx: Math.cos(i), fy: Math.sin(i) } : {}) }); return a; };
let bad = 0, kinds = { raw: 0, pk: 0, kc: 0 }, hits = 0;
for (let i = 0; i < N; i++) {
  const base = mkSrc(), mode = i % 3, h = { x: rnd() < .1 ? rnd() * 3 : rnd() * 100, y: rnd() * 50, team: rnd() < .5 ? 0 : 1 };
  const mk = () => mode === 0 ? base : mode === 1 ? K.pack(base) : Object.assign(base.slice(), { _kc: new Map() });
  const sA = mk(), sB = mode === 2 ? Object.assign(base.slice(), { _kc: new Map() }) : sA;
  const a = rnd() * 6.283, dx = Math.cos(a), dy = Math.sin(a), ch = rnd(), L = rnd() * 60;
  for (let j = 0; j < 3; j++) { const v0 = .05 + rnd() * 3.2, o = old(h, dx, dy, L, v0, sA, ch), n = K.rollTo(h.x, h.y, dx * v0, dy * v0, h.team, ch, sB, h.x, h.y, L, 400);
    if (o >= 0) hits++; if (!Object.is(o, n)) { bad++; if (bad < 5) console.log('FARK', mode, h, L, v0, o, n); } }
  kinds[['raw', 'pk', 'kc'][mode]]++;
}
console.log(`${N * 3} deneme · ${hits} varış · fark ${bad}`, kinds); process.exit(bad ? 1 : 0);
