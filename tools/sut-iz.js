// Şut izi: her şutun şarjı (alırken, vururken), tutma süresi, taşıyıcının barı, xG, uzaklık, hızı ve sonucu. Enerji ölçümü 1'in gol sorunu için.
// Kullanım: BAYRAK=ALAN_TEK_ENERJI_ESKI node tools/sut-iz.js <motor> <a> <b> <tohum> [tik]
const fs = require('fs'), path = require('path'); const { V, VD, AYAR } = require('./taktik-ayar'); const [dir, aAd, bAd, sS, lS] = process.argv.slice(2), seed = +sS, LEN = +lS || 2700;
const OZL = ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', 'enerji', 'refleks', 'yerlesim', 'hakimiyet'];
function rng(sd) { let x = sd >>> 0; return () => { x = (x + 0x6D2B79F5) | 0; let t = Math.imul(x ^ x >>> 15, 1 | x); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
for (const k of (process.env.BAYRAK || "").split(",").filter(Boolean)) globalThis[k] = true; Math.random = rng(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of ['sabit-mat.js', 'core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js']) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const M = globalThis.AlanMatch, kur = ad => { const { _diz, ...t } = AYAR[ad]; return { tac: { ...V, tetik: V.tetik.slice(), ...t }, diz: _diz || VD }; };
const A = seed % 2, K = A === 0 ? [kur(aAd), kur(bAd)] : [kur(bAd), kur(aAd)], [ozK, ozV] = (process.env.OZ || ':').split(':');
const m = M.createMatch(seed * 7919 + 17, { tac: K.map(k => k.tac), len: LEN, diz: K.map(k => k.diz) });
for (const p of m.ps) { const a = {}; for (const k of OZL) a[k] = 10; a.taraf = (p.a && p.a.taraf) || 'sag'; if (ozK && p.team === A && p.role !== 'Bekçi') a[ozK] = +ozV; p.a = a; } for (const t of [0, 1]) M.dizUygula(m, t, K[t].diz);
let fl0 = m.fl, sh = null, tH = null, tA = 0, chA = 0, preC = 0; const O = [];
const bitir = son => { O.push({ ...sh, son }); sh = null; };
while (!m.over && m.tick < LEN) { const sk = m.score.slice(), h0 = m.holder, c0 = m.ch; if (h0 !== tH) { tH = h0; tA = m.tick; chA = m.ch; } M.step(m); const b = m.ball;
  if (sh) { if (m.score[0] !== sk[0] || m.score[1] !== sk[1]) bitir('GOL'); else if (m.holder) bitir(m.holder.role === 'Bekçi' ? 'Bekçi' : (m.holder.team === sh.tm ? 'bizim' : 'rakip')); else if (b && b.done && b.done !== sh.d0) bitir(b.done); else if (m.tick - sh.t > 150) bitir('süre'); }
  if (m.fl && m.fl !== fl0) { fl0 = m.fl; if (m.fl.kind === 'gönder' && b && b.shotPk != null) { const gx = b.team === 0 ? 120 : 0; sh = { t: m.tick, tm: b.team, pk: +b.shotPk.toFixed(2), uz: +Math.abs(gx - b.x).toFixed(0), v: +Math.hypot(b.vx || 0, b.vy || 0).toFixed(2), ch: +(b.ch ?? -1).toFixed(2), d0: b.done, chAl: +chA.toFixed(2), chOn: +c0.toFixed(2), tut: m.tick - tA, en: h0 && h0.en != null ? +h0.en.toFixed(2) : null }; } } }
console.log(JSON.stringify({ seed, gol: m.score, O }));
