// C ölçümü: Rüzgâr'ın 6903 kararında (1) Şimşek'e pasın değerinin parçaları, (2) koşusunun önüne pasın yolu ve kimin araya girdiği
const fs = require('fs'), [dir, anF] = process.argv.slice(2); const { M, kurMac } = require('./yukle')(dir); const { oku } = require('./graf');
const A = JSON.parse(fs.readFileSync(anF)); const m0 = kurMac(A.kurulum.yeni), m = oku(A.kopya, () => M.kopya(m0.r)); const AD = self.AlanDecide, D0 = AD.decide, K = self.AlanCore, f3 = v => (+v).toFixed(3);
AD.decide = function (h, src, ch, ...r) { const res = D0.call(this, h, src, ch, ...r); if (h.name !== 'Rüzgâr' || m.tick !== 6903 || AD.D._inLook) return res;
  const team = h.team, S = src.find(p => p.name === 'Şimşek');
  console.log('== değer parçaları (takım', team, 'kaleye x=' + (team === 0 ? 120 : 0) + ')');
  for (const [ad, x, y] of [['Rüzgâr (şimdi)', h.x, h.y], ['Şimşek ayağı', S.x, S.y], ['Şimşek önü (31.9,46)', 31.9, 46], ['sür varışı (33.4,33.4)', 33.4, 33.4]]) {
    const sq = AD.shotQ(src, team, x, y, .6), sp = AD.space(src, team, x, y), fr = AD.front(src, team, x, y), v = AD.V(src, team, x, y, .6); let n = 0; for (const o of src) if (o.team !== team && o.role !== 'Bekçi' && Math.abs(o.x - 0) < Math.abs(x - 0)) n++;
    let nd = 1e9, nk = ''; for (const o of src) if (o.team !== team && o.role !== 'Bekçi') { const d = Math.hypot(o.x - x, o.y - y); if (d < nd) { nd = d; nk = o.name; } }
    console.log(`  ${ad.padEnd(24)} kaleye ${Math.hypot(x, y - 30).toFixed(1)} br · V ${f3(v)} (gönderme ${f3(sq)} · boşluk ${f3(sp)} · önü ${f3(fr)}) · arkasında ${n} savunmacı · en yakın ${nk} ${nd.toFixed(1)} br`); }
  const o = res.opts.find(o => o.kind === 'önüne' && o.q && o.q.name === 'Şimşek' && Math.abs(o.to.x - 31.9) < .3); if (!o) { console.log('önüne pas bulunamadı'); return res; }
  console.log('\n== koşusunun önüne pas: hedef', o.to.x.toFixed(1), o.to.y.toFixed(1), 'launch', o.launch && [o.launch.vx.toFixed(2), o.launch.vy.toFixed(2)], 'det', JSON.stringify(o.det));
  const b = K.makeBall({ x: h.x, y: h.y, vx: o.launch.vx, vy: o.launch.vy, team, ch, from: h, recv: S }); const pr = K.predict(b, src, null, 120), pts = pr.pts || [];
  console.log('  top yolu:', pts.filter((_, i) => i % 5 === 0).slice(0, 10).map((p, i) => `t${i * 5}(${p.x.toFixed(1)},${p.y.toFixed(1)})`).join(' '));
  for (const p of src) { if (p === h || p.role === 'Bekçi') continue; const v = .17 + (p.a.hiz ?? 10) * .006, rt = p.team === team ? 0 : 6 - (self.ALAN_OZ(p, 'tepki')) * .3; let ilk = -1; for (let i = 0; i < pts.length; i++) if (Math.hypot(p.x - pts[i].x, p.y - pts[i].y) / v + rt <= i) { ilk = i; break; } if (ilk >= 0 && ilk < 80) console.log(`  >> ${p.name.padEnd(8)} topa en erken ${ilk}. tikte yetişir (tepki ${rt.toFixed(1)}) · yer (${pts[ilk].x.toFixed(1)},${pts[ilk].y.toFixed(1)})`); }
  { const r = { ...S }, kA = AD.keepP(r, src, null, AD.D.recvKeepT); console.log('  Şimşek ayağında pası alınca topu tutma ihtimali (keepP, ' + AD.D.recvKeepT + ' tik):', f3(kA)); const r2 = { ...S, x: 31.9, y: 46 }; console.log('  koşusunun önünde alınca:', f3(AD.keepP(r2, src, null, AD.D.recvKeepT))); }
  console.log('  pas hızı', Math.hypot(o.launch.vx, o.launch.vy).toFixed(2), 'br/tik · oyuncu hızı', (.17 + (S.a.hiz ?? 10) * .006).toFixed(2), '· D.arrive', AD.D.arrive, 'vmax', AD.D.vmax);
  for (const p of src) { if (p === h || p.role === 'Bekçi') continue; const v = .17 + (p.a.hiz ?? 10) * .006; let best = 1e9, bt = -1; pts.forEach((q, i) => { const fark = Math.hypot(p.x - q.x, p.y - q.y) / v - i; if (fark < best) { best = fark; bt = i; } }); if (best < 25) console.log(`  ${p.name.padEnd(8)} (${p.team === team ? 'arkadaş' : 'RAKİP'}) yola en erken: topun ${bt}. tikinde, ${best < 0 ? (-best).toFixed(0) + ' tik ÖNCE' : best.toFixed(0) + ' tik sonra'} varır · şimdi (${p.x.toFixed(1)},${p.y.toFixed(1)}) hız (${(p.vx || 0).toFixed(2)},${(p.vy || 0).toFixed(2)})`); }
  return res; };
while (m.tick < 6904) M.step(m); process.exit(0);
