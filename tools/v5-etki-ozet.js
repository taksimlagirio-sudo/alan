// v5-etki.js özeti: her yeni ayarın beklenen etkisi, aynı rakip ve tohumla oynanan karşı kolla. Kullanım: node tools/v5-etki-ozet.js lab/etki33
const fs = require('fs'), path = require('path'), dir = process.argv[2]; const R = {};
for (const f of fs.readdirSync(dir)) { const m = f.match(/^(.+)-(\d)-(\d)\.json$/); if (!m) continue; try { (R[m[1]] = R[m[1]] || []).push(JSON.parse(fs.readFileSync(path.join(dir, f), 'utf8'))); } catch (e) {} }
const mean = a => a.length ? a.reduce((s, x) => s + x, 0) / a.length : NaN, f = (kol, fn) => mean((R[kol] || []).map(fn).filter(x => x != null && !isNaN(x))), d = (x, n = 2) => isNaN(x) ? '–' : x.toFixed(n);
const sat = [], ekle = (ayar, olcu, a, aAd, b, bAd, ok) => sat.push(`| ${isNaN(a) || isNaN(b) ? '✗' : ok(a, b) ? '✓' : '⚠'} | ${ayar} | ${olcu} | ${aAd}: ${d(a)} | ${bAd}: ${d(b)} |`);
ekle('Koşu talimatı (oyuncu 5)', 'taşıyıcıya göre ileri konum', f('kosu0', r => r.oyuncu[5].ileri), 'koşu 0', f('kosu1', r => r.oyuncu[5].ileri), 'koşu 1', (a, b) => b > a + 2);
ekle('Koşu talimatı (oyuncu 5)', 'koştuğu yol (birim)', f('kosu0', r => r.oyuncu[5].yol), 'koşu 0', f('kosu1', r => r.oyuncu[5].yol), 'koşu 1', (a, b) => b > a * 1.05);
ekle('Destek talimatı (oyuncu 3)', 'taşıyıcıya mesafe', f('destek0', r => r.oyuncu[3].mes), 'yakın', f('destek1', r => r.oyuncu[3].mes), 'uzak', (a, b) => b > a + 2);
ekle('Kişisel pres (oyuncu 3)', 'pres görevinde geçen süre', f('pres0', r => r.oyuncu[3].pres), 'pres 0', f('pres1', r => r.oyuncu[3].pres), 'pres 1', (a, b) => b > a * 1.3 + .01);
ekle('Adam adama (oyuncu 2 → rakip 6)', 'tuttuğu rakibe mesafe', f('adam1', r => r.oyuncu[2].mk), 'adam 1', 6, 'eşik', (a, b) => a < b);
ekle('Evre ayarı (sav.kurma: yüksek blok, sert)', 'o evrede presçi', f('taban', r => r.evPres['sav.kurma']), 'yok', f('evreKur', r => r.evPres['sav.kurma']), 'var', (a, b) => b > a * 1.3);
ekle('Evre ayarı (sav.kurma)', 'başka evrede (sav.orta) presçi değişmemeli', f('taban', r => r.evPres['sav.orta']), 'yok', f('evreKur', r => r.evPres['sav.orta']), 'var', (a, b) => Math.abs(b - a) < .25 * Math.max(a, .1) + .05);
ekle('Pres tuzağı', 'tuzak görevi (oyuncu-tik)', f('taban', r => r.J.tuzak), 'yok', f('tuzak', r => r.J.tuzak), 'kenar', (a, b) => b > a + 20);
ekle('Boşluk doldurma (4 → 5)', 'doldurma görevi (oyuncu-tik)', f('kosu1', r => r.J.doldur), 'yok', f('doldur', r => r.J.doldur), 'var', (a, b) => b > a + 20);
ekle('Koşul (berabereyken yüksek blok, sert)', 'etkin süre oranı', f('taban', r => r.kosul), 'yok', f('kosul', r => r.kosul), 'var', (a, b) => b > .1);
ekle('Koşul', 'ortalama presçi (savunmada)', f('taban', r => mean(Object.values(r.evPres))), 'yok', f('kosul', r => mean(Object.values(r.evPres))), 'var', (a, b) => b > a * 1.15);
ekle('Maç planı', 'paslar sol kanada (%)', f('planSag', r => 100 * r.pasY.sol / Math.max(1, r.pasY.sol + r.pasY.sag)), 'sağ', f('planSol', r => 100 * r.pasY.sol / Math.max(1, r.pasY.sol + r.pasY.sag)), 'sol', (a, b) => b > a + 3);
console.log(`Kollar: ${Object.entries(R).map(([k, v]) => k + ' ' + v.length).join(', ')} (zip33, Liman–Saat Kulesi ve Liman–Demir Blok, 2 tohum, 2700 tik)\n\n| | Ayar | Ölçü | Kapalı | Açık |\n|---|---|---|---|---|\n${sat.join('\n')}`);
const ev = {}; for (const r of R.taban || []) for (const [k, v] of Object.entries(r.evre)) (ev[k] = ev[k] || []).push(v); console.log('\nEvre süreleri (Liman, taban): ' + Object.entries(ev).sort().map(([k, v]) => `${k} %${(100 * mean(v)).toFixed(0)}`).join(' · '));
