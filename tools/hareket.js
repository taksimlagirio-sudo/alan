// v4g · hareketin doğallığı (Design'ın test görevi 3). Beyin açık bir maç oynatılır; her tik her oyuncunun konumu, hızı, hedefi ve görevi izlenir.
// Ölçülenler (oyuncu-dakika başına):
//   dönüş: hız > 0,06 iken 6 tikte 45°+ yön değişimi · ivme sıçraması: tek tikte hız vektörü 0,05+ değişimi
//   hedef sıçraması: hedef noktası (tx, ty) tek tikte 2+ birim kayması · dur-kalk: hız 0,03'ün altına inip 30 tik içinde 0,15'in üstüne çıkması
// Her hedef sıçraması bir kaynağa yazılır: görev değişti (refleks yerleşim kendi kararını değiştirdi) · beyin kayması başladı / bitti (m.yer) ·
//   beyin koşusu (m.kosu) · top sahibinin planı (sürme hedefi) · top el değiştirdi / pas atıldı (oyun durumu) · aynı görevde hedef zıpladı (refleks hedef hesabı)
// Dönüşler, öncesindeki 8 tik içinde olan son hedef sıçramasının kaynağına yazılır (yoksa "hedef sabitken").
// Kullanım: node tools/hareket.js <motor> <i> [tik=1800]
const fs = require('fs'), { load } = require('./load');
const [dir, iS, lS] = process.argv.slice(2), i = +iS, LEN = +lS || 1800, seed = 170000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch, B = g.AlanBeyin; (0, eval)(fs.readFileSync(dir + '/vs-table.js', 'utf8')); (0, eval)(fs.readFileSync(dir + '/pas-table.js', 'utf8'));
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const tacOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] }), ks = Object.keys(STY);
const m = M.createMatch(seed, { tac: [tacOf(STY[ks[i % 6]]), tacOf(STY[ks[(i * 5 + 1) % 6]])], len: 5400 });
const N = m.ps.length, prev = m.ps.map(p => ({ x: p.x, y: p.y, vx: p.vx || 0, vy: p.vy || 0, tx: p.tx, ty: p.ty, job: p.job, yer: false, kos: false })), hist = m.ps.map(() => []), lastJump = m.ps.map(() => null);
const C = { donus: {}, ivme: { top: 0, topsuz: 0 }, sicrama: {}, durKalk: { top: 0, topsuz: 0 }, sicramaBuyuklugu: {}, gecis: {} }, add = (o, k, n = 1) => o[k] = (o[k] || 0) + n;
let lowT = m.ps.map(() => -1e9), holderPrev = null, ballPrev = null, ticks = 0;
while (!m.over && m.tick < LEN) { let p = B.planla(m); while (p) p = p.bitir(p.jobs.map(j => B.rollout(m, j))) || null; M.step(m); ticks++;
  const olay = m.holder !== holderPrev || (m.ball && m.ball !== ballPrev); holderPrev = m.holder; ballPrev = m.ball;
  m.ps.forEach((p, k) => { const q = prev[k], isH = p === m.holder, yerNow = !!(m.yer && m.yer[p.id]), kosNow = !!(m.kosu && m.kosu[p.team] && m.kosu[p.team].id === p.id), sp = Math.hypot(p.vx, p.vy), tip = isH ? 'top' : 'topsuz';
    // hedef sıçraması
    if (q.tx != null && p.tx != null) { const dj = Math.hypot(p.tx - q.tx, p.ty - q.ty); if (dj > 2) { const src = isH ? 'top sahibinin planı' : p.job !== q.job ? 'görev değişti' : yerNow !== q.yer ? (yerNow ? 'beyin kayması başladı' : 'beyin kayması bitti') : kosNow !== q.kos ? 'beyin koşusu' : olay ? 'top el değiştirdi / pas' : 'aynı görevde hedef zıpladı'; add(C.sicrama, src); add(C.sicramaBuyuklugu, src, dj); if (src === 'görev değişti') add(C.gecis, `${(q.job || '–').split(' ·')[0]} → ${(p.job || '–').split(' ·')[0]}`); lastJump[k] = { t: m.tick, src }; } }
    // ivme
    if (Math.hypot(p.vx - q.vx, p.vy - q.vy) > .05) C.ivme[tip]++;
    // dönüş (6 tik önceki hız yönüne göre)
    const h = hist[k]; h.push([p.vx, p.vy]); if (h.length > 6) { const [ax, ay] = h.shift(), sa = Math.hypot(ax, ay); if (sp > .06 && sa > .06) { const ang = Math.acos(Math.max(-1, Math.min(1, (ax * p.vx + ay * p.vy) / (sa * sp)))) * 57.3; if (ang > 45) { const lj = lastJump[k], src = lj && m.tick - lj.t <= 8 ? lj.src : 'hedef sabitken'; add(C.donus, tip + ' · ' + src); h.length = 0; } } }
    // dur-kalk
    if (sp < .03) lowT[k] = m.tick; else if (sp > .15 && m.tick - lowT[k] <= 30 && lowT[k] > 0) { C.durKalk[tip]++; lowT[k] = -1e9; }
    prev[k] = { x: p.x, y: p.y, vx: p.vx, vy: p.vy, tx: p.tx, ty: p.ty, job: p.job, yer: yerNow, kos: kosNow }; }); }
const dk = ticks / 3600 * N; // oyuncu-dakika
const per = o => Object.fromEntries(Object.entries(o).map(([k, v]) => [k, +(v / dk).toFixed(1)]));
console.log(JSON.stringify({ i, tik: ticks, oyuncuDk: +dk.toFixed(1), donus: per(C.donus), ivme: per(C.ivme), sicrama: per(C.sicrama), sicramaOrtBuyukluk: Object.fromEntries(Object.entries(C.sicramaBuyuklugu).map(([k, v]) => [k, +(v / C.sicrama[k]).toFixed(1)])), durKalk: per(C.durKalk), gecis: Object.fromEntries(Object.entries(C.gecis).sort((a, b) => b[1] - a[1]).slice(0, 12).map(([k, v]) => [k, +(v / dk).toFixed(1)])) }));
