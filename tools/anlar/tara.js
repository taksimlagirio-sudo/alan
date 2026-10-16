// A aşaması (yavaş): bir maçı oynatır; her tikin kısa izini ve her 30 tikte maçın tam kopyasını yazar. Kullanım: node tara.js motorDizini tohum rakipNo çıktıDizini
const fs = require('fs'), zlib = require('zlib'), path = require('path'); const [dir, tohumS, rakipS, cik] = process.argv.slice(2);
const { M, kurMac } = require('./yukle')(dir); const { yaz } = require('./graf');
(0, eval)(fs.readFileSync(path.join(dir, 'kadro.js'), 'utf8')); const R = self.ALAN_KADRO.RAKIP[+rakipS];
const K = require(process.env.KURULUM || '../alp-kayit.json'), y = JSON.parse(JSON.stringify(K.yeni)); y.tohum = +tohumS; y.tac[1] = { ...R.tac, yz: true }; y.diz[1] = R.diz; y.oyuncular[1] = R.oyuncular.map(([ad, a]) => ({ ad, a: { ...a } })); y.roller[1] = R.roller;
const kurulum = { ad: ['Liman', R.ad], saha: [120, 60], yeni: y };
const m = kurMac(y), ad = `${tohumS}-${rakipS}`; fs.mkdirSync(cik, { recursive: true });
const isler = [], isNo = new Map(), jn = j => { j = String(j || ''); let n = isNo.get(j); if (n == null) { n = isler.length; isler.push(j); isNo.set(j, n); } return n; };
const ix = p => p ? m.ps.indexOf(p) : -1, r2 = v => v == null || !Number.isFinite(v) ? null : Math.round(v * 100) / 100;
const iz = [], kararlar = [], kopyalar = []; let sonDec = null, sonOlay = null; const olaylar = [];
const statik = m.ps.map(p => ({ ad: p.name, team: p.team, role: p.role, rh: p.rh, rs: p.rs }));
function kaydet() { const b = m.ball, f = m.fl; iz.push({ t: m.tick, h: ix(m.holder), ch: r2(m.ch), b: b ? [r2(b.x), r2(b.y), b.team, ix(b.from), b.done ? 1 : 0, b.defl ? 1 : 0, r2(b.vx), r2(b.vy), b.gon ? 1 : 0] : null, fl: f ? [ix(f.q), f.kind, f.to ? r2(f.to.x) : null, f.to ? r2(f.to.y) : null] : null, gc: m.gc ? [m.gc.team, m.gc.hizli ? 1 : 0] : null, ev: m.evre ? m.evre.map(e => e ? e.key : null) : null, att: m.sh ? m.sh.att : null, sc: [...m.score],
    P: m.ps.map(p => [r2(p.x), r2(p.y), r2(p.vx), r2(p.vy), r2(p.tx), r2(p.ty), jn(p.job), p.sprint ? 1 : 0, ix(p.markRef), p.slot ? r2(p.slot.d) : null]) });
  const d = m.lastDec; if (d && d !== sonDec) { sonDec = d; kararlar.push({ t: m.tick, ad: d.name, team: d.team, x: r2(d.x), y: r2(d.y), o: d.opts.slice(0, 8).map(o => ({ k: o.kind, q: o.q ? (o.q.name || o.q) : null, v: r2(o.v), P: r2(o.P), to: o.to ? [r2(o.to.x), r2(o.to.y)] : null })) }); }
  if (m.events.length && m.events[0] !== sonOlay) { let k = 0; while (k < m.events.length && m.events[k] !== sonOlay) k++; for (let j = k - 1; j >= 0; j--) olaylar.push({ tick: m.tick, ...m.events[j] }); sonOlay = m.events[0]; } }
const t0 = Date.now(); kaydet(); const kz = zlib.createGzip(), kout = fs.createWriteStream(path.join(cik, ad + '.kopya.jsonl.gz')); kz.pipe(kout);
while (!m.over && m.tick < m.len) { if (m.tick % 30 === 0) kz.write(JSON.stringify({ t: m.tick, g: yaz(m) }) + '\n'); M.step(m); kaydet(); if (m.tick % 600 === 0) console.error(ad, m.tick, ((Date.now() - t0) / 1000).toFixed(0) + 's'); }
kz.end(); fs.writeFileSync(path.join(cik, ad + '.iz.json.gz'), zlib.gzipSync(JSON.stringify({ kurulum, statik, isler, iz, kararlar, olaylar, skor: m.score, motor: dir })));
console.error(ad, 'bitti', m.score, ((Date.now() - t0) / 1000).toFixed(0) + 's');
