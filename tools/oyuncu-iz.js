// Oyuncu takibi: bir kayıtta seçilen oyuncuların maçı an an. Durum (top bende / hücumda topsuz / savunmada / top havada) ve görev değiştikçe yeni bölüm.
// Her bölüm: süre, ne yaptı (yol, hız, topa / kaleye göre), görev; top bendeyse akılın seçenekleri ve sonucu. Açık kurallarla işaretler.
// Kullanım: node tools/oyuncu-iz.js <kayıt.json.gz> <çıktı.json> <oyuncu adı>...
const { oku } = require('./kayit-oku.js'), fs = require('fs');
const [f, out, ...adlar] = process.argv.slice(2), k = oku(f), N = k.N, W = k.saha.W, H = k.saha.H, EV = ['huc.gecis', 'huc.kurma', 'huc.ilerleme', 'huc.son', 'sav.gecis', 'sav.kurma', 'sav.orta', 'sav.son'];
const X = (t, i) => k.P.x[t * N + i] / 100, Y = (t, i) => k.P.y[t * N + i] / 100, VX = (t, i) => k.P.vx[t * N + i] / 1000, VY = (t, i) => k.P.vy[t * N + i] / 1000;
const BX = t => k.T.bx[t] / 100, BY = t => k.T.by[t] / 100, gorev = (t, i) => (k.gorevler[k.P.job[t * N + i]] || '').replace(/ · tepki$/, '').replace(/ · tepki/, '') || '(görevsiz)';
const takimOf = i => k.oyuncular[i].takim, adOf = i => k.oyuncular[i] ? k.oyuncular[i].ad : '?', kale = T => T === 0 ? 0 : W; /* kendi kalesi */
const ilerle = (T, x) => T === 0 ? x : W - x; /* rakip kaleye doğru ne kadar ileride (0 = kendi çizgimiz) */
function durum(t, i) { const T = takimOf(i), h = k.T.hold[t]; if (h === i) return 'top bende'; if (h !== 255) return takimOf(h) === T ? 'hücum · topsuz' : 'savunma'; const bt = k.T.bt[t]; return bt === T ? 'hücum · top havada' : bt === 1 - T ? 'savunma · top havada' : 'serbest top'; }
const evre = (t, T) => EV[(T === 0 ? k.T.ev0 : k.T.ev1)[t]] || '';
const R = {};
for (const ad of adlar) { const i = k.oyuncular.findIndex(o => o.ad === ad), T = takimOf(i), bol = []; let cur = null;
  for (let t = 0; t < k.n; t++) { const d = durum(t, i), g = d === 'top bende' ? 'top' : gorev(t, i), key = d + '|' + g; if (!cur || cur.key !== key) { if (cur) { cur.t1 = t - 1; bol.push(cur); } cur = { key, d, g, t0: t }; } }
  cur.t1 = k.n - 1; bol.push(cur);
  for (const b of bol) { let yol = 0, dur = 0; for (let t = b.t0 + 1; t <= b.t1; t++) { const s = Math.hypot(X(t, i) - X(t - 1, i), Y(t, i) - Y(t - 1, i)); yol += s; if (s < .03) dur++; }
    const n = b.t1 - b.t0 + 1; Object.assign(b, { sn: +(n / 60).toFixed(2), yol: +yol.toFixed(1), hiz: +(yol / n).toFixed(3), durPay: +(dur / n).toFixed(2), x0: +X(b.t0, i).toFixed(1), y0: +Y(b.t0, i).toFixed(1), x1: +X(b.t1, i).toFixed(1), y1: +Y(b.t1, i).toFixed(1),
      ileri0: +ilerle(T, X(b.t0, i)).toFixed(1), ileri1: +ilerle(T, X(b.t1, i)).toFixed(1), top0: +Math.hypot(X(b.t0, i) - BX(b.t0), Y(b.t0, i) - BY(b.t0)).toFixed(1), top1: +Math.hypot(X(b.t1, i) - BX(b.t1), Y(b.t1, i) - BY(b.t1)).toFixed(1), evre: evre(b.t0, T),
      topIleri: +ilerle(T, BX(b.t0)).toFixed(1) });
    // en yakın rakip (bölüm başında ve sonunda)
    const yr = t => { let d = 1e9, j = -1; for (let q = 0; q < N; q++) { if (takimOf(q) === T) continue; const e = Math.hypot(X(t, q) - X(t, i), Y(t, q) - Y(t, i)); if (e < d) { d = e; j = q; } } return [+d.toFixed(1), adOf(j)]; };
    [b.rakip0, b.rakipAd0] = yr(b.t0); [b.rakip1, b.rakipAd1] = yr(b.t1);
    if (b.d === 'top bende') { b.kararlar = k.karar.filter(c => c.kim === i && c.t >= b.t0 - 1 && c.t <= b.t1 + 1).map(c => ({ t: c.t, s: c.s.map(s => ({ tur: s[0], kime: s[1], P: s[2], v: s[3] })) }));
      const p = k.pas.find(p => p.kim === i && p.t >= b.t0 && p.t <= b.t1 + 2); if (p) b.pas = { tur: p.tur, alici: p.alici != null ? adOf(p.alici) : null, P: p.P, Pk: p.Pk };
      const s = k.sahip.find(s => s.t > b.t1 && s.kim != null); if (s) b.sonra = { t: s.t, kim: adOf(s.kim), biz: takimOf(s.kim) === T };
      b.olay = k.olay.filter(o => o.t >= b.t0 && o.t <= b.t1 + 3 && o.metin.includes(adOf(i))).map(o => o.metin); }
    // işaretler (açık kurallar)
    const im = [];
    if ((b.d === 'hücum · topsuz') && b.sn >= 1 && b.durPay > .7 && b.ileri0 > 60) im.push('Hücumda, rakip yarısında 1 sn+ duruyor');
    if (b.d === 'hücum · topsuz' && /geri koşu/.test(b.g) && b.sn >= .5) im.push('Takımı hücumdayken geri koşuyor');
    if (/^savunma/.test(b.d) && /^pres/.test(b.g) && b.ileri0 < 30) im.push('Kendi kalesine 30 birimden yakın pres (kopyalı ölçümde zararlı)');
    if (/^savunma/.test(b.d) && b.sn >= 1 && b.durPay > .7 && b.top0 < 15 && !/Bekçi/.test(b.g)) im.push('Savunmada topa yakınken 1 sn+ duruyor');
    if (b.g === 'alıcı' && b.t1 - b.t0 >= 10) { const t = b.t1, t2 = b.t1 - 10, d1 = Math.hypot(X(t, i) - BX(t), Y(t, i) - BY(t)), d0 = Math.hypot(X(t2, i) - BX(t), Y(t2, i) - BY(t)); if (d1 > d0 + .5) im.push('Alıcı son 10 tikte topun geleceği yerden uzaklaşıyor'); }
    if (b.d === 'top bende' && b.sonra && !b.sonra.biz && b.sonra.t - b.t1 < 90) im.push('Top bendeyken / pasından sonra 1,5 sn içinde top rakibe geçti');
    if (b.d === 'top bende' && b.kararlar && b.kararlar.length) { const c = b.kararlar[0], s = c.s[0]; if (s && /^(tut|fake)/.test(s.tur)) { const sur = c.s.find(x => x.tur === 'sür'); if (sur && s.v - sur.v < .01) im.push('Sürmek ile durmak başa baş, durmayı seçti'); } }
    b.isaret = im; }
  // özet
  const oz = {}; for (const b of bol) { const o = oz[b.d] = oz[b.d] || { sn: 0, yol: 0, gorev: {} }; o.sn += b.sn; o.yol += b.yol; o.gorev[b.g] = (o.gorev[b.g] || 0) + b.sn; }
  for (const o of Object.values(oz)) { o.hiz = +(o.yol / (o.sn * 60)).toFixed(3); o.sn = +o.sn.toFixed(1); o.yol = +o.yol.toFixed(0); o.gorev = Object.entries(o.gorev).sort((a, b) => b[1] - a[1]).slice(0, 6).map(([g, s]) => [g, +s.toFixed(1)]); }
  R[ad] = { i, T, rol: k.oyuncular[i].rol, oz, bol: bol.map(({ key, ...b }) => b) }; }
// anlık görüntü için konumlar: bir tikte herkes ve top
const resim = t => ({ t, top: [BX(t), BY(t)], hold: k.T.hold[t], ps: k.oyuncular.map((o, i) => [+X(t, i).toFixed(1), +Y(t, i).toFixed(1), o.takim, o.ad, +(k.P.tx[t * N + i] / 100).toFixed(1), +(k.P.ty[t * N + i] / 100).toFixed(1)]) });
const iz = (i, t0, t1) => { const a = []; for (let t = t0; t <= t1; t += 3) a.push([+X(t, i).toFixed(1), +Y(t, i).toFixed(1)]); return a; }, topIz = (t0, t1) => { const a = []; for (let t = t0; t <= t1; t += 3) a.push([BX(t), BY(t)]); return a; };
fs.writeFileSync(out, JSON.stringify({ ad: k.ad, skor: k.skor, saha: k.saha, oyuncular: k.oyuncular.map(o => ({ ad: o.ad, takim: o.takim, rol: o.rol })), R, olay: k.olay.filter(o => o.tur === 'sayi' || o.tur === 'gonder').map(o => [o.t, o.metin]) }));
module.exports = { resim, iz, topIz };
if (require.main === module) for (const [ad, r] of Object.entries(R)) { console.log(`\n== ${ad} (${r.rol}) · bölüm ${r.bol.length} · işaretli ${r.bol.filter(b => b.isaret.length).length}`); for (const [d, o] of Object.entries(r.oz)) console.log(`  ${d}: ${o.sn} sn · hız ${o.hiz} · ${o.gorev.map(g => g.join(' ')).join(', ')}`);
  const say = {}; for (const b of r.bol) for (const s of b.isaret) say[s] = (say[s] || 0) + 1; console.log('  işaretler:', JSON.stringify(say)); }
