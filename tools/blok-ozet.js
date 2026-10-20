// blok-olc.js özeti: her A tarzının Dikine'ye (ya da b'ye) karşı sonucu. Kullanım: node tools/blok-ozet.js <sonuç klasörü>
const fs = require('fs'), path = require('path'); const dir = process.argv[2], L = [];
for (const f of fs.readdirSync(dir)) { if (!f.endsWith('.json')) continue; try { const o = JSON.parse(fs.readFileSync(path.join(dir, f), 'utf8').trim().split('\n').pop()); if (o.ta) L.push(o); } catch (e) {} }
const ms = a => { const n = a.length, m = a.reduce((s, x) => s + x, 0) / n, sd = Math.sqrt(a.reduce((s, x) => s + (x - m) ** 2, 0) / Math.max(1, n - 1)); return [m, sd / Math.sqrt(n), n]; };
const f = (r, d = 2) => `${r[0] >= 0 ? '+' : '−'}${Math.abs(r[0]).toFixed(d)} ±${r[1].toFixed(d)}${Math.abs(r[0]) > 2 * r[1] ? ' *' : ''}`, av = (X, g) => X.reduce((s, o) => s + g(o), 0) / X.length;
const AD = { B_onde: 'Blok, 1 önde', B_hepsi: 'Blok, hepsi geride', T_kale: 'Kale önü', T_kontra: 'Kontra' };
const G = {}; for (const o of L) (G[o.a] = G[o.a] || []).push(o);
console.log(`${L.length} maç\n`);
console.log('| A (rakip Dikine) | maç | G-B-M | gol A / D | şut A / D | xG A / D | xG farkı | geçiş şutu A / D | geçiş xG A / D | top A | en öndeki yeri (f) | ilk pas öndekine | bar A / D | dipte A / D |');
console.log('|---|---|---|---|---|---|---|---|---|---|---|---|---|---|');
for (const k of Object.keys(AD)) { const X = G[k]; if (!X) continue; const g = X.filter(o => o.gol[0] > o.gol[1]).length, b = X.filter(o => o.gol[0] === o.gol[1]).length;
  console.log(`| ${AD[k]} | ${X.length} | ${g}-${b}-${X.length - g - b} | ${av(X, o => o.gol[0]).toFixed(2)} / ${av(X, o => o.gol[1]).toFixed(2)} | ${av(X, o => o.sut[0]).toFixed(1)} / ${av(X, o => o.sut[1]).toFixed(1)} | ${av(X, o => o.ta.xg).toFixed(2)} / ${av(X, o => o.tb.xg).toFixed(2)} | ${f(ms(X.map(o => o.ta.xg - o.tb.xg)))} | ${av(X, o => o.ta.gecis).toFixed(1)} / ${av(X, o => o.tb.gecis).toFixed(1)} | ${av(X, o => o.ta.gecisXg).toFixed(2)} / ${av(X, o => o.tb.gecisXg).toFixed(2)} | ${(100 * av(X, o => o.ta.sahip)).toFixed(0)}% | ${av(X, o => o.ta.ondeF || 0).toFixed(0)} | ${(100 * X.reduce((s, o) => s + o.ta.ilkOnde, 0) / Math.max(1, X.reduce((s, o) => s + o.ta.ilk, 0))).toFixed(0)}% | ${av(X, o => o.ta.en).toFixed(2)} / ${av(X, o => o.tb.en).toFixed(2)} | ${(100 * av(X, o => o.ta.dip)).toFixed(0)}% / ${(100 * av(X, o => o.tb.dip)).toFixed(0)}% |`); }
console.log('\nGol farkı (A − Dikine):'); for (const k of Object.keys(AD)) if (G[k]) console.log(`- ${AD[k]}: ${f(ms(G[k].map(o => o.gol[0] - o.gol[1])))}`);
console.log('\n`*`: fark standart hatanın iki katından büyük. Geçiş şutu: topu kazandıktan sonraki 6 sn içinde. En öndeki: savunma dizilişinin en ileri oyuncusu (top rakipteyken ortalama yeri; 0 kendi Kuyu, 100 rakip Kuyu).');
