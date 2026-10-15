// k3-pres.js çıktılarının özeti: kol başına ortalama (zarlar üzerinden), bölgeye / uzaklığa / hattan öndeliğe göre
const fs = require('fs'), dir = process.argv[2], pre = process.argv[3] || 'pres-'; const E = [];
for (const f of fs.readdirSync(dir).filter(f => f.startsWith(pre) && f.endsWith('.jsonl'))) for (const l of fs.readFileSync(dir + '/' + f, 'utf8').split('\n')) if (l.trim()) E.push(JSON.parse(l));
const ort = (a, k) => a.reduce((s, x) => s + x[k], 0) / a.length;
const K = ['A', 'B', 'C'], AD = { A: 'kural (basar)', B: 'bu presçi basmaz', C: 'kimse basmaz' };
const ozet = (ad, L) => { if (!L.length) return; console.log(`${ad} · ${L.length} an`); for (const k of K) { const s = L.map(e => ort(e.kol[k], 'sut')), y = L.map(e => ort(e.kol[k], 'sayi')), w = L.map(e => ort(e.kol[k], 'kazandi')), i = L.map(e => ort(e.kol[k], 'ilerle'));
  const m = a => a.reduce((x, y2) => x + y2, 0) / a.length; console.log(`  ${AD[k].padEnd(18)} 3 sn'de rakip gönderme ${m(s).toFixed(3)} · rakip sayı ${m(y).toFixed(3)} · top kazanıldı %${(100 * m(w)).toFixed(0)} · topun en ileri noktası ${m(i).toFixed(1)}`); }
  const tehl = (e, k) => ort(e.kol[k], 'sut') + 3 * ort(e.kol[k], 'sayi') - .5 * ort(e.kol[k], 'kazandi'); const iyi = { A: 0, B: 0, C: 0 }; for (const e of L) { const b = K.reduce((x, k) => tehl(e, k) < tehl(e, x) ? k : x, 'A'); iyi[b]++; }
  console.log(`  en iyi kol (tehlike = gönderme + 3×sayı − 0,5×kazanma): ${K.map(k => `${AD[k]} %${(100 * iyi[k] / L.length).toFixed(0)}`).join(' · ')}`); };
ozet('hepsi', E);
for (const [a, b, ad] of [[0, .33, 'taşıyıcı kendi üçte birinde'], [.33, .66, 'orta üçte bir'], [.66, 1.01, 'son üçte bir (bizim kaleye yakın)']]) ozet(ad, E.filter(e => e.bolge >= a && e.bolge < b));
for (const [a, b] of [[0, 6], [6, 12], [12, 99]]) ozet(`basmaya başlarken taşıyıcıya ${a}–${b} br`, E.filter(e => e.d0 >= a && e.d0 < b));
for (const [a, b] of [[-99, 15], [15, 30], [30, 999]]) ozet(`presçi hattından ${a < 0 ? '<' : a + '–'}${b > 900 ? '+' : b} br önde`, E.filter(e => e.onde >= a && e.onde < b));
