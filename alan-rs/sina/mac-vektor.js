// Maç döngüsü vektörleri: kayıttan kurulum (yerleşim çağrıları kaydedilir), macW/macEff, enerji2 (bir tik),
// ivme, ikili katsayıları, govdeAyir, sprint katı/koşu süresi, Kuyu çıkışı, kafa sapması.
// node sina/mac-vektor.js ../alan-v5 sina/mac-vektor.json <kayit.json>...
// İç işlevler (enerji2, vK, kuyuCikYon …) için match.js'in bir kopyası dışa açılır (yama yalnız dışa açar).
const path = require('path'), fs = require('fs'), os = require('os');
const kay = path.resolve(process.argv[2]);
const yama = fs.mkdtempSync(path.join(os.tmpdir(), 'alan-yama-'));
for (const f of fs.readdirSync(kay)) if (f.endsWith('.js')) fs.copyFileSync(path.join(kay, f), path.join(yama, f));
{
  const f = path.join(yama, 'match.js'); let s = fs.readFileSync(f, 'utf8');
  const a = 'window.AlanMatch = { BEKLE_BOZ,';
  if (!s.includes(a)) throw new Error('match.js dışa açma yeri yok');
  s = s.replace(a, 'window.AlanMatch = { __ic: { enerji2, kuyuCikYon, kuyuCikT, vK, kosuPr, macW, tacEff0, kpAkt, yzUyum, setMt: (m) => { _mt = m.tick; _mH = m.holder; _mM = m; } }, BEKLE_BOZ,');
  fs.writeFileSync(f, s);
}
const { M, kurMac } = require('./yukle')(yama);
const I = M.__ic;
const F = new Float64Array(1), U = new BigUint64Array(F.buffer);
const b = v => { if (v === null || v === undefined) return null; if (typeof v === 'boolean') return v; if (typeof v !== 'number') return v; F[0] = v; return U[0].toString(16); };
const MEK = Object.keys(self.ALAN_YET.MEK);
const NUMS = ['id', 'team', 'i', 'R', 'D', 'x', 'y', 'vx', 'vy', 'tx', 'ty', 'cd', 'en', 'den', '_evx', '_evy', '_yog', '_dolYok', '_Dl', '_D0', '_enOnce', '_bosT', 'kuyuT', '_hizK', '_itK', 'stun', 'adandi', 'sendT', 'sendCd', 'ca', '_korT', 'slowT', 'fiske', 'fiskeCd', '_dke', '_rcvAbs'];
const BOOLS = ['noTouch', 'sonuk', 'sprint', '_topSpr', '_sprIst', 'amA', 'press', '_kuyuCik'];
const ser = (p, ps) => {
  const o = { name: p.name, role: p.role, job: p.job ?? null, a: {}, n: {}, bo: {} };
  for (const [k, v] of Object.entries(p.a || {})) o.a[k] = typeof v === 'number' ? b(v) : v;
  for (const k of NUMS) if (p[k] !== undefined && p[k] !== null) o.n[k] = b(p[k]);
  for (const k of BOOLS) o.bo[k] = !!p[k];
  o.sf = p._sf ? { t0: b(p._sf.t0), f: p._sf.f } : null;
  return o;
};
const kopyaPs = ps => ps.map(p => Object.assign(Object.create(Object.getPrototypeOf(p)), p, { a: p.a }));
const out = { mek: MEK, kur: [], macW: [], macEff: [], en2: [], ivme: [], ik: [], gov: [], spr: [], kuyu: [], sapma: [], tac: [] };

// ---- kurulum ----
const AS = self.AlanShape, sb0 = AS.slotBase, bk0 = AS.bekci;
let kayit = null;
AS.slotBase = function (m, p, E, att) { const r = sb0.apply(this, arguments); kayit && kayit.push(['s', p.team, p.id, b(E.x), b(E.y), !!att, b(r.x), b(r.y)]); return r; };
AS.bekci = function (p, E, m) { const r = bk0.apply(this, arguments); kayit && kayit.push(['b', p.team, p.id, b(E.x), b(E.y), false, b(r.x), b(r.y)]); return r; };
const maclar = [];
for (const f of process.argv.slice(4)) {
  const d = JSON.parse(fs.readFileSync(f)).yeni;
  kayit = [];
  const m = kurMac(d); M.kafaAc(m);
  const cagri = kayit; kayit = null;
  const ps = m.ps.map(p => ({ ...ser(p), id: p.id, team: p.team, i: p.i, slot: [b(p.slot.d), b(p.slot.l)], rslot: p.rslot ? [b(p.rslot.d), b(p.rslot.l)] : null, slot0: p.slot0 ? [b(p.slot0.d), b(p.slot0.l)] : null, rslot0: p.rslot0 ? [b(p.rslot0.d), b(p.rslot0.l)] : null, rh: p.rh ?? null, rs: p.rs ?? null, tl: p.tl ? { huc: Object.fromEntries(Object.entries(p.tl.huc).filter(([k, v]) => v != null).map(([k, v]) => [k, b(v)])), sav: Object.fromEntries(Object.entries(p.tl.sav).filter(([k, v]) => v != null).map(([k, v]) => [k, b(v)])) } : null, markRef: p.markRef ? m.ps.indexOf(p.markRef) : null, eg: p.eg ? [b(p.eg.ps), b(p.eg.rk)] : null, Y: MEK.map(k => b(self.ALAN_Y(p, k))), oz: ['gorus', 'tepki', 'guc', 'kesme', 'hakimiyet', 'yeralma'].map(k => b(self.ALAN_OZ(p, k))) }));
  out.kur.push({ dosya: path.basename(f), kayit: d, cagri, ps, holder: m.holder ? m.ps.indexOf(m.holder) : null, ch: b(m.ch), decT: b(m.decT), len: b(m.len), devre: m.devre, tac: m.tac.map(t => ({ blok: t.blok, pres: b(t.pres), risk: b(t.risk), tempo: b(t.tempo), kazaninca: t.kazaninca ?? null, sabir: b(t.sabir), yz: !!t.yz })) });
  maclar.push([d, m]);
}

// ---- macW ----
for (const d of [-3, -1, 0, 1, 2]) for (const kal of [0, 30, 600, 2777, 5400, 9000]) { const w = I.macW(d, kal); out.macW.push([d, kal, b(w.g), b(w.c), b(w.e0)]); }

// ---- maç içi anlık durumlar ----
const [d0, m] = maclar[0];
let s = 777; const lcg = () => { s = (s * 1103515245 + 12345) % 2147483648; return s / 2147483648; };
const mser = m => ({ tick: b(m.tick), len: b(m.len), score: m.score.slice(), holder: m.holder ? m.ps.indexOf(m.holder) : null, ball: m.ball ? { team: m.ball.team, done: m.ball.done || null, x: b(m.ball.x), y: b(m.ball.y) } : null, flq: m.fl && m.fl.q ? m.ps.indexOf(m.fl.q) : null, mh: self.__mH ? [self.__mH.team, self.__mH.id] : null, ps: m.ps.map(p => ser(p)) });
while (m.tick < 2400) {
  M.step(m);
  if (m.tick % 7) continue;
  // macEff: skor/kalan süre değişsin
  for (const t of [0, 1]) { const c = { len: m.len, tick: m.tick, score: m.score.slice() }; const r = M.macEff(c, t, {}); out.macEff.push([b(c.len), b(c.tick), c.score, t, b(r.macK), [0, 30, 300, 1200].map(dt => b(r.macZ(dt)))]); }
  // enerji2: bir tik, kopyada
  { const c = M.kopya(m); c._lite = m.tick % 14 === 0; const S = mser(c); I.enerji2(c); out.en2.push({ m: S, lite: c._lite, son: c.ps.map(p => [b(p.en), b(p.D), b(p._Dl), b(p._D0), b(p._yog), b(p._enOnce), b(p._bosT), b(p._evx), b(p._evy)]) }); }
  // ivme
  for (let k = 0; k < 3; k++) { const p0 = m.ps[Math.floor(lcg() * m.ps.length)]; const q = Object.assign(Object.create(Object.getPrototypeOf(p0)), p0); const wx = (lcg() - .5) * .8, wy = (lcg() - .5) * .8, vm = .1 + lcg() * .4; const S = ser(q); M.ivme(q, wx, wy, vm); out.ivme.push({ p: S, wx: b(wx), wy: b(wy), vm: b(vm), vx: b(q.vx), vy: b(q.vy) }); }
  // ikili katsayıları
  { const h = m.holder; for (const p of m.ps) out.ik.push({ p: ser(p), ph: p === h, h: h ? ser(h) : null, kutle: b(M.kutle(p, h)), direnc: b(M.direnc(p, h)), omuzK: b(M.omuzK(p)), ikEn: b(M.ikEn(p)), durt: h ? b(M.durtFark(p, h)) : null, sendS: b(M.sendS(p)), dk: b(self.ALAN_DENGE_KAYIP(p, m.tick)), dh: b(dengeHiz(m, p)), m: { tick: b(m.tick), holder: h ? m.ps.indexOf(h) : null } }); }
  // govdeAyir: oyuncuları yaklaştırıp ayır
  { const c = M.kopya(m); const ps = c.ps; for (const p of ps) { p.x = 50 + lcg() * 8; p.y = 25 + lcg() * 8; if (lcg() < .5) p.den = .3 + .7 * lcg(); } const S = ps.map(p => ser(p)); const hi = c.holder ? ps.indexOf(c.holder) : -1; M.govdeAyir(ps, c.holder); out.gov.push({ ps: S, h: hi, son: ps.map(p => [b(p.x), b(p.y), b(p.vx), b(p.vy), b(p.den)]) }); }
  // sprint katı, koşu süresi, vK
  { I.setMt(m); const S = mser(m); const per = m.ps.map(p => { const d = 3 + lcg() * 30; const pr = I.kosuPr(p); return { spr: b(self.ALAN_SPR(p)), sprK: b(M.sprK(p)), spre: b(self.ALAN_SPRE(p, d)), spre0: b(self.ALAN_SPRE(p)), d: b(d), kt: b(self.ALAN_KOSU_T(p, d, M.spd(p))), km: b(self.ALAN_KOSU_M(p, d / 3, M.spd(p))), pr: pr ? [b(pr.Ts), b(pr.k)] : null, vK: b(I.vK(p)), sgir: b(self.ALAN_EN3_SGIR(p, 0)) }; }); out.spr.push({ m: S, per }); }
  // tacEff, geçiş, evre (kopyada)
  { const c = M.kopya(m); const kp = c.kp ? { team: c.kp.team, ids: c.kp.ids, bitis: b(c.kp.bitis), sahip: c.kp.sahip ? c.ps.indexOf(c.kp.sahip) : null } : null;
    const gcs = g => g ? { team: g.team, t0: b(g.t0), kendi: g.kendi, git: g.git, on: g.on || [], biz: b(g.biz), rakip: b(g.rakip), hizli: !!g.hizli } : null;
    const S = { m: mser(c), score: c.score.slice(), tac: c.tac, tac0: c.tac0 || null, kp, gc: gcs(c.gc), hhKul: !!c.hhKul, yzBas: b(c._yzBas), yzBekle: !!c._yzBekle, devre: c.devre, macC: Object.entries(c._macC || {}).map(([k, w]) => [k, b(w.g), b(w.c), b(w.e0)]) };
    const r = [0, 1].map(t => { const e = M.tacEff(c, t); return { risk: b(e.risk), tempo: b(e.tempo), sabir: b(e.sabir), kontra: !!e.kontra, kp: e._kp || null, macK: b(e.macK) }; });
    const ev = [0, 1].map(t => M.evreBul(c, t)); M.gecisGuncelle(c);
    out.tac.push({ S, r, ev, gc: gcs(c.gc), gecis: c.gecis, hhKul: !!c.hhKul, yzBekle: !!c._yzBekle, kpVar: !!c.kp }); }
  // Kuyu çıkışı: oyuncuyu Kuyu alanına koy (kopyada)
  { const c = M.kopya(m); const p = c.ps[1 + Math.floor(lcg() * 5)]; const gx = lcg() < .5 ? 0 : 120, a = lcg() * Math.PI - Math.PI / 2, r = lcg() * 12; p.x = gx + (gx ? -1 : 1) * Math.cos(a) * r + .5; p.y = 30 + Math.sin(a) * r; p.vx = (lcg() - .5) * .3; p.vy = (lcg() - .5) * .3; const Y = I.kuyuCikYon(c, p); out.kuyu.push({ m: mser(c), p: c.ps.indexOf(p), Y: Y ? [b(Y.k), b(Y.ux), b(Y.uy), b(Y.L), b(Y.x), b(Y.y)] : null, T: b(I.kuyuCikT(c, p)) }); }
}
// kafa sapması: kafa günlüğünün kayıtları
for (const r of (m._kafa && m._kafa.L || []).slice(0, 400)) out.sapma.push({ r: JSON.parse(JSON.stringify(r, (k, v) => k === 'sapma' ? undefined : v)), s: r.sapma });
fs.writeFileSync(process.argv[3], JSON.stringify(out));
console.log('kurulum', out.kur.length, 'macEff', out.macEff.length, 'enerji2', out.en2.length, 'ivme', out.ivme.length, 'ikili', out.ik.length, 'gövde', out.gov.length, 'sprint', out.spr.length, 'kuyu', out.kuyu.length, 'sapma', out.sapma.length, 'tac', out.tac.length);
