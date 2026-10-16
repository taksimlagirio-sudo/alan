// Kataloğa an üretir: tarama motoruyla (eski) kayıtlı kopyadan istenen tike kadar oynar, oradaki maçı kopyalar; an bugünkü motorla o tikten başlar
// Kullanım: node an-uret.js eskiMotor kopyaDosyası(jsonl.gz) kurulumKaynağı(an json) tik süre id başlık metin anlarDizini
const fs = require('fs'), zlib = require('zlib'), path = require('path'); const [dir, kopF, kurF, tS, nS, id, baslik, metin, OUT] = process.argv.slice(2); const T = +tS;
const { M, kurMac } = require('./yukle')(dir); const { yaz, oku } = require('./graf'); const K0 = JSON.parse(fs.readFileSync(kurF)), kur = K0.kurulum;
let g0 = null, t0 = -1; for (const l of zlib.gunzipSync(fs.readFileSync(kopF)).toString().split('\n')) if (l) { const o = JSON.parse(l); if (o.t <= T && o.t > t0) { t0 = o.t; g0 = o.g; } }
const m0 = kurMac(kur.yeni), m = oku(g0, () => M.kopya(m0.r)); while (m.tick < T) M.step(m);
const P = m.ps.map((p, k) => [Math.round(p.x * 10) / 10, Math.round(p.y * 10) / 10, p.team, p.name, Math.round((p.vx || 0) * 100) / 100, Math.round((p.vy || 0) * 100) / 100, p.role === 'Bekçi' ? 1 : 0]);
const an = { id, kat: 'katalog', baslik, metin, t0: T, t1: T + +nS, tk: T, kurulum: kur, kopya: yaz(m) }; fs.writeFileSync(path.join(OUT, id + '.json'), JSON.stringify(an));
const L = JSON.parse(fs.readFileSync(path.join(OUT, 'anlar.json'))); L.KAT.katalog = L.KAT.katalog || { ad: 'Katalog', acik: 'Kataloğa aldığımız anlar: eski motorda sorunun görüldüğü yerden başlar, bugünkü motorla oynar.' };
const s = Math.floor(T / 30), saat = String(Math.floor(s / 60)).padStart(2, '0') + ':' + String(s % 60).padStart(2, '0'); L.anlar = L.anlar.filter(a => a.id !== id);
const vurgu = (process.env.VURGU || '').split(',').filter(Boolean).map(n => m.ps.findIndex(p => p.name === n)).filter(i => i >= 0);
L.anlar.unshift({ id, kat: 'katalog', metin, rakip: kur.ad[1], tohum: kur.yeni.tohum, saat, t0: T, t1: T + +nS, tk: T, skor: m.score.join('–'), kart: { P, b: m.ball ? [m.ball.x, m.ball.y] : m.holder ? [m.holder.x, m.holder.y] : null, h: m.holder ? m.ps.indexOf(m.holder) : -1, vurgu, cizgi: [], hat: null } });
fs.writeFileSync(path.join(OUT, 'anlar.json'), JSON.stringify(L)); console.log(id, 'tik', T, 'top:', m.holder ? m.holder.name : '-', 'skor', m.score.join('–'));
