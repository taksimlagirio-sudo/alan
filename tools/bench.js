// Hız ölçümü: aynı maçı (tohum, tik) her motor için ayrı süreçte K kez oynatır, en iyi süreyi ve tik/sn'yi yazar.
// Kullanım: node tools/bench.js <tik> <tekrar> <motor klasörü>...
const { execFileSync } = require('child_process'), path = require('path');
const [lenS, repS, ...dirs] = process.argv.slice(2), len = +lenS || 600, rep = +repS || 3;
for (const d of dirs) { const t = []; for (let i = 0; i < rep; i++) { const o = JSON.parse(execFileSync('node', [path.join(__dirname, 'trace.js'), d, '1', String(len)]).toString()); t.push(o.ms); }
  const best = Math.min(...t); console.log(`${d}: en iyi ${best} ms · ${(len / best * 1000).toFixed(1)} tik/sn · [${t.join(', ')}]`); }
