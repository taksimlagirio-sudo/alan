// Modüller arası çağrıları çalışma anında çıkarır: dışa açık nesnelere vekil koyup çağıranın dosyasını yığından okur.
const path = require('path'), fs = require('fs');
const { M, kurMac } = require('./yukle')(path.resolve(process.argv[2]));
const kayit = {}; const dosya = () => { const s = new Error().stack.split('\n').slice(3); for (const l of s) { const m = /(core|match|shape|decide|yetenek|mac-isci|paralel)\.js/.exec(l); if (m) return m[1]; } return '?'; };
const sar = (ad) => { const o = window[ad]; window[ad] = new Proxy(o, { get(t, k) { const v = t[k]; if (typeof k === 'string') { const f = dosya(); const key = ad + '.' + k; (kayit[key] = kayit[key] || {})[f] = ((kayit[key] || {})[f] || 0) + 1; } return v; } }); };
for (const ad of ['AlanShape', 'AlanDecide', 'AlanMatch', 'AlanCore']) sar(ad);
// self.ALAN_* işlevleri: tanımlandığı dosya ve çağıran dosyalar
const gl = {}; for (const k of Object.keys(self).filter(k => /^ALAN_/.test(k) && typeof self[k] === 'function')) { const f0 = self[k]; self[k] = function (...a) { const f = dosya(); (gl[k] = gl[k] || {})[f] = (gl[k][f] || 0) + 1; return f0.apply(this, a); }; }
const m = M.kafaAc(kurMac(JSON.parse(fs.readFileSync(process.argv[3])).yeni));
while (m.tick < +process.argv[4]) M.step(m);
const cikti = { nesne: kayit, kuresel: gl };
fs.writeFileSync(process.argv[5], JSON.stringify(cikti, null, 1));
const ozet = o => Object.entries(o).map(([k, v]) => k + ' ← ' + Object.keys(v).filter(f => f !== '?').join(',')).filter(s => !/← $/.test(s));
console.log(ozet(kayit).join('\n')); console.log('---'); console.log(ozet(gl).join('\n'));
