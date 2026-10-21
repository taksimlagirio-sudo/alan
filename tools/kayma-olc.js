// Kayma ölçümü: taraf değiştiren paslarda (topun yanal yer değişimi ≥ 20 br) savunmanın yana kayması: pas anında, uçuşun ortasında, alındığı anda. Senkron mu (savunmacılar aynı anda mı kayıyor), geç mi kalıyor, alıcı ne kadar boş.
// Düzen blok-olc.js'inki. Kullanım: node tools/kayma-olc.js <motor> <a> <b> <tohum> [tik=9000]
const fs = require('fs'), path = require('path'); const { V, VD, AYAR } = require('./taktik-ayar'); const [dir, aAd, bAd, sS, lS] = process.argv.slice(2), seed = +sS, LEN = +lS || 9000;
const EK = {
  B_onde: { blok: 'Düşük', pres: 0, siddet: .3, tempo: .9, risk: .8, gecisH: 'her', kpYer: 'hic', sabir: .2, pasSurme: .3, _diz: { sav: '3-2-1', huc: '1-2-3' }, onde: 1 },
  B_hepsi: { blok: 'Düşük', pres: 0, siddet: .3, tempo: .9, risk: .8, gecisH: 'her', kpYer: 'hic', sabir: .2, pasSurme: .3, _diz: { sav: '3-2-1', huc: '1-2-3' } }
};
function rng(sd) { let x = sd >>> 0; return () => { x = (x + 0x6D2B79F5) | 0; let t = Math.imul(x ^ x >>> 15, 1 | x); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
Math.random = rng(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of ['sabit-mat.js', 'core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js']) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const M = globalThis.AlanMatch, kur = ad => { const { _diz, ...t } = EK[ad] || AYAR[ad]; return { tac: { ...V, tetik: V.tetik.slice(), ...t }, diz: _diz || VD }; };
const A = seed % 2, K = A === 0 ? [kur(aAd), kur(bAd)] : [kur(bAd), kur(aAd)];
const m = M.createMatch(seed * 7919 + 17, { tac: K.map(k => k.tac), len: LEN, diz: K.map(k => k.diz) });
for (const p of m.ps) { const a = {}; for (const k of ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', 'enerji', 'refleks', 'yerlesim', 'hakimiyet']) a[k] = 10; a.taraf = (p.a && p.a.taraf) || 'sag'; p.a = a; }
const P = [], acik = []; let fl0 = m.fl;
const def = t => m.ps.filter(p => p.team !== t && p.role !== 'Bekçi'), ort = a => a.reduce((s, x) => s + x, 0) / a.length;
while (!m.over && m.tick < LEN) { M.step(m); const h = m.holder, b = m.ball;
  if (m.fl && m.fl !== fl0) { fl0 = m.fl; if (['pas', 'önüne', 'kenardan'].includes(m.fl.kind) && b && m.fl.end) { const t = b.team, D = def(t); acik.push({ t0: m.tick, t, q: m.fl.q, y0: b.y, yE: m.fl.end.y, d0: D.map(p => p.y), c0: ort(D.map(p => p.y)), D, yarim: null }); } }
  for (let i = acik.length - 1; i >= 0; i--) { const a = acik[i], dt = m.tick - a.t0;
    if (h || dt > 150 || (b && b.done)) { if (h && h.team === a.t && h === a.q && dt > 3) { const dY = h.y - a.y0, c1 = ort(a.D.map(p => p.y)), kay = a.D.map((p, k) => p.y - a.d0[k]), yakin = Math.min(...a.D.map(p => Math.hypot(p.x - h.x, p.y - h.y)));
        P.push({ dY: +dY.toFixed(1), sure: dt, cK: +(c1 - a.c0).toFixed(1), cY: a.yarim, kayS: +Math.sqrt(ort(kay.map(k => (k - ort(kay)) ** 2))).toFixed(1), ayni: kay.filter(k => Math.sign(k) === Math.sign(dY) && Math.abs(k) > 1).length, n: kay.length, yakin: +yakin.toFixed(1), hY: +h.y.toFixed(0) }); }
      acik.splice(i, 1); continue; }
    if (a.yarim == null && b && Math.abs(b.y - a.y0) >= Math.abs(a.yE - a.y0) / 2) a.yarim = +(ort(a.D.map(p => p.y)) - a.c0).toFixed(1); } }
console.log(JSON.stringify({ a: aAd, b: bAd, seed, A, P }));
