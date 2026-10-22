// Yetenek ölçümü: iki takım aynı (bütün özellikler 10, taktik varsayılan), A'da tek özellik değişir (OZ=anahtar:değer). Bekçi özellikleri (refleks, yerlesim, hakimiyet, birebir) yalnız A'nın Bekçi'sine, öbürleri A'nın saha oyuncularına.
// Yükleme sırası motorun kendi işçisinden (mac-isci.js importScripts) okunur. Kullanım: OZ=hiz:16 node tools/yet-olc.js <motor> <tohum> [tik=4500]
const fs = require('fs'), path = require('path'); const { V, VD } = require('./taktik-ayar'); const [dir, sS, lS] = process.argv.slice(2), seed = +sS, LEN = +lS || 4500;
const yukle = (() => { const m = /importScripts\(([^)]*)\)/.exec(fs.readFileSync(path.join(dir, 'mac-isci.js'), 'utf8')); return m[1].split(',').map(s => s.trim().replace(/['"]/g, '')); })();
globalThis.window = globalThis; globalThis.self = globalThis; for (const f of yukle) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const M = globalThis.AlanMatch, Y = self.ALAN_YET, BEK = new Set(Object.keys(Y.OZ).filter(k => Y.OZ[k][1] === 'bek')), ESKI = ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', 'enerji', 'refleks', 'yerlesim', 'hakimiyet'];
const A = seed % 2, [ozK, ozV] = (process.env.OZ || ':').split(':');
const m = M.createMatch(seed * 7919 + 17, { tac: [{ ...V, tetik: V.tetik.slice() }, { ...V, tetik: V.tetik.slice() }], len: LEN, diz: [VD, VD] });
for (const p of m.ps) { const a = {}; for (const k of ESKI) a[k] = 10; for (const k of Object.keys(Y.OZ)) a[k] = 10; a.taraf = 'iki'; if (ozK && p.team === A && (p.role === 'Bekçi') === BEK.has(ozK)) a[ozK] = +ozV; p.a = a; }
for (const t of [0, 1]) M.dizUygula(m, t, VD);
const R = [0, 1].map(() => ({ en: 0, n: 0, dip: 0, xg: 0, sh: 0, ch: 0, sutN: 0 })); let fl0 = m.fl, onceki = null;
while (!m.over && m.tick < LEN) { M.step(m); const h = m.holder; if (h) { R[h.team].sh++; onceki = h; }
  if (m.fl && m.fl !== fl0) { fl0 = m.fl; const b = m.ball; if (m.fl.kind === 'gönder' && onceki && b && b.shotPk != null) { R[onceki.team].xg += b.shotPk; R[onceki.team].ch += b.ch ?? 0; R[onceki.team].sutN++; } }
  for (const p of m.ps) { if (p.role === 'Bekçi') continue; const r = R[p.team]; r.en += p.en ?? 1; r.n++; if ((p.en ?? 1) < .15) r.dip++; } }
const st = m.st, ab = x => x ? [x[A], x[1 - A]] : null, top = R[0].sh + R[1].sh || 1, T = t => ({ en: +(R[t].en / R[t].n).toFixed(3), dip: +(R[t].dip / R[t].n).toFixed(3), xg: +R[t].xg.toFixed(3), ch: R[t].sutN ? +(R[t].ch / R[t].sutN).toFixed(3) : null, sahip: +(R[t].sh / top).toFixed(3) });
console.log(JSON.stringify({ oz: process.env.OZ || null, seed, A, tik: m.tick, gol: ab(m.score), sut: ab(st.shot), pas: ab(st.pass), pasOk: ab(st.passOk), kapma: ab(st.steal), ikili: ab(st.duel), ta: T(A), tb: T(1 - A) }));
