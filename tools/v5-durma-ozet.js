// v5-durma.js özeti: iki sürüm, aynı senaryolar. Kullanım: node tools/v5-durma-ozet.js <klasör> <eski> <yeni>   (dosyalar durma-<sürüm>-<sc>.json)
const fs = require('fs'), [dir, A, B] = process.argv.slice(2);
const oku = v => { const R = {}; for (const f of fs.readdirSync(dir)) { const m = f.match(new RegExp(`^durma-${v}-(\\d+)\\.json$`)); if (!m) continue; const s = fs.readFileSync(dir + '/' + f, 'utf8'); if (!s.trim().startsWith('{')) continue; R[m[1]] = JSON.parse(s); } return R; };
const RA = oku(A), RB = oku(B), ort = Object.keys(RA).filter(k => RB[k]);
const ol = (R, f) => { const a = ort.map(k => f(R[k])).filter(x => x != null); return a.length ? a.reduce((s, x) => s + x, 0) / a.length : null; };
const sat = (ad, f, d = 2) => { const a = ol(RA, f), b = ol(RB, f); console.log(`| ${ad} | ${a == null ? '-' : a.toFixed(d)} | ${b == null ? '-' : b.toFixed(d)} |`); };
console.log(`Ortak senaryo: ${ort.length} (${ort.sort((a, b) => a - b).join(',')})\n\n| | ${A} | ${B} |\n|---|---|---|`);
sat('Taşıyıcı 2 sn+ tutuyor (sahiplik payı %)', r => r.uzunTutus.pay * 100, 1);
sat('  o anlarda duran hücumcu (/5)', r => r.uzunTutus.hucDuran); sat('  1 sn+ duran hücumcu', r => r.uzunTutus.hucUzunDuran); sat('  duran savunmacı (/6)', r => r.uzunTutus.savDuran); sat('  açık pas yolu (laneOk ≥ 0,7)', r => r.uzunTutus.acikYol);
sat('Her an: duran hücumcu', r => r.hucDuran); sat('Her an: duran savunmacı', r => r.savDuran); sat('Her an: açık pas yolu', r => r.acikYol);
sat('Hücumcu yeni hedef / dk', r => r.salinim.huc.hedefDk, 0); sat('  hedefe varma %', r => r.salinim.huc.varma * 100, 0); sat('  3 sn içinde eski hedefe dönüş %', r => r.salinim.huc.geriDonus * 100, 0);
sat('Savunmacı (bölge/kapatma/markaj) yeni hedef / dk', r => r.salinim.sav.hedefDk, 0); sat('  hedefe varma %', r => r.salinim.sav.varma * 100, 0); sat('  3 sn içinde eski hedefe dönüş %', r => r.salinim.sav.geriDonus * 100, 0);
sat('Sayı / maç', r => r.skor[0] + r.skor[1]); sat('Pas / maç', r => r.pas ? r.pas[0] + r.pas[1] : null, 0); sat('Pas tutma %', r => r.pas ? 100 * (r.pasOk[0] + r.pasOk[1]) / (r.pas[0] + r.pas[1]) : null, 1); sat('Gönderme / maç', r => r.gon ? r.gon[0] + r.gon[1] : null, 1);
sat('Hesap süresi (sn / maç, paralel çalıştı)', r => r.hesapSn, 0);
