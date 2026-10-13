// Tek değişkenli pres şiddeti deneyi özeti (lab/siddet31): Liman şiddet (0 · 0,5 · 1) × blok, pres 1;  Liman savunmadayken ne oldu. Kullanım: node tools/v5-siddet-ozet.js lab/siddet31
const fs = require('fs'), path = require('path'), dir = process.argv[2]; const R = [];
for (const f of fs.readdirSync(dir)) { const m = f.match(/^s([\d.]+)-(.+)-(\d+)-(\d+)\.json$/); if (!m) continue; try { const j = JSON.parse(fs.readFileSync(path.join(dir, f), 'utf8')); R.push({ p: +m[1], b: m[2], sc: +m[3], j }); } catch (e) {} }
const mean = v => v.length ? v.reduce((s, x) => s + x, 0) / v.length : NaN, B = ['Düşük', 'Orta', 'Yüksek'];
const tab = (ad, f, d) => { console.log(`\n**${ad}**\n\n| | ${B.join(' | ')} | ortalama |\n|---|---|---|---|---|`); for (const p of [0, 0.5, 1]) console.log(`| şiddet ${p} | ` + B.map(b => mean(R.filter(r => r.p === p && r.b === b).map(r => f(r.j))).toFixed(d)).join(' | ') + ' | ' + mean(R.filter(r => r.p === p).map(r => f(r.j))).toFixed(d) + ' |'); console.log('| ortalama | ' + B.map(b => mean(R.filter(r => r.b === b).map(r => f(r.j))).toFixed(d)).join(' | ') + ' | |'); };
console.log(`${R.length} maç (Liman = takım 0; rakip Saat Kulesi ve Demir Blok, 2'şer tohum; hücre başı 4 maç; pres sayısı 1)`);
tab('Liman savunmadayken taşıyıcıya basan (3 birim içinde)', j => j.sv.T[0].presD, 2);
tab('Liman savunmadayken taşıyıcının 6 birim yakınındaki Liman oyuncusu', j => j.sv.T[0].dipD, 2);
tab('Liman top kapma / maç', j => j.sv.T[0].kapma, 1);
tab('İkili / maç (iki takım)', j => j.st.duel[0] + j.st.duel[1], 1);
tab('Liman topu rakip yarısında kazandı (%)', j => 100 * (j.sv.T[0].kazIleri ?? 0), 0);
tab('Rakibin göndermesi / maç', j => j.sv.T[1].gon, 1);
tab('Rakibin sayısı / maç', j => j.sv.T[1].sayi, 2);
tab('Liman sayısı / maç', j => j.sv.T[0].sayi, 2);
