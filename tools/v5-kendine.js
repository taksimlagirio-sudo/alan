// Kendine pas (kenardan çalım · kendine sektirme): İkili Sahne'nin kenar senaryoları. Taşıyıcı ve presçi (+ Bekçiler), sahne sayfasıyla aynı kurulum.
// zorla: taşıyıcının ilk kararında en değerli kendine pas zorla seçilir; aklın tahmini (seçimdeki, az denemeli) ve çok denemeli tahmin, gerçek sonuç (Çekirdek yine taşıyıcıda mı)
// serbest: akıl kendi seçer; kendine pas ne sıklıkla seçiliyor, sonuç ne
// Kullanım: node tools/v5-kendine.js <motor> <sahne> <N=25>
const { load } = require('./load'); const [dir, sahne, nS] = process.argv.slice(2), N = +nS || 25;
const SAH = { kenarDuz: { at: [44, 4, .1, 0, 0], df: [48.5, 4, 0, 0] }, kenarUst: { at: [44, 5, .1, 0, 0], df: [48.5, 3.5, 0, 0] }, kose: { at: [80, 4, .08, 0, 0], df: [84, 6, 0, 0] }, ornD: { at: [44, 4, .1, 0, 0], df: [48.5, 4, 0, 0] }, ornK: { at: [44, 5, .1, 0, 0], df: [48.5, 3.5, 0, 0] }, ornB: { at: [80, 4, .08, 0, 0], df: [84, 6, 0, 0] }, hiza7: { at: [44, 4, .1, 0, 0], df: [51, 4, 0, 0] }, hiza9: { at: [44, 4, .1, 0, 0], df: [53, 4, 0, 0] }, kenarOn: { at: [44, 3, .1, 0, 0], df: [48, 4.2, 0, 0] }, kenarIc: { at: [44, 3.5, .12, 0, 0], df: [47, 8, 0, 0] } };
const SEED = { ornD: 1056, ornK: 1070, ornB: 1014 };
const g = load(dir, 9), M = g.AlanMatch, AD = g.AlanDecide, D = AD.D;
const mk = s => { const f = () => { f.s = (f.s + 0x6D2B79F5) | 0; let a = f.s, t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; f.s = s | 0; return f; };
function yeni(seed, S) { const tac = { sistem: 'Alan', pres: 1, arkada: 0, blok: 'Orta', genislik: 'Normal', tempo: .5, risk: .5, kazaninca: 'Dengeli' };
  const m = M.createMatch(seed, { len: 99999, tac: [tac, tac], attrs: [{ surme: 12, ceviklik: 12, hiz: 12, okuma: 10 }, { kesme: 12, ceviklik: 12, hiz: 12, okuma: 10, yogunluk: 10 }] }), at = m.ps.find(p => p.team === 0 && p.i === 3), df = m.ps.find(p => p.team === 1 && p.i === 3), bk = m.ps.filter(p => p.role === 'Bekçi');
  m.ps = [at, df, ...bk]; [at.x, at.y, at.vx, at.vy, at.ca] = S.at; [df.x, df.y, df.vx, df.vy] = S.df; at.tx = at.x; at.ty = at.y; df.tx = df.x; df.ty = df.y; m.holder = at; m.ball = null; m.fl = null; m.decT = 2; m.ch = .4; m.r = mk(seed); return { m, at, df }; }
// sonuç: kendine pastan sonra Çekirdeği ilk kim aldı (taşıyıcı = 1), 300 tik içinde
function bitir(m, at) { for (let t = 0; t < 300; t++) { M.step(m); if (m.holder) return m.holder === at ? 1 : m.holder.team === 0 ? 1 : 0; } return null; }
const S = SAH[sahne], R = { zorla: { n: 0, uret: 0, P: 0, Pm: 0, g: 0, gn: 0 }, serbest: { n: 0, kendine: 0, g: 0, gn: 0 } };
for (let k = 0; k < N; k++) { const seed = (SEED[sahne] || 1000) + k * 7;
  { const { m, at } = yeni(seed, S), od = AD.decide; let zorlandi = null;
    AD.decide = function (hh, ps, ch, rnd, tac) { const r = od.apply(this, arguments); if (!zorlandi && hh === at) { const ks = r.opts.filter(o => o.kendine).sort((a, b) => b.v - a.v); zorlandi = ks[0] || false; if (ks[0]) { const tr = D.trials; D.trials = 40; let pm = null; try { const r2 = od.call(this, hh, ps, ch, mk(777), tac), x = r2.opts.find(o => o.kendine && Math.hypot(o.lead.x - ks[0].lead.x, o.lead.y - ks[0].lead.y) < .5); pm = x ? x.det.P : null; } finally { D.trials = tr; } zorlandi.Pm = pm; return { best: ks[0], opts: [ks[0], ...r.opts.filter(o => o !== ks[0])] }; } } return r; };
    try { for (let t = 0; t < 60 && zorlandi == null; t++) M.step(m); } finally { AD.decide = od; }
    R.zorla.n++; if (zorlandi) { R.zorla.uret++; R.zorla.P += zorlandi.det.P; if (zorlandi.Pm != null) R.zorla.Pm += zorlandi.Pm; const s = m.holder ? (m.holder === at ? null : 0) : bitir(m, at); if (s != null) { R.zorla.g += s; R.zorla.gn++; } } }
  { const { m, at } = yeni(seed, S), od = AD.decide; let sec = null;
    AD.decide = function (hh) { const r = od.apply(this, arguments); if (sec == null && hh === at) sec = r.best; return r; };
    try { for (let t = 0; t < 60 && sec == null; t++) M.step(m); } finally { AD.decide = od; }
    R.serbest.n++; if (sec && sec.kendine) { R.serbest.kendine++; const s = bitir(m, at); if (s != null) { R.serbest.g += s; R.serbest.gn++; } } } }
const z = R.zorla, f = (a, b) => b ? Math.round(100 * a / b) + '%' : '–';
console.log(`${sahne.padEnd(9)} zorla: üretildi ${z.uret}/${z.n} · akıl (seçimde) ${f(z.P, z.uret)} · akıl (40 deneme) ${f(z.Pm, z.uret)} · gerçek ${f(z.g, z.gn)} (${z.gn})  ||  serbest: kendine seçildi ${R.serbest.kendine}/${R.serbest.n} · başarı ${f(R.serbest.g, R.serbest.gn)}`);
