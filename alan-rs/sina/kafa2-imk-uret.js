// Şut imkân tablosu (ALAN_IMK_SUT_VE, shot-table.js) → motor/src/kafa/kafa2_imk_sut.bin (i8, −1..100) ve eksenler.
// node sina/kafa2-imk-uret.js ../alan-v5
const path = require('path'), fs = require('fs');
require('./yukle')(path.resolve(process.argv[2]));
const T = self.ALAN_IMK_SUT_VE;
const L = T.L, buf = [];
for (const a of L) for (const b of L) { const t = T.v[a + '-' + b]; if (t.length !== T.DS.length * T.SY.length * T.KO.length * T.KD.length * T.CH.length) throw Error('boy'); for (const v of t) { if (!Number.isInteger(v) || v < -1 || v > 100) throw Error('değer ' + v); buf.push(v & 255); } }
fs.writeFileSync(path.join(__dirname, '../motor/src/kafa/kafa2_imk_sut.bin'), Buffer.from(buf));
console.log(JSON.stringify({ surum: T.surum, parmak: T.parmak, DS: T.DS, SY: T.SY, KO: T.KO, KD: T.KD, CH: T.CH, L: T.L }));
