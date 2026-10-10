// v4 paralel hız ve eşdeğerlik: aynı maç tek iş parçacığında ve N işçiyle oynanır; skor ve iz aynı olmalı, süre ölçülür.
// Kullanım: node tools/v4-hiz.js <motor> <tohum> <tik> [N=4] [ayar adı: tam|hepsiz|kosaz]
const fs = require('fs'), { load } = require('./load'), { havuz } = require('./v4-paralel');
const [dir, sS, lS, nS, ayar] = process.argv.slice(2), seed = +sS || 1, len = +lS || 1200, N = +nS || 4;
const g = load(dir, seed), M = g.AlanMatch, B = g.AlanBeyin; (0, eval)(fs.readFileSync(dir + '/vs-table.js', 'utf8')); (0, eval)(fs.readFileSync(dir + '/pas-table.js', 'utf8'));
const HAZIR = { kosaz: { savunma: false, kovala: false, algi: 0, t0: .001, tOk: 0 }, hepsiz: { algi: 0, t0: .001, tOk: 0 }, tam: {}, ichafif: { icHafif: true } }; Object.assign(B.A, HAZIR[ayar || 'tam']);
(async () => { let t = Date.now(); const a = M.createMatch(seed, { len }); B.oyna(a); const t1 = Date.now() - t;
  const P = havuz(N, dir); t = Date.now(); const b = M.createMatch(seed, { len }); await B.oynaAsync(b, { isler: (m, jobs) => P.isler(m, jobs) }); const t2 = Date.now() - t; P.kapat();
  console.log(`tek: ${(t1 / 1000).toFixed(1)} sn, skor ${a.score} · ${N} işçi: ${(t2 / 1000).toFixed(1)} sn, skor ${b.score} · iz ${M.matchHash(a) === M.matchHash(b) ? 'AYNI' : 'FARKLI'} · ${len} tik`); })();
