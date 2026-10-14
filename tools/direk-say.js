// Direk çarpmaları: serbest Çekirdek direk merkezine (yarıçap + 0,1) kadar yaklaşıp yön değiştirirse çarpma. Sonra 90 tik içinde sayı var mı.
const { oku } = require('./kayit-oku.js'), fs = require('fs'), [d, R0] = [process.argv[2], +(process.argv[3] || .3)];
let n = 0, sayi = 0, mac = 0; const ac = [];
for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), W = k.saha.W, H = k.saha.H, D = [[.5, H / 2 - 5], [.5, H / 2 + 5], [W - .5, H / 2 - 5], [W - .5, H / 2 + 5]]; mac++;
  for (let t = 1; t < k.LEN - 1; t++) { if (k.T.hold[t] !== 255) continue; const x = k.T.bx[t] / 100, y = k.T.by[t] / 100;
    for (const [px, py] of D) if (Math.abs(Math.hypot(x - px, y - py) - R0) < .02) {
      const v0x = x - k.T.bx[t - 1] / 100, v0y = y - k.T.by[t - 1] / 100, v1x = k.T.bx[t + 1] / 100 - x, v1y = k.T.by[t + 1] / 100 - y;
      if (v0x * v1x + v0y * v1y < .9 * Math.hypot(v0x, v0y) * Math.hypot(v1x, v1y)) { n++; ac.push(Math.atan2(Math.abs(v0y), Math.abs(v0x)) * 180 / Math.PI);
        const s = k.olay.find(o => o.t > t && o.t <= t + 90 && o.tur === 'sayi'); if (s && !k.olay.some(o => o.t > t && o.t < s.t && o.tur !== 'fake')) sayi++; t += 5; break; } }
  }
}
console.log(`${mac} maç · direkten sekme ${n} (maç başına ${(n / mac).toFixed(2)}) · direkten sekip giren sayı ${sayi}`);
console.log('çarpan Çekirdeğin geliş açısı (arka çizgiye dikle):', ac.map(a => a.toFixed(0) + '°').join(' '));
