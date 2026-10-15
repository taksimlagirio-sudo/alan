// Pres anları: ilk presçinin basmaya başladığı an (görev 'pres' / 'fake: ...' / 'kenara itiyor'); sonuç: top kazanıldı / taşıyıcı pası presçinin arkasına verdi (geçildi) / başka.
// Geçildiyse 3 sn içinde gönderme. Ayrıca basmaya başladığında presçinin taşıyıcıya uzaklığı, kendi hattının ne kadar önüne çıktığı ve sahanın hangi bölgesinde olduğu.
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const E = [];
const presJob = j => /^pres|^fake: |^kenara itiyor/.test(j || '');
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) { const k = oku(d + '/' + f), N = k.oyuncular.length, L = k.LEN, W = k.saha.W, tm = i => k.oyuncular[i].takim, J = (t, i) => k.gorevler[k.P.job[t * N + i]], X = (t, i) => k.P.x[t * N + i] / 100, Y = (t, i) => k.P.y[t * N + i] / 100, ileri = (t, x) => t === 0 ? x : W - x;
  for (let i = 0; i < N; i++) { if (k.oyuncular[i].rol === 'Bekçi') continue; const dt = tm(i); let t = 1;
    while (t < L - 200) { if (!presJob(J(t, i)) || presJob(J(t - 1, i))) { t++; continue; } const h = k.T.hold[t]; if (h === 255 || tm(h) === dt) { t++; continue; }
      const t0 = t, at = tm(h), d0 = Math.hypot(X(t0, i) - X(t0, h), Y(t0, i) - Y(t0, h));
      // presçinin arkadaşlarının (Bekçi hariç) en geri hattı: savunan takımın kalesine en yakın olan
      let hat = 1e9; for (let j = 0; j < N; j++) if (tm(j) === dt && j !== i && k.oyuncular[j].rol !== 'Bekçi') hat = Math.min(hat, ileri(dt, X(t0, j))); const onde = ileri(dt, X(t0, i)) - hat;
      const bolge = ileri(at, X(t0, h)) / W; // taşıyıcının ilerisi (0 kendi kalesi, 1 rakip kale)
      let u = t0, son = null; while (u < t0 + 240 && u < L - 1) { const g = k.T.hold[u]; if (g !== 255 && tm(g) === dt) { son = 'kazandı'; break; } if (g !== h && g !== 255 && tm(g) === at) { // pas alındı
          const gec = ileri(at, X(u, g)) > ileri(at, X(u, i)) + 1; son = gec ? 'arkasına pas' : 'önüne/yana pas'; break; } u++; }
      if (!son) son = 'sürdü / süre doldu';
      const sut = son === 'arkasına pas' && k.olay.some(o => o.tur === 'gonder' && o.takim === at && o.t > u && o.t < u + 180);
      E.push({ son, sut, d0, onde, bolge }); while (t < L && presJob(J(t, i))) t++; } } }
const pc = (a, b) => b ? (100 * a / b).toFixed(0) + '%' : '–', o = (a, k2) => a.length ? (a.reduce((s, e) => s + e[k2], 0) / a.length).toFixed(1) : '–';
console.log(`pres başlangıcı: ${E.length}`); for (const s of ['kazandı', 'arkasına pas', 'önüne/yana pas', 'sürdü / süre doldu']) { const a = E.filter(e => e.son === s); console.log(`  ${s.padEnd(20)} ${pc(a.length, E.length).padStart(4)} · basmaya başlarken taşıyıcıya ${o(a, 'd0')} br · arkadaşlarının en geri hattından ${o(a, 'onde')} br önde`); }
console.log(`arkasına pastan sonra 3 sn içinde gönderme: ${E.filter(e => e.sut).length} (${pc(E.filter(e => e.sut).length, E.filter(e => e.son === 'arkasına pas').length)})`);
console.log('taşıyıcının bölgesine göre (0 kendi kalesi – 1 rakip kalesi):'); for (const [a, b] of [[0, .33], [.33, .66], [.66, 1.01]]) { const A = E.filter(e => e.bolge >= a && e.bolge < b); console.log(`  ${a}–${b > 1 ? 1 : b}: ${A.length} · kazandı ${pc(A.filter(e => e.son === 'kazandı').length, A.length)} · arkasına pas ${pc(A.filter(e => e.son === 'arkasına pas').length, A.length)} · bundan gönderme ${pc(A.filter(e => e.sut).length, A.length)}`); }
console.log('basmaya başlarken uzaklığa göre:'); for (const [a, b] of [[0, 4], [4, 8], [8, 14], [14, 99]]) { const A = E.filter(e => e.d0 >= a && e.d0 < b); console.log(`  ${a}–${b}: ${A.length} · kazandı ${pc(A.filter(e => e.son === 'kazandı').length, A.length)} · arkasına pas ${pc(A.filter(e => e.son === 'arkasına pas').length, A.length)} · bundan gönderme ${pc(A.filter(e => e.sut).length, A.length)}`); }
