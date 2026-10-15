// Seken top yarışı (gol olmayan, Bekçi'nin tutmadığı göndermeler): vuruştan topun yeniden alınmasına kadar,
// savunmacılar ve hücumcular topun düştüğü noktaya ne zaman ve ne hızla koştu. Her taraf için noktaya en yakın 2 oyuncu.
const { oku } = require('./kayit-oku.js'); const S = { n: 0, sav: { d0: 0, d1: 0, tepki: 0, tn: 0, hiz: 0, gor: {} }, huc: { d0: 0, d1: 0, tepki: 0, tn: 0, hiz: 0, gor: {} } };
for (const f of process.argv.slice(2)) { const k = oku(f), N = k.N, X = (t, i) => k.P.x[t * N + i] / 100, Y = (t, i) => k.P.y[t * N + i] / 100;
  for (const p of k.pas.filter(p => p.tur === 'gönder' && p.kim != null)) { const T = k.oyuncular[p.kim].takim, t = p.t; const s = k.sahip.find(s => s.t > t && s.kim != null); if (!s || s.t > t + 180 || s.t - t < 6) continue;
    if (k.olay.find(o => o.tur === 'sayi' && o.t > t && o.t <= s.t) || k.oyuncular[s.kim].rol === 'Bekçi') continue; S.n++; const bx = k.T.bx[s.t] / 100, by = k.T.by[s.t] / 100;
    for (const yan of ['sav', 'huc']) { const L = []; for (let i = 0; i < N; i++) { const o = k.oyuncular[i]; if (o.rol === 'Bekçi' || (yan === 'huc') !== (o.takim === T)) continue; L.push(i); }
      const en2 = L.sort((a, b) => Math.hypot(X(t, a) - bx, Y(t, a) - by) - Math.hypot(X(t, b) - bx, Y(t, b) - by)).slice(0, 2), o = S[yan];
      for (const i of en2) { const d0 = Math.hypot(X(t, i) - bx, Y(t, i) - by), d1 = Math.hypot(X(s.t, i) - bx, Y(s.t, i) - by); o.d0 += d0; o.d1 += d1;
        // tepki: noktaya doğru hızın ilk 0,1'i aştığı tik
        let tp = null; for (let u = t + 1; u <= s.t; u++) { const vx = X(u, i) - X(u - 1, i), vy = Y(u, i) - Y(u - 1, i), dx = bx - X(u, i), dy = by - Y(u, i), L2 = Math.hypot(dx, dy) || 1; if ((vx * dx + vy * dy) / L2 > .1) { tp = u - t; break; } } if (tp != null) { o.tepki += tp; o.tn++; }
        let yol = 0; for (let u = t + 1; u <= s.t; u++) yol += Math.hypot(X(u, i) - X(u - 1, i), Y(u, i) - Y(u - 1, i)); o.hiz += yol / (s.t - t);
        const g = (k.gorevler[k.P.job[(t + 6) * N + i]] || '').replace(/ · tepki/g, '').replace(/ · .*/, '') || '—'; o.gor[g] = (o.gor[g] || 0) + 1; } } } }
console.log(`seken top ${S.n} · her taraftan noktaya en yakın 2 oyuncu:`);
for (const yan of ['huc', 'sav']) { const o = S[yan], m = S.n * 2; console.log(`  ${yan === 'huc' ? 'hücum' : 'savunma'}: vuruş anında noktaya ${(o.d0 / m).toFixed(1)} br → alışta ${(o.d1 / m).toFixed(1)} br · noktaya koşmaya başlama ${(o.tepki / o.tn).toFixed(1)} tik (koşmayan %${(100 * (1 - o.tn / m)).toFixed(0)}) · hız ${(o.hiz / m).toFixed(2)} br/tik · görev (6. tik): ${Object.entries(o.gor).sort((a, b) => b[1] - a[1]).slice(0, 5).map(([g, v]) => g + ' %' + (100 * v / m).toFixed(0)).join(', ')}`); }
