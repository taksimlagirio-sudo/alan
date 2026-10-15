// Ölçüm 1 · Donuk dünya (K1). Doğal deney: taşıyıcının ilk 4 seçeneğinde "sür" ile "fake / tut" değeri birbirine çok yakın (|fark| < 0,02) olan kararlar.
// Akıl ikisini eşit görüyor; seçim fiilen rastgele. Seçilen hamleye göre gerçekte ne oldu: 2 sn sonra top bizde mi, top ne kadar ilerledi, 3 sn'de gönderme, 5 sn'de sayı.
// Taşıyıcının karar anındaki hızına göre ayrılır. K1 doğruysa: hızlıyken "sür" belirgin daha iyi sonuç verir, yavaşken fark küçük.
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const R = [];
const tur = k => /^sür/.test(k) ? 'sür' : /^tut/.test(k) ? 'dur' : /^fake/.test(k) ? 'dur' : 'diğer';
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) { const k = oku(d + '/' + f), N = k.oyuncular.length, L = k.LEN, W = k.saha.W, tm = i => k.oyuncular[i].takim, ileri = (t, x) => t === 0 ? x : W - x;
  for (const kr of k.karar) { if (kr.kim == null || !kr.s || kr.s.length < 2 || kr.t >= L - 300) continue; const h = kr.kim, t = kr.t; if (k.T.hold[t] !== h || k.oyuncular[h].rol === 'Bekçi') continue;
    const sur = kr.s.find(o => tur(o[0]) === 'sür'), dur = kr.s.find(o => tur(o[0]) === 'dur'); if (!sur || !dur || sur[3] == null || dur[3] == null) continue; if (Math.abs(sur[3] - dur[3]) >= .02) continue;
    const sec = tur(kr.s[0][0]); if (sec === 'diğer') continue; const at = tm(h), v = Math.hypot(k.P.vx[t * N + h], k.P.vy[t * N + h]) / 1000;
    const g = k.T.hold[t + 120], bizde = g === 255 ? (k.T.bt[t + 120] === at ? 1 : 0) : (tm(g) === at ? 1 : 0);
    const ilerleme = ileri(at, k.T.bx[t + 120] / 100) - ileri(at, k.T.bx[t] / 100);
    const sut = k.olay.some(o => o.tur === 'gonder' && o.takim === at && o.t > t && o.t <= t + 180) ? 1 : 0, sayi = k.olay.some(o => o.tur === 'sayi' && o.takim === at && o.t > t && o.t <= t + 300) ? 1 : 0;
    let en = 1e9; for (let i = 0; i < N; i++) if (tm(i) !== at && k.oyuncular[i].rol !== 'Bekçi') en = Math.min(en, Math.hypot(k.P.x[t * N + i] / 100 - k.P.x[t * N + h] / 100, k.P.y[t * N + i] / 100 - k.P.y[t * N + h] / 100));
    R.push({ sec, v, bizde, ilerleme, sut, sayi, en, akilFark: kr.s[0][3] - (sec === 'sür' ? dur[3] : sur[3]) }); } }
const o = (a, k2, x = 1) => a.length ? (a.reduce((s, r) => s + r[k2], 0) / a.length * x).toFixed(1) : '–';
console.log(`yakın değerli karar: ${R.length} (sür seçilen ${R.filter(r => r.sec === 'sür').length}, fake/tut seçilen ${R.filter(r => r.sec === 'dur').length})`);
console.log('hız grubu              seçilen   n    2 sn sonra top bizde   ilerleme (br)   3 sn gönderme   5 sn sayı   aklın farkı');
for (const [ad, a, b] of [['yavaş (< 0,08)', 0, .08], ['orta (0,08–0,18)', .08, .18], ['hızlı (> 0,18)', .18, 9]]) for (const s of ['sür', 'dur']) {
  const A = R.filter(r => r.v >= a && r.v < b && r.sec === s); console.log(`${ad.padEnd(20)} ${(s === 'dur' ? 'fake/tut' : 'sür').padEnd(9)} ${String(A.length).padStart(4)}   %${o(A, 'bizde', 100).padStart(5)}               ${o(A, 'ilerleme').padStart(6)}          %${o(A, 'sut', 100).padStart(5)}        %${o(A, 'sayi', 100).padStart(4)}    ${A.length ? (A.reduce((s2, r) => s2 + r.akilFark, 0) / A.length).toFixed(4) : '–'}`); }
console.log('\nönü (rakip) uzaklığına göre, hızlı taşıyıcı:'); for (const [ad, a, b] of [['rakip < 4', 0, 4], ['4–8', 4, 8], ['8+', 8, 99]]) for (const s of ['sür', 'dur']) { const A = R.filter(r => r.v > .18 && r.en >= a && r.en < b && r.sec === s); console.log(`  ${ad.padEnd(10)} ${(s === 'dur' ? 'fake/tut' : 'sür').padEnd(9)} n ${String(A.length).padStart(4)} · bizde %${o(A, 'bizde', 100)} · ilerleme ${o(A, 'ilerleme')} · gönderme %${o(A, 'sut', 100)}`); }
