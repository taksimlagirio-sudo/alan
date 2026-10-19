// Taktik ölçümü: iki takım aynı oyuncularla (bütün özellikler 10). A takımı <a> ayarıyla, rakip <b> ayarıyla oynar.
// Ayarlar aşağıdaki AYAR tablosunda (taktik sayfasının tac() çıktısıyla aynı alanlar). A, tohumun tekliğine göre takım 0 ya da 1 (taraf dengelenir).
// Kullanım: node tools/taktik-olc.js <motor klasörü> <a> <b> <tohum> [tik]   ör. ... pres3 varsayilan 7
const fs = require('fs'), path = require('path'); const [dir, aAd, bAd, sS, lS] = process.argv.slice(2), seed = +sS, LEN = +lS || 7200;
const OZ = ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', 'refleks', 'yerlesim', 'hakimiyet'];
const { V, VD, AYAR } = require('./taktik-ayar');
function rng(sd) { let a = sd >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
Math.random = rng(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of ['sabit-mat.js', 'core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js']) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const M = globalThis.AlanMatch; if (!AYAR[aAd] || !AYAR[bAd]) throw new Error('ayar yok: ' + aAd + ' / ' + bAd);
const kur = ad => { const { _diz, ...t } = AYAR[ad]; return { tac: { ...V, tetik: V.tetik.slice(), ...t }, diz: _diz || VD }; };
const A = seed % 2, B = 1 - A, KA = kur(aAd), KB = kur(bAd), K = A === 0 ? [KA, KB] : [KB, KA], t0 = Date.now();
const m = M.createMatch(seed * 7919 + 17, { tac: K.map(k => k.tac), len: LEN, diz: K.map(k => k.diz) });
for (const p of m.ps) { const a = {}; for (const k of OZ) a[k] = 10; a.taraf = (p.a && p.a.taraf) || 'sag'; p.a = a; } for (const t of [0, 1]) M.dizUygula(m, t, K[t].diz);
const GW = globalThis.ALAN_SAHA.W, GHh = globalThis.ALAN_SAHA.H, ile = (t, x) => (t === 0 ? x : GW - x) / GW; let sahipA = 0, sahipN = 0, yarA = 0, savY = 0, savN = 0, gen = 0, genN = 0, presA = 0;
while (!m.over && m.tick < LEN) { M.step(m); const h = m.holder; if (h) { sahipN++; if (h.team === A) { sahipA++; if (ile(A, h.x) > .5) yarA++; } }
  if (m.tick % 10 === 0 && h) { const As = m.ps.filter(p => p.team === A && p.role !== 'Bekçi'); if (h.team !== A) { savY += As.reduce((s, p) => s + ile(A, p.x), 0) / As.length; presA += As.filter(p => typeof p.job === 'string' && p.job.startsWith('pres')).length; savN++; } else { const ys = As.map(p => p.y); gen += (Math.max(...ys) - Math.min(...ys)) / GHh; genN++; } } }
const st = m.st, ab = k => st[k] ? [st[k][A], st[k][B]] : null;
console.log(JSON.stringify({ a: aAd, b: bAd, seed, A, tik: m.tick, sn: +((Date.now() - t0) / 1000).toFixed(1), gol: [m.score[A], m.score[B]], pas: ab('pass'), pasOk: ab('passOk'), sut: ab('shot'), kapma: ab('steal'), ikili: ab('duel'), ikiliK: ab('duelW'),
  sahip: +(sahipA / Math.max(1, sahipN)).toFixed(3), rakipYari: +(yarA / Math.max(1, sahipA)).toFixed(3), savYuk: +(savY / Math.max(1, savN)).toFixed(3), presci: +(presA / Math.max(1, savN)).toFixed(2), genislik: +(gen / Math.max(1, genN)).toFixed(3) }));
