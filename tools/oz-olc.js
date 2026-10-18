// Özellik etkisi ölçümü: iki takım aynı (aynı taktik, aynı diziliş, bütün özellikler 10). A takımında tek özellik değişir:
// saha oyuncularında (Bekçi özelliklerinde yalnız Bekçi'de) özellik = DEGER. A, tohumun tekliğine göre takım 0 ya da 1 olur (taraf etkisi dengelenir).
// Çıktı: tek satır JSON, A'nın gözünden (a = A, b = rakip).
// Kullanım: node tools/oz-olc.js <motor klasörü> <özellik|taban> <değer> <tohum> [tik]
const fs = require('fs'), path = require('path'); const [dir, oz, vS, sS, lS] = process.argv.slice(2), V = +vS, seed = +sS, LEN = +lS || 7200;
const BEKCI = ['refleks', 'yerlesim', 'hakimiyet'];
const OZ = ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', ...BEKCI];
function rng(sd) { let a = sd >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
Math.random = rng(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of ['sabit-mat.js', 'core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js']) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f); /* maç işçisiyle aynı sıra: sabit-mat önce (node ve bun aynı maçı oynar) */
const M = globalThis.AlanMatch;
const TAC = { sistem: 'Alan', pres: 1, arkada: 1, blok: 'Orta', genislik: 'Normal', tempo: .5, risk: .5, kazaninca: 'Dengeli' }, DIZ = { sav: '2-2-2', huc: '2-1-3' };
const A = seed % 2, t0 = Date.now();
const m = M.createMatch(seed * 7919 + 17, { tac: [{ ...TAC }, { ...TAC }], len: LEN, diz: [DIZ, DIZ] });
for (const p of m.ps) { const a = {}; for (const k of OZ) a[k] = 10; a.taraf = (p.a && p.a.taraf) || 'sag'; if (p.team === A && oz !== 'taban') { const bk = BEKCI.includes(oz); if (bk === (p.role === 'Bekçi')) a[oz] = V; } p.a = a; }
for (const t of [0, 1]) M.dizUygula(m, t, DIZ);
let sahipA = 0, sahipN = 0, hizA = 0, hizB = 0, hN = 0;
while (!m.over && m.tick < LEN) { M.step(m); const h = m.holder; if (h) { sahipN++; if (h.team === A) sahipA++; }
  if (m.tick % 10 === 0) for (const p of m.ps) { if (p.role === 'Bekçi') continue; const v = Math.hypot(p.vx || 0, p.vy || 0); if (p.team === A) hizA += v; else hizB += v; hN++; } }
const st = m.st, B = 1 - A, ab = k => st[k] ? [st[k][A], st[k][B]] : null;
console.log(JSON.stringify({ oz, v: V, seed, A, tik: m.tick, sn: +((Date.now() - t0) / 1000).toFixed(1), gol: [m.score[A], m.score[B]], pas: ab('pass'), pasOk: ab('passOk'), sut: ab('shot'), kapma: ab('steal'), ikili: ab('duel'), ikiliK: ab('duelW'), bos: ab('loose'), direk: ab('direk'),
  sahip: +(sahipA / Math.max(1, sahipN)).toFixed(3), hiz: [+(hizA / hN * 2).toFixed(4), +(hizB / hN * 2).toFixed(4)] }));
