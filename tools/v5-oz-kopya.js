// Özelliğin kopyalı ölçümü: aynı an, aynı zar, yalnız Liman'ın (takım 0) saha oyuncularında tek özellik değişir (6 / 10 / 14).
// Taban: iki takımda bütün özellikler 10. Maç oynanır; her ADIM tikte (top birindeyse) an kopyalanır, her kolda R zarla UFUK tik oynatılır.
// Ölçülen (kol başına, zarlar üzerinden): Liman'ın ilk kararının türü, Liman topu tutarken sürme payı, Liman şutu, gol (atılan/yenen),
//   ufkun sonunda top kimde, Liman taşıyıcısının ilerlemesi. Kullanım: node tools/v5-oz-kopya.js <motor> <senaryo> <özellik> [ADIM=300] [R=3] [UFUK=200]
const { kur } = require('./kurulum'); const [dir, sS, oz, aS, rS, uS] = process.argv.slice(2), sc = +sS, ADIM = +aS || 300, R = +rS || 3, UFUK = +uS || 200, KOL = (process.env.KOL || '6,10,14').split(',').map(Number);
const { M, m, adim, g } = kur(dir, sc, 0); for (const p of m.ps) for (const k of Object.keys(p.a)) if (typeof p.a[k] === 'number') p.a[k] = 10;
const ileri = (t, x) => t === 0 ? x : 100 - x;
function kol(snap, v, r) { const c = M.kopya(snap); for (const p of c.ps) if (p.team === 0 && p.role !== 'Bekçi') p.a = { ...p.a, [oz]: v }; c.r.s = (r * 2654435761 + 12345) >>> 0;
  const s0 = c.st.shot.slice(), g0 = c.score.slice(); let ld = c.lastDec, ilk = null, tut = 0, sur = 0, ilerle = null;
  for (let t = 0; t < UFUK && !c.over; t++) { M.step(c); if (c.lastDec && c.lastDec !== ld) { ld = c.lastDec; if (ilk == null && ld.team === 0 && ld.opts[0]) ilk = ld.opts[0].kind; }
    const h = c.holder; if (h && h.team === 0) { tut++; if (Math.hypot(h.vx || 0, h.vy || 0) > .04) sur++; ilerle = ileri(0, h.x); } }
  const h = c.holder; return { ilk, tut, sur, sut: c.st.shot[0] - s0[0], sutR: c.st.shot[1] - s0[1], at: c.score[0] - g0[0], ye: c.score[1] - g0[1], son: h ? h.team : -1, ilerle }; }
while (!m.over && m.tick < (+process.env.LEN || 5400)) { adim(); if (m.tick % ADIM || !m.holder) continue; const snap = M.kopya(m), kim = m.holder.team, out = { sc, oz, t: m.tick, kim, x: +ileri(0, m.holder.x).toFixed(1), kol: {} };
  for (const v of KOL) out.kol[v] = Array.from({ length: R }, (_, r) => kol(snap, v, r)); console.log(JSON.stringify(out)); }
