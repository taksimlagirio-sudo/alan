// A · tabloyu hafızanın kayıtlarından kurar.
//  A0: eski 2700 hücre, ogrenme.js'in kuralıyla (hücre başına ortalama, α = max(1/n, 0,02), başlangıç n = 3), kayıtlar maç sırasıyla.
//  A : A0 + düzeltme (ek.js hücresi): o hücreye düşen kayıtlarda (gerçek sonuç − A0 değeri) ortalaması, 30 örneklik sıfır önseliyle büzülmüş.
//  Açıklık dilim sınırları verinin üçte birlikleri. Kullanım: node tools/ab/tablo-kur.js <çıktı.json> <kayıt dosyaları...>
const fs = require('fs'), E = require('./ek'), path = require('path');
const [out, ...fl] = process.argv.slice(2); global.window = global; require(path.resolve('alan-v4g/vs-table.js'));
const V0 = window.ALAN_VS, v = Float64Array.from(V0.v), n = Float64Array.from(V0.n); const all = [];
for (const f of fl.sort((a, b) => (+a.match(/(\d+)\.json$/)[1]) - (+b.match(/(\d+)\.json$/)[1]))) { const o = JSON.parse(fs.readFileSync(f, 'utf8')); for (const e of o.ornek) { const i = e[0], y = e[6]; n[i]++; const a = Math.max(1 / n[i], .02); v[i] += a * (y - v[i]); all.push(e); } }
const ac = all.map(e => e[2]).sort((a, b) => a - b), ACK = [ac[Math.floor(ac.length / 3)], ac[Math.floor(2 * ac.length / 3)]];
const s = new Float64Array(36), c = new Float64Array(36); for (const e of all) { const k = E.hucre({ uz: e[1], acik: e[2], bos: e[3] }, ACK); s[k] += e[6] - v[e[0]]; c[k]++; }
const r = Array.from(s, (x, k) => +(x / (c[k] + 30)).toFixed(4));
fs.writeFileSync(out, JSON.stringify({ A0: { v: Array.from(v, x => +x.toFixed(4)) }, A: { v: Array.from(v, x => +x.toFixed(4)), ek: { ACK, r, n: Array.from(c) } } }));
console.log('kayıt', all.length, 'maç', fl.length, 'açıklık sınırları', ACK, '\nr (uzaklık × açıklık × boş):'); for (let iu = 0; iu < 4; iu++) console.log(['<20', '20–35', '35–55', '55+'][iu], [0, 1, 2].map(ia => [0, 1, 2].map(b => { const k = (iu * 3 + ia) * 3 + b; return (r[k] >= 0 ? '+' : '') + r[k].toFixed(2) + '(' + c[k] + ')'; }).join(' ')).join(' | '));
