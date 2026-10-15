// Gönderme ikiz izi: maçtaki her göndermede, top ayaktan çıktıktan hemen sonra
//   akıl: aynı top (aynı hız, isabet hatası yok) aklın kafasındaki denemeyle (imagine) N kez oynatılır → girme oranı, Bekçi'nin yolu
//   maç : maçın kopyası K farklı zarla oynatılır → girme oranı, Bekçi'nin yolu
// Aynı an, aynı top: fark yalnız dünyanın (Bekçi, savunmacılar, temas) kafada nasıl oynatıldığından gelir.
// Kullanım: node tools/gonder-ikiz.js <motor> <senaryo> <çıktı.json> [NA=30] [KM=8] [LEN]
const fs = require('fs'); const { kur } = require('./kurulum');
const [dir, sS, out, naS, kmS] = process.argv.slice(2), NA = +naS || 30, KM = +kmS || 8;
const { M, m, adim, g } = kur(dir, +sS, 0, {
  'match.js': [['m.ball.shotPk = o.det ? o.det.Pk : null;', 'm.ball.shotPk = o.det ? o.det.Pk : null; if (self.__atis && !m.lite && !m._lite) self.__atis(m, h, o);']],
  'decide.js': [['const ox = b.x, oy = b.y; K.stepBall(b, sk()); if (b.done) break;', 'const ox = b.x, oy = b.y; K.stepBall(b, sk()); if (self.__iz) self.__iz(t, b, seen, depth, h); if (b.done) break;']] });
const D = g.AlanDecide, LEN = +process.env.LEN || m.len || 7200;
let bekle = null; g.__atis = (mm, h, o) => { if (mm === m) bekle = { id: h.id, Pk: o.det ? o.det.Pk : null, shot: o.shot, t: mm.tick, team: h.team }; };
const bekciOf = (ps, team) => ps.find(p => p.role === 'Bekçi' && p.team !== team);
const R = [];
// Maç kopyasında bir zarla: gol mü, Bekçi'nin ve topun yolu
function gercek(snap, team, seed, iz) {
  const c = M.kopya(snap); c.r.s = seed; c.lite = true; const s0 = c.score[team], s1 = c.score[1 - team], yol = [];
  for (let t = 0; t < 120 && !c.over; t++) { M.step(c); const bk = bekciOf(c.ps, team), b = c.ball; if (iz && bk) yol.push([+bk.x.toFixed(2), +bk.y.toFixed(2), b ? +b.x.toFixed(2) : null, b ? +b.y.toFixed(2) : null]);
    if (c.score[team] > s0) return { gol: 1, t, yol, son: 'gol' }; if (c.score[1 - team] > s1) return { gol: 0, t, yol, son: 'karsi' };
    if (c.holder) return { gol: 0, t, yol, son: c.holder.team === team ? 'biz' : (c.holder.role === 'Bekçi' ? 'bekci' : 'rakip') }; if (!c.ball || !c.ball.gon) return { gol: 0, t, yol, son: 'serbest' }; }
  return { gol: 0, t: 120, yol, son: 'sure' };
}
// Aklın kafasında: aynı top, isabet hatası yok
function akil(snap, idH, deneme, rnd, iz) {
  const c = M.kopya(snap), h = c.ps.find(p => p.id === idH), b = c.ball; if (!h || !b) return null; const yol = [];
  const o0 = D.D._ot, g0 = D.D._gon; D.D._ot = 1e-9; D.D._gon = true; g.__iz = iz ? (t, bb, seen, depth, hh) => { if (depth) return; const bk = bekciOf(seen, hh.team); if (bk) yol.push([+bk.x.toFixed(2), +bk.y.toFixed(2), +bb.x.toFixed(2), +bb.y.toFixed(2)]); } : null;
  let im; try { im = D.imagine(h, c.ps, { x: b.x, y: b.y, vx: b.vx, vy: b.vy, ch: b.ch }, rnd, deneme); } finally { D.D._ot = o0; D.D._gon = g0; g.__iz = null; }
  return { Pk: im.Pk, P: im.P, Pl: im.Pl, yol };
}
const t0 = Date.now();
while (!m.over && m.tick < LEN) {
  adim();
  if (bekle && m.ball && m.ball.gon) { const a = bekle; bekle = null; const snap = M.kopya(m), h = m.ps.find(p => p.id === a.id), W = m.saha ? m.saha.W : (g.ALAN_SAHA && g.ALAN_SAHA.W) || 120, H = (g.ALAN_SAHA && g.ALAN_SAHA.H) || 60, gx = a.team === 0 ? W : 0;
    const bk = bekciOf(m.ps, a.team), b = m.ball, mes = Math.hypot(gx - b.x, H / 2 - b.y), aci = Math.atan2(Math.abs(H / 2 - b.y), Math.abs(gx - b.x));
    let s = (m.tick * 2654435761) >>> 0; const rnd = () => (s = (Math.imul(s, 1664525) + 1013904223) >>> 0) / 4294967296;
    const A = akil(snap, a.id, NA, rnd, false); let gg = 0; const son = {}; for (let k = 0; k < KM; k++) { const r = gercek(snap, a.team, (m.tick * 7919 + k * 104729) >>> 0, false); gg += r.gol; son[r.son] = (son[r.son] || 0) + 1; }
    let s2 = 777; const r1 = () => (s2 = (Math.imul(s2, 1664525) + 1013904223) >>> 0) / 4294967296; const iA = akil(snap, a.id, 1, r1, true), iG = gercek(snap, a.team, (m.tick * 31) >>> 0, true);
    R.push({ t: m.tick, team: a.team, shot: a.shot, Pk: a.Pk, mes: +mes.toFixed(1), aci: +aci.toFixed(2), v: +Math.hypot(b.vx, b.vy).toFixed(3), bk0: bk ? [+bk.x.toFixed(2), +bk.y.toFixed(2), +Math.hypot(bk.vx || 0, bk.vy || 0).toFixed(3)] : null, top0: [+b.x.toFixed(2), +b.y.toFixed(2)], akilPk: +A.Pk.toFixed(3), gercekP: +(gg / KM).toFixed(3), son, izA: iA && iA.yol, izAPk: iA && iA.Pk, izG: iG.yol, izGson: iG.son, izGgol: iG.gol });
    process.stdout.write(`t ${m.tick} mes ${mes.toFixed(1)} Pk ${a.Pk != null ? a.Pk.toFixed(2) : '-'} akıl ${A.Pk.toFixed(2)} maç ${(gg / KM).toFixed(2)} ${JSON.stringify(son)}\n`); }
}
fs.writeFileSync(out, JSON.stringify({ sc: +sS, skor: m.score, sn: Math.round((Date.now() - t0) / 1000), R }));
