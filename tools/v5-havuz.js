// İş listesini (her satır: ad \t i \t spec JSON) N paralel süreçte v5-deney.js ile oynatır; her sonucu çıktı dosyasına bir satır ekler. Yarıda kalırsa yeniden çalıştırınca biten işleri atlar.
// Kullanım: node tools/v5-havuz.js <motor> <iş listesi> <çıktı.jsonl> [N=4]
const fs = require('fs'), { spawn } = require('child_process'); const [dir, list, out, nS] = process.argv.slice(2), N = +nS || 4;
const done = new Set(fs.existsSync(out) ? fs.readFileSync(out, 'utf8').split('\n').filter(Boolean).map(l => { const j = JSON.parse(l); return j.ad + '#' + j.i; }) : []);
const jobs = fs.readFileSync(list, 'utf8').split('\n').filter(Boolean).map(l => l.split('\t')).filter(([ad, i]) => !done.has(ad + '#' + i)); let k = 0, run = 0, n = 0; const t0 = Date.now();
function next() { if (k >= jobs.length) { if (!run) console.log(`bitti · ${n} maç · ${((Date.now() - t0) / 60000).toFixed(1)} dk`); return; } const [ad, i, spec] = jobs[k++]; run++;
  const p = spawn('node', [__dirname + '/v5-deney.js', dir, ad, i, spec]); let s = ''; p.stdout.on('data', d => s += d); p.stderr.on('data', d => process.stderr.write(d));
  p.on('close', () => { run--; n++; if (s.trim()) fs.appendFileSync(out, s.trim() + '\n'); if (n % 50 === 0) console.log(`${n}/${jobs.length} · ${((Date.now() - t0) / 60000).toFixed(1)} dk`); next(); }); }
for (let j = 0; j < N; j++) next();
