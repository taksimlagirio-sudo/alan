// Beynin zamanının hangi tartmaya gittiğini ölçer: plan türüne göre deneme sayısı, süre, deneme başına süre ve oynatılan tik.
// Kullanım: node tools/v4-prof.js <motor> <i> [tik=900]   (--cpu-prof ile birlikte çalıştırılabilir)
const fs = require('fs'), { load } = require('./load');
const [dir, iS, lS] = process.argv.slice(2), i = +iS, LEN = +lS || 900, seed = 190000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch, B = g.AlanBeyin, D = g.AlanDecide.D; (0, eval)(fs.readFileSync(dir + '/vs-table.js', 'utf8')); (0, eval)(fs.readFileSync(dir + '/pas-table.js', 'utf8'));
const m = M.createMatch(seed, { len: 5400 }); const S = {}; let dec = 0; const d0 = D.decide; D.decide = function () { dec++; return d0.apply(this, arguments); };
const st0 = M.step; let ticks = 0; M.step = function (c) { ticks++; return st0.apply(this, arguments); };
const T0 = Date.now(); let tStep = 0;
while (!m.over && m.tick < LEN) { let p = B.planla(m); while (p) { const s = S[p.tip] = S[p.tip] || { plan: 0, deneme: 0, ms: 0, tik: 0, decide: 0 }, t = process.hrtime.bigint(), k0 = ticks, d1 = dec; const res = p.jobs.map(j => B.rollout(m, j)); s.ms += Number(process.hrtime.bigint() - t) / 1e6; s.plan++; s.deneme += p.jobs.length; s.tik += ticks - k0; s.decide += dec - d1; p = p.bitir(res) || null; }
  const t = process.hrtime.bigint(); M.step(m); tStep += Number(process.hrtime.bigint() - t) / 1e6; }
const tot = Date.now() - T0; console.log(`${m.tick} tik, ${(tot / 1000).toFixed(1)} sn → ${(m.tick / tot * 1000).toFixed(1)} tik/sn · gerçek adım ${tStep.toFixed(0)} ms`);
for (const [k, s] of Object.entries(S).sort((a, b) => b[1].ms - a[1].ms)) console.log(`${k.padEnd(8)} %${(100 * s.ms / tot).toFixed(0).padStart(3)} · ${s.plan} plan · ${s.deneme} deneme · deneme başına ${(s.ms / s.deneme).toFixed(1)} ms, ${(s.tik / s.deneme).toFixed(0)} tik, ${(s.decide / s.deneme).toFixed(1)} decide`);
