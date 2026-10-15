// Forvet topsuz hücumda: rakibin son savunmacısına göre (Bekçi hariç) önünde / hizasında / arkasında; topun önünde mi; arkaya pas (önüne) aldı mı.
const { oku } = require('./kayit-oku.js'); const [f, ...ads] = process.argv.slice(2), k = oku(f), N = k.N;
const X = (t, i) => k.P.x[t * N + i] / 100;
for (const ad of ads) { const i = k.oyuncular.findIndex(o => o.ad === ad), T = k.oyuncular[i].takim, ile = x => T === 0 ? x : k.saha.W - x; const S = { n: 0, arka: 0, hiza: 0, on: 0, topOnu: 0, sonBolge: 0 };
  for (let t = 0; t < k.n; t += 6) { const h = k.T.hold[t]; if (h === 255 || h === i || k.oyuncular[h].takim !== T) continue; let son = 1e9; for (let j = 0; j < N; j++) if (k.oyuncular[j].takim !== T && k.oyuncular[j].rol !== 'Bekçi') son = Math.max(son === 1e9 ? -1e9 : son, ile(X(t, j))); const me = ile(X(t, i)), top = ile(k.T.bx[t] / 100);
    if (top < 60) continue; S.n++; const d = me - son; if (d > 1.5) S.arka++; else if (d > -3) S.hiza++; else S.on++; if (me > top + 3) S.topOnu++; }
  const al = k.pas.filter(p => p.alici === i), tur = {}; for (const p of al) tur[p.tur] = (tur[p.tur] || 0) + 1;
  console.log(`${ad}: takımı rakip yarısında topluyken (${(S.n / 10).toFixed(0)} sn): son savunmacının arkasında %${(100 * S.arka / S.n).toFixed(0)} · hizasında %${(100 * S.hiza / S.n).toFixed(0)} · önünde %${(100 * S.on / S.n).toFixed(0)} · topun önünde %${(100 * S.topOnu / S.n).toFixed(0)} | ona atılan paslar: ${JSON.stringify(tur)}`); }
