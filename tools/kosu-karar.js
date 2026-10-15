// Topu koşarak alan oyuncunun ilk kararı: seçilen hamle, "sür" seçeneğinin değeri ve seçilenle farkı; önü açık / kapalı; sonra koşmaya devam etti mi
const { oku } = require('./kayit-oku.js'), fs = require('fs'), dirs = process.argv.slice(2); const R = [];
const tur = k => /^sür/.test(k) ? 'sür' : /^tut/.test(k) ? 'tut' : /^fake/.test(k) ? 'fake' : /^gönder/.test(k) ? 'gönder' : /^(önüne|pas|kenardan|kendine)/.test(k) ? 'pas' : k.split(' ')[0];
for (const d of dirs) for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) { const k = oku(d + '/' + f), N = k.oyuncular.length, L = k.LEN, W = k.saha.W, tm = i => k.oyuncular[i].takim, hz = (t, i) => Math.hypot(k.P.vx[t * N + i], k.P.vy[t * N + i]) / 1000, ileri = (t, x) => t === 0 ? x : W - x;
  for (const p of k.pas) { if (!['önüne', 'pas', 'kenardan'].includes(p.tur) || p.alici == null) continue; let u = p.t + 1; while (u < L && k.T.hold[u] === 255) u++; if (u >= L - 40 || k.T.hold[u] !== p.alici) continue; const h = p.alici; if (hz(Math.max(0, u - 3), h) <= .15) continue;
    const kr = k.karar.filter(x => x.kim === h && x.t >= u - 15 && x.t <= u + 10).pop() || k.karar.find(x => x.kim === h && x.t >= u && x.t < u + 30); if (!kr || !kr.s || !kr.s.length) continue;
    const sec = tur(kr.s[0][0]), sv = kr.s[0][3], surO = kr.s.find(o => tur(o[0]) === 'sür'), surV = surO ? surO[3] : null;
    const hx = k.P.x[u * N + h] / 100, hy = k.P.y[u * N + h] / 100; let acik = true; for (let i = 0; i < N; i++) if (tm(i) !== tm(h) && k.oyuncular[i].rol !== 'Bekçi' && Math.hypot(k.P.x[u * N + i] / 100 - hx, k.P.y[u * N + i] / 100 - hy) < 8 && ileri(tm(h), k.P.x[u * N + i] / 100) > ileri(tm(h), hx)) acik = false;
    R.push({ sec, sv, surV, acik, devam: hz(u + 10, h) > .15 && hz(Math.min(L - 1, u + 30), h) > .12, ilerde: ileri(tm(h), hx) / W }); } }
const pc = (a, b) => b ? (100 * a / b).toFixed(0) + '%' : '–';
for (const [ad, A] of [['hepsi', R], ['önü açık (8 br içinde önde rakip yok)', R.filter(r => r.acik)], ['önü kapalı', R.filter(r => !r.acik)]]) {
  console.log(`${ad}: n ${A.length} · koşmaya devam ${pc(A.filter(r => r.devam).length, A.length)}`);
  for (const s of ['sür', 'pas', 'fake', 'tut', 'gönder']) { const B = A.filter(r => r.sec === s); if (!B.length) continue; console.log(`  ilk karar ${s.padEnd(6)} ${pc(B.length, A.length).padStart(4)} · sonra koşmaya devam ${pc(B.filter(r => r.devam).length, B.length)}`); }
  const S = A.filter(r => r.sec !== 'sür' && r.surV != null && r.sv != null), Y = A.filter(r => r.sec !== 'sür' && r.surV == null);
  const g = S.map(r => r.sv - r.surV).sort((a, b) => a - b), q = x => g.length ? g[Math.floor(g.length * x)].toFixed(3) : '–';
  console.log(`  "sür" seçilmeyen ${S.length + Y.length}: listede (ilk 4) "sür" var ${S.length}, yok ${Y.length} · seçilen − sür değeri: ortanca ${q(.5)} · %25 ${q(.25)} · %75 ${q(.75)} · 0,01'den az fark ${pc(g.filter(x => x < .01).length, g.length)}`);
}
