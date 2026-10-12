// v5-fake.js çıktılarının özeti: her an için fake (F) ile fake'siz en iyi seçenek (A) aynı zarlarla; farklar anlar üzerinden ortalanır (± standart hata).
// Kullanım: node tools/v5-fake-ozet.js lab/fake/*.jsonl
const fs = require('fs'); const R = process.argv.slice(2).flatMap(f => fs.readFileSync(f, 'utf8').split('\n').filter(Boolean).map(l => JSON.parse(l)));
const pct = x => (x * 100).toFixed(0).padStart(4) + '%', se = a => { const m = a.reduce((s, x) => s + x, 0) / a.length; return Math.sqrt(a.reduce((s, x) => s + (x - m) ** 2, 0) / Math.max(1, a.length - 1) / a.length); }, mean = a => a.reduce((s, x) => s + x, 0) / a.length;
function blok(ad, rs) { if (!rs.length) return; const f = k => rs.map(r => r.F[k]), a = k => rs.map(r => r.A[k]), d = k => rs.map(r => r.F[k] - r.A[k]);
  console.log(`\n## ${ad} (${rs.length} an)`);
  console.log(`| | fake | fake'siz en iyi | fark |\n|---|---|---|---|`);
  for (const [k, n, p] of [['kayip', 'kayıp (2,5 sn)', 1], ['gon', 'gönderme', 1], ['sayi', 'sayı attı', 1], ['yedi', 'sayı yedi', 1], ['il', 'ilerleme (birim)', 0]]) console.log(`| ${n} | ${p ? pct(mean(f(k))) : mean(f(k)).toFixed(1)} | ${p ? pct(mean(a(k))) : mean(a(k)).toFixed(1)} | ${p ? (mean(d(k)) * 100).toFixed(0) + ' ± ' + (se(d(k)) * 100).toFixed(0) : mean(d(k)).toFixed(1) + ' ± ' + se(d(k)).toFixed(1)} |`);
  console.log(`| aklın değeri (v) | ${mean(rs.map(r => r.vF)).toFixed(3)} | ${mean(rs.map(r => r.vA)).toFixed(3)} | ${mean(rs.map(r => r.vF - r.vA)).toFixed(3)} |`);
  console.log(`| aldanma: aklın tahmini → gerçek | ${pct(mean(rs.map(r => r.pf)))} → ${pct(mean(f('ald')))} | | |`); }
blok('Hepsi', R);
blok('Rakip dibinde (< 2,5)', R.filter(r => r.rak < 2.5)); blok('Rakip uzakta (≥ 2,5)', R.filter(r => r.rak >= 2.5));
blok('Kendi yarısı', R.filter(r => r.x < 50)); blok('Rakip yarısı', R.filter(r => r.x >= 50));
for (const k of [...new Set(R.map(r => r.fake))]) blok('Fake türü: ' + k, R.filter(r => r.fake === k));
console.log('\nAlternatif hamleler: ' + Object.entries(R.reduce((o, r) => (o[r.alt.split('→')[0]] = (o[r.alt.split('→')[0]] || 0) + 1, o), {})).map(([k, v]) => k + ' ' + v).join(', '));
console.log('Fake sonrası planlanan: ' + Object.entries(R.reduce((o, r) => (o[r.sonra] = (o[r.sonra] || 0) + 1, o), {})).map(([k, v]) => k + ' ' + v).join(', '));
