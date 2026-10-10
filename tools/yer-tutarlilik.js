// Yerleşim kararı gürültü mü, sinyal mi: her yerleşim planında adaylar iki kez, bağımsız zarlarla tartılır; en iyi aday aynı mı çıkıyor?
// Şans düzeyi ≈ 1/aday sayısı. Ayrıca adaylar arası fark (en iyi − en kötü) ve aynı adayın iki tartısı arasındaki fark (gürültü).
// Kullanım: node tools/yer-tutarlilik.js <motor> <i> [tik=600]
const fs = require('fs'), { load } = require('./load'); const [dir, iS, lS] = process.argv.slice(2), i = +iS, LEN = +lS || 600, seed = 230000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch, B = g.AlanBeyin; (0, eval)(fs.readFileSync(dir + '/vs-table.js', 'utf8')); (0, eval)(fs.readFileSync(dir + '/pas-table.js', 'utf8'));
const m = M.createMatch(seed, { len: 5400 }), A = B.A, R = []; const avgK = (res, nK) => Array.from({ length: nK }, (_, k) => res.slice(k * A.YR, k * A.YR + A.YR).reduce((s, x) => s + x, 0) / A.YR), am = v => v.indexOf(Math.max(...v));
while (m.tick < LEN) { let p = B.planla(m); while (p) { const res = p.jobs.map(j => B.rollout(m, j));
    if (p.tip === 'yer') { const nK = p.jobs.length / A.YR, r2 = p.jobs.map(j => B.rollout(m, { ...j, salt: (j.salt ^ 0x5f3759df) >>> 0 })), v1 = avgK(res, nK), v2 = avgK(r2, nK);
      R.push({ nK, ayni: am(v1) === am(v2), fark: Math.max(...v1) - Math.min(...v1), gurultu: v1.reduce((s, x, k) => s + Math.abs(x - v2[k]), 0) / nK, ref: am(v1) === 0 }); }
    p = p.bitir(res) || null; } M.step(m); }
const mean = f => R.reduce((s, r) => s + f(r), 0) / R.length;
console.log(JSON.stringify({ dir: dir.split('/').pop(), i, plan: R.length, aday: +mean(r => r.nK).toFixed(1), ayniEnIyi: +mean(r => r.ayni).toFixed(2), sans: +mean(r => 1 / r.nK).toFixed(2), fark: +mean(r => r.fark).toFixed(3), gurultu: +mean(r => r.gurultu).toFixed(3) }));
