// Gönderme tablosunu mevcut ızgarada (mesafe 0–19 × açı 0–7 × şarj 0–6) motorun kendi shotCell'iyle yeniden üretir.
// Kullanım: [BAYRAK=...] node tools/tablo-uret.js <motor> <çıktı.json>   → { surum, cells }
const fs = require('fs'), { load } = require('./load'); const [dir, out] = process.argv.slice(2);
const g = load(dir, 1), D = g.AlanDecide; D.SHT.clear(); const cells = [];
const t0 = Date.now(); for (let di = 0; di < 20; di++) for (let ai = 0; ai < 8; ai++) for (let ci = 0; ci < 7; ci++) { const k = di * 10000 + ai * 100 + ci; cells.push([k, +D.shotCell(di, ai, ci).toFixed(3)]); }
fs.writeFileSync(out, JSON.stringify({ surum: D.SHT_SURUM, cells })); console.log('hücre', cells.length, 'sn', ((Date.now() - t0) / 1000).toFixed(1));
