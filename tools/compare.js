// İki iz dosyasını karşılaştırır: ilk ayrılan tiki söyler.
const fs = require('fs'); const [a, b] = process.argv.slice(2).map(f => JSON.parse(fs.readFileSync(f)));
const n = Math.min(a.hs.length, b.hs.length); let i = 0; while (i < n && a.hs[i] === b.hs[i]) i++;
console.log(i === n && a.hs.length === b.hs.length ? `AYNI · ${n} tik · ${a.ms} ms → ${b.ms} ms (${(a.ms / b.ms).toFixed(2)}×)` : `AYRILDI · tik ${i + 1} / ${n}`);
process.exit(i === n && a.hs.length === b.hs.length ? 0 : 1);
