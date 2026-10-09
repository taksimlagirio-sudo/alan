// Oyuncunun kafası: her gerçek kararda tartılan seçenekleri ve bileşenlerini kaydeder (deney/kafa kopyası gerekir: tools/kafa-kur.sh).
// Kullanım: node tools/kafa.js <tempo> <risk> <tohum> [tik] → kendi yarısındaki kararların özeti + örnekler (../kafa-<tohum>.json'a ham kayıt)
const { load } = require('./load'); const [tS, rS, sS, lS] = process.argv.slice(2), seed = +sS || 1, len = +lS || 2700, tac = { tempo: +tS, risk: +rS };
const g = load(__dirname + '/../deney/kafa', seed), M = g.AlanMatch, D = g.AlanDecide.D; D._dbgOn = true;
const m = M.createMatch(seed, { tac: [tac, tac], len }); while (!m.over && m.tick < len) M.step(m);
const L = D._dbgLog || [], adv = (t, x) => t === 0 ? x : 100 - x, F = (v, d = 2) => v == null ? '–' : (+v).toFixed(d);
const own = L.filter(d => adv(d.team, d.x) < 50 && d.opts.length);
const fwdOf = d => o => o.to && !/gönder|tut/.test(o.kind) && (adv(d.team, o.to.x) - adv(d.team, d.x)) > 5;
const R = { n: own.length, chosen: {}, anyF: 0, goodF: 0, fBest: [], cBest: [], dropF: 0, evalF: 0, fBeatenByLook: 0 };
for (const d of own) { const c = d.opts[0], F_ = d.opts.filter(fwdOf(d)); R.chosen[c.kind.split(' ')[0] + (fwdOf(d)(c) ? '·ileri' : '')] = (R.chosen[c.kind.split(' ')[0] + (fwdOf(d)(c) ? '·ileri' : '')] || 0) + 1;
  R.evalF += F_.length; R.dropF += d.drop.filter(o => o.to && (adv(d.team, o.to.x) - adv(d.team, d.x)) > 5).length;
  if (F_.length) { R.anyF++; if (F_.some(o => o.c && o.c.P >= .85)) R.goodF++; const b = F_[0]; R.fBest.push(b); R.cBest.push(c); if (b.pre != null && c.pre != null && b.pre > c.pre && b.v < c.v) R.fBeatenByLook++; } }
const med = a => { a = a.filter(x => x != null).sort((x, y) => x - y); return a.length ? a[a.length >> 1] : null; };
console.log(`tempo ${tS} risk ${rS} · tohum ${seed} · ${len} tik · skor ${m.score.join('-')} · tüm karar ${L.length}, kendi yarısında ${own.length}`);
console.log('seçilen (kendi yarısında):', Object.entries(R.chosen).sort((a, b) => b[1] - a[1]).map(([k, v]) => `${k} ${Math.round(v / own.length * 100)}%`).join(', '));
console.log(`ileri seçenek (5+ br ileri) tartılan kararlar: %${Math.round(R.anyF / own.length * 100)} · tutma ihtimali ≥0,85 olan ileri seçeneği olan: %${Math.round(R.goodF / own.length * 100)}`);
console.log(`ön elemede elenen ileri seçenek: ${R.dropF} · tartılan ileri seçenek: ${R.evalF}`);
console.log(`en iyi ileri seçenek (medyan): tutma ${F(med(R.fBest.map(o => o.c && o.c.P)))} · varış değeri ${F(med(R.fBest.map(o => o.c && o.c.varis)), 3)} · kayıp bedeli ${F(med(R.fBest.map(o => o.c && o.c.kayip)), 3)} · son değer ${F(med(R.fBest.map(o => o.v)), 3)}`);
console.log(`seçilen hamle (medyan):          tutma ${F(med(R.cBest.map(o => o.c && o.c.P)))} · varış değeri ${F(med(R.cBest.map(o => o.c && o.c.varis)), 3)} · kayıp bedeli ${F(med(R.cBest.map(o => o.c && o.c.kayip)), 3)} · son değer ${F(med(R.cBest.map(o => o.v)), 3)}`);
console.log(`ileriye bakıştan önce ileri seçenek öndeyken bakış sonrası geride kalan: ${R.fBeatenByLook} karar`);
const show = d => { console.log(`\n▶ ${d.name} (${d.team ? 'Turuncu' : 'Mavi'}) · kendi Kuyu'sundan ${Math.round(adv(d.team, d.x))} br · ${(d.tick / 60).toFixed(1)} sn`);
  console.log('  seçenek                     ileri  tutma  varış   kayıp   bakış öncesi  bakış   SON');
  for (const o of d.opts.slice(0, 8)) { const f = o.to ? Math.round(adv(d.team, o.to.x) - adv(d.team, d.x)) : 0; console.log(`  ${(o.kind + (o.q ? ' → ' + o.q : '')).padEnd(26)} ${String(f).padStart(5)}  ${F(o.c && o.c.P).padStart(5)}  ${F(o.c && o.c.varis, 3).padStart(6)}  ${F(o.c && o.c.kayip, 3).padStart(6)}  ${F(o.pre, 3).padStart(12)}  ${F(o.look, 3).padStart(6)}  ${F(o.v, 3).padStart(6)}`); }
  if (d.drop.length) console.log('  ön elemede elenen: ' + d.drop.map(o => `${o.kind}${o.q ? '→' + o.q : ''}${o.to ? '(' + Math.round(adv(d.team, o.to.x) - adv(d.team, d.x)) + ' br)' : ''}`).join(', ')); };
const pick = [own.find(d => d.opts.filter(fwdOf(d)).length >= 2 && !fwdOf(d)(d.opts[0])), own.find(d => d.opts.filter(fwdOf(d)).some(o => o.c && o.c.P >= .85) && !fwdOf(d)(d.opts[0])), own[Math.floor(own.length / 2)]].filter(Boolean);
pick.forEach(show); require('fs').writeFileSync(__dirname + `/../../kafa-${seed}.json`, JSON.stringify(L));
