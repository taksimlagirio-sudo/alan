// sut-anatomi.js A/B özeti: dosya adının başındaki kol (ör. yeni-, eski-) ayrı sayılır. Dikine'nin şutlarının nasıl kapatıldığı ve maç sonucu.
// Kullanım: node tools/sut-anatomi-ab.js <sonuç klasörü>
const fs = require('fs'), path = require('path'); const dir = process.argv[2], G = {};
for (const f of fs.readdirSync(dir)) { if (!f.endsWith('.json')) continue; let o; try { o = JSON.parse(fs.readFileSync(path.join(dir, f), 'utf8').trim().split('\n').pop()); } catch (e) { continue; } const k = f.split('-')[0] + ' · ' + o.a; (G[k] = G[k] || []).push(o); }
const pc = (n, d) => d ? (100 * n / d).toFixed(0) + '%' : '–';
console.log('| kol · blok | maç | gol blok / D | D şut | D xG | D gol / xG | çizgide kimse yok | 6 br içinde kimse yok | en yakın (br) | kenardan pasla | uzak (≥ 25 br) | yolda söndü |');
console.log('|---|---|---|---|---|---|---|---|---|---|---|---|');
for (const k of Object.keys(G).sort()) { const L = G[k], n = L.length, S = L.flatMap(o => o.S.filter(s => s.tarz === 'T_dikine')), gB = L.reduce((s, o) => s + o.gol[0], 0) / n, gD = L.reduce((s, o) => s + o.gol[1], 0) / n, xg = S.reduce((s, x) => s + x.pk, 0), gl = S.filter(s => s.son === 'gol').length;
  console.log(`| ${k} | ${n} | ${gB.toFixed(2)} / ${gD.toFixed(2)} | ${(S.length / n).toFixed(1)} | ${(xg / n).toFixed(2)} | ${(gl / xg).toFixed(2)} | ${pc(S.filter(s => s.cizgi === 0).length, S.length)} | ${pc(S.filter(s => s.yakin >= 6).length, S.length)} | ${(S.reduce((s, x) => s + x.yakin, 0) / S.length).toFixed(1)} | ${pc(S.filter(s => s.geldi === 'kenardan').length, S.length)} | ${pc(S.filter(s => s.uz >= 25).length, S.length)} | ${pc(S.filter(s => s.son === 'durdu').length, S.length)} |`); }
