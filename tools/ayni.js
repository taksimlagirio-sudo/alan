// İki kayıt klasörünü karşılaştırır: hesap süresi ve motor adı dışında her şey eşit mi
const fs = require('fs'), z = require('zlib'), [a, b] = process.argv.slice(2);
const oku = f => { const k = JSON.parse(z.gunzipSync(fs.readFileSync(f))); delete k.hesapSn; delete k.motor; return JSON.stringify(k); };
let n = 0, ok = 0;
for (const f of fs.readdirSync(b).filter(f => f.endsWith('.json.gz'))) {
  if (!fs.existsSync(a + '/' + f)) continue; n++;
  const x = oku(a + '/' + f), y = oku(b + '/' + f);
  if (x === y) ok++; else { let i = 0; while (x[i] === y[i]) i++; console.log(f, 'fark', i, x.slice(i - 60, i + 40)); }
}
console.log('birebir aynı:', ok, '/', n);
