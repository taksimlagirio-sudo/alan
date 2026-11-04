// Kafa1 vektörleri: gerçek maçtan anlık durumlar; AlanDecide'in (decide.js 1–4716) işlevleri.
// node sina/kafa1-vektor.js ../alan-v5 <kayit.json> sina/kafa1-vektor.json
const path = require('path'), fs = require('fs');
const { M, kurMac } = require('./yukle')(path.resolve(process.argv[2]));
const C = self.AlanCore, AD = self.AlanDecide, D = AD.D, F = new Float64Array(1), U = new BigUint64Array(F.buffer);
const b = v => { if (v === null || v === undefined) return null; if (typeof v === 'boolean') return v; F[0] = v; return U[0].toString(16); };
const m = M.kafaAc(kurMac(JSON.parse(fs.readFileSync(process.argv[3])).yeni));
const ix = (q) => q ? m.ps.indexOf(q) : null;
const ser = p => ({
  id: p.id, team: p.team, bek: p.role === 'Bekçi', name: p.name,
  a: Object.fromEntries(Object.entries(p.a || {}).filter(([k, v]) => typeof v === 'number').map(([k, v]) => [k, b(v)])),
  x: b(p.x), y: b(p.y), vx: b(p.vx || 0), vy: b(p.vy || 0), tx: b(p.tx), ty: b(p.ty), en: b(p.en), den: b(p.den), noTouch: !!p.noTouch, sonuk: !!p.sonuk, kuyuT: b(p.kuyuT || 0),
  R: b(p.R), D: b(p.D), amA: !!p.amA, am: b(p.am || 0), ca: b(p.ca), don: b(p._don), dke: b(p._dke), sprint: !!p.sprint, job: p.job ?? null, press: !!p.press,
  sf: p._sf ? { t0: b(p._sf.t0), f: p._sf.f } : null, korT: b(p._korT), hamle: p.hamle ? { k: p.hamle.k, hid: p.hamle.hid, t0: b(p.hamle.t0), W: b(p.hamle.W) } : null,
  pplan: p.pplan ? p.pplan.k : null, gvKaz: p._gvKaz ? [b(p._gvKaz.h), b(p._gvKaz.t), b(p._gvKaz.kaz)] : null, fiskeCd: b(p.fiskeCd), sendT: b(p.sendT), sendCd: b(p.sendCd), stun: b(p.stun), adandi: b(p.adandi),
  kal: !!p._kal, kalH: ix(p._kalH), tutQ: ix(p.tutQ), markRef: ix(p.markRef), sarj: p.sarj ?? null, yer: !!p._yer,
  // dış değerler
  spre: b(self.ALAN_SPRE(p)), spr: b(self.ALAN_SPR(p)), sprhk: b(self.ALAN_SPR_HK(p)), dk: b(self.ALAN_DENGE_KAYIP ? self.ALAN_DENGE_KAYIP(p, D._tick || 0) : 0),
  pr: (k => k ? [b(k.Ts), b(k.k)] : null)(self.ALAN_KOSU_PR(p))
});
let s = 777; const lcg = () => { s = (s * 1103515245 + 12345) % 2147483648; return s / 2147483648; };
const kaydet = () => ({ _mac: D._mac, _tick: D._tick, _gon: D._gon, _sut: D._sut, _ot: D._ot, _savT: D._savT, _savCa: D._savCa, _savW: D._savW, _savU: D._savU, _kanca: D._kanca, _sYon: D._sYon, _itme: D._itme, _kenar: D._kenar, _tekD: D._tekD });
const out = [];
const S0 = self.AlanShape.S;
console.log('DOKUNUS', JSON.stringify(self.ALAN_DOKUNUS));
while (m.tick < 5400 && out.length < 150) {
  M.step(m);
  const h = m.holder; if (!h || m.tick % 9) continue;
  const ps = m.ps, team = h.team, gx = team === 0 ? 120 : 0, k0 = kaydet();
  D._mac = null; D._tick = m.tick + .25; D._gon = false; D._sut = .5; D._ot = 1; D._savT = 0; D._savCa = null; D._savW = null; D._savU = null; D._kanca = null; D._sYon = 0; D._itme = null; D._kenar = false; D._tekD = false;
  const o = { tick: D._tick, ps: ps.map(ser), h: ps.indexOf(h), ch: b(m.ch ?? .5) };
  try {
    o.tan = []; for (let i = 0; i < 20; i++) { const x = (lcg() - .5) * 3.2; o.tan.push([b(x), b(Math.tan(x))]); }
    o.thr = []; for (let i = 0; i < 6; i++) { const t = lcg() < .5 ? 0 : 1, x = lcg() * 120, y = lcg() * 60; o.thr.push([t, b(x), b(y), b(AD.threat(t, x, y))]); }
    o.ut = []; for (let i = 0; i < 6; i++) { const L = 1 + lcg() * 85, ar = [D.arrive, D.arriveFirm, D.arriveSpace, .02 + lcg() * 2.4][Math.floor(lcg() * 4)], al = lcg() < .4 ? null : lcg() * L * 1.2; o.ut.push([b(L), b(ar), b(al), b(AD.ucusT(L, ar, al ?? undefined))]); }
    o.yer = []; for (let i = 0; i < 4; i++) { const x = h.x + (lcg() - .5) * 40, y = Math.max(1, Math.min(59, h.y + (lcg() - .5) * 30)), c = lcg(); o.yer.push([b(x), b(y), b(c), b(AD.pressAt(ps, team, x, y)), b(AD.freeAt(ps, team, x, y)), b(AD.holdVal(ps, team, x, y)), b(AD.V(ps, team, x, y, c)), b(AD.shotQ(ps, team, x, y, c)), b(AD.sutKapali(ps, team, x, y, c)), b(AD.oppAt(ps, team, x, y))]); }
    o.sh = AD.sutHedef(h, ps, gx).map(b);
    const cs = AD.coreSide(h); o.cs = [b(cs.ux), b(cs.uy), b(cs.off)]; o.trap = b(AD.trapped(h, ps, 1 - team));
    o.ik = ps.map((q, i) => q.team === team ? null : [i, b(AD.duelP(q, h, ps)), b(AD.govdeVurP(q, h, null, false)), b(AD.commitThr(q, h, ps)), b(AD.girThr(q, h, ps))]).filter(Boolean);
    o.kp = []; for (const T of [8, 20]) { const v = AD.keepP(h, ps, null, T); o.kp.push([null, b(T), b(v), b(AD.keepP.son)]); }
    for (let i = 0; i < 2; i++) { const a = lcg() * 6.283, dir = { x: Math.cos(a), y: Math.sin(a) }, T = 10 + Math.floor(lcg() * 25), v = AD.keepP(h, ps, dir, T); o.kp.push([[b(dir.x), b(dir.y)], b(T), b(v), b(AD.keepP.son)]); }
    // uçuş: şut ve pas (savurma yok: _savT = 0)
    o.uf = [];
    { const ty = o.sh.length ? AD.sutHedef(h, ps, gx)[0] : 30, dx = gx - h.x, dy = ty - h.y, l = Math.hypot(dx, dy) || 1, v = 1.6 + lcg(), ch = .3 + .6 * lcg();
      D._gon = true; D._sut = 'güçlü'; const b0 = { x: h.x, y: h.y, vx: dx / l * v, vy: dy / l * v, ch }; const r = AD.ucusFormul(h, ps, b0, null, null); D._gon = false; D._sut = .5;
      o.uf.push({ gon: true, b0: [b(b0.x), b(b0.y), b(b0.vx), b(b0.vy), b(b0.ch)], r: [r.P, r.Pk, r.Pl, r.end.x, r.end.y, r.endM.x, r.endM.y, r.endO.x, r.endO.y, r.Ps, r.endS.x, r.endS.y, r.endD.x, r.endD.y, r.Psek, r.endSek.x, r.endSek.y].map(b) }); }
    for (let i = 0; i < 2; i++) { const a = lcg() * 6.283, v = .6 + 1.6 * lcg(), ch = .3 + .6 * lcg(), b0 = { x: h.x, y: h.y, vx: Math.cos(a) * v, vy: Math.sin(a) * v, ch }; const r = AD.ucusFormul(h, ps, b0, null, null);
      o.uf.push({ gon: false, b0: [b(b0.x), b(b0.y), b(b0.vx), b(b0.vy), b(b0.ch)], r: [r.P, r.Pk, r.Pl, r.end.x, r.end.y, r.endM.x, r.endM.y, r.endO.x, r.endO.y, r.Ps, r.endS.x, r.endS.y, r.endD.x, r.endD.y, r.Psek, r.endSek.x, r.endSek.y].map(b) }); }
    // fırlatma
    o.lf = []; for (let i = 0; i < 2; i++) { const to = { x: 5 + lcg() * 110, y: 3 + lcg() * 54 }, ar = .1 + lcg(), ch = lcg(); const r = AD.launchFor(h, to, ar, ps, ch); o.lf.push([b(to.x), b(to.y), b(ar), b(ch), b(r.vx), b(r.vy), r.ok]); }
    o.bf = []; { const B = { x: h.x + (lcg() - .5) * 30, y: lcg() < .5 ? .6 : 59.4 }, q = { x: h.x + (lcg() - .5) * 40, y: 10 + lcg() * 40 }, ar = .3, ch = .5; const r = AD.bankFor(h, B, q, ar, ps, ch); o.bf.push([b(B.x), b(B.y), b(q.x), b(q.y), b(ar), b(ch), b(r.vx), b(r.vy), r.ok]); }
    // savurma
    o.sp = []; for (let i = 0; i < 2; i++) { const a = lcg() * 6.283, v = .8 + 1.8 * lcg(), V = { vx: Math.cos(a) * v, vy: Math.sin(a) * v }, ca0 = lcg() * 6.283 - 3.14, it = lcg() < .3; const r = AD.savurPlan(h, V, ca0, 0, 0, ps, it, 1); const bb = { vx: Math.cos(a + 1), vy: Math.sin(a + 1) }, p2 = AD.onPlan(h, V, bb, it, 1);
      o.sp.push([b(V.vx), b(V.vy), b(ca0), it, r ? [r.s, r.caR, r.A, r.P, r.T, r.wR, r.wRel, r.uSh, r.k].map(b) : null, p2 ? [p2.s, p2.wS, p2.phi, p2.caR, p2.kal, p2.Tsw].map(b) : null, b(bb.vx), b(bb.vy)]); }
    o.ae = []; { const v = { vx: 1.2, vy: .4 }, rr = [], rnd = () => { const x = lcg(); rr.push(b(x)); return x; }; const r1 = AD.applyError(h, v, rnd, 1, false, ps); const r2 = AD.applyErrorSav(h, v, rnd, 1, true, .7, .8, ps, 'güçlü'); o.ae = [rr, b(r1.vx), b(r1.vy), b(r2.vx), b(r2.vy)]; }
    // fake
    o.fk = { T: b(AD.fakeT(h)), p: [] }; for (const q of ps) if (q.team !== team) { const sh = { k: ['gönder', 'pas', 'gövde'][Math.floor(lcg() * 3)], to: { x: h.x + (lcg() - .5) * 30, y: h.y + (lcg() - .5) * 20 }, c: lcg() }; const t = AD.fakeHedef(q, h, sh); o.fk.p.push([ps.indexOf(q), sh.k, b(sh.to.x), b(sh.to.y), b(sh.c), b(AD.fakeP(h, q, sh.k, sh.c)), b(t.x), b(t.y), AD.fakeIlgili(q, h, sh)]); }
    o.cr = []; for (const [st, ham] of [[1, true], [0, false], [.5, true]]) { const c = lcg(); o.cr.push([b(st), ham, b(c), b(AD.chargeRate(h, ps, st, c, ham))]); }
    // kovalama planı
    { const a = lcg() * 6.283, bl = C.makeBall({ x: h.x, y: h.y, vx: Math.cos(a) * 1.5, vy: Math.sin(a) * 1.5, team, ch: .5 }), pts = C.predict(bl, ps, null, 120).pts, opp = ps.filter(q => q.team !== team); const pl = AD.chasePlan(pts, opp, 0, { lag: 0, loose: false });
      o.cp = { pts: pts.map(p => [b(p.x), b(p.y)]), r: [...pl].map(([p, g]) => [ps.indexOf(p), b(g.t), b(g.x), b(g.y), b(g.rc)]) }; }
    // girme eşiği maçla (D._mac; pres tartısı varsa yerDeger'i kaydet)
    D._mac = m; D._tick = m.tick + .5; o.mtick = m.tick; const tP = m._prT;
    o.mac = { ch: b(m.ch), gs: m.sh && m.sh.gs ? { t: b(m.sh.gs.t), dt: m.sh.gs.dt, oy: !!m.sh.gs.oy } : null, tac: [0, 1].map(k => { const tq = m.tac[k] || {}, ME = self.AlanMatch.macEff; return [b(tq.siddet), !!tq.yz, b(tq.yz && ME ? (ME(m, k, {}).macK ?? 1) : 1)]; }),
      prT: tP ? { team: tP.team, hId: tP.hId ?? null, t: b(tP.t), h: b(tP.h), hm: tP.hm ? [...tP.hm].map(([k, v]) => [k, b(v)]) : null, hmk: tP.hm && tP.hm.k ? [...tP.hm.k] : null } : null, vk: [], ge: [] };
    for (const q of ps) { if (q.team === team) continue; const c = m.ch ?? .3, cs2 = AD.coreSide(h), kx = h.x + cs2.ux * C.Q.body, ky = h.y + cs2.uy * C.Q.body; if (tP) o.mac.vk.push([q.team + '|' + Math.round(kx) + ',' + Math.round(ky), b(AD.yerDeger(ps, q.team, kx, ky, c))]); }
    for (const q of ps) { if (q.team === team) continue; const g = AD.girEsik(q, h, ps); const thr = AD.girThr(q, h, ps); o.mac.ge.push([ps.indexOf(q), b(g), b(thr), b(AD.commitThr(q, h, ps))]); }
  } finally { Object.assign(D, k0); }
  out.push(o);
}
fs.writeFileSync(process.argv[4], JSON.stringify(out)); console.log('anlık', out.length);
