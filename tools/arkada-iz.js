// Arkadaki (süpürücü) adam seçiminin girdileri, maçı birebir yeniden oynatarak: her tikte topun gerisindeki rakiplerin tehlikesi ve kim kimi tuttu.
// Kullanım: node tools/arkada-iz.js <motor> <senaryo> <t0> <t1> <kayıt.json.gz (doğrulama)>
const { kur } = require('./kurulum'), { oku } = require('./kayit-oku.js'); const [dir, sS, a, b, kf] = process.argv.slice(2);
const { m, adim, g } = kur(dir, +sS, 0, { 'shape.js': [["swL.forEach((p, k) => swT.set(p, deepO[k] || null));", "swL.forEach((p, k) => swT.set(p, deepO[k] || null)); if (self.__sw && !m._lite) self.__sw(m, dt, deepA.map(q => [q.name, +dng(q).toFixed(3), +threat(att, q.x, q.y).toFixed(3)]), dMax, swL.map(p => [p.name, (swT.get(p) || {}).name || 'alan']), E);"]] });
const k = oku(kf), N = k.N; const R = [];
g.__sw = (mm, dt, deep, dMax, as, E) => { if (mm !== m || dt !== 0 || mm.tick < +a || mm.tick > +b) return; R.push({ t: mm.tick, deep, esik: +(dMax * .35).toFixed(3), as, top: [+E.x.toFixed(1), +E.y.toFixed(1)] }); };
while (m.tick <= +b) { adim(); if (m.tick === +a) { let d = 0; for (let i = 0; i < N; i++) d = Math.max(d, Math.abs(m.ps[i].x - k.P.x[(m.tick - 1) * N + i] / 100)); console.log('kayıtla fark (birim):', d.toFixed(3)); } }
let son = ''; for (const r of R) { const s = r.as.map(x => x.join('→')).join(' | '); const ch = s !== son; son = s; console.log(`${(r.t / 60).toFixed(2)} ${ch ? '*' : ' '} ${s.padEnd(34)} eşik ${r.esik} · ${r.deep.map(d => d[0] + ' ' + d[1]).join(', ')}`); }
