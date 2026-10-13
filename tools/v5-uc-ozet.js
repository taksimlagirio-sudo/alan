// v5-savunma.js çıktılarını birden çok motor için yan yana koyar; ayrıca takım başına "taktik ayarı → sahadaki davranış" tablosu. Kullanım: node tools/v5-uc-ozet.js <klasör> <motor1> <motor2> ...
const fs = require('fs'), path = require('path'); const [dir, ...E] = process.argv.slice(2);
const R = {}; for (const f of fs.readdirSync(dir)) { const m = f.match(/^(.+)-(\d+)-(\d+)\.json$/); if (!m) continue; try { const j = JSON.parse(fs.readFileSync(path.join(dir, f), 'utf8')); (R[m[1]] = R[m[1]] || {})[m[2] + '-' + m[3]] = j; } catch (e) {} }
const keys = Object.keys(R[E[0]] || {}).filter(k => E.every(e => R[e] && R[e][k])), t2 = x => x[0] + x[1], mean = v => v.length ? v.reduce((s, x) => s + x, 0) / v.length : NaN, se = v => { const m = mean(v); return Math.sqrt(v.reduce((s, x) => s + (x - m) ** 2, 0) / Math.max(1, v.length - 1) / v.length); };
const S = [['sayı / maç', r => t2(r.skor), 2], ['gönderme / maç', r => t2(r.st.shot), 1], ['pas / maç', r => t2(r.st.pass), 0], ['pas tutma %', r => 100 * t2(r.st.passOk) / t2(r.st.pass), 1], ['top kapma / maç', r => t2(r.st.steal), 1], ['top el değiştirme / maç', r => r.sv.degis, 1],
  ['taşıyıcıya basan (3 birim içinde)', r => r.sv.pres, 2], ['taşıyıcının 6 birim yakınında savunmacı', r => r.sv.dip, 2], ['son bölgede boş hücumcu', r => r.sv.bos ?? 0, 2], ['top son bölgede (%)', r => 100 * r.sv.sonPay, 1], ['hesap süresi (sn)', r => r.sure, 0]];
console.log(`${keys.length} maç her motorda (aynı kurulum ve tohum); ± = ilk motora göre farkın standart hatası\n\n| | ${E.join(' | ')} |\n|---|${E.map(() => '---').join('|')}|`);
for (const [ad, f, d] of S) { const v0 = keys.map(k => f(R[E[0]][k])); console.log(`| ${ad} | ` + E.map((e, i) => { const v = keys.map(k => f(R[e][k])); return mean(v).toFixed(d) + (i ? ` (±${se(v.map((x, j) => x - v0[j])).toFixed(d)})` : ''); }).join(' | ') + ' |'); }
// takım başına
const T = e => keys.flatMap(k => R[e][k].sv.T || []);
const grup = (ad, kf, vf, d, sira) => { console.log(`\n**${ad}**\n\n| ayar | ${E.join(' | ')} | takım |\n|---|${E.map(() => '---').join('|')}|---|`); const ks = sira || [...new Set(T(E[0]).map(kf))].sort(); for (const g of ks) { const n = T(E[0]).filter(x => kf(x) === g).length; if (!n) continue; console.log(`| ${g} | ` + E.map(e => mean(T(e).filter(x => kf(x) === g).map(vf)).toFixed(d)).join(' | ') + ` | ${n} |`); } };
grup('Savunmada taşıyıcıya basan (3 birim içinde), pres ayarına göre', x => 'pres ' + x.pres, x => x.presD, 2);
grup('Savunmada taşıyıcıya basan, bloğa göre', x => x.blok, x => x.presD, 2, ['Düşük', 'Orta', 'Yüksek']);
grup('Top kapma / maç, pres ayarına göre', x => 'pres ' + x.pres, x => x.kapma, 1);
const tb = x => x.tempo <= .3 ? 'tempo ≤ 0,3' : x.tempo >= .7 ? 'tempo ≥ 0,7' : 'tempo 0,4–0,6';
grup('Hücumda topun yerinde beklediği anlar (%), tempoya göre', tb, x => 100 * x.dur, 0, ['tempo ≤ 0,3', 'tempo 0,4–0,6', 'tempo ≥ 0,7']);
grup('Pas / maç (takım), tempoya göre', tb, x => x.pas, 1, ['tempo ≤ 0,3', 'tempo 0,4–0,6', 'tempo ≥ 0,7']);
