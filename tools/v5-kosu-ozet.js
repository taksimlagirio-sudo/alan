// v5-kosu.js özeti: takım başına topsuz hücum ölçüleri, genel ve taktik ayarına göre. Kullanım: node tools/v5-kosu-ozet.js <klasör>
const fs = require('fs'), path = require('path'), dir = process.argv[2]; const T = [];
for (const f of fs.readdirSync(dir)) { if (!f.endsWith('.json')) continue; try { const j = JSON.parse(fs.readFileSync(path.join(dir, f), 'utf8')); for (const x of j.T) T.push({ ...x, ad: j.ad }); } catch (e) {} }
const mean = v => v.length ? v.reduce((s, x) => s + x, 0) / v.length : NaN, pc = v => (100 * v).toFixed(0) + '%';
const S = [['Pas seçeneği: karar anında tutma ihtimali ≥ %70 olan arkadaş', x => x.sec, v => v.toFixed(2)], ['Hiç güvenli pas seçeneği olmayan kararlar', x => x.sec0, pc],
  ['Arkadaşın en yakın savunmacıya mesafesi (birim, en çok 15)', x => x.acik, v => v.toFixed(1)], ['5 birimden açık arkadaş', x => x.acik5, pc], ['Pas hattı açık arkadaş', x => x.hat, pc], ['Hem açık hem hattı açık (gerçek pas seçeneği)', x => x.secAcik, pc],
  ['Kapalıyken 5 birim yakınında açık nokta vardı (kaçırılan boşluk)', x => x.kacir, pc], ['Topun 3+ birim önündeki arkadaş (kişi)', x => x.onde, v => v.toFixed(2)], ['Son savunmacının gerisindeki arkadaş (kişi)', x => x.arka, v => v.toFixed(2)],
  ['Genişlik: arkadaşların en üst–en alt farkı (birim, saha 50)', x => x.gen, v => v.toFixed(1)], ['Bir arkadaşıyla 4 birimden yakın (kümelenme)', x => x.kume, pc], ['Taşıyıcının 5 birim dibindeki arkadaş (kişi)', x => x.dibe, v => v.toFixed(2)],
  ['Koşan arkadaş', x => x.kos, pc], ['Koşunun hedefi şimdikinden açık', x => x.kosAcik, pc], ['Koşunun hedefi daha ileride', x => x.kosIleri, pc]];
console.log(`${T.length} takım-maç (${T.length / 2} maç, zip32)\n\n| | ortalama |\n|---|---|`); for (const [ad, f, b] of S) console.log(`| ${ad} | ${b(mean(T.map(f)))} |`);
const grup = (baslik, kf, sira) => { const ks = sira.filter(k => T.some(x => kf(x) === k)); console.log(`\n**${baslik}**\n\n| | ${ks.map(k => `${k} (${T.filter(x => kf(x) === k).length})`).join(' | ')} |\n|---|${ks.map(() => '---').join('|')}|`); for (const [ad, f, b] of S) console.log(`| ${ad} | ` + ks.map(k => b(mean(T.filter(x => kf(x) === k).map(f)))).join(' | ') + ' |'); };
grup('Genişlik ayarına göre', x => x.tac.genislik, ['Dar', 'Normal', 'Geniş']);
grup('Tempoya göre', x => x.tac.tempo <= .3 ? 'tempo ≤ 0,3' : x.tac.tempo >= .7 ? 'tempo ≥ 0,7' : 'tempo 0,4–0,6', ['tempo ≤ 0,3', 'tempo 0,4–0,6', 'tempo ≥ 0,7']);
