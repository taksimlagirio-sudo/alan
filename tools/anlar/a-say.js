const fs = require('fs'), [dir, anF, n] = process.argv.slice(2); const { M, kurMac } = require('./yukle')(dir); const { oku } = require('./graf');
const A = JSON.parse(fs.readFileSync(anF)); const m0 = kurMac(A.kurulum.yeni), m = oku(A.kopya, () => M.kopya(m0.r)); const say = {}; let thv = 0, top = 0; const t0 = Date.now();
for (let i = 0; i < +n; i++) { M.step(m); if (m.holder) { top++; if (m._sec && m._sec.h === m.holder && m.tick - m._sec.t <= 45) thv++; } for (const p of m.ps) { const j = String(p.job || '').split(' · ').slice(0, 2).join(' · '); if (/^(pres|geri geri|ikisini)/.test(j)) say[j] = (say[j] || 0) + 1; } }
console.log('tik', n, 'süre', ((Date.now() - t0) / 1000).toFixed(0), 'sn · top sahipliği tiki', top, '· tehlike okunabilen', thv); console.log(say); process.exit(0);
