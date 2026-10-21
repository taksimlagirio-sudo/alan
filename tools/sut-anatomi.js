// Şut anatomisi: her şutun yeri, geldiği yol (topa sahip olmanın süresi, pas sayısı, nerede kazanıldığı, şutçuya nasıl geldiği), şut anında savunma (kaleyle şutçu arasındaki savunmacı, şut çizgisini kapatan, en yakın savunmacı) ve sonucu.
// Düzen blok-olc.js'inki (B_onde, B_hepsi, taktik-ayar tarzları). Kullanım: node tools/sut-anatomi.js <motor> <a> <b> <tohum> [tik=9000]
const fs = require('fs'), path = require('path'); const { V, VD, AYAR } = require('./taktik-ayar'); const [dir, aAd, bAd, sS, lS] = process.argv.slice(2), seed = +sS, LEN = +lS || 9000;
const EK = {
  B_onde: { blok: 'Düşük', pres: 0, siddet: .3, tempo: .9, risk: .8, gecisH: 'her', kpYer: 'hic', sabir: .2, pasSurme: .3, _diz: { sav: '3-2-1', huc: '1-2-3' }, onde: 1 },
  B_hepsi: { blok: 'Düşük', pres: 0, siddet: .3, tempo: .9, risk: .8, gecisH: 'her', kpYer: 'hic', sabir: .2, pasSurme: .3, _diz: { sav: '3-2-1', huc: '1-2-3' } }
};
function rng(sd) { let x = sd >>> 0; return () => { x = (x + 0x6D2B79F5) | 0; let t = Math.imul(x ^ x >>> 15, 1 | x); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
for (const k of (process.env.BAYRAK || "").split(",").filter(Boolean)) globalThis[k] = true; /* deney bayrakları (self.ALAN_..._ESKI) */
Math.random = rng(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of ['sabit-mat.js', 'core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js']) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const M = globalThis.AlanMatch, kur = ad => { const { _diz, ...t } = EK[ad] || AYAR[ad]; return { tac: { ...V, tetik: V.tetik.slice(), ...t }, diz: _diz || VD }; };
const A = seed % 2, K = A === 0 ? [kur(aAd), kur(bAd)] : [kur(bAd), kur(aAd)];
const m = M.createMatch(seed * 7919 + 17, { tac: K.map(k => k.tac), len: LEN, diz: K.map(k => k.diz) });
for (const p of m.ps) { const a = {}; for (const k of ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', 'enerji', 'refleks', 'yerlesim', 'hakimiyet']) a[k] = 10; a.taraf = (p.a && p.a.taraf) || 'sag'; p.a = a; }
const GW = 120, GH = 60, GCY = 30, kale = t => t === 0 ? GW : 0, fx = (t, x) => t === 0 ? x : GW - x; /* fx: takımın kendi Kuyu'sundan uzaklık (0 → 120) */
const S = []; let fl0 = m.fl, hT = null, pos = null, sh = null;
const bitir = son => { sh.son = son; S.push(sh); sh = null; };
while (!m.over && m.tick < LEN) { const sk = m.score.slice(), h0 = m.holder; M.step(m); const h = m.holder, b = m.ball;
  if (h && (!pos || pos.t !== h.team)) pos = { t: h.team, t0: m.tick, x0: fx(h.team, h.x), pas: 0, nasil: null };
  if (h && h !== h0) { pos.son = h; pos.alT = m.tick; pos.alNasil = h0 === null && pos.sonPas ? pos.sonPas : 'kapma/sekme'; }
  if (sh) { if (m.score[0] !== sk[0] || m.score[1] !== sk[1]) bitir('gol'); else if (h) bitir(h.role === 'Bekçi' ? 'Bekçi tuttu' : h.team === sh.t ? 'bizden biri aldı' : 'savunmacı kesti'); else if (b && b.done && b.done !== 'kuyu-sol' && b.done !== 'kuyu-sag') bitir(b.done); else if (m.tick - sh.tick > 150) bitir('süre'); }
  if (m.fl && m.fl !== fl0) { fl0 = m.fl; const q = m.fl.q;
    if (['pas', 'önüne', 'kenardan'].includes(m.fl.kind) && pos) { pos.pas++; pos.sonPas = m.fl.kind; }
    if (m.fl.kind === 'gönder' && b && b.shotPk != null && pos) { const t = pos.t, s = pos.son || h0; if (!s) continue; const gx = kale(t), dx = gx - s.x, dy = GCY - s.y, L = Math.hypot(dx, dy) || 1;
      let arada = 0, cizgi = 0, yakin = 1e9; for (const p of m.ps) { if (p.team === t || p.role === 'Bekçi') continue; const d = Math.hypot(p.x - s.x, p.y - s.y); yakin = Math.min(yakin, d); const u = ((p.x - s.x) * dx + (p.y - s.y) * dy) / L; if (u > 0 && u < L) { arada++; const v = Math.abs((p.x - s.x) * dy - (p.y - s.y) * dx) / L; if (v < 3) cizgi++; } }
      const savGeri = m.ps.filter(p => p.team !== t && p.role !== 'Bekçi' && Math.abs(p.x - gx) < Math.abs(s.x - gx)).length;
      sh = { tick: m.tick, t, tarz: t === A ? aAd : bAd, uz: +L.toFixed(1), aci: +(Math.atan2(Math.abs(dy), Math.abs(dx)) * 180 / Math.PI).toFixed(0), pk: +b.shotPk.toFixed(2), ch: +(b.ch ?? 0).toFixed(2), sure: +((m.tick - pos.t0) / 30).toFixed(1), pas: pos.pas, kazX: +pos.x0.toFixed(0), tutT: +((m.tick - (pos.alT ?? pos.t0)) / 30).toFixed(1), geldi: pos.alNasil || 'başta', savGeri, arada, cizgi, yakin: +yakin.toFixed(1), son: null }; } } }
console.log(JSON.stringify({ a: aAd, b: bAd, seed, A, gol: [m.score[A], m.score[1 - A]], S }));
