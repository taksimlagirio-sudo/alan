// Wasm motorunun hızı ve kareleri (sitedeki işçi gibi: her tikte kare + yenilikler okunur).
//   node alan-rs/sina/wasm-hiz.js <alan.wasm> <an.json> [tik=600] [tekrar=3]
// Çıktı: en iyi koşunun ms/tik'i ve karelerin özeti (iki wasm'ın aynı maçı oynadığını sınamak için).
const fs = require('fs');
const [wasm, an, tikA, tekA] = process.argv.slice(2);
const N = +(tikA || 600), TEK = +(tekA || 3);
const kur = JSON.parse(fs.readFileSync(an)).kurulum;
const bayt = fs.readFileSync(wasm);
const NF = 174, dek = new TextDecoder(), kod = new TextEncoder();
async function kos() {
  const { instance } = await WebAssembly.instantiate(bayt, {});
  const W = instance.exports, mem = W.memory;
  // sitedeki işçinin varsayılanları (rust-isci.js)
  for (const [f, v] of [['seyrek', 1], ['govde_yol', 1], ['sur_e', 1], ['bek_dalis', 1]]) if (W[f]) W[f](v);
  const b = kod.encode(JSON.stringify({ ...kur, tip: 'yeni', gen: 1, len: 5400, dur: 5400 }));
  const p = W.ayir(b.length) >>> 0;
  new Uint8Array(mem.buffer, p, b.length).set(b);
  if (W.kur(p, b.length) !== 0) throw new Error('kur: ' + dek.decode(new Uint8Array(mem.buffer, W.metin_ptr() >>> 0, W.metin_len())));
  W.birak(p, b.length);
  let oz = 2166136261 >>> 0;
  const t0 = performance.now();
  for (let i = 0; i < N && !W.bitti(); i++) {
    W.adim();
    const k = new Uint32Array(mem.buffer, W.kare() >>> 0, NF);
    for (let j = 0; j < NF; j++) oz = Math.imul(oz ^ k[j], 16777619) >>> 0;
    W.yenilikler();
  }
  return [(performance.now() - t0) / N, oz.toString(16)];
}
(async () => {
  let en = 1e9, oz = null;
  for (let r = 0; r < TEK; r++) {
    const [ms, o] = await kos();
    if (oz && o !== oz) throw new Error('kareler koşudan koşuya değişti');
    oz = o;
    en = Math.min(en, ms);
  }
  console.log(`${wasm.split('/').pop()} ${N} tik: en iyi ${en.toFixed(2)} ms/tik, kare özeti ${oz}`);
})();
