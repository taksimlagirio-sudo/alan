// Maç anlatısı: gollerin hikâyesi, gerilim, saçmalık dedektörleri. Skorun ötesinde "maç mantıklı ve ilginç mi" sorusu için.
// Kullanım: node tools/anlati.js <motor> <hücre: canli|stiller|ayna> <i> [tik] → JSON satırı
const { load } = require('./load');
const [dir, cell, iS, lenS] = process.argv.slice(2), i = +iS, len = +lenS || 5400, seed = 9000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch, S = M.STYLES, hyp = Math.hypot;
const LIVE = [{ sistem: 'Alan', pres: 1, arkada: 1, blok: 'Düşük', genislik: 'Dar', tempo: .8, risk: .8, kazaninca: 'Kontra' }, { sistem: 'Adam adama', pres: 2, arkada: 2, blok: 'Yüksek', genislik: 'Geniş', tempo: .2, risk: .2, kazaninca: 'Dengeli' }];
const C = { ayna: { tac: [{}, {}] }, canli: { tac: LIVE }, stiller: { tac: [S['Hücumcu'].tac, S['Savunmacı'].tac], roles: [S['Hücumcu'].roles, S['Savunmacı'].roles] } }[cell];
const m = M.createMatch(seed, { tac: C.tac, roles: C.roles }); m.len = len;
const dirOf = t => t === 0 ? 1 : -1, ownX = t => t === 0 ? 0 : 100, oppX = t => 100 - ownX(t), adv = (t, x) => (x - ownX(t)) * dirOf(t);
const field = t => m.ps.filter(p => p.team === t && p.role !== 'Bekçi'), keeper = t => m.ps.find(p => p.team === t && p.role === 'Bekçi');
const nearOpp = p => Math.min(...field(1 - p.team).map(q => hyp(q.x - p.x, q.y - p.y)));
const sec = t => (t / 60).toFixed(1);
const out = { cell, i, seed, goals: [], score: null, an: {}, ex: {}, danger: [], loose: 0, ticks: 0, leadCh: 0, eq: 0 };
const A = (k, ex) => { out.an[k] = (out.an[k] || 0) + 1; if (ex && (!out.ex[k] || out.ex[k].length < 3)) (out.ex[k] = out.ex[k] || []).push(ex); };
let seq = [], seqTeam = -1, seqStart = null, prevH = null, lastFl = null, holdP = null, holdPressT = 0, looseT = 0, backRun = 0, lastLeader = 0, stackT = new Map(), swarmT = 0;
function startSeq(t, how) { seq = []; seqTeam = t; seqStart = how; }
while (!m.over && m.tick < len) {
  const sc = m.score.slice(); M.step(m); out.ticks++; const h = m.holder, T = m.tick;
  // sayı
  if (m.score[0] !== sc[0] || m.score[1] !== sc[1]) { const t = m.score[0] !== sc[0] ? 0 : 1, own = seqTeam !== t;
    if (own) A('kendi Kuyu\'suna sayı', `${sec(T)} sn`);
    const ld = Math.sign(m.score[0] - m.score[1]); if (ld !== 0 && lastLeader !== 0 && ld !== lastLeader) out.leadCh++; if (ld === 0) out.eq++; if (ld !== 0) lastLeader = ld;
    out.goals.push({ t: sec(T), team: t ? 'Turuncu' : 'Mavi', own, score: m.score.join('–'), start: seqStart, steps: own ? [] : seq.slice(-8), dur: seq.length ? +(T / 60 - +seq[0].t).toFixed(1) : 0, n: seq.length }); startSeq(-1, null); }
  // yola çıkış
  if (m.fl && m.fl !== lastFl && m.fl.t0 === T) { lastFl = m.fl; const p = (m.ball && m.ball.from) || prevH; if (p) { const fl = m.fl, to = fl.to || fl.end, ot = !!(m.ball && m.ball.from && m.ball.from !== prevH);
      if (p.team !== seqTeam) startSeq(p.team, { how: 'tek dokunuşla kesme', by: p.name, at: Math.round(adv(p.team, p.x)) });
      const fwd = adv(p.team, to.x) - adv(p.team, p.x), dec = m.lastDec && m.lastDec.name === p.name ? m.lastDec.opts[0] : null;
      seq.push({ t: sec(T), who: p.name, kind: fl.kind + (fl.shot ? ' ' + fl.shot : ''), to: fl.q ? fl.q.name : '', at: Math.round(adv(p.team, p.x)), dist: Math.round(hyp(to.x - p.x, to.y - p.y)), fwd: Math.round(fwd), ot, press: +nearOpp(p).toFixed(1), P: dec && dec.P != null ? +dec.P.toFixed(2) : null });
      if (fl.kind !== 'gönder') { if (fwd < -2) backRun++; else backRun = 0; if (backRun === 4) A('geri pas zinciri (4+)', `${sec(T)} sn ${p.name}`); }
      if (fl.kind === 'gönder') { const gd = hyp(oppX(p.team) - p.x, 25 - p.y), bk = keeper(1 - p.team), bkOff = hyp(bk.x - oppX(p.team), bk.y - 25); seq[seq.length - 1].gd = Math.round(gd); seq[seq.length - 1].bk = +bkOff.toFixed(1);
        out.danger.push(T); if (dec && dec.P != null && dec.P < .1) A('umutsuz gönderiş (tahmin <%10)', `${sec(T)} sn ${p.name} ${Math.round(gd)} br`); } } }
  // sahiplik
  if (h && h !== prevH) { if (h.team !== seqTeam) startSeq(h.team, prevH && prevH.team !== h.team ? { how: m.ball ? 'pas kesme' : 'ikili mücadele', by: h.name, from: prevH.name, at: Math.round(adv(h.team, h.x)) } : { how: 'boştaki Çekirdek', by: h.name, at: Math.round(adv(h.team, h.x)) }); holdP = h; holdPressT = 0; }
  if (h) { if (nearOpp(h) < 2.5) holdPressT++; else holdPressT = 0; if (holdPressT === 90) A('baskı altında 1,5 sn+ topu bırakmayan', `${sec(T)} sn ${h.name}`); }
  // sahipsiz Çekirdek
  if (!h && m.ball && m.ball.done) { looseT++; out.loose++; if (looseT === 180) A('3 sn+ sahipsiz duran Çekirdek', `${sec(T)} sn (${m.ball.x.toFixed(0)},${m.ball.y.toFixed(0)})`); } else looseT = 0;
  // tehlike: rakip Kuyu'ya 25 birimden yakın top
  if (h && hyp(oppX(h.team) - h.x, 25 - h.y) < 25 && T % 30 === 0) out.danger.push(T);
  // sürü: aynı takımdan 4+ oyuncu topa 8 birim yakın
  if (T % 10 === 0) { const bx = h ? h.x : m.ball.x, by = h ? h.y : m.ball.y; for (const t of [0, 1]) { const n = field(t).filter(p => hyp(p.x - bx, p.y - by) < 8).length; if (n >= 4) { swarmT++; if (swarmT % 30 === 1) A('sürüleşme (bir takımdan 4+ kişi topa 8 br yakın)', `${sec(T)} sn ${t ? 'Turuncu' : 'Mavi'} ${n} kişi`); } }
    // üst üste: iki takım arkadaşı 2,5 birimden yakın, 1 sn+
    for (const t of [0, 1]) { const f = field(t); for (let a = 0; a < f.length; a++) for (let b = a + 1; b < f.length; b++) { const k = f[a].name + f[b].name, near = hyp(f[a].x - f[b].x, f[a].y - f[b].y) < 2.5 && f[a] !== h && f[b] !== h; const v = near ? (stackT.get(k) || 0) + 10 : 0; stackT.set(k, v); if (v === 60) A('üst üste duran iki arkadaş (1 sn+)', `${sec(T)} sn ${f[a].name}-${f[b].name}`); } } }
  if (h) prevH = h;
}
out.score = m.score;
console.log(JSON.stringify(out));
