// Taktik eşleşmesinin mekanik izi: iki takım aynı oyuncularla (herkes 10), A <a>, rakip <b> ayarıyla (taktik-ayar.js).
// Takım başına: savunmanın arkasına atılan toplar ve isabeti, rakibin son hattının yüksekliği, şut ve şut kalitesi (motorun shotPk'si = gol ihtimali),
// şutların kaynağı (top kazanıldıktan kısa süre sonra = geçiş; arkaya pastan sonra), top kayıpları ve yeri, rakip yarısında top kazanma.
// Kullanım: node tools/taktik-iz.js <motor> <a> <b> <tohum> [tik]
const fs = require('fs'), path = require('path'); const { V, VD, AYAR } = require('./taktik-ayar');
const [dir, aAd, bAd, sS, lS] = process.argv.slice(2), seed = +sS, LEN = +lS || 3600;
const OZ = ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', 'refleks', 'yerlesim', 'hakimiyet'];
function rng(sd) { let a = sd >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
Math.random = rng(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of ['sabit-mat.js', 'core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js']) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const M = globalThis.AlanMatch, GW = globalThis.ALAN_SAHA.W, GH = globalThis.ALAN_SAHA.H, SN = 60;
const kur = ad => { const { _diz, ...t } = AYAR[ad]; return { tac: { ...V, tetik: V.tetik.slice(), ...t }, diz: _diz || VD }; };
const A = seed % 2, KA = kur(aAd), KB = kur(bAd), K = A === 0 ? [KA, KB] : [KB, KA];
const m = M.createMatch(seed * 7919 + 17, { tac: K.map(k => k.tac), len: LEN, diz: K.map(k => k.diz) });
for (const p of m.ps) { const a = {}; for (const k of OZ) a[k] = 10; a.taraf = (p.a && p.a.taraf) || 'sag'; p.a = a; } for (const t of [0, 1]) M.dizUygula(m, t, K[t].diz);
const ile = (t, x) => (t === 0 ? x : GW - x) / GW; // takımın hücum yönünde 0 (kendi kalesi) … 1 (rakip kalesi)
const sonHat = t => { let mx = 0; for (const p of m.ps) if (p.team !== t && p.role !== 'Bekçi') mx = Math.max(mx, ile(t, p.x)); return mx; }; // t'nin rakibinin en derindeki saha oyuncusu (t'nin yönünde)
const Z = () => ({ pas: 0, pasOk: 0, arka: 0, arkaOk: 0, hatT: 0, hatN: 0, sut: 0, xg: 0, sutGec: 0, xgGec: 0, sutArka: 0, xgArka: 0, sutMes: 0, gol: 0, kayip: 0, kayipKendi: 0, kayipErken: 0, kazanRakip: 0, sahip: 0 });
const S = [Z(), Z()];
let sahip = null, sahipT0 = 0, sahipArka = false, onceki = null, fl0 = m.fl, bekleyen = null, skor = m.score.slice();
while (!m.over && m.tick < LEN) { M.step(m); const h = m.holder;
  if (h) { S[h.team].sahip++; if (m.tick % 10 === 0) { S[h.team].hatT += sonHat(h.team); S[h.team].hatN++; }
    if (bekleyen) { const s = S[bekleyen.t]; if (h.team === bekleyen.t) { s.pasOk++; if (bekleyen.arka) s.arkaOk++; } bekleyen = null; }
    if (sahip !== h.team) { const t = h.team, kim = 1 - t; if (sahip != null) { const ks = S[kim], g = ile(kim, h.x); ks.kayip++; if (g < .5) ks.kayipKendi++; if (m.tick - sahipT0 < 3 * SN) ks.kayipErken++; if (ile(t, h.x) > .5) S[t].kazanRakip++; } sahip = t; sahipT0 = m.tick; sahipArka = false; }
    onceki = h; }
  if (m.fl && m.fl !== fl0) { fl0 = m.fl; const p = onceki; if (p) { const t = p.team, s = S[t];
      if (m.fl.kind === 'gönder') { const pk = m.ball.shotPk ?? 0, gec = m.tick - sahipT0 < 6 * SN; s.sut++; s.xg += pk; s.sutMes += Math.hypot((t === 0 ? GW : 0) - p.x, GH / 2 - p.y); if (gec) { s.sutGec++; s.xgGec += pk; } if (sahipArka) { s.sutArka++; s.xgArka += pk; } }
      else if (m.fl.end) { const arka = ile(t, m.fl.end.x) > sonHat(t) + .02; s.pas++; if (arka) { s.arka++; sahipArka = true; } bekleyen = { t, arka }; } } }
  if (m.score[0] !== skor[0] || m.score[1] !== skor[1]) { for (const t of [0, 1]) S[t].gol += m.score[t] - skor[t]; skor = m.score.slice(); sahip = null; bekleyen = null; } }
const r = x => +x.toFixed(3), cik = s => { const o = {}; for (const [k, v] of Object.entries(s)) o[k] = r(v); return o; };
console.log(JSON.stringify({ a: aAd, b: bAd, seed, A, tik: m.tick, ta: cik(S[A]), tb: cik(S[1 - A]) }));
