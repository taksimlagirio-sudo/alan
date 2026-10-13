// İkiz iz: aynı pasın kafadaki oyunu (simOne, ilk deneme) ile gerçek maçı tik tik yan yana koyar. Çekirdek ve her oyuncu için iki dünya arasındaki farkı verir.
// Yalnız ölçüm: decide.js'e kayıt satırı eklenir (akış değişmez). Kullanım: node tools/v5-ikiz.js <motor> <senaryo> <tik,tik,...>  (tik: kalib3'ün t'si)
const { kur } = require('./kurulum'); const [dir, sS, tS] = process.argv.slice(2), sc = +sS, TS = tS.split(',').map(Number);
const yama = { 'match.js': ['m.fl = { t0: m.tick, kind: o.kind,', 'm.fl = { o, t0: m.tick, kind: o.kind,'],
  'decide.js': [['  function imagine(h, src, b0, rnd, trials, q, to) {', '  function imagine(h, src, b0, rnd, trials, q, to) { self.__IZA = self.__IZL && (q ? q.id : null) === self.__IZQ ? Object.assign([], { v: [b0.vx, b0.vy], P: null }) : null; if (self.__IZA) self.__IZL.push(self.__IZA); const __izA = self.__IZA;'],
    ['let hit = null; const qq', 'const __iz = depth === 0 && self.__IZA && !self.__IZA._bas ? self.__IZA : null; if (__iz) __iz._bas = 1; let hit = null; const qq'],
    ['const ox = b.x, oy = b.y; K.stepBall(b, sk()); if (b.done) break;', 'const ox = b.x, oy = b.y; K.stepBall(b, sk()); if (__iz) __iz.push({ b: { x: b.x, y: b.y }, ps: seen.map(p => ({ id: p.id, x: p.x, y: p.y })) }); if (b.done) break;'],
    ['    return { P: mate / trials, Pk: kuyu / trials, Pl: opp / trials, end: { x: ex / trials, y: ey / trials } };', '    if (__izA) __izA.P = mate / trials; return { P: mate / trials, Pk: kuyu / trials, Pl: opp / trials, end: { x: ex / trials, y: ey / trials } };'],
    ['    const X = { x: b.x, y: b.y };', '    const X = { x: b.x, y: b.y }; if (__iz) __iz.hit = hit ? hit.id : null;']] };
const { M, m, adim, g } = kur(dir, sc, 0, yama); const AD = g.AlanDecide;
const ayni = (x, o) => x.kind === o.kind && !!x.kendine === !!o.kendine && (x.q ? x.q.id : null) === (o.q ? o.q.id : null) && (!o.to || (x.to && Math.hypot(x.to.x - o.to.x, x.to.y - o.to.y) < .5));
const r1 = v => Math.round(v * 10) / 10;
let once = null;
for (const T of TS) { while (m.tick < T - 1) adim(); once = M.kopya(m); adim(); const o = m.fl && m.fl.o; if (!o || !o.launch) { console.log(JSON.stringify({ sc, T, hata: 'pas yok' })); continue; }
  const team = m.ball.team, hid = (o.kendine ? o.q : m.ball.from).id;
  // kafadaki oyun: aynı andan aynı hamle için decide (okuma 20 → algı hatası yok)
  const c0 = M.kopya(once), h0 = c0.ps.find(p => p.id === hid); g.__IZL = []; g.__IZQ = o.q ? o.q.id : null;
  try { AD.decide(h0, c0.ps, c0.ch, M.kopya(c0.r), M.tacEff(c0, team)); } finally { g.__IZA = null; }
  const L = g.__IZL; g.__IZL = null; let hayal = [], vd = 1e9; for (const a of L) { const d = Math.hypot(a.v[0] - o.launch.vx, a.v[1] - o.launch.vy); if (d < vd) { vd = d; hayal = a; } }
  // gerçek: m'in kendisi pastan sonra
  const c = M.kopya(m), ger = []; for (let t = 0; t < 240 && !c.holder && !c.over; t++) { M.step(c); ger.push({ b: c.ball ? { x: c.ball.x, y: c.ball.y } : null, ps: c.ps.map(p => ({ id: p.id, x: p.x, y: p.y })) }); }
  const alan = c.holder, ad = id => m.ps.find(p => p.id === id).name, alici = o.q ? o.q.id : null;
  // hizalama: gerçeğin ilk Çekirdek konumuna en yakın hayal adımı
  let kay = 0, bd = 1e9; for (let k = 0; k < Math.min(4, hayal.length); k++) { const d = ger[0] && ger[0].b ? Math.hypot(hayal[k].b.x - ger[0].b.x, hayal[k].b.y - ger[0].b.y) : 1e9; if (d < bd) { bd = d; kay = k; } }
  const sat = []; let ilkB = null, ilkO = null; for (let t = 0; t < ger.length; t++) { const hz = hayal[t + kay], gr = ger[t]; if (!hz || !gr.b) break; const db = Math.hypot(hz.b.x - gr.b.x, hz.b.y - gr.b.y); if (ilkB == null && db > 1) ilkB = t;
    let en = null; for (const p of gr.ps) { const q = hz.ps.find(x => x.id === p.id); if (!q) continue; const d = Math.hypot(p.x - q.x, p.y - q.y); if (!en || d > en.d) en = { id: p.id, d }; } if (ilkO == null && en && en.d > 1.5) ilkO = { t, ad: ad(en.id), d: r1(en.d) };
    if (t % 6 === 0 || t === ger.length - 1) { const a = alan && hz.ps.find(x => x.id === alan.id), ar = alan && gr.ps.find(x => x.id === alan.id); sat.push(`${t}: top G(${r1(gr.b.x)},${r1(gr.b.y)}) H(${r1(hz.b.x)},${r1(hz.b.y)})` + (a && ar ? ` · ${alan.name} G(${r1(ar.x)},${r1(ar.y)}) H(${r1(a.x)},${r1(a.y)})` : '') + (alici != null && (!alan || alici !== alan.id) ? (() => { const a2 = hz.ps.find(x => x.id === alici), g2 = gr.ps.find(x => x.id === alici); return a2 && g2 ? ` · alıcı ${ad(alici)} G(${r1(g2.x)},${r1(g2.y)}) H(${r1(a2.x)},${r1(a2.y)})` : ''; })() : '') + ` · en büyük fark ${ad(en.id)} ${r1(en.d)}`); } }
  console.log(JSON.stringify({ sc, T, tur: o.kendine ? 'kendine' : o.kind, P: o.det.P, hayalAdim: hayal.length, hayalP: hayal.P, hizFarki: +vd.toFixed(4), aday: L.length, kay, gercekAdim: ger.length, hayalAlan: hayal.hit != null ? ad(hayal.hit) : 'kimse', alan: alan ? `${alan.name} (${alan.team === team ? 'bizden' : 'rakip'})` : null, topIlkAyrilma: ilkB, oyuncuIlkAyrilma: ilkO, iz: sat })); }
