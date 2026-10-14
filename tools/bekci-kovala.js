// Sayılardan önce (gönderme anından geriye 2,5 sn) yenen takımın Bekçisi bir pası kovalamış mı; açıya göre. Karşılaştırma: kaçan göndermeler.
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const G = {};
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), N = k.oyuncular.length, W = k.saha.W, H = k.saha.H;
  k.olay.forEach((o, oi) => { if (o.tur !== 'gonder') return;
    let s = false; for (let j = oi + 1; j < k.olay.length; j++) { const p = k.olay[j]; if (p.tur === 'fake') continue; s = p.tur === 'sayi' && p.takim === o.takim; break; }
    const t = o.t, bi = k.oyuncular.findIndex(p => p.takim !== o.takim && p.rol === 'Bekçi'), gx = o.takim === 0 ? W : 0;
    const a = Math.atan2(Math.abs(k.T.by[t] / 100 - H / 2), Math.abs(gx - k.T.bx[t] / 100)) * 180 / Math.PI, ad = a < 60 ? '0–60°' : '60–90°';
    let v = false; for (let u = t - 150; u < t - 1; u++) if (u > 0 && k.gorevler[k.P.job[u * N + bi]] === 'kovala') { v = true; break; }
    const key = ad + (s ? ' sayı' : ' kaçan'), r = (G[key] = G[key] || [0, 0]); r[0]++; if (v) r[1]++;
  });
}
for (const [kk, [n, v]] of Object.entries(G).sort()) console.log(kk.padEnd(14), `${v}/${n} (%${(100 * v / n).toFixed(0)}) gönderme öncesi 2,5 sn içinde Bekçi bir pası kovalamış`);
