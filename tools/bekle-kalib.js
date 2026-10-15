// Bekleme kalibrasyonu (zip63+): ilk seçeneği "tut" olan kararlar. Beklenen kayıp = 1 − Pk × sonraP (Pk: beklemede tutma, sonraP: sonraki hamlenin tutma ihtimali).
// Gerçek kayıp: karardan sonra top rakibe, takımdan biri sonraki hamleyi temiz almadan geçti mi (bekleme süresi T + 60 tik içinde). Sonraki hamle gönderme ise dışarıda bırakılır.
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const R = [];
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), N = k.oyuncular.length, L = k.LEN, tm = i => k.oyuncular[i].takim;
  for (const kr of k.karar) { if (kr.kim == null || !kr.s || !kr.s.length || kr.t >= L) continue; const o = kr.s[0]; if (!/^tut/.test(o[0]) || !o[4]) continue; const hw = o[4]; const [kayip, tik, T, Pk, sonraP, sonra] = hw; if (Pk == null || sonraP == null) continue; if (/gönder/.test(sonra || '')) continue;
    const t = kr.t, h = kr.kim; if (k.T.hold[t] !== h) continue; let en = 1e9; for (let i = 0; i < N; i++) if (tm(i) !== tm(h)) en = Math.min(en, Math.hypot(k.P.x[t * N + i] / 100 - k.P.x[t * N + h] / 100, k.P.y[t * N + i] / 100 - k.P.y[t * N + h] / 100));
    const surS = /^sür/.test(sonra || ''); let ger = 0, ayrildi = false; for (let u = t; u < t + (T || 30) + 60 && u < L; u++) { const g = k.T.hold[u]; if (g === 255) { if (surS) break; ayrildi = true; continue; } if (tm(g) !== tm(h)) { ger = 1; break; } if (ayrildi && g !== h) break; if (ayrildi && g === h) ayrildi = false; } /* sür: kayıp ancak taşıyıcı topu bırakmadan (sürerken) rakibe geçerse; bıraktıysa (pas, gönderme) sonraki hamlenin kaybı sürmeye yazılmaz */
    R.push({ bek: 1 - Pk * sonraP, ger, Pk, sonraP, sonra: (sonra || '?').split(' ')[0], baski: en < 4, pres: k.tac[1 - tm(h)].pres, kopyaKayip: kayip }); } }
const ort = (a, f) => a.length ? (a.reduce((s, x) => s + f(x), 0) / a.length * 100).toFixed(1) : '–';
const sat = (ad, a) => console.log(`${ad.padEnd(28)} n ${String(a.length).padStart(5)} · beklenen kayıp %${ort(a, x => x.bek)} · gerçek %${ort(a, x => x.ger)} · beklemede tutma (Pk) %${ort(a, x => x.Pk)} · sonraki hamle tutma %${ort(a, x => x.sonraP)}`);
sat('hepsi', R); sat('baskı altında (rakip < 4)', R.filter(x => x.baski)); sat('baskısız', R.filter(x => !x.baski));
for (const p of [...new Set(R.map(x => x.pres))].sort()) sat(`rakip pres ${p}`, R.filter(x => x.pres === p));
for (const p of [...new Set(R.map(x => x.pres))].sort()) sat(`  baskı altında, rakip pres ${p}`, R.filter(x => x.pres === p && x.baski));
console.log('beklenen kayba göre dilimler:'); for (const [a, b] of [[0, .05], [.05, .1], [.1, .2], [.2, .4], [.4, 1.01]]) sat(`  beklenen ${a}–${b > 1 ? 1 : b}`, R.filter(x => x.bek >= a && x.bek < b));
console.log('sonraki hamleye göre:'); for (const s of [...new Set(R.map(x => x.sonra))]) { const a = R.filter(x => x.sonra === s); if (a.length >= 20) sat(`  ${s}`, a); }
