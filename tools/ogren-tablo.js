// Hafıza: tablo kopyalarını (0, 25, 50) karşılaştırır. Uzaklık dilimine göre (önde 0–2 savunmacı) ortalama değer ve örnek sayısı; kaç hücre değişti.
// Kullanım: node tools/ogren-tablo.js <motor> <klasör>...
const fs = require('fs'), path = require('path'), { load } = require('./load'); const [dir, ...ks] = process.argv.slice(2); const g = load(dir, 1), S = g.AlanState, ST = S.ST;
for (const k of ks) { const T = n => { const f = path.join(k, `tablo-${n}.json`); return fs.existsSync(f) ? JSON.parse(fs.readFileSync(f)) : null; }, t = [0, 25, 50].map(T); console.log(`\n== ${k}`);
  console.log('uzaklık (merkez) · önde 0–2 savunmacı: ortalama değer 0 → 25 → 50 (o hücrelerde biriken örnek)');
  for (let iD = 0; iD < ST.nD; iD++) { const cells = []; for (let iW = 0; iW < ST.nW; iW++) for (let iS = 0; iS <= 2; iS++) for (let iB = 0; iB < ST.nB; iB++) for (let iT = 0; iT < ST.nT; iT++) for (let iK = 0; iK < ST.nK; iK++) cells.push(S.cell(iD, iW, iS, iB, iT, iK));
    const av = x => x ? (cells.reduce((s, c) => s + x.v[c], 0) / cells.length).toFixed(3) : '–', nn = x => x && x.n ? cells.reduce((s, c) => s + (x.n[c] || 0), 0) : 0;
    console.log(`${String(ST.dC[iD]).padStart(3)}  ${t.map(av).join(' → ')}   (örnek ${t.map(nn).join(' / ')})`); }
  const ch = (a, b) => a && b ? a.v.reduce((s, v, i) => s + (Math.abs(v - b.v[i]) > 1e-9 ? 1 : 0), 0) : '–'; console.log(`değişen hücre: 0→25 ${ch(t[0], t[1])}, 25→50 ${ch(t[1], t[2])} / ${t[0].v.length}`); }
