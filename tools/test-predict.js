// predict'i eski yolla (kopya + stepBall döngüsü) rastgele girdilerde karşılaştırır: noktalar, son durum ve olay kaydı bit bit aynı olmalı.
const { load } = require('./load'); const g = load(process.argv[2] || 'alan-v3f', 5), K = g.AlanCore, N = +process.argv[3] || 50000;
const hyp = K.hyp; let r = 777; const rnd = () => (r = (Math.imul(r, 1664525) + 1013904223) >>> 0) / 4294967296;
const old = (b0, src, p, maxT) => { const b = Object.assign({}, b0, { log: [] }); const pts = [{ x: b.x, y: b.y }]; for (let i = 0; i < (maxT || 900) && !b.done; i++) { K.stepBall(b, src, p); pts.push({ x: b.x, y: b.y, e: b.e, sp: hyp(b.vx, b.vy) }); } return { pts, end: b }; };
const mkSrc = () => { const a = []; for (let i = 0; i < 14; i++) a.push({ x: rnd() * 100, y: rnd() * 50, team: i < 7 ? 0 : 1, R: i % 7 === 0 ? 7 : 8, D: i % 7 === 0 ? 1.2 : 1, ...(rnd() < .2 ? { cone: true, fx: Math.cos(i), fy: Math.sin(i) } : {}) }); return a; };
const same = (a, b) => { if (typeof a !== typeof b) return false; if (a && typeof a === 'object') { const ka = Object.keys(a), kb = Object.keys(b); if (ka.length !== kb.length) return false; for (const k of ka) if (!same(a[k], b[k])) return false; return true; } return Object.is(a, b); };
let bad = 0, ev = 0, goals = 0;
for (let i = 0; i < N; i++) {
  const base = mkSrc(), mode = i % 3, a = rnd() * 6.283, v = rnd() * 3.2, edge = rnd() < .3;
  const b0 = K.makeBall({ x: edge ? (rnd() < .5 ? 1 + rnd() * 8 : 91 + rnd() * 8) : rnd() * 100, y: edge ? 20 + rnd() * 10 : rnd() * 50, vx: Math.cos(a) * v, vy: Math.sin(a) * v, team: rnd() < .5 ? 0 : 1, ch: rnd(), from: null });
  if (rnd() < .2) { b0.e = rnd() * .3; } if (rnd() < .1) { b0.alive = false; b0.dead = 3; }
  const mk = () => mode === 0 ? base : mode === 1 ? K.pack(base) : Object.assign(base.slice(), { _kc: new Map() });
  const maxT = [undefined, 180, 200, 240, 600][i % 5], A = old(b0, mk(), null, maxT), B = K.predict(b0, mk(), null, maxT);
  ev += A.end.log.length; if (String(A.end.done).startsWith('kuyu')) goals++;
  if (!same(A, B)) { bad++; if (bad < 4) console.log('FARK', i, mode, JSON.stringify(A.end).slice(0, 300), JSON.stringify(B.end).slice(0, 300)); }
}
console.log(`${N} yörünge · ${ev} olay · ${goals} kuyu · fark ${bad}`); process.exit(bad ? 1 : 0);
