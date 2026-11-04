// Kullanım: node alan-rs/sina/isci-panik-sina.js alan-v5/rust-isci.js alan-v5/alan.wasm anlar/<an>.json
// İlk wasm örneği 30. adımda panikler ve bozuk kalır (çıplak denemede görüldüğü gibi: sonraki her çağrı tuzağa düşer).
// Sonra yeni maç: işçi taze örnekle kurabiliyor mu?
const fs = require('fs'), vm = require('vm');
const [dosya, wasm, an] = process.argv.slice(2);
const kurulum = JSON.parse(fs.readFileSync(an)).kurulum;
let ornek = 0;
const WA = { instantiate: async (b, i) => { const r = await WebAssembly.instantiate(b, i); if (ornek++ > 0) return r;
  const E = r.instance.exports, X = {}; let n = 0, bozuk = false;
  for (const k of Object.keys(E)) X[k] = typeof E[k] === 'function' ? (...a) => { if (bozuk) throw new WebAssembly.RuntimeError('unreachable'); if (k === 'adim' && ++n === 30) { bozuk = true; throw new WebAssembly.RuntimeError('unreachable'); } return E[k](...a); } : E[k];
  return { instance: { exports: X } }; } };
const gelen = [];
const ctx = { self: { location: { search: '' } }, location: { search: '' }, TextDecoder, TextEncoder, WebAssembly: WA, performance, setTimeout, console,
  fetch: async () => ({ ok: true, arrayBuffer: async () => fs.readFileSync(wasm) }), postMessage: m => gelen.push(m) };
vm.createContext(ctx); vm.runInContext(fs.readFileSync(dosya, 'utf8'), ctx);
(async () => {
  await ctx.onmessage({ data: { ...kurulum, tip: 'yeni', gen: 1 } });
  await new Promise(r => setTimeout(r, 1500));
  await ctx.onmessage({ data: { ...kurulum, tip: 'yeni', gen: 2 } });
  await new Promise(r => setTimeout(r, 2500));
  console.log(dosya.split('/').pop(), '| örnek sayısı', ornek, '| hatalar:', gelen.filter(m => m.tip === 'hata').map(m => m.msg.slice(0, 60)).join(' / '), '| gen2 kare', gelen.filter(m => m.gen === 2 && m.tip === 'kare').length);
  process.exit(0);
})();
