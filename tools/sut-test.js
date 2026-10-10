// v4g · göndermede beynin hayali ile gerçeğin karşılaştırması (Design'ın test görevi 1 ve 2).
// Gerçek maçta beyin gönderme seçtiğinde o an (adımdan önce) kopyalanır. Kaydedilen: uzaklık, açı, şarj, Bekçi'nin Kuyu'ya ve gönderme hattına
// uzaklığı, beynin değeri, gerçek sonuç, 10 sn içinde sıradaki sayı. Sonra aynı an:
//   hayal (beynin ileri oynatması, 20 deneme) dört koşulda: varsayılan · hafif kapalı · algı hatası yok · Bekçi derinlik planı ileri oynatmada da açık
//   gerçek (aynı gönderme, gerçek maç kurallarıyla: hafif değil, algı hatası yok, Bekçi planı her tik, 20 farklı zar)
// Motor dosyalarına dokunulmaz; beyin.js bu süreçte tek bir bayrakla yamalanır (Bekçi planı ileri oynatmada da çalışsın).
// Kullanım: node tools/sut-test.js <motor> <i> [N=20] [en çok gönderme=40]
const fs = require('fs'), path = require('path');
const [dir, iS, nS, mxS] = process.argv.slice(2), i = +iS, N = +nS || 20, MX = +mxS || 40, seed = 150000 + i * 7919;
function rngF(s) { let a = s >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
Math.random = rngF(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of ['core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js', 'beyin.js', 'vs-table.js', 'pas-table.js']) { let src = fs.readFileSync(path.join(dir, f), 'utf8');
  if (f === 'beyin.js') { const a = 'if (A.bek) { if (!m._ic) bekPlan(m);'; if (!src.includes(a)) throw new Error('yama yeri yok'); src = src.replace(a, 'if (A.bek) { if (!m._ic || globalThis.__BEK_IC) bekPlan(m);'); }
  (0, eval)(src + '\n//# sourceURL=' + f); }
const M = AlanMatch, B = AlanBeyin, D = AlanDecide.D, K = AlanCore, A0 = { ...B.A };
let r0 = seed ^ 0x9e3779b9; const R = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296, pick = a => a[Math.floor(R() * a.length)];
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const tacOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] });
const TA = pick(Object.keys(STY)), TB = pick(Object.keys(STY)), m = M.createMatch(seed, { tac: [tacOf(STY[TA]), tacOf(STY[TB])], len: 5400 }); // Herkes 10 (varsayılan)
const same = (o, d) => o.kind === d.kind && (o.q != null ? (o.q.id ?? o.q) : null) === (d.q != null ? (d.q.id ?? d.q) : null) && (o.shot || null) === (d.shot || null) && (!d.to || (o.to && Math.hypot(o.to.x - d.to.x, o.to.y - d.to.y) < .6));
function hayal(S, job, kosul) { Object.assign(B.A, A0, kosul.A || {}); globalThis.__BEK_IC = !!kosul.bek; let s = 0; const keep = [D._m, D._tick];
  for (let r = 0; r < N; r++) s += B.rollout(S, { ...job, sigma: kosul.algi0 ? 0 : job.sigma, salt: (job.salt ^ (r * 2654435761 + 77)) | 0 });
  Object.assign(B.A, A0); globalThis.__BEK_IC = false; D._m = keep[0]; D._tick = keep[1]; return s / N; }
function gercek(S, team) { const keep = [D._m, D._tick], out = { v: 0, sayi: 0, bek: 0, biz: 0, rakip: 0 };
  for (let r = 0; r < N; r++) { const c = M.cloneMatch(S), s0 = c.score.slice(); c.r.s = (r * 977 + 31337) | 0; let ev = null;
    for (let t = 0; t < 300; t++) { M.step(c); if (c.score[0] !== s0[0] || c.score[1] !== s0[1]) { ev = c.score[team] !== s0[team] ? 'sayi' : 'rakipSayi'; break; } if (c.holder && t > 0) { ev = c.holder.team !== team && c.holder.role === 'Bekçi' ? 'bek' : c.holder.team === team ? 'biz' : 'rakip'; break; } if (c.over) break; }
    const v = B.endVal(c, team, s0); out.v += v; if (ev === 'sayi') out.sayi++; else if (ev === 'bek') out.bek++; else if (ev === 'biz') out.biz++; else if (ev === 'rakip' || ev === 'rakipSayi') out.rakip++; }
  D._m = keep[0]; D._tick = keep[1]; for (const k of ['v', 'sayi', 'bek', 'biz', 'rakip']) out[k] = +(out[k] / N).toFixed(3); return out; }
const KOSUL = { varsayilan: {}, hafifYok: { A: { hafif: false } }, algiYok: { algi0: true }, bekIc: { bek: true } };
const rows = []; let acik = null;
while (!m.over && m.tick < 5400 && rows.length < MX) { let p = B.planla(m), j0 = null, ilk = true;
  while (p) { if (ilk && p.tip === 'karar') j0 = p.jobs; ilk = false; p = p.bitir(p.jobs.map(j => B.rollout(m, j))) || null; }
  const z = m._zorla, s = m._beyinSon;
  if (z && z.d && z.d.kind === 'gönder' && s && s.tick === m.tick && j0) { const h = m.ps.find(q => q.id === z.hid), job = j0.find(j => same(j.d, z.d));
    if (h && job) { const S = M.cloneMatch(m), team = h.team, gx = team === 0 ? 100 : 0, bk = m.ps.find(q => q.team !== team && q.role === 'Bekçi'), to = z.d.to || { x: gx, y: 25 };
      const L = Math.hypot(to.x - h.x, to.y - h.y) || 1, ux = (to.x - h.x) / L, uy = (to.y - h.y) / L, bekHat = bk ? Math.abs(-(bk.x - h.x) * uy + (bk.y - h.y) * ux) : null;
      const row = { t: m.tick, team, uz: +Math.hypot(gx - h.x, 25 - h.y).toFixed(1), aci: +Math.abs(Math.atan2(25 - h.y, Math.abs(gx - h.x)) * 57.3).toFixed(0), sarj: +m.ch.toFixed(2), bekKuyu: bk ? +Math.hypot(bk.x - gx, bk.y - 25).toFixed(1) : null, bekHat: bekHat != null ? +bekHat.toFixed(2) : null, beyin: +s.aday[s.secilen].v.toFixed(3), shot: z.d.shot };
      for (const [k, c] of Object.entries(KOSUL)) row['h_' + k] = +hayal(S, job, c).toFixed(3);
      row.g = gercek(S, team); acik = row; rows.push(row); } }
  const sc0 = m.score.slice(); M.step(m);
  if (acik && !acik.sonuc) { if (m.score[0] !== sc0[0] || m.score[1] !== sc0[1]) acik.sonuc = m.score[acik.team] !== sc0[acik.team] ? 'sayi' : 'rakipSayi'; else if (m.holder && m.tick > acik.t + 1) acik.sonuc = m.holder.team !== acik.team && m.holder.role === 'Bekçi' ? 'bek' : m.holder.team === acik.team ? 'biz' : 'rakip'; }
  for (const r of rows) if (r.sira10 == null && m.tick - r.t <= 600 && (m.score[0] !== sc0[0] || m.score[1] !== sc0[1])) r.sira10 = m.score[r.team] !== sc0[r.team] ? 1 : -1; }
console.log(JSON.stringify({ i, tac: [TA, TB], skor: m.score, tik: m.tick, rows }));
