// Her sayıda Çekirdeğin ağza girişi: ağız ortasından uzaklık (ağız yarı genişliği 5) ve geliş açısı (arka çizgiye dikle)
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const R = [];
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), W = (k.saha && k.saha.W) || 100, H = (k.saha && k.saha.H) || 50;
  for (const o of k.olay) { if (o.tur !== 'sayi') continue; let t = o.t; while (t > 0 && Math.abs(k.T.bx[t] / 100 - W / 2) < 1 && Math.abs(k.T.by[t] / 100 - H / 2) < 1) t--;
    const x1 = k.T.bx[t] / 100, y1 = k.T.by[t] / 100, x0 = k.T.bx[t - 1] / 100, y0 = k.T.by[t - 1] / 100;
    R.push({ f, t, uz: Math.abs(y1 - H / 2), aci: Math.atan2(Math.abs(y1 - y0), Math.abs(x1 - x0)) * 180 / Math.PI }); }
}
const b = (a, lo, hi) => a.filter(r => r.aci >= lo && r.aci < hi).length;
console.log('sayı', R.length);
console.log('geliş açısı: 0–30°', b(R, 0, 30), '· 30–60°', b(R, 30, 60), '· 60–75°', b(R, 60, 75), '· 75°+', b(R, 75, 91));
console.log('ağız kenarına 1 birimden yakın giren:', R.filter(r => r.uz > 4).length, '· bunların geliş açısı:', R.filter(r => r.uz > 4).map(r => r.aci.toFixed(0) + '°').join(' '));
