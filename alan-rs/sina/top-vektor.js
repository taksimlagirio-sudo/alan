// Top fiziği sınama vektörleri: rastgele alan kaynakları ve toplar; K, stepBall dizisi, predict, rollTo, solveLaunch, direk.
// node sina/top-vektor.js ../alan-v5 sina/top-vektor.json
const yukle = require('./yukle'); yukle(require('path').resolve(process.argv[2]));
const C = self.AlanCore, F = new Float64Array(1), U = new BigUint64Array(F.buffer), b = v => { if (v === null || v === undefined) return null; if (typeof v === 'boolean') return v; if (typeof v === 'string') return v; F[0] = v; return U[0].toString(16); };
let s = 777; const r = () => { s = (s * 1103515245 + 12345) % 2147483648; return s / 2147483648; };
const kaynak = n => { const a = []; for (let i = 0; i < n; i++) { const koni = r() < .4, an = r() * 6.283, amA = r() < .5; a.push({ x: r() * 120, y: r() * 60, R: r() < .3 ? undefined : 4 + r() * 8, D: r() < .3 ? undefined : .3 + r() * 1.2, cone: koni, fx: Math.cos(an), fy: Math.sin(an), team: r() < .5 ? 0 : 1, amA, am: amA ? [-1, 0, 1][Math.floor(r() * 3)] : undefined, en: r() < .2 ? undefined : r() }); } return a; };
const E = p => ({ x: b(p.x), y: b(p.y), r: b(p.R || 8), d: b(p.D ?? 1), koni: !!p.cone, fx: b(p.fx), fy: b(p.fy), team: p.team, am_a: !!p.amA, am: b(p.am || 0), en: b(p.en == null ? 1 : p.en) });
const TB = t => ({ team: t.team, x: b(t.x), y: b(t.y), vx: b(t.vx), vy: b(t.vy), e: b(t.e), ch: b(t.ch), alive: t.alive, dead: b(t.dead), t: b(t.t), dist: b(t.dist), walls: b(t.walls), done: t.done || null, direk: b(t.direk || 0), tp: t.tp == null ? null : b(t.tp) });
const out = [];
for (let c = 0; c < 300; c++) {
  const src = kaynak(Math.floor(r() * 12));
  const k = [];
  for (let i = 0; i < 20; i++) { const x = r() * 120, y = r() * 60, tm = r() < .5 ? 0 : 1; k.push([b(x), b(y), tm, b(C.K(src, x, y, tm))]); }
  const an = r() * 6.283, v = r() * 2.5, tm = r() < .5 ? 0 : 1, ch = r();
  const top = C.makeBall({ x: r() * 120, y: r() * 60, vx: Math.cos(an) * v, vy: Math.sin(an) * v, team: tm, ch });
  const b0 = TB(top); const adim = [];
  const tt = Object.assign({}, top, { log: [] }); for (let i = 0; i < 60 && !tt.done; i++) { C.stepBall(tt, src); adim.push(TB(tt)); }
  const pr = C.predict(top, src, undefined, 120);
  const to = { x: r() * 120, y: r() * 60 }, arr = r() * .6;
  const sl = C.solveLaunch(top, to, arr, src, undefined, 2.5);
  const hx = top.x, hy = top.y, L = 3 + r() * 30;
  const rt = C.rollTo(top.x, top.y, top.vx, top.vy, tm, ch, src, hx, hy, L, 200);
  out.push({ src: src.map(E), k, top: b0, adim, pr_n: pr.pts.length, pr_son: TB(pr.end), pr_pts: pr.pts.filter((_, i) => i % 7 === 0).map(p => [b(p.x), b(p.y), b(p.e)]), sl_to: [b(to.x), b(to.y)], sl_arr: b(arr), sl: [b(sl.v0), sl.ok], rt_l: b(L), rt: b(rt) });
}
require('fs').writeFileSync(process.argv[3], JSON.stringify(out));
console.log('vaka', out.length);
