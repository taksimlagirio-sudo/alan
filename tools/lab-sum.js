// lab.js satırlarını hücre × motor olarak özetler.
const fs = require('fs'), rows = fs.readFileSync(process.argv[2], 'utf8').trim().split('\n').filter(Boolean).map(JSON.parse);
const key = r => r.cell + ' · ' + r.dir.split('/').slice(-2, -1)[0], G = new Map();
for (const r of rows) { const k = key(r); if (!G.has(k)) G.set(k, []); G.get(k).push(r); }
const avg = (a, f) => a.reduce((s, r) => s + f(r), 0) / a.length, f1 = v => v.toFixed(2), pc = v => '%' + Math.round(v * 100);
console.log('hücre · motor | n | sayı/maç (A–B) | toplam | A puan oranı | gönderme (A–B) | pas tutma (A–B) | topa sahip A | kazanma (A–B) | maç süresi');
for (const k of [...G.keys()].sort()) { const a = G.get(k), n = a.length, w = a.filter(r => r.g[0] > r.g[1]).length, d = a.filter(r => r.g[0] === r.g[1]).length, p = (w + d / 2) / n, sd = Math.sqrt(p * (1 - p) / n);
  console.log(`${k} | ${n} | ${f1(avg(a, r => r.g[0]))}–${f1(avg(a, r => r.g[1]))} | ${f1(avg(a, r => r.g[0] + r.g[1]))} | ${pc(p)} ±${Math.round(sd * 196)} (${w}G ${d}B ${n - w - d}M) | ${f1(avg(a, r => r.shot[0]))}–${f1(avg(a, r => r.shot[1]))} | ${pc(avg(a, r => r.passOk[0]) / avg(a, r => r.pass[0]))}–${pc(avg(a, r => r.passOk[1]) / avg(a, r => r.pass[1]))} | ${pc(avg(a, r => r.poss[0] / (r.poss[0] + r.poss[1])))} | ${f1(avg(a, r => r.steal[0]))}–${f1(avg(a, r => r.steal[1]))} | ${(avg(a, r => r.ms) / 1000).toFixed(0)} sn`); }
