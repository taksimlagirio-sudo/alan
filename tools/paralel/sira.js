// Dalganın işleri farklı sırayla hesaplanınca maç aynı mı? SIRA=duz|ters|karisik. Çıktı: her tikte konum özeti.
const fs = require('fs'), path = require('path'); const [dir, lS] = process.argv.slice(2), LEN = +lS || 3000, SIRA = process.env.SIRA || 'yok';
const yukle = d => { const s = fs.readFileSync(path.join(d, 'mac-isci.js'), 'utf8'), i = /importScripts\(([^;]*)\)/.exec(s)[1], a = /\[([^\]]*)\]/.exec(i); return (a ? a[1] : i).split(',').map(x => x.trim().replace(/['"]/g, '')).filter(x => x.endsWith('.js')); };
globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of yukle(dir)) { const s = fs.readFileSync(path.join(dir, f), 'utf8'); (0, eval)(s + '\n//# sourceURL=' + f); }
let w = 0, r0 = 12345; const R = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296;
if (SIRA !== 'yok') self.ALAN_PARALEL = { n: 2, id: 0, aktif: () => true, kapali: () => false, kodla: x => x, coz: x => x,
  dagit(n, hesapla) { w++; const s = [...Array(n).keys()]; if (SIRA === 'ters') s.reverse(); else if (SIRA === 'karisik') for (let i = n - 1; i > 0; i--) { const j = Math.floor(R() * (i + 1)); [s[i], s[j]] = [s[j], s[i]]; } for (const i of s) hesapla(i, true); } };
const M = globalThis.AlanMatch, k = JSON.parse(fs.readFileSync(process.env.KAYIT, 'utf8')), d = k.yeni;
const m = M.createMatch(d.tohum, { tac: d.tac, len: d.len || 9000, diz: d.diz, devre: d.devre ?? false });
if (d.oyuncular) for (const t of [0, 1]) { const A = d.oyuncular[t]; if (!A) continue; for (const p of m.ps) { if (p.team !== t) continue; const o = A[p.i]; if (o) { p.name = o.ad; p.a = { ...p.a, ...o.a }; } } }
if (d.roller) M.assignRoles(m, d.roller); if (d.diz) for (const t of [0, 1]) if (d.diz[t]) M.dizUygula(m, t, d.diz[t]); M.kafaAc(m);
const out = []; while (!m.over && m.tick < LEN) { M.step(m); let h = 0; for (const p of m.ps) h = (h * 31 + Math.round(p.x * 1e4) * 7 + Math.round(p.y * 1e4)) | 0; out.push(m.tick + ' ' + (h >>> 0).toString(36)); }
fs.writeFileSync(process.env.CIKTI, out.join('\n')); console.log(SIRA, 'dalga', w, 'skor', m.score.join('-'));
