// Kontrol tablosu: bugüne kadar bulunan sorunların hepsini aynı ölçülerle, kayıt klasörlerinden (kayit.js) ölçer ve yan yana koyar.
// Kullanım: node tools/kontrol-tablosu.js <klasör1>[=ad] <klasör2>[=ad] ...   (ilk klasör karşılaştırma tabanı)
// Maç süreleri farklı olabilir: sayımlar dakika başına. Her satırın yanında "iyi yön" (↑ / ↓ / 0) yazar; tabana göre kötüye giden ⚠ alır.
const { oku, ol } = require('./kayit-oku.js'), fs = require('fs');

function olc(d) {
  const A = { mac: 0, dk: 0, tik: 0, hs: 0, say: 0, gon: 0, gonSay: 0, pas: 0, pasTut: 0, kesme: 0, disari: 0,
    kov: 0, kovBiz: 0, sayYer: 0, yakinDirek: 0, dar: 0, darSay: 0, darYakin: 0, dirN: 0, dirUz: 0, uzGen: 0, uzGenSay: 0, yakGen: 0, yakGenSay: 0,
    kazBos: 0, kazDur: 0, verN: 0, verGeri: 0, baskiN: 0, tutSec: 0, tutKay: 0,
    savT: 0, bosVar: 0, iki4: 0, kaz3: 0, kaz3gon: 0, git: 0, kontraSay: 0, hwN: 0, hwKay: 0, hwGer: 0, temVar: 0, onN: 0, onYakin: 0, onUz: 0, kosAl: 0, kosDevam: 0, kosDur: 0, ilkK: 0, ilkKN: 0, ilkOT: 0, tasT: 0, tasKos: 0, temN: 0, temTut: 0, temKuyu: 0, temKuyuSay: 0, saySay: 0, sayDeg: 0, cift: 0, ciftArka: 0, holdT: 0, presYok: 0, hucDur: 0, uzunDur: 0, durN: 0 };
  for (const f of fs.readdirSync(d).filter(f => f.endsWith('.json.gz'))) {
    const k = oku(d + '/' + f), W = k.saha.W, H = k.saha.H, N = k.oyuncular.length, L = k.LEN, tm = i => k.oyuncular[i].takim;
    const X = (t, i) => k.P.x[t * N + i] / 100, Y = (t, i) => k.P.y[t * N + i] / 100, J = (t, i) => k.gorevler[k.P.job[t * N + i]], ileri = (t, x) => t === 0 ? x : W - x;
    const bekci = tk => k.oyuncular.findIndex(p => p.takim === tk && p.rol === 'Bekçi'), santra = t => Math.hypot(k.T.bx[t] / 100 - W / 2, k.T.by[t] / 100 - H / 2) < 3;
    A.mac++; A.dk += L / 3600; A.tik += L; A.hs += k.hesapSn;
    const g = ol.genel(k); A.pas += g.pas; A.pasTut += g.pasTut * g.pas;
    const du = ol.durma(k); A.hucDur += du.hucDuran; A.uzunDur += du.uzun.uzunDuran || 0; A.durN++;
    // Çekirdek saha dışında (Kuyu ağzı hariç)
    for (let t = 0; t < L; t++) if (k.T.hold[t] === 255) { const x = k.T.bx[t] / 100, y = k.T.by[t] / 100; if ((x < .49 || x > W - .49 || y < .49 || y > H - .49) && Math.abs(y - H / 2) > 5.5) A.disari++; }
    // Göndermeler ve sayılar
    k.olay.forEach((o, oi) => {
      if (o.tur === 'kesme') A.kesme++;
      if (o.tur === 'sayi') A.say++;
      if (o.tur !== 'gonder') return; A.gon++;
      let s = false; for (let j = oi + 1; j < k.olay.length; j++) { const p = k.olay[j]; if (p.tur === 'fake') continue; s = p.tur === 'sayi' && p.takim === o.takim; break; }
      const t = o.t, sx = k.T.bx[t] / 100, sy = k.T.by[t] / 100, gx = o.takim === 0 ? W : 0, dx = Math.abs(gx - sx), dy = Math.abs(sy - H / 2), a = Math.atan2(dy, dx) * 180 / Math.PI, m = Math.hypot(dx, dy);
      const bi = bekci(1 - o.takim), q = t * N + bi, hed = k.P.tx[q] === -32768 ? 0 : Math.hypot(X(t, bi) - k.P.tx[q] / 100, Y(t, bi) - k.P.ty[q] / 100);
      if (a >= 60) { A.dar++; A.dirN++; A.dirUz += Math.hypot(X(t, bi) - (gx === 0 ? .5 : W - .5), Y(t, bi) - (sy < H / 2 ? H / 2 - 5 : H / 2 + 5)); }
      if (a >= 60 && a < 75 && m >= 25) { A.uzGen++; if (s) A.uzGenSay++; }
      if (a >= 60 && a < 75 && m < 15) { A.yakGen++; if (s) A.yakGenSay++; }
      if (s && k.tem) { A.saySay++; const sn = k.olay.find((p, j) => j > oi && p.tur === 'sayi').t; if (k.tem.some(e => e.gon && e.t >= t && e.t <= sn)) A.sayDeg++; }
      if (!s) return; A.gonSay++; if (a >= 60) A.darSay++; if (hed <= 3) A.sayYer++;
      // yakın direk: gönderenin yarısından, Bekçi'nin dış tarafından giren
      let te = k.olay.find((p, j) => j > oi && p.tur === 'sayi').t; while (te > t && Math.abs(k.T.bx[te] / 100 - W / 2) < 1 && Math.abs(k.T.by[te] / 100 - H / 2) < 1) te--;
      const ey = k.T.by[te] / 100, by = Y(te, bi);
      if ((sy - H / 2) * (ey - H / 2) > 0 && Math.abs(ey - H / 2) > Math.abs(by - H / 2) * ((sy - H / 2) * (by - H / 2) > 0 ? 1 : -1)) { A.yakinDirek++; if (a >= 60) A.darYakin++; }
    });
    if (k.tem) { A.temVar++; for (const e of k.tem) { if (!e.gon) continue; A.temN++; if (e.ovx == null) { A.temTut++; continue; } const gx = e.bek === 0 ? 0 : W, tt = Math.abs(e.ovx) > 1e-6 ? (gx - e.ox) / e.ovx : -1; if (tt > 0 && Math.abs(e.oy + e.ovy * tt - H / 2) <= 6) { A.temKuyu++; if (k.olay.some(o => o.tur === 'sayi' && o.t > e.t && o.t <= e.t + 60)) A.temKuyuSay++; } } }
    { const hz = (t, i) => Math.hypot(k.P.vx[t * N + i], k.P.vy[t * N + i]) / 1000;
      for (const p of k.pas) { if (p.tur !== 'önüne' || p.alici == null || !p.to) continue; let u = p.t + 1; while (u < L && k.T.hold[u] === 255) u++; if (u >= L - 40 || k.T.hold[u] !== p.alici) continue; const h = p.alici;
        A.onN++; const dd = Math.hypot(k.T.bx[u] / 100 - p.to[0] / 100, k.T.by[u] / 100 - p.to[1] / 100); A.onUz += dd; if (dd <= 3) A.onYakin++;
        if (hz(Math.max(0, u - 3), h) > .15) { A.kosAl++; let dur = false; for (let w = u; w < u + 20 && k.T.hold[w] === h; w++) if (hz(w, h) < .05) { dur = true; break; } if (dur) A.kosDur++; if (hz(u + 10, h) > .15 && hz(u + 30, h) > .12) A.kosDevam++;
          const dks = k.dk ? k.dk.find(x => x >= u) : (k.karar.find(x => x.kim === h && x.t >= u) || {}).t; const ot = k.ot ? k.ot.find(([x, i]) => i === h && x >= u - 2 && x <= u + 2) : null;
          if (ot) A.ilkOT++; else if (dks != null && dks < u + 60) { A.ilkK += dks - u; A.ilkKN++; } } }
      for (let t = 0; t < L; t++) { const h = k.T.hold[t]; if (h === 255 || k.oyuncular[h].rol === 'Bekçi') continue; A.tasT++; if (hz(t, h) > .15) A.tasKos++; } }
    // Bekçi pas kovalaması (rakibin göndermesine gidiş hariç) ve sonucu
    for (const bi of [0, 1].map(bekci)) { const bt = tm(bi); let t = 0;
      while (t < L) { if (J(t, bi) !== 'kovala') { t++; continue; } const t0 = t; while (t < L && J(t, bi) === 'kovala') t++; if (t - t0 < 6) continue;
        if (k.olay.some(o => o.tur === 'gonder' && o.takim !== bt && o.t <= t0 && o.t > t0 - 40)) continue;
        let u = t0; while (u < L && k.T.hold[u] === 255) u++; const h = k.T.hold[u]; if (h === 255 || u >= L) continue; A.kov++; if (tm(h) === bt) A.kovBiz++; } }
    // Sahiplik: kontra (top kazanınca, santra hariç, önünde 8 birim boş: 12+ tik topla duran), pasçının geri koşusu, baskısız taşıyıcı
    let sonT = -1, sonH = 255;
    for (let t = 1; t < L; t++) { const h = k.T.hold[t]; if (h === 255) continue; const ht = tm(h);
      if (sonT >= 0 && ht !== sonT && !santra(t) && t < L - 180) { let bos = true; for (let i = 0; i < N; i++) if (tm(i) !== ht && Math.hypot(X(t, i) - X(t, h), Y(t, i) - Y(t, h)) < 8 && ileri(ht, X(t, i)) > ileri(ht, X(t, h))) bos = false;
        if (bos) { A.kazBos++; let u = t, dur = 0; while (u < t + 180 && k.T.hold[u] === h) { if (Math.hypot(k.P.vx[u * N + h], k.P.vy[u * N + h]) < 50) dur++; u++; } if (dur >= 12) A.kazDur++; } }
      if (h !== sonH && sonH !== 255 && sonT === ht) { let t0 = t - 1; while (t0 > 0 && k.T.hold[t0] === 255) t0--; const t1 = Math.min(L - 1, t0 + 60); A.verN++; if (ileri(ht, X(t1, sonH)) - ileri(ht, X(t0, sonH)) < -2) A.verGeri++; }
      sonT = ht; sonH = h;
      if (ileri(ht, X(t, h)) > W / 2) { A.holdT++; let yak = false; for (let i = 0; i < N; i++) if (tm(i) !== ht && k.oyuncular[i].rol !== 'Bekçi' && Math.hypot(X(t, i) - X(t, h), Y(t, i) - Y(t, h)) < 12) { yak = true; break; } if (!yak) A.presYok++; } }
    for (let t = 0; t < L; t++) if (k.T.gcGit[t]) A.git++;
    { let st = -1; for (let t = 1; t < L - 1; t++) { const h = k.T.hold[t]; if (h === 255) continue; const ht = tm(h); if (st >= 0 && ht !== st && !santra(t)) { A.kaz3++; if (k.olay.some(o => o.tur === 'gonder' && o.takim === ht && o.t >= t && o.t < t + 180)) A.kaz3gon++; if (k.olay.some(o => o.tur === 'sayi' && o.takim === ht && o.t >= t && o.t < t + 300)) A.kontraSay++; } st = ht; } }
    // Baskı altında "tut" ve sonrası
    for (const kr of k.karar) { if (kr.kim == null || !kr.s || !kr.s.length || kr.t >= L) continue; const t = kr.t, h = kr.kim; if (k.T.hold[t] !== h) continue;
      let en = 1e9; for (let i = 0; i < N; i++) if (tm(i) !== tm(h)) en = Math.min(en, Math.hypot(X(t, i) - X(t, h), Y(t, i) - Y(t, h))); if (en >= 4) continue;
      A.baskiN++; const hw = kr.s[0][4]; if (/^tut/.test(kr.s[0][0]) && hw) { A.hwN++; A.hwKay += hw[0]; for (let u = t; u < t + Math.max(hw[2] || 30, 30) && u < L; u++) { const g2 = k.T.hold[u]; if (g2 !== 255 && tm(g2) !== tm(h)) { A.hwGer++; break; } } } if (/^tut/.test(kr.s[0][0])) { A.tutSec++; for (let u = t; u < t + 60 && u < L; u++) { const g2 = k.T.hold[u]; if (g2 !== 255 && tm(g2) !== tm(h)) { A.tutKay++; break; } } } }
    // Savunma: boş hücumcu ve çift adam (6 tikte bir)
    for (let t = 0; t < L; t += 6) { const h = k.T.hold[t]; if (h === 255) continue; const at = tm(h); A.savT++; let bos = false, cift = null;
      for (let i = 0; i < N; i++) { if (tm(i) !== at || i === h || k.oyuncular[i].rol === 'Bekçi' || ileri(at, X(t, i)) < W / 2) continue;
        let en = 1e9; const yak = []; for (let j = 0; j < N; j++) if (tm(j) !== at && k.oyuncular[j].rol !== 'Bekçi') { const dd = Math.hypot(X(t, j) - X(t, i), Y(t, j) - Y(t, i)); en = Math.min(en, dd); if (dd < 4) yak.push(j); }
        if (en > 8) bos = true; if (yak.length >= 2 && !cift) cift = yak; }
      for (let i = 0; i < N; i++) { if (tm(i) !== at || i === h || k.oyuncular[i].rol === 'Bekçi') continue; let c = 0; for (let j = 0; j < N; j++) if (tm(j) !== at && k.oyuncular[j].rol !== 'Bekçi' && Math.hypot(X(t, j) - X(t, i), Y(t, j) - Y(t, i)) < 4) c++; if (c >= 2) { A.iki4++; break; } }
      if (bos) A.bosVar++; if (bos && cift) { A.cift++; if (cift.filter(j => /^arkada/.test(J(t, j) || '')).length >= 2) A.ciftArka++; } }
  }
  const p = (a, b) => b ? 100 * a / b : null;
  return { ad: d, mac: A.mac, sn: A.dk * 60 / A.mac, satir: {
    'Sayı / maç': [A.say / A.mac, 0], 'Sayı / dk': [A.say / A.dk, 0], 'Gönderme / dk': [A.gon / A.dk, 0], 'Gönderme başına sayı %': [p(A.gonSay, A.gon), 0],
    'Pas / dk': [A.pas / A.dk, 0], 'Pas tutma %': [p(A.pasTut, A.pas), 1], 'Kesme / dk': [A.kesme / A.dk, 0], 'Hesap ms/tik': [1000 * A.hs / A.tik, -1],
    'Saha dışında Çekirdek (tik)': [A.disari, -1],
    'Bekçi pas kovalaması / dk': [A.kov / A.dk, -1], 'Bekçi kovaladığını kazanıyor %': [p(A.kovBiz, A.kov), 1], 'Sayılarda Bekçi yerinde %': [p(A.sayYer, A.gonSay), 1],
    'Yakın direkten sayı / dk': [A.yakinDirek / A.dk, -1], 'Sayılarda Bekçi topa değmiş %': [A.temVar ? p(A.sayDeg, A.saySay) : null, 1], 'Bekçi teması (gönderme): tuttu %': [A.temVar ? p(A.temTut, A.temN) : null, 0], 'Bekçi teması: Kuyu\'ya seken % (n)': [A.temVar ? p(A.temKuyu, A.temN) : null, -1, A.temVar ? A.temKuyu : null], 'Dar açı (60°+) gönderme başına sayı %': [p(A.darSay, A.dar), -1], 'Dar açıda Bekçi–yakın direk': [A.dirUz / A.dirN, -1],
    'Uzak geniş (25+, 60–75°) sayı % (n)': [p(A.uzGenSay, A.uzGen), -1, A.uzGen], 'Yakın geniş (<15, 60–75°) sayı % (n)': [p(A.yakGenSay, A.yakGen), -1, A.yakGen],
    'Kontra: önü boşken 12+ tik topla duran / dk': [A.kazDur / A.dk, -1], 'Pastan sonra geri koşan pasçı %': [p(A.verGeri, A.verN), -1],
    'Baskı altında "tut" seçimi %': [p(A.tutSec, A.baskiN), -1], '"Tut"tan sonra 1 sn\'de kayıp %': [p(A.tutKay, A.tutSec), -1],
    'Boş hücumcu (rakip yarısı, 8 br) zamanın %': [p(A.bosVar, A.savT), -1], 'Boş adam + çift adam anı / dk (6 tikte bir örnek)': [A.cift / A.dk, -1],
    '  bunlarda iki kişi "arkada" / dk': [A.ciftArka / A.dk, -1], 'Taşıyıcıya baskı yok (rakip yarısı) %': [p(A.presYok, A.holdT), -1], 'Topsuz rakipte 4 br içinde 2 savunmacı (bütün saha) / dk': [A.iki4 / A.dk, -1],
    'Top kazanınca 3 sn içinde gönderme %': [p(A.kaz3gon, A.kaz3), 1], 'Top kazanınca 5 sn içinde sayı / dk': [A.kontraSay / A.dk, 0], 'Geçiş "git" tik / dk': [A.git / A.dk, 1],
    'Baskı altında "tut": akıl kayıp % (n)': [p(A.hwKay, A.hwN), 0, A.hwN || null], 'Baskı altında "tut": gerçek kayıp % (aynı anlar)': [p(A.hwGer, A.hwN), 0],
    'Önüne pas: hedefin ±3 içinde alınan %': [p(A.onYakin, A.onN), 1], 'Önüne pas: alış–hedef uzaklık': [A.onUz / A.onN, -1], 'Koşarak alıp koşmaya devam eden %': [p(A.kosDevam, A.kosAl), 1], 'Koşarak alıp ilk 20 tikte duran %': [p(A.kosDur, A.kosAl), -1],
    'Koşarak alınca ilk karara kadar (tik; tek dokunuş hariç)': [A.ilkKN ? A.ilkK / A.ilkKN : null, -1], 'Koşarak alınca tek dokunuş kararı %': [A.ilkOT ? p(A.ilkOT, A.kosAl) : null, 0], 'Taşıyıcı topla koşuyor %': [p(A.tasKos, A.tasT), 1],
    'Her an duran hücumcu': [A.hucDur / A.durN, -1], 'Uzun tutuşta 1 sn+ duran hücumcu': [A.uzunDur / A.durN, -1] } };
}

if (require.main === module) {
  const R = process.argv.slice(2).map(a => { const [d, ad] = a.split('='); const r = olc(d); r.ad = ad || d.split('/').pop(); return r; });
  const fmt = v => v == null || isNaN(v) ? '–' : Math.abs(v) >= 100 ? v.toFixed(0) : Math.abs(v) >= 10 ? v.toFixed(1) : v.toFixed(2);
  console.log('| Ölçü | ' + R.map(r => `${r.ad} (${r.mac} maç, ${r.sn.toFixed(0)} sn)`).join(' | ') + ' |');
  console.log('|---|' + R.map(() => '---').join('|') + '|');
  for (const key of Object.keys(R[0].satir)) {
    const yon = R[0].satir[key][1], taban = R[0].satir[key][0];
    console.log(`| ${key} ${yon > 0 ? '↑' : yon < 0 ? '↓' : ''} | ` + R.map((r, i) => { const [v, , n] = r.satir[key]; let s = fmt(v) + (n != null ? ` (${n})` : '');
      if (i > 0 && yon && v != null && taban != null && !isNaN(v) && !isNaN(taban)) { const fark = (v - taban) * yon, esik = Math.max(Math.abs(taban) * .15, .02); if (fark < -esik) s += ' ⚠'; else if (fark > esik) s += ' ✓'; } return s; }).join(' | ') + ' |');
  }
}
module.exports = { olc };
