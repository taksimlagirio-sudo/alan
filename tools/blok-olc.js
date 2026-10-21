// Blok + kontra ölçümü: derin blok, bir oyuncu önde bekler (savunmaya gelmez) — Dikine'ye karşı. enerji-olc.js'in düzeni.
// Önde bekleyen: motorun kendi ayarı tac.onde = 1 (shape.js: savunma dizilişinin en öndeki oyuncusu 'önde bekle', rakibin son adamının 3 br önünde; pres ve geri koşuya katılmaz).
// Ek ölçüler: geçiş şutu (topu kazandıktan sonra 6 sn içinde), en öndekinin (savunma dizilişinde en ileri slot; B_onde'de önde bekleyen) ortalama yeri (top rakipteyken, f 0–100), top kazanılınca ilk pasın ona gitme payı.
// Kullanım: [ONDE_OZ=hiz:16] node tools/blok-olc.js <motor> <a> <b> <tohum> [tik=9000]
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
for (const t of [0, 1]) M.dizUygula(m, t, K[t].diz);
const onde = [null, null];
for (const t of [0, 1]) { const ps = m.ps.filter(p => p.team === t && p.role !== 'Bekçi'), o = ps.reduce((a, b) => b.slot.d > a.slot.d ? b : a); onde[t] = o; 
}
if (process.env.ONDE_OZ) { const [k, v] = process.env.ONDE_OZ.split(':'); onde[A].a[k] = +v; } /* A'nın en öndeki oyuncusuna tek özellik (ör. ONDE_OZ=hiz:16) */
const R = [0, 1].map(() => ({ en: 0, n: 0, dip: 0, xg: 0, sh: 0, gecis: 0, gecisXg: 0, ondeF: 0, ondeN: 0, ilk: 0, ilkOnde: 0, gen: 0, genN: 0, savGen: 0, kanatSut: 0, sutN: 0 }));
const DT = []; let fl0 = m.fl, onceki = null, win = [-1e9, -1e9], ilkBek = [false, false];
while (!m.over && m.tick < LEN) { const h0 = m.holder; M.step(m); const h = m.holder;
  if (h) { R[h.team].sh++; if (!onceki || onceki.team !== h.team) { win[h.team] = m.tick; ilkBek[h.team] = true; } onceki = h; }
  if (m.fl && m.fl !== fl0) { fl0 = m.fl; const b = m.ball;
    if (onceki && b) { const t = onceki.team;
      if (m.fl.kind === 'gönder' && b.shotPk != null) { R[t].xg += b.shotPk; R[t].sutN++; if (Math.abs(b.y - 30) > 12) R[t].kanatSut++; if (m.tick - win[t] <= 6 * 30) { R[t].gecis++; R[t].gecisXg += b.shotPk; } }
      else if (ilkBek[t] && ['pas', 'önüne', 'kenardan'].includes(m.fl.kind)) { ilkBek[t] = false; R[t].ilk++; if (m.fl.q === onde[t]) { R[t].ilkOnde++; if (process.env.DETAY) DT.push({ t0: m.tick, tm: t, q: onde[t], x0: (t === 0 ? b.x : 120 - b.x), son: null }); } } } }
  if (process.env.DETAY) for (const d of DT) { if (d.son) continue; const dt = m.tick - d.t0; if (d.al == null) { if (h === d.q) { d.al = dt; d.alX = +((d.tm === 0 ? h.x : 120 - h.x)).toFixed(0); } else if (h && h.team !== d.tm) d.son = 'pas kesildi'; else if (h && h !== d.q) d.son = 'başkası aldı'; else if (dt > 150) d.son = 'pas sönmüş/boş'; } else { if (m.fl && m.fl.t0 === m.tick && m.fl.kind === 'gönder' && m.ball && m.ball.shotPk != null && onceki && onceki.team === d.tm) d.son = 'şut'; else if (h && h.team !== d.tm) d.son = 'kayıp' + (dt - d.al < 60 ? ' (2 sn içinde)' : ''); else if (dt > 300) d.son = '10 sn şutsuz'; } }
  if (process.env.IZ && m.tick % 300 === 0) console.error(m.tick, onde.map(o => o && (o.team === 0 ? o.x : 120 - o.x).toFixed(0)), m.ps.filter(p => p.job === 'önde bekle').length, h ? h.team : '-');
  if (h) { const yay = t => { const ys = m.ps.filter(p => p.team === t && p.role !== 'Bekçi').map(p => p.y); return Math.max(...ys) - Math.min(...ys); }; R[h.team].gen += yay(h.team); R[h.team].savGen += yay(1 - h.team); R[h.team].genN++; } /* top bizdeyken: kendi yayılımımız ve rakip savunmanın yayılımı (y, saha 60) */
  for (const t of [0, 1]) { const o = onde[t]; if (o && (!h || h.team !== t)) { R[t].ondeF += (t === 0 ? o.x : 120 - o.x) / 120 * 100; R[t].ondeN++; } }
  for (const p of m.ps) { if (p.role === 'Bekçi') continue; const r = R[p.team]; r.en += p.en ?? 1; r.n++; if ((p.en ?? 1) < .15) r.dip++; } }
const st = m.st, ab = x => x ? [x[A], x[1 - A]] : null, top = R[0].sh + R[1].sh || 1, T = t => ({ en: +(R[t].en / R[t].n).toFixed(3), dip: +(R[t].dip / R[t].n).toFixed(3), xg: +R[t].xg.toFixed(3), sahip: +(R[t].sh / top).toFixed(3), gecis: R[t].gecis, gecisXg: +R[t].gecisXg.toFixed(3), ondeF: R[t].ondeN ? +(R[t].ondeF / R[t].ondeN).toFixed(1) : null, ilk: R[t].ilk, ilkOnde: R[t].ilkOnde, gen: R[t].genN ? +(R[t].gen / R[t].genN).toFixed(1) : null, savGen: R[t].genN ? +(R[t].savGen / R[t].genN).toFixed(1) : null, kanatSut: R[t].kanatSut, sutN: R[t].sutN, enH: st.enH ? st.enH[t] : null });
if (process.env.DETAY) console.error(JSON.stringify(DT.map(d => ({ x0: +d.x0.toFixed(0), al: d.al, alX: d.alX, son: d.son || 'süre' }))));
console.log(JSON.stringify({ a: aAd, b: bAd, seed, A, tik: m.tick, gol: ab(m.score), sut: ab(st.shot), pas: ab(st.pass), kapma: ab(st.steal), ta: T(A), tb: T(1 - A) }));
