// Pas havadayken alıcı: ne kadar süre duruyor, topa doğru mu koşuyor, uzaklaşıyor mu; rakip araya girdiyse alıcı ne yapıyordu
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const G = {};
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) { const k = oku(d + '/' + f), N = k.oyuncular.length, L = k.LEN, tm = i => k.oyuncular[i].takim;
  for (const p of k.pas) { if (p.tur === 'gönder' || p.tur === 'kendine' || p.alici == null || p.kim == null) continue; let u = p.t + 1; while (u < L && k.T.hold[u] === 255) u++; if (u >= L || u - p.t < 6) continue;
    const a = p.alici, h = k.T.hold[u], sonuc = h === a ? 'alıcı aldı' : tm(h) === tm(p.kim) ? 'arkadaş' : 'rakip aldı'; let dur = 0, topa = 0, ucus = 0, hiz = 0;
    for (let w = p.t + 2; w < u; w++) { ucus++; const vx = k.P.vx[w * N + a] / 1000, vy = k.P.vy[w * N + a] / 1000, v = Math.hypot(vx, vy); hiz += v; if (v < .05) { dur++; continue; }
      const bx = k.T.bx[w] / 100 - k.P.x[w * N + a] / 100, by = k.T.by[w] / 100 - k.P.y[w * N + a] / 100, bl = Math.hypot(bx, by) || 1; if ((vx * bx + vy * by) / (v * bl) > .5) topa++; }
    const key = p.tur + ' · ' + sonuc, g = (G[key] = G[key] || { n: 0, dur: 0, topa: 0, ucus: 0, hiz: 0, cogDur: 0 }); g.n++; g.dur += dur; g.topa += topa; g.ucus += ucus; g.hiz += hiz; if (dur > ucus / 2) g.cogDur++; } }
console.log('pas · sonuç'.padEnd(26), 'n'.padStart(5), ' uçuş (tik)', ' alıcı duruyor', ' topa doğru koşuyor', ' ort. hız', ' uçuşun yarısından çoğunda duran');
for (const [k2, g] of Object.entries(G).sort()) console.log(k2.padEnd(26), String(g.n).padStart(5), (g.ucus / g.n).toFixed(0).padStart(11), `%${(100 * g.dur / g.ucus).toFixed(0)}`.padStart(14), `%${(100 * g.topa / g.ucus).toFixed(0)}`.padStart(19), (g.hiz / g.ucus).toFixed(2).padStart(9), `%${(100 * g.cogDur / g.n).toFixed(0)}`.padStart(31));
