// Enerjili motorun genel ölçümü: iki takım aynı oyuncularla (herkes 10), A <a>, rakip <b> tarzıyla (taktik-ayar.js). İstenirse A'nın saha oyuncularında tek özellik: OZ=hiz:16.
// Takım başına: gol, şut, xG (shotPk), pas, kapma, ikili (sayaç), top sahipliği, ortalama bar, dibe vurma (<0,15) payı, sprint payı, harcamanın türlere dağılımı (m.st.enH).
// Kullanım: OZ=hiz:16 node tools/enerji-olc.js <motor> <a> <b> <tohum> [tik]
const fs = require('fs'), path = require('path'); const { V, VD, AYAR } = require('./taktik-ayar'); const [dir, aAd, bAd, sS, lS] = process.argv.slice(2), seed = +sS, LEN = +lS || 2700;
const OZL = ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', 'enerji', 'refleks', 'yerlesim', 'hakimiyet'];
function rng(sd) { let x = sd >>> 0; return () => { x = (x + 0x6D2B79F5) | 0; let t = Math.imul(x ^ x >>> 15, 1 | x); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
Math.random = rng(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of ['sabit-mat.js', 'core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js']) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const M = globalThis.AlanMatch, kur = ad => { const { _diz, ...t } = AYAR[ad]; return { tac: { ...V, tetik: V.tetik.slice(), ...t }, diz: _diz || VD }; };
const A = seed % 2, K = A === 0 ? [kur(aAd), kur(bAd)] : [kur(bAd), kur(aAd)], [ozK, ozV] = (process.env.OZ || ':').split(':');
const m = M.createMatch(seed * 7919 + 17, { tac: K.map(k => k.tac), len: LEN, diz: K.map(k => k.diz) });
for (const p of m.ps) { const a = {}; for (const k of OZL) a[k] = 10; a.taraf = (p.a && p.a.taraf) || 'sag'; if (ozK && p.team === A && p.role !== 'Bekçi') a[ozK] = +ozV; p.a = a; } for (const t of [0, 1]) M.dizUygula(m, t, K[t].diz);
const R = [0, 1].map(() => ({ en: 0, n: 0, dip: 0, spr: 0, xg: 0, sh: 0 })); let fl0 = m.fl, onceki = null;
while (!m.over && m.tick < LEN) { M.step(m); const h = m.holder; if (h) { R[h.team].sh++; onceki = h; }
  if (m.fl && m.fl !== fl0) { fl0 = m.fl; if (m.fl.kind === 'gönder' && onceki) R[onceki.team].xg += m.ball.shotPk ?? 0; }
  for (const p of m.ps) { if (p.role === 'Bekçi') continue; const r = R[p.team]; r.en += p.en ?? 1; r.n++; if ((p.en ?? 1) < .15) r.dip++; if (p.sprint) r.spr++; } }
const st = m.st, ab = x => x ? [x[A], x[1 - A]] : null, top = R[0].sh + R[1].sh || 1, T = t => ({ en: +(R[t].en / R[t].n).toFixed(3), dip: +(R[t].dip / R[t].n).toFixed(3), spr: +(R[t].spr / R[t].n).toFixed(3), xg: +R[t].xg.toFixed(3), sahip: +(R[t].sh / top).toFixed(3), enH: st.enH ? st.enH[t] : null });
console.log(JSON.stringify({ a: aAd, b: bAd, oz: process.env.OZ || null, seed, A, tik: m.tick, gol: ab(m.score), sut: ab(st.shot), pas: ab(st.pass), pasOk: ab(st.passOk), kapma: ab(st.steal), ikili: ab(st.duel), ikiliK: ab(st.duelW), ta: T(A), tb: T(1 - A) }));
