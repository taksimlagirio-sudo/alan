// İki kayıt klasöründe top ve bütün oyuncuların her tikteki konumu, hızı, hedefi ve skor aynı mı (olay metinleri hariç)
const { oku } = require('./kayit-oku.js'), fs = require('fs'), [a, b] = process.argv.slice(2); let n = 0, ok = 0;
for (const f of fs.readdirSync(b).filter(f => f.endsWith('.json.gz'))) { if (!fs.existsSync(a + '/' + f)) continue; n++;
  const x = oku(a + '/' + f), y = oku(b + '/' + f); let fark = null;
  for (const g of ['T', 'P']) for (const k of Object.keys(x[g])) { if (fark) break; const p = x[g][k], q = y[g][k]; if (!q || p.length !== q.length) { fark = g + '.' + k + ' yok'; break; } for (let i = 0; i < p.length; i++) if (p[i] !== q[i]) { fark = g + '.' + k + ' @' + i; break; } }
  if (!fark && JSON.stringify(x.skor) !== JSON.stringify(y.skor)) fark = 'skor';
  if (fark) console.log(f, 'fark', fark); else ok++; }
console.log('birebir aynı (konum, hız, hedef, görev, skor):', ok, '/', n);
