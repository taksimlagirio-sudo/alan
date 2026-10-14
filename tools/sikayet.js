// Kullanıcının üç şikâyeti için doğrudan ölçüt. Arg: klasör [senaryo sayısı]
const { oku } = require('./kayit-oku.js'), fs = require('fs'), d = process.argv[2], S = +(process.argv[3] || 99);
const A = { dk: 0, say: 0, yakinDirek: 0, dar: 0, darYakin: 0, kaz: 0, kazIleri: 0, kazDur: 0, kazTik: 0, savT: 0, bosVar: 0, ciftBos: 0, ciftT: 0 };
for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz') && +f.split('.')[0] < S)) {
  const k = oku(d + '/' + f), W = k.saha.W, H = k.saha.H, N = k.oyuncular.length, L = k.LEN, tm = i => k.oyuncular[i].takim, X = (t, i) => k.P.x[t * N + i] / 100, Y = (t, i) => k.P.y[t * N + i] / 100;
  const ileri = (t, x) => t === 0 ? x : W - x; A.dk += L / 3600;
  // 1 · yakın direk sayısı: gönderilen top ağza, gönderenin tarafındaki yarıdan (Bekçi ile yakın direk arasından) giriyor
  k.olay.forEach((o, oi) => { if (o.tur !== 'sayi') return; A.say++;
    let g = null; for (let j = oi - 1; j >= 0; j--) { const p = k.olay[j]; if (p.tur === 'fake') continue; if (p.tur === 'gonder' && p.takim === o.takim) g = p; break; } if (!g) return;
    let t = o.t; while (t > 0 && Math.abs(k.T.bx[t] / 100 - W / 2) < 1 && Math.abs(k.T.by[t] / 100 - H / 2) < 1) t--;
    const sy = k.T.by[g.t] / 100, ey = k.T.by[t] / 100, gx = o.takim === 0 ? W : 0, a = Math.atan2(Math.abs(sy - H / 2), Math.abs(gx - k.T.bx[g.t] / 100)) * 180 / Math.PI;
    const bi = k.oyuncular.findIndex(p => p.takim !== o.takim && p.rol === 'Bekçi'), byy = Y(t, bi);
    const yakin = (sy - H / 2) * (ey - H / 2) > 0 && Math.abs(ey - H / 2) > Math.abs(byy - H / 2) * ((sy - H / 2) * (byy - H / 2) > 0 ? 1 : -1); // gönderenin yarısından ve Bekçi'nin dışından
    if (yakin) A.yakinDirek++; if (a >= 60) { A.dar++; if (yakin) A.darYakin++; } });
  // 2 · kontra: topu kendi yarısında kazanan ilk taşıyıcı, önünde 10 birim boşluk varken; ilk 2 sn tuttuğu sürede ileri gitme ve durma
  let sonT = -1;
  for (let t = 1; t < L - 120; t++) { const h = k.T.hold[t]; if (h === 255) continue; const ht = tm(h);
    if (sonT >= 0 && ht !== sonT && ileri(ht, X(t, h)) < W / 2 && Math.hypot(k.T.bx[t] / 100 - W / 2, k.T.by[t] / 100 - H / 2) > 3) { let bos = true; for (let i = 0; i < N; i++) if (tm(i) !== ht && Math.hypot(X(t, i) - X(t, h), Y(t, i) - Y(t, h)) < 10 && ileri(ht, X(t, i)) > ileri(ht, X(t, h))) bos = false;
      if (bos) { A.kaz++; let u = t, dur = 0; while (u < t + 120 && k.T.hold[u] === h) { if (Math.hypot(k.P.vx[u * N + h], k.P.vy[u * N + h]) < 50) dur++; u++; } A.kazTik += u - t; A.kazIleri += ileri(ht, X(u - 1, h)) - ileri(ht, X(t, h)); A.kazDur += dur; } }
    sonT = ht; }
  // 3 · savunmada boş bırakılan: savunan takımın yarısında, Bekçi hariç bir hücumcunun 8 birim içinde savunmacı yok, aynı anda başka bir hücumcuya 2+ savunmacı 4 birim içinde
  for (let t = 0; t < L; t += 6) { const h = k.T.hold[t]; if (h === 255) continue; const at = tm(h); A.savT++;
    let bos = false, cift = false; for (let i = 0; i < N; i++) { if (tm(i) !== at || i === h || k.oyuncular[i].rol === 'Bekçi') continue; if (ileri(at, X(t, i)) < W / 2) continue;
      let en = 1e9, yak = 0; for (let j = 0; j < N; j++) if (tm(j) !== at && k.oyuncular[j].rol !== 'Bekçi') { const dd = Math.hypot(X(t, j) - X(t, i), Y(t, j) - Y(t, i)); en = Math.min(en, dd); if (dd < 4) yak++; }
      if (en > 8) bos = true; if (yak >= 2) cift = true; }
    if (bos) A.bosVar++; if (bos && cift) A.ciftBos++; }
}
const p = (a, b) => b ? '%' + (100 * a / b).toFixed(0) : '-';
console.log(`1 · sayı ${A.say} · Bekçi ile yakın direk arasından giren ${A.yakinDirek} (${p(A.yakinDirek, A.say)}, dk başına ${(A.yakinDirek / A.dk).toFixed(2)}) · dar açıdan (60°+) sayı ${A.dar}, bunların yakın direkten ${A.darYakin}`);
console.log(`2 · kendi yarısında boşken topu kazanan ${A.kaz} · ilk 2 sn: ileri ort. ${(A.kazIleri / A.kaz).toFixed(1)} birim · topla durduğu süre %${(100 * A.kazDur / A.kazTik).toFixed(0)}`);
console.log(`3 · rakip yarısındaki hücumcu 8 birimde savunmacısız: zamanın ${p(A.bosVar, A.savT)} · bu sırada başka bir hücumcuda 2 savunmacı: ${p(A.ciftBos, A.savT)}`);
