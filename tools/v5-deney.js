// v5 deney koşucusu: A takımı ile B takımı arasında bir maç. A'nın taktiği/özellikleri spec'ten; taraflar i'nin tekliğine göre değişir (kale ve başlama avantajı birbirini götürsün).
// Kullanım: node tools/v5-deney.js <motor> <deney adı> <i> '<spec JSON>'   spec: { tacA, tacB, atA: {ozellik: değer}, atB, diz }
// Çıktı (A'nın gözünden): sayı, gönderme, topa sahip olma, pas ve tutma, top kazanma
const { load } = require('./load'); const [dir, ad, iS, specS] = process.argv.slice(2), i = +iS, spec = JSON.parse(specS || '{}'), seed = 300000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch;
const VARS = { sistem: 'Alan', blok: 'Orta', pres: 1, arkada: 1, onde: 0, genislik: 'Normal', risk: .5, tempo: .5, kazaninca: 'Dengeli', pasSurme: .5, sabir: .5, sut: .5, tetik: ['Geri pas', 'Kanatta'], kontraN: 3, kayip: .6, kpSure: 3 };
const tA = { ...VARS, ...(spec.tacA || {}) }, tB = { ...VARS, ...(spec.tacB || {}) }, aSide = i % 2; // A bu maçta hangi takım
const tac = aSide === 0 ? [tA, tB] : [tB, tA], dz = spec.diz || { sav: '2-2-2', huc: '2-2-2' };
const m = M.createMatch(seed, { len: 5400, tac, diz: [dz, dz] });
for (const p of m.ps) { const at = p.team === aSide ? spec.atA : spec.atB; if (at && p.role !== 'Bekçi') Object.assign(p.a, at); }
while (!m.over && m.tick < 5400) M.step(m);
const a = aSide, b = 1 - aSide, S = m.st, P = (S.poss[0] + S.poss[1]) || 1;
console.log(JSON.stringify({ ad, i, sA: m.score[a], sB: m.score[b], gA: S.shot[a], gB: S.shot[b], posA: +(S.poss[a] / P).toFixed(3), pasA: S.pass[a], pasB: S.pass[b], tutA: +(S.passOk[a] / Math.max(1, S.pass[a])).toFixed(3), tutB: +(S.passOk[b] / Math.max(1, S.pass[b])).toFixed(3), kazA: S.steal[a], kazB: S.steal[b] }));
