// Motoru Node'da (tarayıcısız) yükler: dosyalar tarayıcıdaki gibi global kapsamda çalışır (window = global).
// Math.random tohumlanır: aynı tohum her zaman aynı maçı verir. Süreç başına bir motor yüklenir.
const fs = require('fs'), path = require('path');
function rng(seed) { let a = seed >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
function load(dir, seed = 1, yama = {}) { /* yama: { 'dosya.js': [aranan, yerine] } — ölçüm için kayıt alanı eklemek (akışı değiştirmeyen) */
  Math.random = rng(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis; if (process.env.SAHA) globalThis.ALAN_SAHA_BOY = process.env.SAHA.split('x').map(Number); if (process.env.DIREK != null) globalThis.ALAN_DIREK = +process.env.DIREK; if (process.env.BAYRAK) for (const b of process.env.BAYRAK.split(',')) globalThis[b] = true; /* saha deneyi: SAHA=140x70; direk: DIREK=.6; BAYRAK=ALAN_BEKCI_ESKI */
  for (const f of ['core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js', 'beyin.js']) if (f !== 'beyin.js' || fs.existsSync(path.join(dir, f))) { let t = fs.readFileSync(path.join(dir, f), 'utf8'); const y = yama[f]; if (y) for (const [a, b] of (Array.isArray(y[0]) ? y : [y])) { if (!t.includes(a)) throw new Error('yama yeri yok: ' + f); t = t.replace(a, b); } (0, eval)(t + '\n//# sourceURL=' + f); }
  return globalThis;
}
module.exports = { load };
