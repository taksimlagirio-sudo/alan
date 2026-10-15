// "Kesin" (P ≥ 0,99) paslardan kaybedilenlerin sebebi: araya giren (top alıcıya varmadan rakip aldı), alıcı kaçırdı (top alıcının 2,5 birimine geldi ama alamadı, sonra rakip), sekme / başka
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const C = {}, T = {}; let n = 0, kay = 0;
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) { const k = oku(d + '/' + f), N = k.oyuncular.length, L = k.LEN, tm = i => k.oyuncular[i].takim;
  for (const p of k.pas) { if (p.tur === 'gönder' || p.P == null || p.P < .99 || p.kim == null || p.alici == null) continue; n++; let u = p.t + 1, yak = 1e9, yakT = -1; const a = p.alici;
    while (u < L && k.T.hold[u] === 255) { const dd = Math.hypot(k.T.bx[u] / 100 - k.P.x[u * N + a] / 100, k.T.by[u] / 100 - k.P.y[u * N + a] / 100); if (dd < yak) { yak = dd; yakT = u; } u++; } if (u >= L) continue; const h = k.T.hold[u]; if (tm(h) === tm(p.kim)) continue; kay++;
    const ara = k.olay.filter(o => o.t > p.t && o.t <= u && /çarptı|sekti|dokundu/.test(o.metin)).length;
    const neden = yak < 2.5 && yakT < u ? (ara ? 'alıcıya geldi, sekti, rakip aldı' : 'alıcıya geldi, alamadı, rakip aldı') : ara ? 'yolda sekti / çarptı, rakip aldı' : 'alıcıya varmadan rakip kesti';
    C[neden] = (C[neden] || 0) + 1; const t2 = p.tur; T[t2] = T[t2] || { n: 0, k: 0 }; T[t2].k++; }
  for (const p of k.pas) if (p.tur !== 'gönder' && p.P != null && p.P >= .99 && p.alici != null) { T[p.tur] = T[p.tur] || { n: 0, k: 0 }; T[p.tur].n++; } }
console.log(`"kesin" pas ${n} · kaybedilen ${kay} (%${(100 * kay / n).toFixed(1)})`); for (const [a, b] of Object.entries(C).sort((x, y) => y[1] - x[1])) console.log(`  ${a}: ${b} (%${(100 * b / kay).toFixed(0)})`);
console.log('türe göre kayıp:'); for (const [a, b] of Object.entries(T)) console.log(`  ${a}: ${b.k}/${b.n} (%${(100 * b.k / b.n).toFixed(1)})`);
