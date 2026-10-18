// oz-kopya2.js özeti: kol başına (4 / 10 / 16) ortalamalar; A takım 0. Aynı an ve zarlar, fark yalnız özellikten.
// Kullanım: node tools/oz-kopya2-ozet.js <klasör>
const fs = require('fs'), path = require('path'); const dir = process.argv[2]; const T = {};
for (const f of fs.readdirSync(dir)) { if (!f.endsWith('.jsonl')) continue; for (const l of fs.readFileSync(path.join(dir, f), 'utf8').split('\n')) { let o; try { o = JSON.parse(l); } catch (e) { continue; }
  const t = T[o.oz] = T[o.oz] || {}; for (const [v, rs] of Object.entries(o.kol)) { const a = t[v] = t[v] || { n: 0, nA: 0, pas: 0, pasOk: 0, sutA: 0, sutB: 0, golA: 0, golB: 0, kay: 0, sonA: 0, sonB: 0 };
    for (const r of rs) { a.n++; if (o.kim === 0) a.nA++; a.pas += r.pas; a.pasOk += r.pasOk; a.sutA += r.sut[0]; a.sutB += r.sut[1]; a.golA += r.gol[0]; a.golB += r.gol[1]; a.kay += r.kay; if (r.son === 0) a.sonA++; if (r.son === 1) a.sonB++; } } } }
console.log('| özellik | değer | kol | pas isabeti A | şut A / rakip (100 kolda) | gol A − rakip (100 kolda) | top kaybı (100 kolda) | ufkun sonunda top A / rakip |'); console.log('|---|---|---|---|---|---|---|---|');
for (const [oz, t] of Object.entries(T)) for (const v of Object.keys(t).sort((a, b) => a - b)) { const a = t[v], h = x => (100 * x / a.n).toFixed(1);
  console.log(`| ${oz} | ${v} | ${a.n} | ${(100 * a.pasOk / Math.max(1, a.pas)).toFixed(1)}% (${a.pas}) | ${h(a.sutA)} / ${h(a.sutB)} | ${(100 * (a.golA - a.golB) / a.n).toFixed(1)} | ${h(a.kay)} | ${h(a.sonA)}% / ${h(a.sonB)}% |`); }
