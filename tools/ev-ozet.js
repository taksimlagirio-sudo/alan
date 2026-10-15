// ev-iz.js çıktısı + kayıt: takım şekli (ev) ile gerçek konum ve görevin çekişi. Savunma / hücum ayrı, görev türüne göre.
// Kullanım: node tools/ev-ozet.js <ev.json> <kayıt.json.gz> [anlar: t1,t2,...] → stdout özet; anlar verilirse <ev.json>-anlar.json
const fs = require('fs'), { oku } = require('./kayit-oku.js'); const [ef, kf, anS] = process.argv.slice(2);
const E0 = JSON.parse(fs.readFileSync(ef)), k = oku(kf), N = k.N, W = k.saha.W;
const ac = (s, C) => { const b = Buffer.from(s, 'base64'), a = new Uint8Array(b.length); a.set(b); return new C(a.buffer); };
const att = ac(E0.att, Int8Array), ev = ac(E0.ev, Int16Array), EB = ac(E0.E, Int16Array), n = E0.ids.length, idx = E0.ad.map(a => k.oyuncular.findIndex(o => o.ad === a));
const tur = j => /^pres/.test(j) ? 'pres' : /^arkada/.test(j) ? 'arkada' : /^bölge/.test(j) ? 'bölge' : /^kapatma/.test(j) ? 'kapatma' : /^geri/.test(j) ? 'geri koşu' : /^kovala|^alıcı|^sekme/.test(j) ? 'topa' : /^boşluk|^açılan/.test(j) ? 'boşluk' : /^güvence/.test(j) ? 'güvence' : j ? 'diğer' : 'top bende';
const S = {}, blok = { sav: [], huc: [] }; const ek = (key, v) => { const o = S[key] = S[key] || { n: 0, evUz: 0, hedEv: 0 }; o.n++; o.evUz += v[0]; o.hedEv += v[1]; };
for (let t = 1; t < k.n; t++) { const r = t - 1; if (ev[(t * n) * 2] === -32768) continue; const a = att[t] === 0 ? 'huc' : 'sav'; const Ap = [], Ae = [];
  for (let j = 0; j < n; j++) { const i = idx[j], x = k.P.x[r * N + i] / 100, y = k.P.y[r * N + i] / 100, tx = k.P.tx[r * N + i] / 100, ty = k.P.ty[r * N + i] / 100, hx = ev[(t * n + j) * 2] / 100, hy = ev[(t * n + j) * 2 + 1] / 100; if (hx < -300) continue;
    const j0 = k.T.hold[r] === i ? '' : (k.gorevler[k.P.job[r * N + i]] || ''), g = tur(j0); Ap.push([x, y]); Ae.push([hx, hy]);
    ek(`${a}|${E0.ad[j]}|${g}`, [Math.hypot(x - hx, y - hy), Math.hypot(tx - hx, ty - hy)]); ek(`${a}|hepsi|${g}`, [Math.hypot(x - hx, y - hy), Math.hypot(tx - hx, ty - hy)]); }
  if (Ap.length === 6) { const fx = A => A.map(p => p[0]), fy = A => A.map(p => p[1]), boy = A => Math.max(...fx(A)) - Math.min(...fx(A)), en = A => Math.max(...fy(A)) - Math.min(...fy(A)), son = A => Math.min(...fx(A)); blok[a].push([boy(Ap), en(Ap), boy(Ae), en(Ae), son(Ap), son(Ae), EB[t * 2] / 100]); } }
const ort = (A, i) => A.reduce((s, r) => s + r[i], 0) / A.length;
for (const a of ['sav', 'huc']) { const B = blok[a]; console.log(`\n== ${a === 'sav' ? 'SAVUNMA' : 'HÜCUM'} · ${(B.length / 60).toFixed(0)} sn`);
  console.log(`blok boyu: gerçek ${ort(B, 0).toFixed(1)} · ev ${ort(B, 2).toFixed(1)} | eni: gerçek ${ort(B, 1).toFixed(1)} · ev ${ort(B, 3).toFixed(1)} | son çizgi (ileri): gerçek ${ort(B, 4).toFixed(1)} · ev ${ort(B, 5).toFixed(1)} · top ${ort(B, 6).toFixed(1)}`);
  for (const ad of ['hepsi', ...E0.ad]) { const rows = Object.entries(S).filter(([kk]) => kk.startsWith(a + '|' + ad + '|')).sort((x, y) => y[1].n - x[1].n);
    console.log(`  ${ad.padEnd(6)} ` + rows.map(([kk, o]) => `${kk.split('|')[2]} ${(o.n / 60).toFixed(0)}sn: evden ${(o.evUz / o.n).toFixed(1)} (hedef ${(o.hedEv / o.n).toFixed(1)})`).join(' · ')); } }
if (anS) { const out = {}; for (const t of anS.split(',').map(Number)) { const r = t - 1; out[t] = { att: att[t], top: [EB[t * 2] / 100, EB[t * 2 + 1] / 100], hold: k.T.hold[r], tumu: k.oyuncular.map((o, i) => [k.P.x[r * N + i] / 100, k.P.y[r * N + i] / 100, o.takim]),
  lim: E0.ad.map((ad, j) => { const i = idx[j]; return { ad, x: k.P.x[r * N + i] / 100, y: k.P.y[r * N + i] / 100, tx: k.P.tx[r * N + i] / 100, ty: k.P.ty[r * N + i] / 100, hx: ev[(t * n + j) * 2] / 100, hy: ev[(t * n + j) * 2 + 1] / 100, job: k.T.hold[r] === i ? 'top bende' : k.gorevler[k.P.job[r * N + i]] }; }) }; }
  fs.writeFileSync(ef.replace('.json', '-anlar.json'), JSON.stringify(out)); }
