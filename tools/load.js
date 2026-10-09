// Motoru Node'da (tarayıcısız) yükler: dosyalar tarayıcıdaki gibi global kapsamda çalışır (window = global).
// Math.random tohumlanır: aynı tohum her zaman aynı maçı verir. Süreç başına bir motor yüklenir.
const fs = require('fs'), path = require('path');
function rng(seed) { let a = seed >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
function load(dir, seed = 1) {
  Math.random = rng(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
  for (const f of ['core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js', 'beyin.js']) if (f !== 'beyin.js' || fs.existsSync(path.join(dir, f))) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
  return globalThis;
}
module.exports = { load };
