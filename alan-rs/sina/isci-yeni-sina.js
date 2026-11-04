// Kullanım: node alan-rs/sina/isci-yeni-sina.js alan-v5/rust-isci.js alan-v5/alan.wasm anlar/<an>.json
// rust-isci.js'i Node'da işçi gibi çalıştırır: yeni maç, 300 ms sonra ikinci yeni maç; kareler sayılır.
const fs = require('fs'), vm = require('vm');
const [dosya, wasm, an] = process.argv.slice(2);
const kur = JSON.parse(fs.readFileSync(an)).kurulum;
const gelen = [];
const ctx = { self: { location: { search: '' } }, location: { search: '' }, TextDecoder, TextEncoder, WebAssembly, performance, setTimeout, console,
  fetch: async () => ({ ok: true, arrayBuffer: async () => fs.readFileSync(wasm) }),
  postMessage: m => gelen.push([Date.now(), m.tip, m.gen, m.tick]) };
vm.createContext(ctx); vm.runInContext(fs.readFileSync(dosya, 'utf8'), ctx);
const yolla = d => ctx.onmessage({ data: d });
yolla({ ...kur, tip: 'yeni', gen: 1, len: 5400, dur: 5400 });
setTimeout(() => yolla({ ...kur, tip: 'yeni', gen: 2, len: 5400, dur: 5400 }), 1500);
setTimeout(() => { const g2 = gelen.filter(x => x[2] === 2); console.log(dosya.split('/').pop(), 'gen1 kare', gelen.filter(x => x[2] === 1 && x[1] === 'kare').length, '· gen2 basla', g2.filter(x => x[1] === 'basla').length, 'kare', g2.filter(x => x[1] === 'kare').length, 'son tik', (g2.filter(x => x[1] === 'kare').pop() || [])[3], 'hata', gelen.filter(x => x[1] === 'hata').length); process.exit(0); }, 5000);
