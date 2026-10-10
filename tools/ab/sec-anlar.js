// Değerlendirilecek anlar: her maçtan eşit aralıklı N tane + sahne türlerinden (pres kırma, derin blok, kanat değiştirme) eksik kalanlar. Çıktı: dosya listesi
const fs = require('fs'), path = require('path'), O = require('./ortak');
const [dir, kl, nS] = process.argv.slice(2), N = +nS || 24; const g = O.yukle('alan-v4g', 1), B = g.AlanBeyin, hyp = Math.hypot;
const fl = fs.readdirSync(kl).filter(f => f.endsWith('.bin')), by = {};
for (const f of fl) { const [, mc, t] = f.match(/^a(\d+)_(\d+)\.bin$/); (by[mc] = by[mc] || []).push({ f, t: +t }); }
const tag = f => { const S = B.ac(O.oku(path.join(kl, f)).S), h = S.holder, team = h.team, d1 = team === 0 ? 1 : -1, gx = team === 0 ? 100 : 0; let nd = 1e9, n8 = 0, ah = 0; for (const p of S.ps) { if (p.team === team || p.role === 'Bekçi') continue; const dd = hyp(p.x - h.x, p.y - h.y); nd = Math.min(nd, dd); if (dd < 8) n8++; if ((p.x - h.x) * d1 > 0) ah++; }
  const dist = hyp(gx - h.x, 25 - h.y), uzak = S.ps.some(p => p.team === team && p !== h && p.role !== 'Bekçi' && (p.y - 25) * (h.y - 25) < 0 && Math.abs(p.y - 25) >= 8); return { pres: nd < 3.5 && n8 >= 2, derin: ah >= 5 && dist >= 25 && dist <= 60, kanat: Math.abs(h.y - 25) >= 10 && uzak }; };
const sec = new Set(); for (const k in by) { const a = by[k].sort((x, y) => x.t - y.t); for (let j = 0; j < N && j < a.length; j++) sec.add(a[Math.floor(j * a.length / Math.min(N, a.length))].f); }
const tags = {}; for (const f of fl) tags[f] = tag(f); const say = t => [...sec].filter(f => tags[f][t]).length;
for (const t of ['pres', 'derin', 'kanat']) for (const f of fl) { if (say(t) >= 40) break; if (tags[f][t]) sec.add(f); }
console.error('seçilen', sec.size, 'pres', say('pres'), 'derin', say('derin'), 'kanat', say('kanat'), '(toplam an', fl.length + ')'); console.log([...sec].sort().join('\n'));
