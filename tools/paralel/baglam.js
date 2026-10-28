// Önbellek bağlam denetimi: kayipF hücresi dolduğu bağlamdan farklı bir bağlamda okunursa sayar. Tek çekirdek, belirlenimci.
const fs = require('fs'), path = require('path'); const [dir, lS] = process.argv.slice(2), LEN = +lS || 3000;
const yukle = d => { const s = fs.readFileSync(path.join(d, 'mac-isci.js'), 'utf8'), i = /importScripts\(([^;]*)\)/.exec(s)[1], a = /\[([^\]]*)\]/.exec(i); return (a ? a[1] : i).split(',').map(x => x.trim().replace(/['"]/g, '')).filter(x => x.endsWith('.js')); };
globalThis.window = globalThis; globalThis.self = globalThis;
const SAY = new Map(); globalThis.__BG = (yer, a, b) => { const f = []; for (const k of new Set([...Object.keys(a), ...Object.keys(b)])) if (a[k] !== b[k]) f.push(k + ':' + String(a[k]).slice(0, 12) + '→' + String(b[k]).slice(0, 12)); const k = yer + ' ' + f.map(x => x.split(':')[0]).join(','); const e = SAY.get(k) || { n: 0, ilk: f.join(' '), t: globalThis.__t }; e.n++; SAY.set(k, e); };
for (const f of yukle(dir)) { let s = fs.readFileSync(path.join(dir, f), 'utf8');
  if (f === 'decide.js') {
    const a = "hucre = (X, Y) => { const k = X * 1000 + Y; let v = c.get(k); if (v == null) { v = Math.max(kazanImk(X, Y), self.ALAN_IMK_TABAN_ESKI ? 0 : zeminS(src, 1 - team, X, Y, cc), pasR(X, Y)); c.set(k, v); } return v; };";
    if (s.split(a).length !== 2) throw new Error('hucre bulunamadı');
    s = s.replace(a, "hucre = (X, Y) => { const k = X * 1000 + Y, sg = __SIG(); let v = c.get(k); if (v == null) { v = Math.max(kazanImk(X, Y), self.ALAN_IMK_TABAN_ESKI ? 0 : zeminS(src, 1 - team, X, Y, cc), pasR(X, Y)); c.set(k, v); c.set('s' + k, sg); } else { const s0 = c.get('s' + k); if (s0 && __SIGK(s0) !== __SIGK(sg)) globalThis.__BG('kayipF.hucre', s0, sg); } return v; };");
    const b = "let SAH_F = null;";
    if (s.split(b).length !== 2) throw new Error('SAH_F bulunamadı');
    s = s.replace(b, b + " const __SID = new WeakMap(); let __sidN = 0; const __SIG = () => { const r = { gam: gam(), ofs: IMK_OFS, sah: SAH_F ? (__SID.get(SAH_F) || (__SID.set(SAH_F, ++__sidN), __sidN)) : 0 }; for (const k in D) if (k[0] === '_' && k !== '_tick' && k !== '_mac' && k !== '_kanca' && k !== '_dalgada') { const v = D[k], t = typeof v; r[k] = v == null ? v : t === 'object' || t === 'function' ? 'nesne' : v; } return r; }, __SIGK = o => JSON.stringify(o); globalThis.__SIGG = () => ({ gam: gam(), ofs: IMK_OFS, sah: SAH_F ? (__SID.get(SAH_F) || (__SID.set(SAH_F, ++__sidN), __sidN)) : 0 });");
  }
  (0, eval)(s + '\n//# sourceURL=' + f); }
if (process.env.GENEL) { const MS = new WeakMap(), g0 = Map.prototype.get, s0 = Map.prototype.set; let ic = false;
  Map.prototype.set = function (k, v) { if (!ic && globalThis.__SIGG && globalThis.__t >= 0) { ic = true; try { let T = MS.get(this); if (!T) { T = new Map(); MS.set(this, T); } s0.call(T, k, JSON.stringify(globalThis.__SIGG())); } finally { ic = false; } } return s0.call(this, k, v); };
  Map.prototype.get = function (k) { const v = g0.call(this, k); if (!ic && v !== undefined && globalThis.__SIGG && globalThis.__t >= 0) { ic = true; try { const T = MS.get(this); const a = T && g0.call(T, k); if (a) { const b = JSON.stringify(globalThis.__SIGG()); if (a !== b) { const st = new Error().stack.split('\n').slice(2, 4).map(x => x.trim().replace(/^at /, '').replace(/\(.*\/([^/]+:\d+):\d+\)/, '$1')).join(' < '); const A = JSON.parse(a), B = JSON.parse(b); globalThis.__BG('Map ' + st, A, B); } } } finally { ic = false; } } return v; }; }
const M = globalThis.AlanMatch, k = JSON.parse(fs.readFileSync(process.env.KAYIT, 'utf8')), d = k.yeni;
const m = M.createMatch(d.tohum, { tac: d.tac, len: d.len || 9000, diz: d.diz, devre: d.devre ?? false });
if (d.oyuncular) for (const t of [0, 1]) { const A = d.oyuncular[t]; if (!A) continue; for (const p of m.ps) { if (p.team !== t) continue; const o = A[p.i]; if (o) { p.name = o.ad; p.a = { ...p.a, ...o.a }; } } }
if (d.roller) M.assignRoles(m, d.roller); if (d.diz) for (const t of [0, 1]) if (d.diz[t]) M.dizUygula(m, t, d.diz[t]); M.kafaAc(m);
while (!m.over && m.tick < LEN) { M.step(m); globalThis.__t = m.tick; }

for (const [k, e] of [...SAY].sort((a, b) => b[1].n - a[1].n)) console.log(e.n, k, '| ilk tik', e.t, '|', e.ilk);
