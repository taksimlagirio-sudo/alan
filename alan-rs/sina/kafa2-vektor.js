// Kafa 2 vektörleri: gerçek maçtan anlık durumlar; kafa 2'nin (ve geçici kopyalarının) yabancıya
// uğramayan saf işlevleri: imkSap/imkTau/imkSg/imkInan, fr, riskOf, prGam, otFactor, sutEngel, geriCekil,
// paylastir, sutTab (şutçusuz, Bekçi yeri verili), threat, pressAt, freeAt, kuyuD, ALAN_SPRE, ALAN_TEPKI.
// İç işlevler dışa açık değil: decide.js'in kopyası `window.AlanDecide = {` satırına ek adlarla yüklenir.
// node sina/kafa2-vektor.js <alan-v5 ya da sade dizin> <kayit.json> sina/kafa2-vektor.json
const path = require('path'), fs = require('fs'), os = require('os');
const kay = path.resolve(process.argv[2]);
const tmp = fs.mkdtempSync(path.join(os.tmpdir(), 'kafa2-'));
for (const f of fs.readdirSync(kay)) if (f.endsWith('.js')) fs.copyFileSync(path.join(kay, f), path.join(tmp, f));
{
  const p = path.join(tmp, 'decide.js'); let s = fs.readFileSync(p, 'utf8');
  const ek = 'imkSap, imkTau, imkSg, imkInan, riskOf, prGam, sutEngel, geriCekil, paylastir, kuyuD, __imk: { get OFS() { return IMK_OFS; } },';
  if (!s.includes('window.AlanDecide = {')) throw Error('AlanDecide satırı yok');
  s = s.replace('window.AlanDecide = {', 'window.AlanDecide = { ' + ek);
  fs.writeFileSync(p, s);
}
const { M, kurMac } = require('./yukle')(tmp);
const AD = self.AlanDecide, F = new Float64Array(1), U = new BigUint64Array(F.buffer), b = v => { if (v === null || v === undefined) return null; if (typeof v === 'boolean') return v; F[0] = v; return U[0].toString(16); };
const m = M.kafaAc(kurMac(JSON.parse(fs.readFileSync(process.argv[3])).yeni));
const ser = p => ({ id: p.id, team: p.team, role: p.role, name: p.name, a: Object.entries(p.a || {}).filter(([k, v]) => typeof v === 'number').map(([k, v]) => [k, b(v)]), x: b(p.x), y: b(p.y), vx: b(p.vx || 0), vy: b(p.vy || 0), tx: b(p.tx), ty: b(p.ty), en: b(p.en), den: b(p.den), noTouch: !!p.noTouch, press: !!p.press, R: b(p.R), D: b(p.D), tlRisk: p.tl && p.tl.huc ? b(p.tl.huc.risk) : null, tlTempo: p.tl && p.tl.huc ? b(p.tl.huc.tempo) : null, egRk: p.eg ? b(p.eg.rk) : null });
let s = 777; const r = () => { s = (s * 1103515245 + 12345) % 2147483648; return s / 2147483648; };
const out = { sap: [], anlar: [] };
for (let i = 0; i < 400; i++) { const t = r() < .1 ? Math.floor(r() * 200) : r() * 200; out.sap.push([b(t), b(AD.imkSap(t)), b(AD.imkTau(t)), b(AD.imkSg(t / 20 - 5)), b(AD.imkInan(t))]); }
while (m.tick < 1300 && out.anlar.length < 35) {
  M.step(m);
  if (m.tick % 37) continue;
  const ps = m.ps, A = { tick: m.tick, ps: ps.map(ser), per: [], nok: [], gc: [], pl: [], st: [], rk: [] };
  for (const p of ps) A.per.push([b(self.ALAN_SPRE(p)), b(self.ALAN_SPR(p)), b(self.ALAN_TEPKI(p, 'kovala')), b(self.ALAN_TEPKI(p, 'algi')), b(self.ALAN_TEPKI(p, 'bosTop')), b(AD.otFactor(p, r()))]);
  for (let j = 0; j < 12; j++) {
    const team = r() < .5 ? 0 : 1, x = r() * 120, y = r() * 60;
    A.nok.push([team, b(x), b(y), b(AD.threat(team, x, y)), b(AD.pressAt(ps, team, x, y)), b(AD.freeAt(ps, team, x, y)), b(AD.kuyuD(x, y)), b(AD.sutEngel(ps, team, x, y))]);
  }
  for (let j = 0; j < 4; j++) {
    const team = r() < .5 ? 0 : 1, A0 = { x: 20 + r() * 80, y: 5 + r() * 50 }, E = { x: A0.x + (r() - .5) * 16, y: A0.y + (r() - .5) * 16 }, T = r() * 30, ilk = r() < .5;
    A.gc.push([team, b(A0.x), b(A0.y), b(E.x), b(E.y), b(T), ilk, AD.geriCekil(ps, team, A0, E, T, ilk).map(q => [b(q.x), b(q.y), b(q.vx), b(q.vy)])]);
    const T2 = r() * 40; A.pl.push([team, b(T2), AD.paylastir(ps, team, T2, new Set()).map(q => [b(q.x), b(q.y)])]);
  }
  for (let j = 0; j < 8; j++) {
    const team = r() < .5 ? 0 : 1, gx = team === 0 ? 120 : 0, x = gx + (team === 0 ? -1 : 1) * (6 + r() * 34), y = 30 + (r() - .5) * 36, ch = r(), yer = { x: gx + (team === 0 ? -1 : 1) * r() * 4, y: 30 + (r() - .5) * 6 }, pos = { x: yer.x + (r() - .5) * 3, y: yer.y + (r() - .5) * 3 }, tO = r() < .5 ? undefined : r() * 30, sig = r() < .5 ? undefined : r() * 2;
    // imkKov'un önbelleği maçın tikine bağlı (aynı tikte başka kopyanın değeri kalabilir): taze tik
    const mk = AD.D._mac; AD.D._mac = { tick: -1 - 100 * m.tick - j, tac: mk && mk.tac };
    let v; try { v = AD.sutTab(ps, team, x, y, ch, null, sig, yer, pos, tO); } finally { AD.D._mac = mk; }
    A.st.push([team, b(x), b(y), b(ch), b(yer.x), b(yer.y), b(pos.x), b(pos.y), b(tO), b(sig), b(v)]);
  }
  for (let j = 0; j < 6; j++) {
    const h = ps[Math.floor(r() * ps.length)], tac = r() < .2 ? null : { risk: r() < .3 ? undefined : r(), tempo: r() < .3 ? undefined : r(), sabir: r() < .3 ? undefined : r(), kontra: r() < .3, macK: r() < .5 ? undefined : .5 + r() };
    A.rk.push([ps.indexOf(h), tac && { risk: b(tac.risk), tempo: b(tac.tempo), sabir: b(tac.sabir), kontra: tac.kontra, macK: b(tac.macK) }, b(AD.__riskOf ? 0 : AD.riskOf(h, tac)), b(AD.prGam(h, tac))]);
  }
  out.anlar.push(A);
}
fs.writeFileSync(process.argv[4], JSON.stringify(out));
fs.rmSync(tmp, { recursive: true, force: true });
console.log('anlık', out.anlar.length);
