// Davranış analizi: bir maçı oynatır; kararları, pasları, gönderişleri, topa sahip olma dizilerini, savunma ve hücum şeklini
// "taktik etiketi" (A/B) bazında kaydeder. Skorun ötesine bakmak için.
// Kullanım: node tools/davranis.js <motor> <hücre: canli|stiller> <i> [tik]  → JSON satırı
const { load } = require('./load');
const [dir, cell, iS, lenS] = process.argv.slice(2), i = +iS, len = +lenS || 5400, seed = 5000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch, S = M.STYLES, hyp = Math.hypot;
const LIVE = [{ sistem: 'Alan', pres: 1, arkada: 1, blok: 'Düşük', genislik: 'Dar', tempo: .8, risk: .8, kazaninca: 'Kontra' }, { sistem: 'Adam adama', pres: 2, arkada: 2, blok: 'Yüksek', genislik: 'Geniş', tempo: .2, risk: .2, kazaninca: 'Dengeli' }];
const C = { canli: { tac: LIVE }, stiller: { tac: [S['Hücumcu'].tac, S['Savunmacı'].tac], roles: [S['Hücumcu'].roles, S['Savunmacı'].roles] } }[cell];
const sw = i % 2 === 1, pick = v => v && (sw ? [v[1], v[0]] : v), lab = t => (t === 0) !== sw ? 'A' : 'B'; // A = hücrenin ilk taktiği
const m = M.createMatch(seed, { tac: pick(C.tac), roles: pick(C.roles) }); m.len = len;
const dirOf = t => t === 0 ? 1 : -1, ownX = t => t === 0 ? 0 : 100, oppX = t => 100 - ownX(t);
const zone = (t, x) => { const d = (x - ownX(t)) * dirOf(t); return d < 33 ? 'kendi' : d < 67 ? 'orta' : 'rakip'; };
const R = { A: null, B: null }; for (const k of ['A', 'B']) R[k] = { dec: {}, decZone: {}, launch: [], shots: [], poss: [], snapD: [], snapA: [], jobs: {}, goals: [] };
const inc = (o, k, n = 1) => o[k] = (o[k] || 0) + n;
const settle = () => { for (const L of pending) if (L.res != null) (L.kind === 'gönder' ? R[lab(L.t)].shots : R[lab(L.t)].launch).push(L); pending = pending.filter(L => L.res == null); };
let lastDec = null, lastFl = null, pending = [], poss = null, prevHolder = null;
function endPoss(reason, x) { if (!poss) return; poss.end = reason; poss.endZone = zone(poss.team, x); poss.dur = m.tick - poss.t0; R[lab(poss.team)].poss.push(poss); poss = null; }
while (!m.over && m.tick < len) {
  const sc = m.score.slice(); M.step(m); const h = m.holder;
  // kararlar
  if (m.lastDec && m.lastDec !== lastDec) { lastDec = m.lastDec; const t = lastDec.team, k = lastDec.opts[0] ? lastDec.opts[0].kind.split(' ')[0] : '?'; inc(R[lab(t)].dec, k); inc(R[lab(t)].decZone, zone(t, lastDec.x) + ':' + k); }
  // Çekirdeğin yola çıkışı (pas, önüne, aşırt, kenardan, gönder)
  if (m.fl && m.fl !== lastFl && m.fl.t0 === m.tick) { lastFl = m.fl; const p = prevHolder, t = p ? p.team : m.ball.team, fl = m.fl, to = fl.to || fl.end;
    const d0 = lastDec && lastDec.name === (p && p.name) ? lastDec : null, o0 = d0 && d0.opts[0], o1 = d0 && d0.opts[1];
    const L = { t, kind: fl.kind, x: p ? p.x : m.ball.x, y: p ? p.y : m.ball.y, dist: p ? hyp(to.x - p.x, to.y - p.y) : 0, fwd: p ? (to.x - p.x) * dirOf(t) : 0, P: o0 ? o0.P : null, margin: o0 && o1 ? o0.v - o1.v : null, alt: o1 ? o1.kind.split(' ')[0] : null, ot: !!(p && m.tick - (p._takeT || -9) < 2), res: null, t0: m.tick };
    if (fl.kind === 'gönder') { const gx = oppX(t); L.gd = hyp(L.x - gx, L.y - 25); L.ang = Math.abs(L.y - 25); }
    pending.push(L); if (poss && poss.team === t && fl.kind !== 'gönder') poss.passes++; if (poss && poss.team === t && fl.kind === 'gönder') poss.shot = true; }
  // sayı
  if (m.score[0] !== sc[0] || m.score[1] !== sc[1]) { const t = m.score[0] !== sc[0] ? 0 : 1; for (const L of pending) if (L.res == null) L.res = L.t === t ? 'sayı' : 'kendi kalesine'; settle();
    if (poss && poss.team === t) { R[lab(t)].goals.push({ passes: poss.passes, dur: m.tick - poss.t0, from: poss.how, startZone: poss.startZone }); endPoss('sayı', oppX(t)); } else { R[lab(t)].goals.push({ passes: 0, dur: 0, from: 'diğer', startZone: '?' }); endPoss('yenildi', 50); } }
  // sahiplik
  if (h && h !== prevHolder) { for (const L of pending) if (L.res == null) L.res = h.team === L.t ? (L.kind === 'gönder' ? 'kurtarış sonrası bizde' : 'tuttu') : (L.kind === 'gönder' ? 'kurtarıldı/kesildi' : 'kesildi'); settle();
    if (!poss || poss.team !== h.team) { endPoss('kayıp', h.x); poss = { team: h.team, t0: m.tick, passes: 0, how: prevHolder && prevHolder.team !== h.team ? 'kazanma' : 'boştaki', startZone: zone(h.team, h.x), shot: false }; } }
  if (h) h._takeT = h === prevHolder ? h._takeT : m.tick; prevHolder = h || prevHolder;
  // şekil anlık görüntüsü (her 10 tikte, Çekirdek birindeyken)
  if (h && m.tick % 10 === 0) { const at = h.team, dt = 1 - at, A = m.ps.filter(p => p.team === at && p.role !== 'Bekçi'), D = m.ps.filter(p => p.team === dt && p.role !== 'Bekçi');
    const dd = p => (p.x - ownX(dt)) * dirOf(dt), deep = Math.min(...D.map(dd)), high = Math.max(...D.map(dd)), ys = D.map(p => p.y);
    const unmarked = A.filter(a => a !== h && !D.some(d => hyp(d.x - a.x, d.y - a.y) < 5)).length, behind = A.filter(a => dd(a) < deep).length, near = D.filter(d => hyp(d.x - h.x, d.y - h.y) < 8).length;
    const ballD = dd(h), goalSide = A.filter(a => a !== h && dd(a) < 45).filter(a => D.some(d => dd(d) < dd(a) && hyp(d.x - a.x, d.y - a.y) < 7)).length, inDang = A.filter(a => a !== h && dd(a) < 45).length;
    R[lab(dt)].snapD.push({ ballD, deep, high, len: high - deep, wid: Math.max(...ys) - Math.min(...ys), press: D.filter(p => p.press).length, near, unmarked, behind, goalSide, inDang });
    const ay = A.map(p => p.y), ax = A.map(p => (p.x - ownX(at)) * dirOf(at)); R[lab(at)].snapA.push({ wid: Math.max(...ay) - Math.min(...ay), len: Math.max(...ax) - Math.min(...ax), sprint: A.filter(p => p.sprint).length });
    for (const p of m.ps) if (p.role !== 'Bekçi' && p.job) inc(R[lab(p.team)].jobs, (p.team === at ? 'H:' : 'S:') + p.job.split(' · ')[0]); }
}
endPoss('bitti', 50); for (const L of pending) L.res = 'çözülmedi'; settle();
console.log(JSON.stringify({ cell, i, score: { A: m.score[sw ? 1 : 0], B: m.score[sw ? 0 : 1] }, R }));
