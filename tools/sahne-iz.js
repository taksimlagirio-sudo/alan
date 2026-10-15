// Sahne takibi: maç, topa sahip olma dönemlerine (sahne) bölünür; her sahnede seçilen oyuncuların adım adım ne yaptığı.
// Sahne: aynı takımın topu tuttuğu ya da pasının havada olduğu kesintisiz dönem. Her oyuncu için: görev sırası (0,25 sn'den kısa görevler birleşir,
// iki görev arasında gidip gelme "A ↔ B ×n" diye yazılır), hareket (nereden nereye, ileri / geri, durduğu süre), top ona geldiyse ne yaptı ve akıl neyi gördü.
// Kullanım: node tools/sahne-iz.js <kayıt.json.gz> <çıktı.json> <oyuncu>...  (metin dökümü stdout'a)
const { oku } = require('./kayit-oku.js'), fs = require('fs');
const [f, out, ...adlar] = process.argv.slice(2), k = oku(f), N = k.N, W = k.saha.W, EV = ['geçiş', 'kurma', 'ilerleme', 'son bölge', 'geçiş', 'kurma', 'orta', 'son bölge'];
const X = (t, i) => k.P.x[t * N + i] / 100, Y = (t, i) => k.P.y[t * N + i] / 100, BX = t => k.T.bx[t] / 100, BY = t => k.T.by[t] / 100, TM = i => k.oyuncular[i].takim, AD = i => k.oyuncular[i] ? k.oyuncular[i].ad : '?';
const G = (t, i) => (k.gorevler[k.P.job[t * N + i]] || '').replace(/ · tepki/g, '') || '—', ilerle = (T, x) => T === 0 ? x : W - x, sn = t => (t / 60).toFixed(1);
// sahip takım: tutan ya da havadaki topun takımı
const tk = new Int8Array(k.n); for (let t = 0; t < k.n; t++) { const h = k.T.hold[t]; tk[t] = h !== 255 ? TM(h) : k.T.bt[t]; }
const sahneler = []; let s = null; for (let t = 0; t < k.n; t++) { const T = tk[t]; if (T < 0) { if (s) s.t1 = t; continue; } if (!s || s.T !== T) { if (s) sahneler.push(s); s = { T, t0: t, t1: t }; } else s.t1 = t; } sahneler.push(s);
const olaylar = k.olay; const sonuc = sc => { const o = olaylar.filter(o => o.t >= sc.t0 && o.t <= sc.t1 + 2 && (o.tur === 'sayi' || o.tur === 'gonder' || o.tur === 'kesme' || o.tur === 'direk')); return o.map(o => o.metin); };
function oyuncuSahne(i, sc) { const T = TM(i), a = sc.t0, b = sc.t1, gs = []; let cur = null;
  for (let t = a; t <= b; t++) { const g = k.T.hold[t] === i ? '◆ top bende' : G(t, i); if (!cur || cur.g !== g) { if (cur) gs.push(cur); cur = { g, t0: t, n: 0 }; } cur.n++; } gs.push(cur);
  // kısa görevleri birleştir, gidip gelmeyi say
  const uzun = gs.filter(x => x.n >= 15 || x.g === '◆ top bende'), sira = []; for (const x of uzun) { const l = sira[sira.length - 1]; if (l && l.g === x.g) { l.n += x.n; continue; } if (sira.length >= 2 && sira[sira.length - 2].g === x.g && !l.git) { const p = sira[sira.length - 2]; p.git = (p.git || 0) + 1; p.ile = l.g; p.n += x.n + l.n; sira.pop(); continue; } if (sira.length >= 2 && sira[sira.length - 2].g === x.g && l.git == null) { } sira.push({ ...x }); }
  let gidip = 0; for (let j = 2; j < gs.length; j++) if (gs[j].g === gs[j - 2].g && gs[j - 1].n < 30) gidip++;
  let yol = 0, dur = 0; for (let t = a + 1; t <= b; t++) { const d = Math.hypot(X(t, i) - X(t - 1, i), Y(t, i) - Y(t - 1, i)); yol += d; if (d < .03) dur++; }
  const r = { gorev: sira.map(x => x.git ? `${x.g} ↔ ${x.ile} ×${x.git + 1}` + ` (${sn(x.n)} sn)` : `${x.g} (${sn(x.n)} sn)`), gidip, yol: +yol.toFixed(1), durSn: +(dur / 60).toFixed(1), ileri0: +ilerle(T, X(a, i)).toFixed(0), ileri1: +ilerle(T, X(b, i)).toFixed(0), y0: +Y(a, i).toFixed(0), y1: +Y(b, i).toFixed(0), topIleri0: +ilerle(T, BX(a)).toFixed(0), topIleri1: +ilerle(T, BX(b)).toFixed(0) };
  // top bendeyken: kararlar ve pas
  const kr = k.karar.filter(c => c.kim === i && c.t >= a && c.t <= b).map(c => ({ t: c.t, s: c.s.map(x => [x[0], x[1], x[2], x[3]]) })), ps = k.pas.filter(p => p.kim === i && p.t >= a && p.t <= b + 1).map(p => ({ t: p.t, tur: p.tur, alici: p.alici != null ? AD(p.alici) : null, P: p.P, Pk: p.Pk }));
  const al = []; for (let t = Math.max(1, a); t <= b; t++) if (k.T.hold[t] === i && k.T.hold[t - 1] !== i) al.push(t); r.kararlar = kr; r.paslar = ps; r.alis = al;
  return r; }
const R = sahneler.filter(sc => sc.t1 - sc.t0 >= 20 || sonuc(sc).length).map((sc, n) => { const ev = []; for (let t = sc.t0; t <= sc.t1; t += 10) { const e = EV[(sc.T === 0 ? k.T.ev0 : k.T.ev1)[t]]; if (e && ev[ev.length - 1] !== e) ev.push(e); }
  return { n: n + 1, T: sc.T, takim: sc.T === 0 ? 'Liman' : 'Demir Blok', t0: sc.t0, t1: sc.t1, evre: ev, sonuc: sonuc(sc), oy: Object.fromEntries(adlar.map(ad => [ad, oyuncuSahne(k.oyuncular.findIndex(o => o.ad === ad), sc)])) }; });
fs.writeFileSync(out, JSON.stringify({ ad: k.ad, skor: k.skor, oyuncular: k.oyuncular.map(o => [o.ad, o.takim, o.rol]), sahneler: R }));
for (const sc of R) { console.log(`\n#${sc.n} ${sn(sc.t0)}–${sn(sc.t1)} sn · ${sc.takim} topta · ${sc.evre.join(' → ')}${sc.sonuc.length ? ' · ' + sc.sonuc.join(' / ') : ''}`);
  for (const [ad, r] of Object.entries(sc.oy)) { const kr = r.kararlar.map(c => `[${sn(c.t)}] seçti ${c.s[0][0]}${c.s[0][1] ? '→' + c.s[0][1] : ''} ${c.s[0][3].toFixed(3)}; diğer: ${c.s.slice(1).map(x => x[0] + (x[1] ? '→' + x[1] : '') + ' ' + x[3].toFixed(3)).join(', ')}`).join(' | ');
    console.log(`  ${ad}: ${r.gorev.join(' → ')} · ileri ${r.ileri0}→${r.ileri1} (top ${r.topIleri0}→${r.topIleri1}) · yol ${r.yol} · durdu ${r.durSn} sn · gidip-gelme ${r.gidip}${kr ? '\n     ' + kr : ''}${r.paslar.length ? '\n     pas: ' + r.paslar.map(p => `${p.tur}→${p.alici} P ${p.P}`).join(', ') : ''}`); } }
