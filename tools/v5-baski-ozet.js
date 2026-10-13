// v5-baski.js özeti: pasçının üstündeki baskıya göre pasın sonucu, aklın tahmini ve pasın yönü. Kullanım: node tools/v5-baski-ozet.js lab/baski32/*.jsonl
const fs = require('fs'); const R = process.argv.slice(2).flatMap(f => fs.readFileSync(f, 'utf8').split('\n').filter(Boolean).map(l => JSON.parse(l))).filter(r => r.tut != null);
const mean = a => a.length ? a.reduce((s, x) => s + x, 0) / a.length : NaN, pc = v => isNaN(v) ? '–' : (100 * v).toFixed(0) + '%';
const tab = (ad, gruplar) => { console.log(`\n**${ad}**\n\n| | pas | gerçek tutma | aklın tahmini | geriye pas (2+ birim) | ileri gidiş (ort.) | mesafe (ort.) |\n|---|---|---|---|---|---|---|`);
  for (const [g, f] of gruplar) { const r = R.filter(f); if (!r.length) continue; const P = r.filter(x => x.P != null); console.log(`| ${g} | ${r.length} | ${pc(mean(r.map(x => x.tut)))} | ${pc(mean(P.map(x => x.P)))} | ${pc(mean(r.map(x => x.ileri != null && x.ileri < -2 ? 1 : 0)))} | ${mean(r.filter(x => x.ileri != null).map(x => x.ileri)).toFixed(1)} | ${mean(r.filter(x => x.uz != null).map(x => x.uz)).toFixed(1)} |`); } };
console.log(`${R.length} pas (gönderme hariç; 16 kurulum)`);
tab('Pasçıya en yakın rakibin mesafesine göre', [['2 birimden yakın', x => x.nd < 2], ['2–4', x => x.nd >= 2 && x.nd < 4], ['4–7', x => x.nd >= 4 && x.nd < 7], ['7+ (baskısız)', x => x.nd >= 7]]);
tab('Pasçının 4 birim içindeki rakip sayısına göre', [['0', x => x.n4 === 0], ['1', x => x.n4 === 1], ['2+', x => x.n4 >= 2]]);
tab('Sahanın neresinde (pasçının kendi Kuyu\'suna uzaklığı)', [['kendi üçte biri (< 33)', x => x.x < 33], ['orta (33–67)', x => x.x >= 33 && x.x < 67], ['rakip üçte biri (67+)', x => x.x >= 67]]);
tab('Rakipte görevi pres olan biri var mı', [['pres var', x => x.pr === 1], ['pres yok', x => x.pr === 0]]);
tab('Pas türü', [...new Set(R.map(x => x.k))].map(k => [k, x => x.k === k]));
