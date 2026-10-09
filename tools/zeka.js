// Zekâ testleri: oyuncular gerçekten akıllı oynuyor mu?
//  1 · Karar pişmanlığı: her gerçek kararda aynı durum "kâhin"e de çözdürülür (Okuma 20, 3 kat deneme, ayrı zar). Seçilen hamlenin kâhinin ölçüsüne göre kaybı.
//  2 · Topsuz yerleşim: her hücumcunun yeri ile 1 sn'de varabileceği en iyi yer arasındaki fark (pas alabilirlik × oradan topla oynamanın değeri, motorun kendi ölçüsü).
//  3 · Koşular: savunma arkasına koşu, koşuya pas, koşucuyu takip eden savunmacı.
//  4 · Savunma: kayıptan sonra Kuyu tarafına dönüş süresi, tehlikeli bölgede serbest alıcı, gönderende baskı.
// Kullanım: node tools/zeka.js <motor> <hücre: canli|stiller|ayna> <i> [tik] [kâhin: 1|0]
const { load } = require('./load');
const [dir, cell, iS, lenS, orS] = process.argv.slice(2), i = +iS, len = +lenS || 2700, oracle = orS !== '0', seed = 7000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch, AD = g.AlanDecide, D = AD.D, S = M.STYLES, hyp = Math.hypot;
const LIVE = [{ sistem: 'Alan', pres: 1, arkada: 1, blok: 'Düşük', genislik: 'Dar', tempo: .8, risk: .8, kazaninca: 'Kontra' }, { sistem: 'Adam adama', pres: 2, arkada: 2, blok: 'Yüksek', genislik: 'Geniş', tempo: .2, risk: .2, kazaninca: 'Dengeli' }];
const CL = { ayna: { tac: [{}, {}] }, canli: { tac: LIVE }, stiller: { tac: [S['Hücumcu'].tac, S['Savunmacı'].tac], roles: [S['Hücumcu'].roles, S['Savunmacı'].roles] } }[cell];
const m = M.createMatch(seed, { tac: CL.tac, roles: CL.roles }); m.len = len;
const dirOf = t => t === 0 ? 1 : -1, ownX = t => t === 0 ? 0 : 100, oppX = t => 100 - ownX(t), adv = (t, x) => (x - ownX(t)) * dirOf(t);
const field = t => m.ps.filter(p => p.team === t && p.role !== 'Bekçi');
let r0 = 99991; const orng = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296;
const O = { dec: [], pos: [], runs: [], rec: [], recv: [], shoot: [], cover: [] };
// 1 · kâhin
const sig = o => o.kind + '|' + (o.q ? o.q.id : '') + '|' + (o.shot || '');
const od = AD.decide; let busy = false;
if (oracle) AD.decide = function (h, src, ch, rnd, tac) {
  const res = od.call(this, h, src, ch, rnd, tac);
  if (busy || D._inLook || D._ot || !m.ps.includes(h) || src !== m.ps || !res.best) return res;
  busy = true; const save = { trials: D.trials }; try {
    const hc = { ...h, a: { ...h.a, okuma: 20 }, _lc: null }, w = src.map(p => p === h ? hc : p); D.trials = save.trials * 3;
    const ro = od.call(this, hc, w, ch, orng, tac), best = ro.opts[0], find = (L, x) => L.find(o => sig(o) === sig(x) && (!o.to || !x.to || hyp(o.to.x - x.to.x, o.to.y - x.to.y) < 4)), same = find(ro.opts, res.best);
    // gürültü tabanı: ikinci bir kâhin (ayrı zar) aynı durumda ne seçer, ilk kâhinin ölçüsüne göre ne kaybettirir
    const hc2 = { ...h, a: { ...h.a, okuma: 20 }, _lc: null }, w2 = src.map(p => p === h ? hc2 : p), ro2 = od.call(this, hc2, w2, ch, orng, tac), b2 = ro2.opts[0], same2 = b2 && find(ro.opts, b2);
    O.dec.push({ k: res.best.kind, ok: best ? sig(best) === sig(res.best) : null, ob: best ? best.kind : null, reg: same && best ? +(best.v - same.v).toFixed(4) : null, miss: !same, ok2: best && b2 ? sig(best) === sig(b2) : null, reg2: same2 && best ? +(best.v - same2.v).toFixed(4) : null, v: best ? +best.v.toFixed(3) : null, adv: Math.round(adv(h.team, h.x)) });
  } catch (e) { O.dec.push({ err: String(e).slice(0, 80) }); } finally { D.trials = save.trials; busy = false; }
  return res; };
let prevH = null, lastFl = null, lossT = null, lossTeam = -1, runs = new Map();
while (!m.over && m.tick < len) {
  M.step(m); const h = m.holder, T = m.tick;
  // yola çıkış: koşuya pas mı, tehlikeli bölgede serbest alıcı mı, gönderende baskı
  if (m.fl && m.fl !== lastFl && m.fl.t0 === T) { lastFl = m.fl; const p = (m.ball && m.ball.from) || prevH;
    if (p && m.fl.kind === 'gönder') O.shoot.push(+Math.min(...field(1 - p.team).map(q => hyp(q.x - p.x, q.y - p.y))).toFixed(1));
    if (p && m.fl.q) { const r = runs.get(m.fl.q); if (r && !r.done) r.passed = true; } }
  if (h && h !== prevH) {
    if (prevH && h.team === prevH.team && adv(h.team, h.x) > 60) O.recv.push(+Math.min(...field(1 - h.team).map(q => hyp(q.x - h.x, q.y - h.y))).toFixed(1));
    if (prevH && h.team !== prevH.team) { lossT = T; lossTeam = prevH.team; } }
  // 4 · kayıptan sonra dönüş: kaybeden takımın kaç saha oyuncusu topla kendi Kuyu'su arasında
  if (lossT != null && h) { const bx = h.x, n = field(lossTeam).filter(p => adv(lossTeam, p.x) < adv(lossTeam, bx)).length; if (n >= 4 || T - lossT > 300 || h.team === lossTeam) { O.rec.push({ t: +((T - lossT) / 60).toFixed(2), ok: n >= 4, bx: Math.round(adv(lossTeam, bx)) }); lossT = null; } }
  if (h && T % 10 === 0) { const t = h.team, ps = m.ps, ch = m.ch;
    // 2 · yerleşim verimi
    for (const p of field(t)) { if (p === h) continue; const val = (x, y) => AD.laneOk(ps, t, h, { x, y }) * AD.V(ps, t, x, y, ch), cur = val(p.x, p.y); let best = cur; const R = 7;
      for (let dx = -R; dx <= R; dx += 1.75) for (let dy = -R; dy <= R; dy += 1.75) { if (dx * dx + dy * dy > R * R) continue; const x = p.x + dx, y = p.y + dy; if (x < 2 || x > 98 || y < 2 || y > 48) continue; const v = val(x, y); if (v > best) best = v; }
      O.pos.push({ cur: +cur.toFixed(4), best: +best.toFixed(4), adv: Math.round(adv(t, p.x)), job: (p.job || '').split(' · ')[0] }); }
    // 4 · tehlikeli bölgede kapatma: son 35 birimdeki hücumcuların kaçının Kuyu tarafında 6 birimde savunmacısı var
    const dang = field(t).filter(a => a !== h && adv(t, a.x) > 65), D2 = field(1 - t); if (dang.length) O.cover.push(dang.filter(a => D2.some(d => adv(t, d.x) > adv(t, a.x) && hyp(d.x - a.x, d.y - a.y) < 6)).length / dang.length); }
  // 3 · koşular: rakip yarıda Kuyu'ya doğru depar başlatan topsuz hücumcu
  for (const p of m.ps) { if (p.role === 'Bekçi') continue; const att = h ? h.team === p.team : m.ball && m.ball.team === p.team, fw = (p.vx || 0) * dirOf(p.team);
    if (att && p !== h && p.sprint && fw > .15 && adv(p.team, p.x) > 45 && !runs.has(p)) { const Dd = field(1 - p.team), last = Math.max(...Dd.map(d => adv(p.team, d.x))); runs.set(p, { t0: T, x0: adv(p.team, p.x), behindStart: adv(p.team, p.x) > last - 2, behind: false, track: 0, n: 0, passed: false, done: false, job: (p.job || '').split(' · ')[0] }); }
    const r = runs.get(p); if (r && !r.done) { const Dd = field(1 - p.team), last = Math.max(...Dd.map(d => adv(p.team, d.x))); if (adv(p.team, p.x) > last) r.behind = true; r.n++; if (Dd.some(d => hyp(d.x - p.x, d.y - p.y) < 4)) r.track++;
      if (T - r.t0 >= 60 || (!p.sprint && T - r.t0 > 15)) { r.done = true; r.dur = T - r.t0; O.runs.push({ behind: r.behind, track: +(r.track / r.n).toFixed(2), passed: r.passed, job: r.job }); runs.delete(p); } } }
  if (h) prevH = h;
}
console.log(JSON.stringify({ cell, i, score: m.score, O }));
