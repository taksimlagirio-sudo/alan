// 8. madde · bir karar anında yöntemlerin seçimi ve kâhin.
// Yöntemler: S (şimdiki arama, gerçek seçimi) · C (arama kapalı: ilk bakış, decide'ın en iyisi) · B (hücumun sonuna kadar oynatma, ilk bakışla; aday başına NB deneme, ortak zar)
//            · A0 / A (şimdiki arama, hafızanın öğrendiği tabloyla; A0: sadece eski boyutlar, A: + blok açıklığı ve boştaki arkadaş). Tablo dosyası verilirse.
// Kâhin: aday hamle gerçek kurallarla (ileri bakış kapalı, hafif kip yok, algı hatası yok: herkes refleks/ilk bakışla) hücumun sonuna kadar oynatılır.
//   Hücumun sonu: sayı (±1) · top rakibe geçer (değer: −rakibin o noktadaki tablo değeri, eski tablo) · 8 sn (480 tik; top havadaysa sonuçlanana kadar en çok 240 tik daha; değer: tablo).
//   Bütün adaylar NT denemeyle (ortak zar), herhangi bir yöntemin seçtiği adaylar NK denemeye tamamlanır.
// Kullanım: node tools/ab/degerlendir.js <motor> <an dosyası> <çıktı> [NT=10] [NK=30] [NB=6] [tablo.json]
const fs = require('fs'), O = require('./ortak');
const [dir, fn, out, ntS, nkS, nbS, tabloF] = process.argv.slice(2), NT = +ntS || 10, NK = +nkS || 30, NB = +nbS || 6;
const g = O.yukle(dir, 1), M = g.AlanMatch, B = g.AlanBeyin, Dm = g.AlanDecide, D = Dm.D, ST = g.AlanState, A0 = { ...B.A }, VS0 = Float64Array.from(g.ALAN_VS.v);
const kayit = O.oku(fn), S = B.ac(kayit.S); D._m = S; D._tick = S.tick;
const h0 = S.holder, team = h0.team, hid = h0.id, dir1 = team === 0 ? 1 : -1, gx = team === 0 ? 100 : 0, hyp = Math.hypot;
const pid = q => q == null ? null : (typeof q === 'object' ? q.id : q);
const anahtar = (d, run) => [d.kind, pid(d.q), d.to ? d.to.x.toFixed(1) + ',' + d.to.y.toFixed(1) : '', d.shot || '', d.hold || '', d.W ? d.W.x.toFixed(1) + ',' + d.W.y.toFixed(1) : '', d.yavas ? 'y' : '', run ? run.id + '@' + run.x.toFixed(1) + ',' + run.y.toFixed(1) : ''].join('|');
const ayni = (o, d) => o.kind === d.kind && !!o.yavas === !!d.yavas && (o.hold || null) === (d.hold || null) && !!o.W === !!d.W && (!d.W || hyp(o.W.x - d.W.x, o.W.y - d.W.y) < .6) && pid(o.q) === pid(d.q) && (o.shot || null) === (d.shot || null) && (!d.to || (o.to && hyp(o.to.x - d.to.x, o.to.y - d.to.y) < .6));
function tabloKur(T) { if (!T) { g.ALAN_VS.v = Float64Array.from(VS0); globalThis.__VSEK = null; return; } g.ALAN_VS.v = Float64Array.from(T.v); globalThis.__VSEK = T.ek ? require('./ek').okuyucu(g, T.ek) : null; }
// aramanın aday listesi (birleşik: hamle + koşu) ve seçimi
function ara() { const c = M.cloneMatch(S); D._m = c; D._tick = c.tick; let p = B.planla(c), combos = null; const t0 = Date.now();
  while (p) { if (!combos && p.tip === 'karar') { combos = []; let last = null; for (const j of p.jobs) { if (last && last.d === j.d && last.run === j.run) continue; last = j; combos.push({ d: j.d, o: j.o, run: j.run }); } } p = p.bitir(p.jobs.map(j => B.rollout(c, j))) || null; }
  const ms = Date.now() - t0, s = c._beyinSon; if (!combos || !s || s.tip !== 'karar') return null;
  let ib = -1, vb = -1e9; s.aday.forEach((a, i) => { if (/\(2 adım\)/.test(a.kind) && a.v > vb) { vb = a.v; ib = i; } });
  return { combos, sec: s.secilen, argmax: ib >= 0 ? ib : s.secilen, v: s.aday.map(a => a.v), ms }; }
function ilkBakis() { /* ilk bakış maçtaki gibi adımın içinde verilir (önce herkes bir tik hareket eder): karar komut noktası yakalanır */ const c = M.cloneMatch(S); D._m = c; D._tick = c.tick; const k0 = B.karar, nk = globalThis.__NOKARAR; let o = null, ms = 0;
  B.karar = (m, h) => { const t0 = Date.now(), r = k0(m, h); if (!o && h.id === hid) { o = r; ms = Date.now() - t0; } return r; }; globalThis.__NOKARAR = true; try { M.step(c); } finally { B.karar = k0; globalThis.__NOKARAR = nk; } return { o: o || { kind: 'tut' }, ms }; }
function bSec(combos) { const sigma = Math.max(0, 20 - self.ALAN_OKC(h0, 'algi')) * B.A.algi, t0 = Date.now(), v = combos.map(x => { let s = 0; for (let r = 0; r < NB; r++) s += B.rollout(S, { tip: 'karar', hid, d: x.d, o: x.o, run: x.run, team, sigma, sona: true, H: 480, HMAX: 480, salt: ((S.tick * 40503) ^ (r * 2654435761 + 99)) >>> 0 }); return s / NB; });
  let k = 0; v.forEach((x, i) => { if (x > v[k]) k = i; }); return { k, v, ms: Date.now() - t0 }; }
// kâhin: gerçek kurallar, herkes ilk bakışla. Değer eski tabloyla (sabit cetvel).
function kahin(x, r0, n) { const keep = { bak: B.A.bak }, Tk = { v: g.ALAN_VS.v, ek: globalThis.__VSEK }; B.A.bak = false; tabloKur(null); const res = [], ev = [];
  for (let r = r0; r < r0 + n; r++) { const c = M.cloneMatch(S), s0 = c.score.slice(); c.r.s = (0x51ED + r * 1000003) | 0; c._zorla = { hid, d: x.d, o: x.o }; c._pm = !!x.o; /* paketli hâli yoksa (olmamalı) karar, ilk bakışın listesinde aynı hamleyi arar */ c.kosu = c.kosu ? c.kosu.slice() : [null, null]; c.kosu[team] = x.run ? { ...x.run, until: c.tick + B.A.KT } : (c.kosu[team] && c.kosu[team].until > c.tick ? c.kosu[team] : null);
    M.step(c); c._pm = false; c._zorla = null; let t = 1, son = 'sure';
    for (; ; t++) { if (c.score[0] !== s0[0] || c.score[1] !== s0[1]) { son = c.score[team] !== s0[team] ? 'sayi' : 'yedi'; break; } if (c.holder && c.holder.team !== team) { son = 'kayip'; break; } if (c.over || (t >= 480 && c.holder) || t >= 720) break; M.step(c); }
    res.push(B.endVal(c, team, s0)); ev.push(son[0] + t); }
  B.A.bak = keep.bak; g.ALAN_VS.v = Tk.v; globalThis.__VSEK = Tk.ek; return { v: res, ev }; }
// sınıflar
function sinif(x) { const d = x.d; if (d.kind === 'gönder') return 'gönder'; if (d.kind === 'tut') return d.hold ? 'bekle' : 'tut'; if (d.kind === 'sür') return (d.to.x - h0.x) * dir1 < 0 ? 'sür geri' : 'sür';
  const to = d.to || (d.q && S.ps.find(p => p.id === pid(d.q))) || h0, ilerle = (to.x - h0.x) * dir1, kanat = Math.abs(to.y - h0.y) >= 18 && (to.y - 25) * (h0.y - 25) < 0;
  return kanat ? 'kanat değiştir' : ilerle < 3 ? 'geri/yan pas' : 'ileri pas'; }
function sahne() { let nd = 1e9, n8 = 0, ahead = 0; for (const p of S.ps) { if (p.team === team || p.role === 'Bekçi') continue; const dd = hyp(p.x - h0.x, p.y - h0.y); nd = Math.min(nd, dd); if (dd < 8) n8++; if ((p.x - h0.x) * dir1 > 0) ahead++; }
  const dist = hyp(gx - h0.x, 25 - h0.y), uzak = S.ps.some(p => p.team === team && p !== h0 && p.role !== 'Bekçi' && (p.y - 25) * (h0.y - 25) < 0 && Math.abs(p.y - 25) >= 8);
  return { dist: +dist.toFixed(1), nd: +nd.toFixed(1), ahead, pres: nd < 3.5 && n8 >= 2, derin: ahead >= 5 && dist >= 25 && dist <= 60, kanat: Math.abs(h0.y - 25) >= 10 && uzak }; }
const avg = a => a.reduce((s, x) => s + x, 0) / a.length;
// ── çalıştır
const sonuc = { an: fn.split('/').pop(), mac: kayit.mac, tick: S.tick, tac: kayit.tac, team, sahne: sahne(), yon: {} };
const s = ara(); if (!s) { console.log(JSON.stringify({ ...sonuc, hata: 'karar yok' })); process.exit(0); }
const combos = s.combos, ix = new Map(); combos.forEach((x, i) => ix.set(anahtar(x.d, x.run), i));
const ekle = (d, run, o) => { const k = anahtar(d, run); if (ix.has(k)) return ix.get(k); let j = combos.findIndex(x => !x.run === !run && (!run || run.id === x.run.id) && ayni(x.d, d)); if (j >= 0) return j; combos.push({ d, o, run }); ix.set(k, combos.length - 1); return combos.length - 1; };
sonuc.yon.S = { k: s.sec, ms: s.ms }; sonuc.yon.Sx = { k: s.argmax };
const c1 = ilkBakis(); { const o = c1.o, d = { kind: o.kind, q: pid(o.q), to: o.to ? { x: o.to.x, y: o.to.y } : null, shot: o.shot || null, hold: o.hold || null, W: o.W ? { x: o.W.x, y: o.W.y } : null, yavas: !!o.yavas }; sonuc.yon.C = { k: ekle(d, null, null), ms: c1.ms }; }
if (tabloF) { const TT = JSON.parse(fs.readFileSync(tabloF, 'utf8'));
  for (const [ad, T] of Object.entries(TT)) { tabloKur(T); const s2 = ara(); if (s2) { sonuc.yon[ad] = { k: ekle(s2.combos[s2.sec].d, s2.combos[s2.sec].run, s2.combos[s2.sec].o), ms: s2.ms }; } const c2 = ilkBakis(); const o = c2.o; sonuc.yon['C' + ad] = { k: ekle({ kind: o.kind, q: pid(o.q), to: o.to ? { x: o.to.x, y: o.to.y } : null, shot: o.shot || null, hold: o.hold || null, W: o.W ? { x: o.W.x, y: o.W.y } : null, yavas: !!o.yavas }, null, null) }; }
  tabloKur(null); }
// aşama 2 (ASAMA2=<aşama 1 dosyası>): aynı an, aynı aday listesi (arama belirlenimci); A yöntemlerinin seçimleri eklenir, yeni ya da az denenmiş adaylar aynı zarlarla NK'ye tamamlanır
const onceki = process.env.ASAMA2 ? fs.readFileSync(process.env.ASAMA2, 'utf8').trim().split('\n').map(l => JSON.parse(l)).find(r => r.an === sonuc.an) : null;
let kv; const t0 = Date.now();
if (!onceki) { const b = bSec(combos.slice(0, s.combos.length)); sonuc.yon.B = { k: b.k, ms: b.ms }; sonuc.bv = b.v.map(x => +x.toFixed(3));
  kv = combos.map(x => kahin(x, 0, NT)); let kb = 0; kv.forEach((x, i) => { if (avg(x.v) > avg(kv[kb].v)) kb = i; }); sonuc.yon.K = { k: kb }; /* kâhinin kendi seçimi (NT deneme), NK'ye tamamlanıp yansızca ölçülür: "en iyi hamle"nin alt sınırı */ }
else { for (const [a, y] of Object.entries(onceki.yon)) if (!(a in sonuc.yon)) sonuc.yon[a] = y; sonuc.bv = onceki.bv; kv = combos.map((x, i) => onceki.aday[i] ? { v: onceki.aday[i].k.slice(), ev: onceki.aday[i].ev || [] } : { v: [], ev: [] }); }
const secilen = new Set(Object.values(sonuc.yon).map(y => y.k));
for (let k = 0; k < combos.length; k++) { const hedef = secilen.has(k) ? NK : NT, var0 = kv[k].v.length; if (var0 < hedef) { const ek = kahin(combos[k], var0, hedef - var0); kv[k].v.push(...ek.v); kv[k].ev.push(...ek.ev); } }
sonuc.kahinMs = Date.now() - t0 + (onceki ? onceki.kahinMs : 0);
sonuc.aday = combos.map((x, i) => ({ s: sinif(x), sv: i < s.v.length ? s.v[i] : null, k: kv[i].v.map(v => +v.toFixed(3)), ev: secilen.has(i) ? kv[i].ev : undefined, run: x.run ? 1 : 0, uz: x.d.kind === 'gönder' ? +hyp(gx - h0.x, 25 - h0.y).toFixed(1) : undefined }));
fs.appendFileSync(out, JSON.stringify(sonuc) + '\n');
console.log(sonuc.an, 'aday', combos.length, 'kâhin', (sonuc.kahinMs / 1000).toFixed(0) + 's', Object.entries(sonuc.yon).map(([a, y]) => a + ':' + y.k + '(' + (y.ms ?? '') + ')').join(' '));
