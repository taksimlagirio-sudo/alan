// Pas kalibrasyonu (Design görevi): maçta atılan her pas için aklın tahmini (o.det.P: arkadaş alır) ile gerçek sonuç (sıradaki sahip pas atanın takımından mı).
// Motor dosyalarına dokunmaz; yükleme sırasında match.js'e tek bir kayıt eklenir (m.fl.o = seçilen hamle).
// Kullanım: node tools/v5-kalib.js <motor> <i>   → her pas bir satır JSON
const fs = require('fs'), path = require('path'); const [dir, iS] = process.argv.slice(2), i = +iS, seed = 410000 + i * 7919;
function rngF(s) { let a = s >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
Math.random = rngF(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of ['core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js']) { let s = fs.readFileSync(path.join(dir, f), 'utf8');
  if (f === 'match.js') { const a = 'm.fl = { t0: m.tick, kind: o.kind,'; if (!s.includes(a)) throw new Error('yama yeri yok'); s = s.replace(a, 'm.fl = { o, t0: m.tick, kind: o.kind,'); }
  (0, eval)(s + '\n//# sourceURL=' + f); }
const M = AlanMatch;
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const ks = Object.keys(STY), tOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7], pasSurme: .5, sabir: .5, sut: .5, tetik: ['Geri pas', 'Kanatta'], kontraN: 3, kayip: .6, kpSure: 3 });
const m = M.createMatch(seed, { len: 5400, tac: [tOf(STY[ks[i % 6]]), tOf(STY[ks[(i * 5 + 2) % 6]])], diz: [{ sav: '2-2-2', huc: '2-2-2' }, { sav: '3-2-1', huc: '2-3-1' }] });
let fl = null, acik = null; const out = [];
while (!m.over && m.tick < 5400) { const sc = m.score.slice(); M.step(m);
  if (m.fl && m.fl !== fl && m.fl.o) { fl = m.fl; const o = fl.o;
    if (o.kind !== 'gönder' && o.kind !== 'çalım' && o.det && o.det.P != null) { const h = o.kendine ? o.q : m.ball.from, b = m.ball, to = o.lead || o.to || o.q; let nd = 1e9; for (const p of m.ps) if (p.team !== b.team && p.role !== 'Bekçi') nd = Math.min(nd, Math.hypot(p.x - b.x, p.y - b.y));
      acik = { tur: o.kendine ? 'kendine' : o.kind, P: +o.det.P.toFixed(3), uz: +Math.hypot(to.x - b.x, to.y - b.y).toFixed(1), rak: +nd.toFixed(1), ot: !!o.ot, sonuc: null, t: m.tick, team: b.team }; out.push(acik); } else acik = null; }
  if (acik && acik.sonuc == null) { if (m.score[0] !== sc[0] || m.score[1] !== sc[1]) acik.sonuc = m.score[acik.team] !== sc[acik.team] ? 1 : 0; else if (m.holder) acik.sonuc = m.holder.team === acik.team ? 1 : 0; } }
for (const r of out) if (r.sonuc != null) console.log(JSON.stringify(r));
