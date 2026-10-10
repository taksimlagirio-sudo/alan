// v4g · top saklama ve savunma zekâsı (Design'ın görevi 6). Beyin açık bir maç oynatılır, şunlar ölçülür:
//  Top saklama: taşıyıcının dibinde (≤ 2,5 birim) rakip varken geçen süreler; kayba kadar geçen süre; ikili sayısı ve savunmacının kazanma oranı
//               ikilideki koşullara göre: Çekirdek gövdeyle korunuyor mu (shielded), savunmacının açısı (top tarafından / yandan / gövde tarafından: Çekirdeğin taşıyıcının hangi yanında durduğuna göre),
//               Sürme − Kesme farkı, ikinci presçi (taşıyıcıya ≤ 4 birim ikinci rakip) var mı
//  Kapma: ikili fırsatlarının (savunmacı menzilde, beyne soruldu) kaçında giriliyor, girilenlerin kaçı kazanılıyor
//  Baskı: taşıyıcı topu aldıktan sonra ilk rakibin 2,5 birime gelme süresi; taktiğin pres ayarına göre
//  Yerleşim (rakip topa sahipken): Çekirdek ile kendi Kuyu'su arasında (hatta ≤ 3 birim) en az bir savunmacı olan tik oranı;
//               en tehlikeli rakibin (Kuyu'ya en yakın rakip saha oyuncusu) 4 birim içinde savunmacı olmadan geçen tik oranı;
//               iki savunmacının ≤ 3 birim üst üste durduğu tik oranı; beynin yerleşim kararlarında refleks yeri dışı seçim oranı
// Kullanım: node tools/savunma.js <motor> <i> [tik=5400]
const fs = require('fs'), { load } = require('./load');
const [dir, iS, lS] = process.argv.slice(2), i = +iS, LEN = +lS || 5400, seed = 210000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch, B = g.AlanBeyin, DD = g.AlanDecide, K = g.AlanCore; (0, eval)(fs.readFileSync(dir + '/vs-table.js', 'utf8')); (0, eval)(fs.readFileSync(dir + '/pas-table.js', 'utf8'));
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const tacOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] }), ks = Object.keys(STY);
const m = M.createMatch(seed, { tac: [tacOf(STY[ks[i % 6]]), tacOf(STY[ks[(i * 5 + 3) % 6]])], len: LEN });
const hyp = Math.hypot, S = { firsat: 0, girdi: 0, ikili: [], basinc: [], yer: { n: 0, arada: 0, tehlikeBos: 0, ustUste: 0 }, yerKarar: { n: 0, disi: 0 }, kayip: [] };
// ikili fırsatları: beynin komut noktası sarılır
let son = null; const ik0 = B.ikili; B.ikili = function (mm, q, h, P, est) { const r = ik0.call(this, mm, q, h, P, est); if (mm === m) { S.firsat++; if (r) { S.girdi++;
      const cs = DD.coreSide(h), qx = q.x - h.x, qy = q.y - h.y, qd = hyp(qx, qy) || 1, cosA = (qx * cs.ux + qy * cs.uy) / qd, co = M.corePos(m), shd = co ? K.Turn.shielded(h, q, co.x, co.y) : null;
      const ikinci = m.ps.some(o => o !== q && o.team === q.team && o.role !== 'Bekçi' && hyp(o.x - h.x, o.y - h.y) <= 4);
      son = { P: +P.toFixed(3), aci: cosA > .5 ? 'top tarafından' : cosA < -.5 ? 'gövde tarafından' : 'yandan', korunuyor: shd, fark: (h.a.surme ?? 10) - (q.a.kesme ?? 10), ikinci, kazandi: null, tick: m.tick }; S.ikili.push(son); } } return r; };
let hPrev = null, alT = 0, ilkBas = null, dipT = null;
while (!m.over && m.tick < LEN) { let p = B.planla(m); while (p) { if (p.tip === 'yer') { const b0 = p.bitir; p.bitir = function (res) { const r = b0.call(this, res); const s = m._beyinSon; if (s && s.tip === 'yer') { S.yerKarar.n++; if (s.secilen !== 0) S.yerKarar.disi++; } return r; }; } p = p.bitir(p.jobs.map(j => B.rollout(m, j))) || null; }
  const dw0 = m.st.duelW.slice(); M.step(m);
  if (son && son.kazandi == null && son.tick === m.tick) { son.kazandi = (m.st.duelW[0] + m.st.duelW[1]) > (dw0[0] + dw0[1]); }
  const h = m.holder;
  if (h !== hPrev) { if (hPrev && dipT != null) S.kayip.push(m.tick - dipT); if (h) { alT = m.tick; ilkBas = null; dipT = null; } hPrev = h; }
  if (h) { const t = h.team, opp = m.ps.filter(o => o.team !== t && o.role !== 'Bekçi'), nd = Math.min(...opp.map(o => hyp(o.x - h.x, o.y - h.y)));
    if (nd <= 2.5) { if (ilkBas == null) { ilkBas = m.tick - alT; S.basinc.push({ pres: m.tac[1 - t].pres, t: ilkBas }); } if (dipT == null) dipT = m.tick; } else dipT = null;
    // yerleşim (savunan takım: 1 − t)
    const gx = t === 0 ? 100 : 0, dx = gx - h.x, dy = 25 - h.y, L = hyp(dx, dy) || 1; S.yer.n++;
    if (opp.some(o => { const u = ((o.x - h.x) * dx + (o.y - h.y) * dy) / L; return u > 0 && u < L && Math.abs(-(o.x - h.x) * dy + (o.y - h.y) * dx) / L <= 3; })) S.yer.arada++;
    const att = m.ps.filter(o => o.team === t && o.role !== 'Bekçi' && o !== h).sort((a, b) => hyp(a.x - gx, a.y - 25) - hyp(b.x - gx, b.y - 25))[0];
    if (att && !opp.some(o => hyp(o.x - att.x, o.y - att.y) <= 4)) S.yer.tehlikeBos++;
    let uu = false; for (let a = 0; a < opp.length && !uu; a++) for (let b = a + 1; b < opp.length; b++) if (hyp(opp[a].x - opp[b].x, opp[a].y - opp[b].y) <= 3) { uu = true; break; } if (uu) S.yer.ustUste++; } }
console.log(JSON.stringify({ i, tac: [ks[i % 6], ks[(i * 5 + 3) % 6]], skor: m.score, firsat: S.firsat, girdi: S.girdi, ikili: S.ikili, basinc: S.basinc, kayip: S.kayip, yer: S.yer, yerKarar: S.yerKarar }));
