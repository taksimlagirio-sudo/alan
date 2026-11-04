// Yerleşim (shape.js) vektörleri: gerçek maçtan anlık durumlar; vSp, tepki, gec/gecTab/gecOf, seen, bak, gorulen,
// flightRead, slotBase, durusW, bekci, tehlike, tasHamle, yolBag/yolKesis/presKesis, koyHedef, coverSpots,
// laneOpen, yolT, metrics.
// node sina/sekil-vektor.js ../alan-v5 <kayit.json> sina/sekil-vektor.json
// shape.js'in iç işlevleri (yolKesis, presKesis, tasHamle, yolBag, koyHedef, coverSpots, laneOpen, yolT) yükleme
// sırasında AlanShape'e eklenir (yalnız sınama için).
const fs = require('fs'), path = require('path');
const dir = path.resolve(process.argv[2]);
globalThis.self = globalThis; globalThis.window = globalThis; globalThis.location = { search: '' }; globalThis.postMessage = () => {};
globalThis.importScripts = (...F) => {
  for (const f0 of F) {
    const f = f0.replace(/\?.*$/, '');
    let s = fs.readFileSync(path.join(dir, f), 'utf8');
    if (f === 'shape.js') s = s.replace('window.AlanShape = { aliciDirenc,', 'window.AlanShape = { yolKesis, presKesis, tasHamle, yolBag, koyHedef, coverSpots, laneOpen, yolT, aliciDirenc,');
    (0, eval)(s + '\n//# sourceURL=' + f);
  }
};
const isci = fs.readFileSync(path.join(dir, 'mac-isci.js'), 'utf8');
(0, eval)(isci + '\nself.__isci = { kur: (d) => { kur = d; const mm = M.createMatch(d.tohum, { tac: d.tac, len: d.len, diz: d.diz, devre: d.devre ?? false }); if (d.oz === "rastgele") rastgele(mm); kurUygula(mm, d); return mm; } };');
const M = self.AlanMatch, AS = self.AlanShape, kurMac = self.__isci.kur;
if (!AS.yolKesis) throw new Error('shape.js iç işlevleri eklenemedi');
const F = new Float64Array(1), U = new BigUint64Array(F.buffer), b = v => { if (v === null || v === undefined) return null; if (typeof v === 'boolean') return v; F[0] = v; return U[0].toString(16); };
const m = M.kafaAc(kurMac(JSON.parse(fs.readFileSync(process.argv[3])).yeni));
let s = 9191; const lcg = () => { s = (s * 1103515245 + 12345) % 2147483648; return s / 2147483648; };
const ix = q => q ? m.ps.indexOf(q) : null;
const ser = (p, ball) => ({
  id: p.id, team: p.team, i: p.i, role: p.role, name: p.name || '', bek: p.role === 'Bekçi',
  a: Object.fromEntries(Object.entries(p.a || {}).filter(([k, v]) => typeof v === 'number').map(([k, v]) => [k, b(v)])),
  slot: { d: b(p.slot.d), l: b(p.slot.l) }, rslot: p.rslot ? { d: b(p.rslot.d), l: b(p.rslot.l) } : null,
  x: b(p.x), y: b(p.y), vx: b(p.vx || 0), vy: b(p.vy || 0), tx: b(p.tx), ty: b(p.ty), en: b(p.en), den: b(p.den), R: b(p.R), D: b(p.D ?? 1),
  press: !!p.press, noTouch: !!p.noTouch, sonuk: !!p.sonuk, kuyuT: b(p.kuyuT), cd: b(p.cd || 0),
  hist: (p.hist || []).map(h => [b(h.x), b(h.y), b(h.vx), b(h.vy)]),
  past: (p.past || []).map(h => [b(h.x), b(h.y)]),
  bk: p._bk ? Array.from(p._bk, r => r ? [b(r.x), b(r.y), b(r.vx), b(r.vy), b(r.t), b(r.z)] : null) : null,
  spr: b(self.ALAN_SPR(p)),
  sprH: b(self.ALAN_SPR({ a: p.a, x: p.x, y: p.y, R: p.R, D: p.D, team: p.team, role: p.role, press: p.press })),
  sprF: b(self.ALAN_SPR({ a: p.a, x: p.x, y: p.y, vx: 0, vy: 0 })),
  rcvB: !!(ball && p._rcvB === ball), rcvAbs: b(p._rcvAbs), rcvV: b(p._rcvV), rcvX: b(p._rcvX), rcvY: b(p._rcvY), amA: !!p.amA, am: b(p.am || 0), cone: !!p.cone, chB: !!(ball && p.chB === ball),
});
const serBall = B => B ? { x: b(B.x), y: b(B.y), vx: b(B.vx), vy: b(B.vy), e: b(B.e), ch: b(B.ch), team: B.team, alive: B.alive, dead: b(B.dead || 0), t: b(B.t), dist: b(B.dist || 0), walls: b(B.walls || 0), tp: b(B.tp), w0: b(B._w0), d0: B._d0 === true ? 'true' : b(B._d0), defl: B.defl === true ? 'true' : b(B.defl || 0), done: B.done || null, kt: b(B.kt), kz: b(B.kz || 0), gon: !!B.gon, kBek: !!B.kBek, from: ix(B.from), recv: ix(B.recv), cue: b(B.cue || 0), bekId: B.bekId ?? null, bekM: b(B.bekM || 0), sap: !!B.sap, cds: B.cds ? [...B.cds].map(([q, t]) => [m.ps.indexOf(q), b(t)]) : [] } : null;
const serTac = t => ({ blok: t.blok, genislik: t.genislik, risk: b(t.risk ?? .5), durus: t.durus ? { huc: (t.durus.huc || []).map(a => a.map(q => q ? [b(q.f), b(q.y)] : null)), sav: (t.durus.sav || []).map(a => a.map(q => q ? [b(q.f), b(q.y)] : null)) } : null });
const pt = q => ({ x: b(q.x), y: b(q.y) });
const out = [];
let n0 = 0;
while (m.tick < 3600 && out.length < 220) {
  M.step(m);
  if (m.tick % 11) continue;
  const ps = m.ps, B = m.ball, h = m.holder;
  const bk0 = ps.map(p => p._bk ? Array.from(p._bk, r => r ? [b(r.x), b(r.y), b(r.vx), b(r.vy), b(r.t), b(r.z)] : null) : null);
  const kayit = { tick: m.tick, seed: m.seed, holder: ix(h), ball: serBall(B), tac: m.tac.map(serTac), fl: m.fl ? { q: ix(m.fl.q), t0: b(m.fl.t0), kind: m.fl.kind } : null, ps: ps.map(p => ser(p, B)), bk0 };
  // görüş: bak (şimdiki geçmişle) ve seen
  AS.bak(m);
  kayit.bak = ps.map(p => p._bk ? Array.from(p._bk, r => r ? [b(r.x), b(r.y), b(r.vx), b(r.vy), b(r.t), b(r.z)] : null) : null);
  m._snT = undefined;
  kayit.seen = ps.map(p => AS.seen(m, p).map(o => [ix(o.ref), b(o.x), b(o.y), b(o.gvx), b(o.gvy)]));
  kayit.gor = ps.map(p => AS.gorulen(m, p).map(o => o.__g ? [b(o.x), b(o.y), b(o.vx), b(o.vy), b(o.__yas), b(o.__e)] : null));
  // oyuncu başına: hız, tepki, varış tabloları
  kayit.per = ps.map(p => {
    const G = AS.gecTab(p), pts = [];
    for (let k = 0; k < 4; k++) { const x = lcg() * 120, y = lcg() * 60, sv = lcg() < .2 ? null : lcg() * 30 - 2; pts.push([b(x), b(y), b(sv), b(AS.gec(p, x, y, sv ?? undefined)), b(AS.gecOf(G, x, y, sv ?? undefined))]); }
    return { vSp: b(AS.vSp(p)), tepki: b(AS.tepki(p)), gt: G.t.map(b), gy: G.Y.map(y => [b(y.g), y.m ? y.m.map(b) : null]), gvm: b(G.vm), pts };
  });
  // yerler
  kayit.yer = [];
  for (let k = 0; k < 6; k++) {
    const p = ps[Math.floor(lcg() * ps.length)], E = { x: lcg() * 120, y: lcg() * 60 }, att = lcg() < .5;
    kayit.yer.push({ p: ps.indexOf(p), E: pt(E), att, sb: pt(AS.slotBase(m, p, E, att)), el: b(AS.slotBase(m, p, E, att).el), bk: pt(AS.bekci(p, E)), bkm: (() => { try { return pt(AS.bekci(p, E, m)); } catch (e) { return null; } })() });
  }
  kayit.dw = []; for (let k = 0; k < 4; k++) { const f = lcg() * 110; kayit.dw.push([b(f), AS.durusW(f).map(b)]); }
  kayit.metr = AS.metrics(m).swarm;
  // uçuş okuma (top serbestken)
  if (B && !h) { m.sh.fr = null; const fr = AS.flightRead(m); kayit.fr = fr ? { first: ix(fr.first), arr: ps.map(p => { const r = fr.arr.get(p); return [b(r.t), b(r.x), b(r.y), b(r.est)]; }), pts: fr.pts.map(q => [b(q.x), b(q.y)]) } : null; kayit.psSon = ps.map(p => [b(p._rcvV), b(p._rcvAbs), b(p._rcvX), b(p._rcvY)]); }
  // taşıyıcı: tehlike, yollar, kesişme
  if (h) {
    kayit.sec = m._sec ? { h: ix(m._sec.h), t: b(m._sec.t), tut: b(m._sec.tut), o: m._sec.o.map(o => ({ k: o.k, x: b(o.x), y: b(o.y), v: b(o.v), dv: b(o.dv ? 1 : 0) })) } : null;
    const TH = AS.tehlike(m, h); kayit.TH = TH ? TH.map(u => [b(u.an), b(u.w), b(u.w0), b(u.x), b(u.y), u.k, b(u.L)]) : null;
    const dt = 1 - h.team, yol = AS.tasHamle(h, dt);
    kayit.yol = yol.map(Y => [Y.ad, b(Y.ux), b(Y.uy), b(Y.sM)]);
    kayit.kes = [];
    for (const p of ps) {
      if (p.team === h.team || p.role === 'Bekçi') continue;
      const cx = AS.yolBag(p, h); if (!cx) continue;
      const prH = self.ALAN_KOSU_PR(h), prP = self.ALAN_KOSU_PR(p);
      const K = yol.map(Y => { const X = AS.yolKesis(p, h, Y, cx); return [b(X.x), b(X.y), b(X.s), X.once]; });
      const tehl = yol.map(Y => b(AS.__thr ? 0 : self.AlanDecide.threat(h.team, h.x + Y.ux * Y.sM, h.y + Y.uy * Y.sM)));
      const PK = AS.presKesis(p, h, dt, m);
      kayit.kes.push({ p: ps.indexOf(p), vH: b(cx.vH), vP: b(cx.vP), tR0: b(cx.tR0), vSh: b(cx.vSh), prH: prH ? [b(prH.Ts), b(prH.k)] : null, prP: prP ? [b(prP.Ts), b(prP.k)] : null, K, tehl, PK: PK ? pt(PK) : null });
      const ax = lcg() * 2 - 1, ay = lcg() * 2 - 1; kayit.kes[kayit.kes.length - 1].koy = [b(ax), b(ay), pt(AS.koyHedef(p, h, ax, ay))];
    }
  }
  // takım: coverSpots, laneOpen, yolT
  {
    const o = { x: lcg() * 120, y: lcg() * 60 }, E = { x: lcg() * 120, y: lcg() * 60 }, T = [{ x: lcg() * 120, y: lcg() * 60 }, { x: lcg() * 120, y: lcg() * 60 }], R = lcg() * 8, att = lcg() < .5 ? 0 : 1;
    kayit.cs = { o: pt(o), E: pt(E), T: T.map(pt), R: b(R), att, out: AS.coverSpots(o, att, E, T, R).map(pt) };
    const A = { x: lcg() * 120, y: lcg() * 60 }, Bp = { x: lcg() * 120, y: lcg() * 60 }, v = .2 + lcg() * .4;
    kayit.lo = { A: pt(A), B: pt(Bp), att, v: b(v), r: b(AS.laneOpen(ps, att, A, Bp, v)) };
    const p = ps[Math.floor(lcg() * ps.length)], c = { x: lcg() * 120, y: lcg() * 60, kv: { x: lcg() * 120, y: lcg() * 60 } }, yl = AS.yolT(p, c);
    kayit.yt = { p: ps.indexOf(p), c: pt(c), kv: pt(c.kv), t1: b(yl.t1), t: b(yl.t) };
  }
  out.push(kayit);
}
fs.writeFileSync(process.argv[4], JSON.stringify(out));
console.log('anlık', out.length, 'serbest top', out.filter(o => o.fr).length, 'taşıyıcılı', out.filter(o => o.yol).length);
