// Temas vektörleri: gerçek maçtan anlık durumlar; touchCap, ctrlP, menzil, bekR, thruP, kuyuGirer, contact.
// node sina/temas-vektor.js ../alan-v5 <kayit.json> sina/temas-vektor.json
const path = require('path'), fs = require('fs');
const { M, kurMac } = require('./yukle')(path.resolve(process.argv[2]));
const C = self.AlanCore, F = new Float64Array(1), U = new BigUint64Array(F.buffer), b = v => { if (v === null || v === undefined) return null; if (typeof v === 'boolean') return v; F[0] = v; return U[0].toString(16); };
const m = M.kafaAc(kurMac(JSON.parse(fs.readFileSync(process.argv[3])).yeni));
const RAW = ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', 'refleks', 'yerlesim', 'hakimiyet', 'enerji', 'govde', 'caliskanlik'];
const ser = (p, ball) => ({ id: p.id, team: p.team, bek: p.role === 'Bekçi', a: Object.fromEntries(Object.entries(p.a || {}).filter(([k, v]) => typeof v === 'number').map(([k, v]) => [k, b(v)])), x: b(p.x), y: b(p.y), vx: b(p.vx || 0), vy: b(p.vy || 0), tx: b(p.tx), ty: b(p.ty), en: b(p.en), den: b(p.den), noTouch: !!p.noTouch, sonuk: !!p.sonuk, kuyuT: b(p.kuyuT || 0), R: b(p.R), D: b(p.D), cone: !!p.cone, fx: b(p.fx || 0), fy: b(p.fy || 0), amA: !!p.amA, am: b(p.am || 0), rcvB: !!(ball && p._rcvB === ball), spr: b(self.ALAN_SPR(p)), dalis: b(p._dalis || 0) });
let s = 4242; const lcg = () => { s = (s * 1103515245 + 12345) % 2147483648; return s / 2147483648; };
const out = [];
while (m.tick < 3000 && out.length < 400) {
  M.step(m);
  const B = m.ball; if (!B || m.holder || B.done || m.tick % 3) continue;
  const ps = m.ps, ix = q => q ? ps.indexOf(q) : null;
  const bs = { x: b(B.x), y: b(B.y), vx: b(B.vx), vy: b(B.vy), e: b(B.e), ch: b(B.ch), team: B.team, alive: B.alive, dead: b(B.dead || 0), t: b(B.t), dist: b(B.dist || 0), walls: b(B.walls || 0), tp: b(B.tp), w0: b(B._w0), d0: B._d0 === true ? 'true' : b(B._d0), defl: B.defl === true ? 'true' : b(B.defl || 0), kt: b(B.kt), kz: b(B.kz || 0), gon: !!B.gon, kBek: !!B.kBek, from: ix(B.from), recv: ix(B.recv), cue: b(B.cue || 0), bekId: B.bekId ?? null, bekM: b(B.bekM || 0), sap: !!B.sap, cds: B.cds ? [...B.cds].map(([q, t]) => [ps.indexOf(q), b(t)]) : [] };
  // touchCap / ctrlP / menzil / bekR / thruP: her oyuncu (topun kopyasıyla: _oku önbelleği yan etki)
  const kop = () => { const c = Object.assign({}, B, { log: [], _oku: undefined, cds: B.cds ? new Map(B.cds) : undefined }); return c; };
  const per = ps.map(p => { const c = kop(); const T = C.touchCap(c, p, ps); const c2 = kop(); return { cap: b(T.cap), vr: b(T.vr), s: b(T.s), sk: b(T.sk), cp: b(C.ctrlP(kop(), p, ps, undefined, .7)), mz: b(C.menzil(p)), br: p.role === 'Bekçi' ? b(C.bekR(c2, p)) : null, th: b(C.thruP(B, p)) }; });
  const kg = []; for (let i = 0; i < 6; i++) { const p = ps[Math.floor(lcg() * ps.length)], x = lcg() < .5 ? lcg() * 16 : 104 + lcg() * 16, y = 20 + lcg() * 20, t = lcg() * 60, cik = lcg() < .3; kg.push([ps.indexOf(p), b(x), b(y), b(t), cik, C.kuyuGirer(p, x, y, t, cik)]); }
  // contact: yapay segment (top bir tik geri)
  const st = ser; const pss = ps.map(p => ser(p, B));
  const cB = kop(), cps = ps.map(p => Object.assign(Object.create(Object.getPrototypeOf(p)), p)); const map = new Map(ps.map((p, i) => [p, cps[i]]));
  cB.from = B.from ? map.get(B.from) : B.from; cB.recv = B.recv ? map.get(B.recv) : B.recv; if (B.cds) cB.cds = new Map([...B.cds].map(([q, t]) => [map.get(q), t]));
  for (const q of cps) if (q._rcvB === B) q._rcvB = cB;
  const rr = []; const rnd = () => { const v = lcg(); rr.push(b(v)); return v; };
  const ox = B.x, oy = B.y, hq = cps[Math.floor(lcg() * cps.length)]; cB.x = hq.x + (lcg() - .5) * 3; cB.y = hq.y + (lcg() - .5) * 3; const bas = { x: b(cB.x), y: b(cB.y) };
  const R = C.contact(cB, cps, undefined, rnd, ox, oy);
  out.push({ tick: m.tick, ps: pss, ball: bs, per, kg, ox: b(ox), oy: b(oy), bas, rnd: rr, res: R ? { p: cps.indexOf(R.p), took: R.took, P: b(R.P), thru: !!R.thru } : null, son: { x: b(cB.x), y: b(cB.y), vx: b(cB.vx), vy: b(cB.vy), e: b(cB.e), ch: b(cB.ch), done: cB.done || null, defl: cB.defl === true ? 'true' : b(cB.defl || 0) }, pson: cps.map(q => [b(q.x), b(q.y), b(q._dalis || 0)]) });
}
fs.writeFileSync(process.argv[4], JSON.stringify(out)); console.log('anlık', out.length, 'temaslı', out.filter(o => o.res).length, 'tutulan', out.filter(o => o.res && o.res.took).length);
