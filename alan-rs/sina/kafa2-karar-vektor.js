// Kafa 2 karar vektörleri (birleştirmeden sonra; kafa1, yerleşim ve maç işlevleri gerçek olunca):
// gerçek maçta predictWorld, bekKay, imkan, sahiplikKur, kayipYer ve decide çağrılarından örnekler:
// girdi dünyası (oyuncuların kafanın okuduğu alanları), D'nin `_` alanları, maç görünümü, zar ve çıktı.
// İç çağrıları da yakalamak için decide.js'in kopyasında bu işlevler kayıt sarmalına alınır.
// node sina/kafa2-karar-vektor.js <alan-v5 ya da sade dizin> <kayit.json> sina/kafa2-karar-vektor.json [tik]
const path = require('path'), fs = require('fs'), os = require('os');
const kay = path.resolve(process.argv[2]), TIK = +(process.argv[5] || 900);
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'kafa2k-'));
for (const f of fs.readdirSync(kay)) if (f.endsWith('.js')) fs.copyFileSync(path.join(kay, f), path.join(tmp, f));
const SAR = ['predictWorld', 'bekKay', 'imkan', 'sahiplikKur', 'decide'];
{
  const p = path.join(tmp, 'decide.js'); let s = fs.readFileSync(p, 'utf8');
  for (const f of SAR) {
    const a = 'function ' + f + '(';
    if (!s.includes(a)) throw Error('yok: ' + f);
    s = s.replace(a, 'const ' + f + ' = (...A) => self.__K2(' + JSON.stringify(f) + ', ' + f + '__, A, D);\n    function ' + f + '__(');
  }
  fs.writeFileSync(p, s);
}
const F = new Float64Array(1), U = new BigUint64Array(F.buffer), b = v => { if (v === null || v === undefined) return null; if (typeof v === 'boolean') return v; if (typeof v !== 'number') return String(v); F[0] = v; return U[0].toString(16); };
let M0 = null; const kayit = {}, N = { predictWorld: 40, bekKay: 60, imkan: 40, sahiplikKur: 6, decide: 30 }, P = { predictWorld: .05, bekKay: .05, imkan: .02, sahiplikKur: .5, decide: 1 };
let s0 = 99; const r = () => { s0 = (s0 * 1103515245 + 12345) % 2147483648; return s0 / 2147483648; };
let ic = 0;
const ix = q => { if (!q || !M0) return null; const i = M0.ps.findIndex(z => z.id === q.id && z.team === q.team); return i < 0 ? null : i; };
const nok = o => o && typeof o.x === 'number' ? { x: b(o.x), y: b(o.y) } : null;
const oy = p => ({ id: p.id, team: p.team, i: p.i, role: p.role, name: p.name, a: Object.entries(p.a || {}).filter(([k, v]) => typeof v === 'number').map(([k, v]) => [k, b(v)]),
  x: b(p.x), y: b(p.y), vx: b(p.vx), vy: b(p.vy), tx: b(p.tx), ty: b(p.ty), en: b(p.en), den: b(p.den), R: b(p.R), D: b(p.D), noTouch: !!p.noTouch, press: !!p.press, sprint: !!p.sprint, job: p.job ?? null,
  markRef: ix(p.markRef), tutQ: ix(p.tutQ), kv: nok(p._kv), otx: b(p.otx), oty: b(p.oty), ca: b(p.ca), bekBas: b(p.bekBas), adandi: b(p.adandi), adT: nok(p.adT), yer: !!p._yer,
  slot: p.slot ? { role: p.slot.role, d: b(p.slot.d), l: b(p.slot.l) } : null, tl: p.tl && p.tl.huc ? Object.fromEntries(Object.entries(p.tl.huc).map(([k, v]) => [k, b(v)])) : null, tlSav: p.tl && p.tl.sav ? Object.fromEntries(Object.entries(p.tl.sav).map(([k, v]) => [k, b(v)])) : null, eg: p.eg ? { ps: b(p.eg.ps), rk: b(p.eg.rk) } : null,
  sf: p._sf ? { t0: b(p._sf.t0), f: p._sf.f } : null });
const dun = w => w.map(oy);
const dAlt = D => Object.fromEntries(Object.keys(D).filter(k => k[0] === '_' && k !== '_mac' && k !== '_kuyruk').map(k => [k, typeof D[k] === 'object' && D[k] ? JSON.parse(JSON.stringify(D[k], (kk, v) => typeof v === 'number' ? b(v) : v)) : b(D[k])]));
const sec = o => ({ kind: o.kind, v: b(o.v), q: ix(o.q), to: nok(o.to), shot: o.shot ?? null, prensip: o.prensip ?? null });
globalThis.__K2 = (ad, f, A, D) => {
  if (ic || !M0 || M0.tick < 30 || (kayit[ad] || []).length >= N[ad] || r() > P[ad]) return f(...A);
  ic++;
  try {
    const g = { tick: M0.tick, D: dAlt(D), mac: D._mac ? { tick: D._mac.tick, holder: ix(D._mac.holder), tac: JSON.parse(JSON.stringify(D._mac.tac, (kk, v) => typeof v === 'number' ? b(v) : typeof v === 'function' ? null : v)) } : null };
    let R;
    if (ad === 'predictWorld') { const [src, team, B, T, ca, resp, pasta, tas] = A; Object.assign(g, { src: dun(src), team, B: nok(B), T: b(T), ca: !!ca, resp: resp ? [...resp].map(p => src.indexOf(p)) : null, pasta: !!pasta, tas: nok(tas) }); R = f(...A); g.R = R.map(q => [b(q.x), b(q.y), b(q.vx), b(q.vy)]); }
    else if (ad === 'bekKay') { const [p, B, T] = A; Object.assign(g, { p: oy(p), B: nok(B), T: b(T) }); R = f(...A); g.R = [b(R.x), b(R.y)]; }
    else if (ad === 'imkan') { const [w, team, B, h, ch] = A; Object.assign(g, { src: dun(w), team, B: nok(B), h: h ? oy(h) : null, ch: b(ch) }); R = f(...A); g.R = b(R); }
    else if (ad === 'sahiplikKur') { const [w, team, ch, B0] = A; Object.assign(g, { src: dun(w), team, ch: b(ch), B0: nok(B0) }); R = f(...A); g.R = []; for (let t = 0; t < 2; t++) for (let x = 3; x < 120; x += 13) for (let y = 2; y < 60; y += 9) g.R.push([t, b(x), b(y), b(R(t, x, y))]); }
    else if (ad === 'decide') {
      const [h, src, ch, rnd, tac] = A, zar = []; const rz = () => { const v = rnd(); zar.push(b(v)); return v; };
      Object.assign(g, { src: dun(src), hi: src.indexOf(h), h: oy(h), ch: b(ch), tac: tac ? JSON.parse(JSON.stringify(tac, (kk, v) => typeof v === 'number' ? b(v) : typeof v === 'function' ? null : v)) : null });
      R = f(h, src, ch, rz, tac); g.zar = zar; g.R = { best: R.best ? sec(R.best) : null, opts: R.opts.map(sec), T: R.T, prT: D._mac && D._mac._prT ? { h: b(D._mac._prT.h), m: [...D._mac._prT.m].map(([k, v]) => [k, b(v)]) } : null };
    }
    (kayit[ad] = kayit[ad] || []).push(g);
    return R;
  } finally { ic--; }
};
const { M, kurMac } = require('./yukle')(tmp);
const m = M.kafaAc(kurMac(JSON.parse(fs.readFileSync(process.argv[3])).yeni)); M0 = m;
while (m.tick < TIK) M.step(m);
fs.writeFileSync(process.argv[4], JSON.stringify(kayit));
fs.rmSync(tmp, { recursive: true, force: true });
console.log(Object.entries(kayit).map(([k, v]) => k + ' ' + v.length).join(', '));
