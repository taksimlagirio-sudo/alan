// Bağlılık deneyi karşılaştırması: aynı senaryonun kayıtları ve ev izleri yan yana. Liman (takım 0) saha oyuncuları.
// Kullanım: node tools/baglilik-kiyas.js ad=kayıt.json.gz:ev.json ...
const fs = require('fs'), { oku } = require('./kayit-oku.js');
const ac = (s, C) => { const b = Buffer.from(s, 'base64'), a = new Uint8Array(b.length); a.set(b); return new C(a.buffer); };
const kol = process.argv.slice(2).map(a => { const [ad, r] = a.split('='), [kf, ef] = r.split(':'); return { ad, kf, ef }; });
const satir = {}, ekle = (ad, s, v) => { (satir[s] = satir[s] || {})[ad] = v; };
for (const { ad, kf, ef } of kol) { const k = oku(kf), N = k.N, n = k.n, dk = n / 3600, lim = k.oyuncular.map((o, i) => i).filter(i => k.oyuncular[i].takim === 0 && k.oyuncular[i].rol !== 'Bekçi');
  const G = (t, i) => (k.gorevler[k.P.job[t * N + i]] || '').replace(/ · tepki/g, ''), AD = i => k.oyuncular[i].ad, I = a => k.oyuncular.findIndex(o => o.ad === a);
  ekle(ad, 'Skor (Liman–Demir)', k.skor.join('–')); ekle(ad, 'Gönderme (L / D)', k.st.shot.join(' / ')); ekle(ad, 'Pas tutma % (L)', (100 * k.st.passOk[0] / k.st.pass[0]).toFixed(1)); ekle(ad, 'Hesap ms/tik', (1000 * k.hesapSn / n).toFixed(1));
  // top olayı penceresi
  const ev = new Uint8Array(n); for (let t = 1; t < n; t++) if (k.T.hold[t] !== k.T.hold[t - 1]) for (let d = -2; d <= 6; d++) if (t + d >= 0 && t + d < n) ev[t + d] = 1;
  let gc = 0, sic = 0, ters = 0, yol = 0; for (const i of lim) { for (let t = 1; t < n; t++) { if (G(t, i) !== G(t - 1, i)) gc++; const d = Math.hypot(k.P.tx[t * N + i] - k.P.tx[(t - 1) * N + i], k.P.ty[t * N + i] - k.P.ty[(t - 1) * N + i]) / 100; if (d > 8 && !ev[t]) sic++; yol += Math.hypot(k.P.x[t * N + i] - k.P.x[(t - 1) * N + i], k.P.y[t * N + i] - k.P.y[(t - 1) * N + i]) / 100; }
    for (let t = 30; t < n; t += 15) { const a = [k.P.vx[(t - 15) * N + i], k.P.vy[(t - 15) * N + i]], b = [k.P.vx[t * N + i], k.P.vy[t * N + i]], la = Math.hypot(...a), lb = Math.hypot(...b); if (la > 80 && lb > 80 && (a[0] * b[0] + a[1] * b[1]) / (la * lb) < -.5) ters++; } }
  ekle(ad, 'Görev değişimi / sn (oyuncu başı)', (gc / lim.length / (n / 60)).toFixed(2)); ekle(ad, 'Top olayı yokken hedef 8+ br sıçraması / sn (oyuncu başı)', (sic / lim.length / (n / 60)).toFixed(2)); ekle(ad, 'Koşarken 1/4 sn\'de ters yöne dönüş / dk (oyuncu başı)', (ters / lim.length / dk).toFixed(1)); ekle(ad, 'Koşulan yol / dk (oyuncu başı)', (yol / lim.length / dk).toFixed(0));
  // arkadaki adam değişimi
  for (const a of ['Mert', 'Deniz']) { const i = I(a); let son = null, c = 0; for (let t = 0; t < n; t++) { const g = G(t, i); const m0 = /^arkada · (.+)$/.exec(g); if (!m0) continue; if (son && m0[1] !== son) c++; son = m0[1]; } ekle(ad, `${a}: arkada · X değişimi / dk`, (c / dk).toFixed(1)); }
  // geri koşu ve forvetin savunmada en derini
  let gk = 0; for (const i of lim) for (let t = 0; t < n; t++) if (/^geri koşu/.test(G(t, i))) gk++; ekle(ad, 'Geri koşu payı % (savunma süresine göre)', (() => { let sv = 0; for (let t = 0; t < n; t++) { const h = k.T.hold[t]; const tm = h !== 255 ? k.oyuncular[h].takim : k.T.bt[t]; if (tm === 1) sv++; } return (100 * gk / lim.length / sv).toFixed(1); })());
  for (const a of ['Tuna', 'Kaan']) { const i = I(a), d = []; for (let t = 0; t < n; t += 6) { const h = k.T.hold[t]; if (h === 255 || k.oyuncular[h].takim !== 1) continue; d.push(k.P.x[t * N + i] / 100); } d.sort((x, y) => x - y); ekle(ad, `${a}: rakip topluyken konum (ileri) medyan / %10`, `${d[d.length >> 1].toFixed(0)} / ${d[Math.floor(d.length * .1)].toFixed(0)}`); }
  // ev izi
  if (ef && fs.existsSync(ef)) { const E0 = JSON.parse(fs.readFileSync(ef)), at = ac(E0.att, Int8Array), evv = ac(E0.ev, Int16Array), m6 = E0.ids.length; const ch = []; for (let t = 2; t < 7200; t++) if (at[t] !== at[t - 1]) ch.push(t); let geri = 0; for (let j = 1; j < ch.length; j++) if (ch[j] - ch[j - 1] <= 30) geri++;
    ekle(ad, 'Şekil (hücum↔savunma) değişimi / dk', (ch.length / 2).toFixed(0)); ekle(ad, '  bunlardan 0,5 sn içinde geri dönen', geri);
    let s = 0, c = 0, sc = 0; for (let j = 0; j < m6; j++) for (let t = 2; t < 7200; t++) { const a = evv[(t * m6 + j) * 2], b = evv[((t - 1) * m6 + j) * 2]; if (a === -32768 || b === -32768) continue; const d = Math.hypot(a - b, evv[(t * m6 + j) * 2 + 1] - evv[((t - 1) * m6 + j) * 2 + 1]) / 100; s += d; c++; if (d > 3) sc++; }
    ekle(ad, 'Ev kayması br/tik (depar 0,36)', (s / c).toFixed(3)); ekle(ad, 'Ev tek tikte 3+ br sıçraması / dk (oyuncu başı)', (sc / m6 / 2).toFixed(1));
    const idx = E0.ad.map(a => I(a)); let bz = 0, bn = 0, az = 0, an = 0; for (let t = 1; t < 7200; t++) { if (at[t] === 0) continue; for (let j = 0; j < m6; j++) { const hx = evv[(t * m6 + j) * 2]; if (hx === -32768) continue; const i = idx[j], r = t - 1, g = G(r, i), d = Math.hypot(k.P.x[r * N + i] - hx, k.P.y[r * N + i] - evv[(t * m6 + j) * 2 + 1]) / 100; if (/^bölge/.test(g)) { bz += d; bn++; } if (/^arkada/.test(g)) { az += d; an++; } } }
    ekle(ad, 'Savunmada "bölge"deki oyuncunun evine uzaklığı', (bz / bn).toFixed(1)); }
}
const ads = kol.map(k => k.ad); console.log('| Ölçü | ' + ads.join(' | ') + ' |'); console.log('|---|' + ads.map(() => '---').join('|') + '|');
for (const [s, v] of Object.entries(satir)) console.log(`| ${s} | ${ads.map(a => v[a] ?? '—').join(' | ')} |`);
