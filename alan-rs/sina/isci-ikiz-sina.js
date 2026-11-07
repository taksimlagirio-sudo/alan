// Kullanım: node alan-rs/sina/isci-ikiz-sina.js alan-v5/rust-isci.js alan-v5/alan.wasm alan-rs/analiz/mac-217468.json
// Aynı kayıt aynı işçide iki kez oynatılır (araya bir taktik değişikliği girer); iki oynatma birebir aynı olmalı.
// Karşılaştırma: ilk 300 tikin kareleri + olaylar + son skor. Değişiklikli oynatma da iki kez yapılır.
const fs = require('fs'), vm = require('vm'), crypto = require('crypto');
const [dosya, wasm, kayit] = process.argv.slice(2);
const k = JSON.parse(fs.readFileSync(kayit)); const yeni = k.yeni || k.kurulum;
const LEN = +(process.env.LEN || 600);
const ctx = { self: { location: { search: '' } }, location: { search: '' }, TextDecoder, TextEncoder, WebAssembly, performance, setTimeout, console,
  fetch: async () => ({ ok: true, arrayBuffer: async () => fs.readFileSync(wasm) }) };
let oturum = null;
ctx.postMessage = m => { if (!oturum || m.gen !== oturum.gen) return; if (m.tip === 'hata') { console.log('HATA', m.msg); process.exit(2); }
  if (m.tip === 'kare') { for (const [t, f] of m.kare) if (t <= 300 || t >= oturum.bas) oturum.h.update(t + ':' + Buffer.from(f.buffer).toString('hex')); for (const o of m.olay) { oturum.h.update(JSON.stringify(o)); if (/SAYI|gol/i.test(JSON.stringify(o))) oturum.gol++; } oturum.son = m.tick; if (m.son || m.yari) oturum.bitti(); } };
vm.createContext(ctx); vm.runInContext(fs.readFileSync(dosya, 'utf8'), ctx);
let g = 0;
const oynat = (deg) => new Promise(res => { g++; oturum = { gen: g, h: crypto.createHash('sha1'), gol: 0, bas: 0, bitti: () => res({ iz: oturum.h.digest('hex').slice(0, 16), son: oturum.son }) };
  ctx.onmessage({ data: { ...yeni, tip: 'yeni', gen: g, len: LEN, dur: deg ? 300 : LEN } });
  if (deg) { const once = oturum.bitti; oturum.bitti = () => { oturum.bitti = once; oturum.bas = 300; ctx.onmessage({ data: { tip: 'degistir', gen: g, snapT: 300, t: 0, tac: { ...yeni.tac[0], tempo: 0.9, risk: 0.9 } } }); ctx.onmessage({ data: { tip: 'devam', gen: g } }); }; } });
(async () => {
  const a = await oynat(false), b = await oynat(false), c = await oynat(true), d = await oynat(true), e = await oynat(false);
  console.log('düz 1', a, '· düz 2', b, '· değişiklikli 1', c, '· değişiklikli 2', d, '· düz 3', e);
  const ok = a.iz === b.iz && b.iz === e.iz && c.iz === d.iz;
  console.log(ok ? 'AYNI' : 'FARKLI'); process.exit(ok ? 0 : 1); })();
