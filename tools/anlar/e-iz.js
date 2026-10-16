const fs = require('fs'), [dir, anF] = process.argv.slice(2); globalThis.ALAN_IZ = 'Şimşek'; const { M, kurMac } = require('./yukle')(dir); const { oku } = require('./graf');
const A = JSON.parse(fs.readFileSync(anF)); const m0 = kurMac(A.kurulum.yeni), m = oku(A.kopya, () => M.kopya(m0.r)); while (m.tick < 6930) M.step(m);
const I = (self.__iz || []).filter(x => x.t > 6850); const ts = [...new Set(I.map(x => x.t))]; console.log('yeniden tartılan tikler', ts.join(' '));
for (const t of ts.slice(0, 3)) { console.log('tik', t); for (const x of I.filter(x => x.t === t).sort((a, b) => b.v - a.v)) console.log('   ', JSON.stringify(x.c), 'v', x.v, 'koşu', x.run, 'tv', x.tv, 'bağ', x.lsh, 'y', x.y); }
process.exit(0);
