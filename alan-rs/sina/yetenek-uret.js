// yetenek.js'in tablolarından Rust kaynağı üretir (motor/src/yetenek_tablo.rs).
const yukle = require('./yukle'); yukle(require('path').resolve(process.argv[2]));
const Y = self.ALAN_YET, OZK = Object.keys(Y.OZ), MK = Object.keys(Y.MEK);
const L = [];
L.push('//! Üretildi: sina/yetenek-uret.js (yetenek.js OZ ve MEK tablolarından). Elle değiştirme.');
L.push('#![allow(dead_code)]');
L.push(`pub const N_OZ: usize = ${OZK.length};`);
L.push(`pub const OZ_AD: [&str; N_OZ] = [${OZK.map(k => JSON.stringify(k)).join(', ')}];`);
L.push('pub mod oz {'); OZK.forEach((k, i) => L.push(`    pub const ${k.replace(/([A-Z])/g, '_$1').toUpperCase()}: usize = ${i};`)); L.push('}');
L.push(`pub const N_MEK: usize = ${MK.length};`);
L.push(`pub const MEK_AD: [&str; N_MEK] = [${MK.map(k => JSON.stringify(k)).join(', ')}];`);
L.push('pub mod mek {'); MK.forEach((k, i) => L.push(`    pub const ${k.replace(/([A-Z])/g, '_$1').toUpperCase()}: usize = ${i};`)); L.push('}');
L.push('/// Her mekaniğin özellik payları (özellik, yüzde).');
L.push(`pub const MEK_PAY: [&[(usize, f64)]; N_MEK] = [${MK.map(k => '&[' + Y.MEK[k][4].map(([o, w]) => `(${OZK.indexOf(o)}, ${w}.0)`).join(', ') + ']').join(', ')}];`);
L.push('/// Her mekaniğin durum katmanı harfleri (B baskı, D denge, H hareket, Hz hazırlık, E enerji).');
L.push(`pub const MEK_DURUM: [&[&str]; N_MEK] = [${MK.map(k => '&[' + Y.MEK[k][5].map(d => JSON.stringify(d)).join(', ') + ']').join(', ')}];`);
require('fs').writeFileSync(process.argv[3], L.join('\n') + '\n');
for (const k of MK) for (const [o] of Y.MEK[k][4]) if (OZK.indexOf(o) < 0) throw new Error('bilinmeyen özellik ' + o);
console.log('oz', OZK.length, 'mek', MK.length);
