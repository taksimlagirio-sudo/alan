// Pas alışı ve sürme: pas türüne göre alıcının topu alması, alış anındaki ve sonrasındaki hızı, durması; taşıyıcıların topla koşma payı
const { oku } = require('./kayit-oku.js'), fs = require('fs'), d = process.argv[2]; const G = {}, S = { t: 0, kos: 0, dur: 0, yavas: 0, ileri: 0 }, U = [];
const hz = (k, N, t, i) => Math.hypot(k.P.vx[t * N + i], k.P.vy[t * N + i]) / 1000;
for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), N = k.oyuncular.length, L = k.LEN, W = k.saha.W, tm = i => k.oyuncular[i].takim, ileri = (t, x) => t === 0 ? x : W - x;
  for (const p of k.pas) { if (p.tur === 'gönder' || p.alici == null || p.kim == null) continue; const g = (G[p.tur] = G[p.tur] || { n: 0, alici: 0, arkadas: 0, rakip: 0, sek: 0, v0: 0, v1: 0, v2: 0, dur: 0, hedefUz: 0, nA: 0, kosarak: 0, kosDevam: 0 }); g.n++;
    let u = p.t + 1; while (u < L && k.T.hold[u] === 255) u++; if (u >= L) continue; const h = k.T.hold[u];
    if (k.olay.some(o => o.t > p.t && o.t < u && /çarptı|sekti|dokundu/.test(o.metin))) g.sek++;
    if (h === p.alici) { g.alici++; g.nA++; const v0 = hz(k, N, Math.max(0, u - 3), h), v1 = hz(k, N, Math.min(L - 1, u + 10), h), v2 = hz(k, N, Math.min(L - 1, u + 30), h); g.v0 += v0; g.v1 += v1; g.v2 += v2;
      if (p.to) g.hedefUz += Math.hypot(k.T.bx[u] / 100 - p.to[0] / 100, k.T.by[u] / 100 - p.to[1] / 100);
      let dur = false; for (let w = u; w < u + 20 && w < L && k.T.hold[w] === h; w++) if (hz(k, N, w, h) < .05) { dur = true; break; } if (dur) g.dur++;
      if (v0 > .15) { g.kosarak++; if (v1 > .15 && v2 > .12) g.kosDevam++; } }
    else if (tm(h) === tm(p.kim)) g.arkadas++; else g.rakip++; }
  for (let t = 0; t < L; t++) { const h = k.T.hold[t]; if (h === 255 || k.oyuncular[h].rol === 'Bekçi') continue; S.t++; const v = hz(k, N, t, h); if (v > .15) S.kos++; else if (v < .05) S.dur++; else S.yavas++; if (t > 0 && k.T.hold[t - 1] === h) S.ileri += ileri(tm(h), k.P.x[t * N + h] / 100) - ileri(tm(h), k.P.x[(t - 1) * N + h] / 100); }
  // tutuş uzunluğu (taşıyıcının topu tuttuğu kesintisiz süre)
  let h0 = 255, t0 = 0; for (let t = 0; t <= L; t++) { const h = t < L ? k.T.hold[t] : 255; if (h !== h0) { if (h0 !== 255 && k.oyuncular[h0].rol !== 'Bekçi') U.push(t - t0); h0 = h; t0 = t; } }
}
const pc = (a, b) => b ? (100 * a / b).toFixed(0) + '%' : '–';
console.log('pas türü   n    alıcı aldı  arkadaş  rakip  arada sekme/çarpma  hedefe uzaklık  alırken hız → 10 tik → 30 tik  ilk 20 tikte durdu  koşarak aldı · koşmaya devam etti');
for (const [t, g] of Object.entries(G)) console.log(`${t.padEnd(9)} ${String(g.n).padStart(4)}  ${pc(g.alici, g.n).padStart(10)} ${pc(g.arkadas, g.n).padStart(8)} ${pc(g.rakip, g.n).padStart(6)} ${pc(g.sek, g.n).padStart(19)} ${(g.hedefUz / g.nA).toFixed(1).padStart(15)}  ${(g.v0 / g.nA).toFixed(2)} → ${(g.v1 / g.nA).toFixed(2)} → ${(g.v2 / g.nA).toFixed(2)}   ${pc(g.dur, g.nA).padStart(16)}  ${pc(g.kosarak, g.nA)} · ${pc(g.kosDevam, g.kosarak)}`);
U.sort((a, b) => a - b); const q = x => U[Math.floor(U.length * x)];
console.log(`\ntaşıyıcı (Bekçi hariç) topla: koşuyor (>0,15) ${pc(S.kos, S.t)} · yavaş ${pc(S.yavas, S.t)} · duruyor (<0,05) ${pc(S.dur, S.t)} · tik başına ileri ${(S.ileri / S.t).toFixed(3)}`);
console.log(`tutuş süresi (tik): ortanca ${q(.5)} · %75 ${q(.75)} · %90 ${q(.9)} · 30 tikten uzun ${pc(U.filter(x => x > 30).length, U.length)} · 60+ ${pc(U.filter(x => x > 60).length, U.length)}`);
