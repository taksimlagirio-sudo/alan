// Genel deney: üç topsuz / donuk karar türü maçta kopyada tartılarak verilir ("kâhin"), motor değişmeden.
//   1 · Pres: bir savunmacı ilk presçi olarak basmaya başladığında: bas / bu presçi basmasın (60 tik).
//   2 · Alıcı: takım arkadaşına pas çıktığında: topa gel (zip68 kuralı) / kendi noktasına koş (eski kural), pas uçuşu boyunca.
//   3 · Koşan taşıyıcı (hız > 0,15) karar verecekken: "sür" sınıfı / "fake·tut" sınıfı / aklın kendi seçimi.
// Her seçenek maçın kopyasında aynı zarla UFUK tik oynatılır; değer (karar veren takım için): sayı farkı + 0,2 × gönderme farkı + 0,5 × topun sonunda kimde olduğuna göre tehlike.
// Kullanım: node tools/kahin.js <motor> <senaryo> <çıktı.json> [mod: kahin|kontrol] [UFUK=120] [türler: pres,alici,surme]
const fs = require('fs'); const { kur } = require('./kurulum'); const [dir, sS, out, mod = 'kahin', uS, tS] = process.argv.slice(2), UFUK = +uS || 120, TUR = new Set((tS || 'pres,alici,surme').split(','));
const { M, m, g } = kur(dir, +sS, 0, {
  'shape.js': [["cand = defs.filter(p => !up.has(p)", "cand = defs.filter(p => !(self.__pyasak && self.__pyasak.has(p.id)) && !up.has(p)"],
    ["const q = defs.find(d => d.id === id); if (q) press.push(q);", "const q = defs.find(d => d.id === id); if (q && !(self.__pyasak && self.__pyasak.has(q.id))) press.push(q);"]],
  'match.js': [['m.fl = { t0: m.tick, kind: o.kind,', 'm.fl = { o, t0: m.tick, kind: o.kind,'], ["if (!r) r = D.decide(h, ps, m.ch, m.r, tE); let o = r.best;", "if (!r) r = D.decide(h, ps, m.ch, m.r, tE); let o = r.best; if (self.__karar) o = self.__karar(m, h, r, o) || o;"]] });
const D = g.AlanDecide, W = (g.ALAN_SAHA && g.ALAN_SAHA.W) || 120, sinif = k => /^sür/.test(k) ? 'sür' : /^(tut|fake)/.test(k) ? 'dur' : 'diğer', presJob = j => /^pres|^fake: |^kenara itiyor/.test(j || '');
const deger = (c, T, s0, g0) => { let v = (c.score[T] - g0[T]) - (c.score[1 - T] - g0[1 - T]) + .2 * ((c.st.shot[T] - s0[T]) - (c.st.shot[1 - T] - s0[1 - T])); const h = c.holder; if (h) v += (h.team === T ? .5 : -.5) * D.threat(h.team, h.x, h.y); return v; };
// Kopyada bir kolu oynat: kur(c) kolun ayarını uygular, her tikten önce adimOnce(c, t) çağrılır
function kol(snap, T, seed, kur2, adimOnce) { const c = M.kopya(snap); c.r.s = seed; const s0 = c.st.shot.slice(), g0 = c.score.slice(); const eski = { k: g.__karar, y: g.__pyasak, f: g.ALAN_TOPA_GEL_ESKI };
  kur2(c); for (let t = 0; t < UFUK && !c.over; t++) { if (adimOnce) adimOnce(c, t); M.step(c); } const v = deger(c, T, s0, g0); g.__karar = eski.k; g.__pyasak = eski.y; g.ALAN_TOPA_GEL_ESKI = eski.f; return v; }
const S = { pres: { n: 0, bas: 0, basma: 0 }, alici: { n: 0, gel: 0, eski: 0 }, surme: { n: 0, sur: 0, dur: 0, akil: 0, degisti: 0 }, kopyaTik: 0 };
let yasakBitis = 0, yasakId = null, aliciBitis = 0, aliciEski = false, flSon = null; const onceki = new Map(), sonPres = new Map();
g.__pyasak = null; g.__karar = null;
const t0 = Date.now(), LEN = +process.env.LEN || m.len || 7200;
while (!m.over && m.tick < LEN) {
  // 3 · koşan taşıyıcı: kararı bu tikte gelecekse adımdan önce anlık görüntü; karar geri çağrısında sür ile fake/tut başa başsa kopyada tartılır
  if (mod === 'kahin' && TUR.has('surme') && m.holder && m.decT === 1 && m.holder.role !== 'Bekçi' && Math.hypot(m.holder.vx || 0, m.holder.vy || 0) > .15) {
    const h = m.holder, T = h.team, snap = M.kopya(m), seed = (m.tick * 2654435761 + 99) >>> 0, tik = m.tick;
    const zorla = cl => (mm, hh, r, o) => { if (hh.id !== h.id || mm.tick > tik + 1) return o; const z = r.opts.find(x => sinif(x.kind) === cl); return z || o; };
    g.__karar = (mm, hh, r, o) => { g.__karar = null; if (hh.id !== h.id || sinif(o.kind) === 'diğer') return o; const a = r.opts.find(x => sinif(x.kind) === 'sür'), b = r.opts.find(x => sinif(x.kind) === 'dur'); if (!a || !b || Math.abs(a.v - b.v) > .03) return o;
      const V = { 'sür': kol(snap, T, seed, () => { g.__karar = zorla('sür'); }), 'dur': kol(snap, T, seed, () => { g.__karar = zorla('dur'); }) }; S.kopyaTik += 2 * UFUK; S.surme.n++;
      const en = V['sür'] >= V['dur'] ? 'sür' : 'dur'; S.surme[en === 'sür' ? 'sur' : 'dur']++; if (sinif(o.kind) !== en) S.surme.degisti++; return en === 'sür' ? a : b; }; }
  M.step(m); if (mod === 'kahin') g.__karar = null;
  if (m.tick >= yasakBitis) { g.__pyasak = null; yasakId = null; }
  if (m.tick >= aliciBitis || !m.fl) g.ALAN_TOPA_GEL_ESKI = false;
  if (mod !== 'kahin') { continue; }
  // 1 · pres başlangıcı
  if (TUR.has('pres')) for (const p of m.ps) { const j = presJob(p.job), o = onceki.get(p.id); onceki.set(p.id, j); if (!j || o || p.role === 'Bekçi' || p.id === yasakId || m.tick - (sonPres.get(p.id) ?? -1e9) < 120) continue; sonPres.set(p.id, m.tick); const h = m.holder; if (!h || h.team === p.team) continue;
    const T = p.team, snap = M.kopya(m), seed = (m.tick * 2654435761 + 7) >>> 0;
    const vB = kol(snap, T, seed, () => { g.__pyasak = null; }), vY = kol(snap, T, seed, () => { g.__pyasak = new Set([p.id]); }); S.kopyaTik += 2 * UFUK; S.pres.n++;
    if (vY > vB) { S.pres.basma++; g.__pyasak = new Set([p.id]); yasakId = p.id; yasakBitis = m.tick + 60; } else S.pres.bas++; }
  // 2 · arkadaşa pas çıktı
  if (TUR.has('alici') && m.fl && m.fl !== flSon) { flSon = m.fl; const o = m.fl.o; if (o && o.q && o.kind !== 'gönder' && !o.kendine) { const T = o.q.team, snap = M.kopya(m), seed = (m.tick * 2654435761 + 31) >>> 0;
      const vG = kol(snap, T, seed, () => { g.ALAN_TOPA_GEL_ESKI = false; }, (c, t) => { if (!c.fl) g.ALAN_TOPA_GEL_ESKI = false; }), vE = kol(snap, T, seed, () => { g.ALAN_TOPA_GEL_ESKI = true; }, (c, t) => { if (!c.fl) g.ALAN_TOPA_GEL_ESKI = false; }); S.kopyaTik += 2 * UFUK; S.alici.n++;
      if (vE > vG) { S.alici.eski++; g.ALAN_TOPA_GEL_ESKI = true; aliciBitis = m.tick + 90; } else S.alici.gel++; } }
}
const sn = (Date.now() - t0) / 1000, st = m.st, res = { sc: +sS, mod, skor: m.score, sut: st.shot, pas: st.pass, pasTut: st.passOk, kapma: st.steal, sn: +sn.toFixed(0), msTik: +(1000 * sn / m.tick).toFixed(1), karar: S };
fs.writeFileSync(out, JSON.stringify(res)); console.log(JSON.stringify(res));
