// Top el değiştirince yeni taşıyıcının ilk kararına kadar geçen süre (tik), ve bu sürede hızı
const { oku } = require('./kayit-oku.js'), fs = require('fs'), d = process.argv[2]; const R = { kaz: [], pas: [] }, V = [];
for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) { const k = oku(d + '/' + f), N = k.oyuncular.length, L = k.LEN, tm = i => k.oyuncular[i].takim;
  let sonH = 255, sonT = -1; for (let t = 1; t < L - 200; t++) { const h = k.T.hold[t]; if (h === 255 || h === sonH) { if (h !== 255) sonH = h; continue; }
    const tur = sonT >= 0 && tm(h) !== sonT ? 'kaz' : 'pas'; sonH = h; sonT = tm(h);
    const kr = k.karar.find(x => x.kim === h && x.t >= t); let u = t; while (u < L && k.T.hold[u] === h) u++;
    const ilk = kr && kr.t < u ? kr.t - t : null; R[tur].push(ilk == null ? -1 : ilk);
    if (tur === 'kaz' && ilk != null) { let s = 0; for (let w = t; w < t + ilk; w++) s += Math.hypot(k.P.vx[w * N + h], k.P.vy[w * N + h]) / 1000; V.push(ilk ? s / ilk : 0); } } }
for (const [a, v] of Object.entries(R)) { const ok = v.filter(x => x >= 0).sort((x, y) => x - y), q = p => ok[Math.floor(ok.length * p)];
  console.log(`${a === 'kaz' ? 'top kazanınca' : 'pas alınca  '} n ${v.length} · karar vermeden topu bırakan ${v.filter(x => x < 0).length} · ilk karara kadar: ortanca ${q(.5)} tik, %75 ${q(.75)}, %90 ${q(.9)} · 12+ tik ${ok.filter(x => x >= 12).length}`); }
console.log('top kazanınca ilk karara kadar ort. hız (birim/tik):', (V.reduce((a, b) => a + b, 0) / V.length).toFixed(3));
