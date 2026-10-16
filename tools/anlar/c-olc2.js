// C ölçümü 2: Rüzgâr'ın kararında sürmenin yer değeri ile "sür, sonra en iyi hamle" değeri; sonraki hamlenin ne olduğu
const fs = require('fs'), [dir, anF, tS] = process.argv.slice(2), T = +tS; const { M, kurMac } = require('./yukle')(dir); const { oku } = require('./graf');
const A = JSON.parse(fs.readFileSync(anF)); const m0 = kurMac(A.kurulum.yeni), m = oku(A.kopya, () => M.kopya(m0.r)); const AD = self.AlanDecide, D0 = AD.decide, f3 = v => (+v).toFixed(3); let kayit = [];
AD.decide = function (h, src, ch, ...r) { if (h.name !== 'Rüzgâr' || m.tick !== T || AD.D._inLook || AD.D._noHold) return D0.call(this, h, src, ch, ...r);
  kayit = []; AD.D._dbgSonra = (o, b2, r2, HW, Ts, h2) => kayit.push({ o, b2, top: r2.opts.slice(0, 5).map(x => `${x.kind}${x.q ? '→' + x.q.name : ''} ${f3(x.v)}`), Ts, at: [h2.x, h2.y], lost: HW.lost });
  const res = D0.call(this, h, src, ch, ...r); AD.D._dbgSonra = null;
  console.log(`Rüzgâr ${m.tick} (${h.x.toFixed(1)},${h.y.toFixed(1)}) seçilen: ${res.opts[0].kind} ${f3(res.opts[0].v)}`);
  for (const k of kayit) console.log(`  sür → (${k.o.to.x.toFixed(1)},${k.o.to.y.toFixed(1)}) ${k.Ts} tik · kopyada vardığı yer (${k.at[0].toFixed(1)},${k.at[1].toFixed(1)}) · oradaki en iyi: ${k.b2.kind}${k.b2.q ? '→' + k.b2.q.name : ''} ${f3(k.b2.v)} · son değer ${f3(k.o.v)}\n      oradaki ilk 5: ${k.top.join(' | ')}`);
  self.ALAN_SUR_SONRA_ESKI = true; const r0 = D0.call(this, h, src, ch, ...r); self.ALAN_SUR_SONRA_ESKI = false;
  console.log('  "sonra" hesabı olmadan ilk 6:', r0.opts.slice(0, 6).map(x => `${x.kind}${x.q ? '→' + x.q.name : ''}${x.to ? `(${x.to.x.toFixed(0)},${x.to.y.toFixed(0)})` : ''} ${f3(x.v)}`).join(' | '));
  process.exit(0); };
while (true) M.step(m);
