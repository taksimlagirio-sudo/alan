// Üç şikâyetin anları: (1) yakın direkten giren sayılarda Bekçi, (2) kontrada topla duran taşıyıcının kararı, (3) çift adam anlarında savunmacıların görevi
const { oku } = require('./kayit-oku.js'), fs = require('fs'), d = process.argv[2];
const B = [], K = { n: 0, ilk: {}, ornek: [] }, C = { n: 0, cift: {}, bos: {} };
const gor = j => (j || '').split(' · ')[0] || '?', say = (o, k) => o[k] = (o[k] || 0) + 1;
for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
  const k = oku(d + '/' + f), W = k.saha.W, H = k.saha.H, N = k.oyuncular.length, L = k.LEN, tm = i => k.oyuncular[i].takim, X = (t, i) => k.P.x[t * N + i] / 100, Y = (t, i) => k.P.y[t * N + i] / 100, J = (t, i) => k.gorevler[k.P.job[t * N + i]];
  const ileri = (t, x) => t === 0 ? x : W - x;
  // 1
  k.olay.forEach((o, oi) => { if (o.tur !== 'sayi') return; let g = null; for (let j = oi - 1; j >= 0; j--) { const p = k.olay[j]; if (p.tur === 'fake') continue; if (p.tur === 'gonder' && p.takim === o.takim) g = p; break; } if (!g) return;
    let t = o.t; while (t > 0 && Math.abs(k.T.bx[t] / 100 - W / 2) < 1 && Math.abs(k.T.by[t] / 100 - H / 2) < 1) t--;
    const sy = k.T.by[g.t] / 100, ey = k.T.by[t] / 100, gx = o.takim === 0 ? W : 0, bi = k.oyuncular.findIndex(p => p.takim !== o.takim && p.rol === 'Bekçi'), byy = Y(t, bi);
    const yakin = (sy - H / 2) * (ey - H / 2) > 0 && Math.abs(ey - H / 2) > Math.abs(byy - H / 2) * ((sy - H / 2) * (byy - H / 2) > 0 ? 1 : -1); if (!yakin) return;
    const ts = g.t, ex = X(ts, bi), ey0 = Y(ts, bi), tx = k.P.tx[ts * N + bi] / 100, ty = k.P.ty[ts * N + bi] / 100, py = sy < H / 2 ? H / 2 - 5 : H / 2 + 5, px = gx === 0 ? .5 : W - .5;
    let kov = false; for (let u = ts - 150; u < ts; u++) if (u > 0 && J(u, bi) === 'kovala') { kov = true; break; }
    B.push({ f, t: ts, a: Math.atan2(Math.abs(sy - H / 2), Math.abs(gx - k.T.bx[ts] / 100)) * 180 / Math.PI, m: Math.hypot(gx - k.T.bx[ts] / 100, sy - H / 2), hed: Math.hypot(ex - tx, ey0 - ty), direk: Math.hypot(ex - px, ey0 - py), hedDirek: Math.hypot(tx - px, ty - py), cizgi: Math.abs(ex - gx), kov, sure: t - ts, gecis: Math.abs(-(ex - k.T.bx[ts] / 100) * (k.T.by[ts + 3] - k.T.by[ts]) / 100 + (ey0 - k.T.by[ts] / 100) * (k.T.bx[ts + 3] - k.T.bx[ts]) / 100) / (Math.hypot(k.T.bx[ts + 3] - k.T.bx[ts], k.T.by[ts + 3] - k.T.by[ts]) / 100 || 1) }); });
  // 2 · kendi yarısında boşken kazanan, 15+ tik topla duran (hız < 0,05)
  let sonT = -1;
  for (let t = 1; t < L - 120; t++) { const h = k.T.hold[t]; if (h === 255) continue; const ht = tm(h);
    if (sonT >= 0 && ht !== sonT && Math.hypot(k.T.bx[t] / 100 - W / 2, k.T.by[t] / 100 - H / 2) > 3) { let bos = true; for (let i = 0; i < N; i++) if (tm(i) !== ht && Math.hypot(X(t, i) - X(t, h), Y(t, i) - Y(t, h)) < 8 && ileri(ht, X(t, i)) > ileri(ht, X(t, h))) bos = false;
      if (bos) { let u = t, dur = 0, ilkDur = -1; while (u < t + 180 && k.T.hold[u] === h) { if (Math.hypot(k.P.vx[u * N + h], k.P.vy[u * N + h]) < 50) { dur++; if (ilkDur < 0) ilkDur = u; } u++; }
        if (dur >= 12) { K.n++; const kr = k.karar.filter(x => x.kim === h && x.t >= t && x.t <= ilkDur + 15).pop(); if (!kr) say(K.ilk, 'topu aldığından beri karar yok: ' + gor(J(ilkDur, h))); if (kr) { say(K.ilk, kr.s[0][0].split(' ')[0]); if (K.ornek.length < 8) K.ornek.push(`${f} t${ilkDur} ${k.oyuncular[h].ad} (${X(t, h).toFixed(0)},${Y(t, h).toFixed(0)}) ${dur} tik durdu · ` + kr.s.slice(0, 3).map(s => `${s[0].slice(0, 28)}${s[1] ? '→' + s[1] : ''} ${(+s[3]).toFixed(3)}`).join(' | ')); } } } }
    sonT = ht; }
  // 3 · çift adam ve boş adam aynı anda (savunan yarısında)
  for (let t = 0; t < L; t += 6) { const h = k.T.hold[t]; if (h === 255) continue; const at = tm(h); let bosI = -1, ciftJ = null;
    for (let i = 0; i < N; i++) { if (tm(i) !== at || i === h || k.oyuncular[i].rol === 'Bekçi' || ileri(at, X(t, i)) < W / 2) continue;
      const yak = []; let en = 1e9; for (let j = 0; j < N; j++) if (tm(j) !== at && k.oyuncular[j].rol !== 'Bekçi') { const dd = Math.hypot(X(t, j) - X(t, i), Y(t, j) - Y(t, i)); en = Math.min(en, dd); if (dd < 4) yak.push(j); }
      if (en > 8 && bosI < 0) bosI = i; if (yak.length >= 2 && !ciftJ) ciftJ = { i, yak }; }
    if (bosI >= 0 && ciftJ) { C.n++; const js = ciftJ.yak.map(j => gor(J(t, j))).sort().join(' + '); say(C.cift, (ciftJ.i === h ? 'taşıyıcıda: ' : 'topsuzda: ') + js);
      let en = 1e9, ej = -1; for (let j = 0; j < N; j++) if (tm(j) !== at && k.oyuncular[j].rol !== 'Bekçi') { const dd = Math.hypot(X(t, j) - X(t, bosI), Y(t, j) - Y(t, bosI)); if (dd < en) { en = dd; ej = j; } } say(C.bos, gor(J(t, ej))); } }
}
const top = o => Object.entries(o).sort((a, b) => b[1] - a[1]).slice(0, 8).map(([a, b]) => `${b} ${a}`).join(' · ');
console.log(`(1) Bekçi ile yakın direk arasından giren sayı: ${B.length}`);
const o = k2 => (B.reduce((s, x) => s + x[k2], 0) / B.length).toFixed(1);
console.log(`  gönderme anında Bekçi: hedefine ${o('hed')} · yakın direğe ${o('direk')} (hedefinin direğe uzaklığı ${o('hedDirek')}) · çizgiden ${o('cizgi')} · top yolunun Bekçi'ye uzaklığı ${o('gecis')} · yol ${o('sure')} tik · öncesinde kovalamış ${B.filter(x => x.kov).length}/${B.length} · yerinde (hedefine ≤ 3) ${B.filter(x => x.hed <= 3).length}/${B.length}`);
for (const x of B) console.log(`  ${x.f} t${x.t} ${x.a.toFixed(0)}° ${x.m.toFixed(0)}br · Bekçi hedefine ${x.hed.toFixed(1)} · direğe ${x.direk.toFixed(1)} · hedef direğe ${x.hedDirek.toFixed(1)} · yola ${x.gecis.toFixed(1)} · ${x.sure} tik${x.kov ? ' · kovalamıştı' : ''}`);
console.log(`(2) top kazanılınca (santra hariç, önünde 8 birim rakip yokken) ilk 3 sn'de topla 12+ tik duran: ${K.n} · durduğu andaki kararın ilk seçeneği: ${top(K.ilk)}`); console.log(K.ornek.map(x => '  ' + x).join('\n'));
console.log(`(3) boş adam + çift adam anı (6 tikte bir örnek): ${C.n}\n  çift adamdaki savunmacıların görevi: ${top(C.cift)}\n  boş adama en yakın savunmacının görevi: ${top(C.bos)}`);
