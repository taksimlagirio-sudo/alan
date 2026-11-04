// Yetenek vektörleri: kayıtlardaki oyuncular ve rastgele kayıtlar → 34 özellik, 101 mekanik, sutY, AH, AG.
const yukle = require('./yukle'); const YK = yukle(require('path').resolve(process.argv[2]));
const fs = require('fs'), path = require('path'), Y = self.ALAN_YET, F = new Float64Array(1), U = new BigUint64Array(F.buffer), b = v => { F[0] = v; return U[0].toString(16); };
const kul = process.argv[3], out = [];
let s = 99; const r = () => { s = (s * 1103515245 + 12345) % 2147483648; return s / 2147483648; };
const OZK = Object.keys(Y.OZ), MK = Object.keys(Y.MEK);
const ekle = a => { const p = { a: JSON.parse(JSON.stringify(a)) }; out.push({ a: Object.fromEntries(Object.entries(p.a).filter(([k, v]) => typeof v === 'number').map(([k, v]) => [k, b(v)])), oz: OZK.map(k => b(Y.oz(p, k))), mek: MK.map(k => b(self.ALAN_Y(p, k))), sut: [5, 15, 22, 35].map(d => b(Y.sutY(p, d))), ah: b(self.ALAN_AH(p)), ag: b(self.ALAN_AG(p)) }); };
for (const f of fs.readdirSync(kul).filter(f => /^kayit-.*\.json$/.test(f)).slice(0, 10)) { const d = JSON.parse(fs.readFileSync(path.join(kul, f))); const m = YK.kurMac(d.yeni); for (const p of m.ps) if (p.a) ekle(Object.fromEntries(Object.entries(p.a).filter(([k]) => !k.startsWith('__')))); }
const ESKI = ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', 'refleks', 'yerlesim', 'hakimiyet', 'enerji', 'govde', 'caliskanlik'];
for (let i = 0; i < 200; i++) { const a = {}; const ad = i % 3 === 0 ? OZK : ESKI; for (const k of ad) if (r() < .85) a[k] = Math.round(r() * 20 * 10) / 10; if (i % 5 === 0) { a.okuma = Math.round(r() * 20); a.aktarim = Math.round(r() * 20); a.tutus = 7; a.yogunluk = 13; } ekle(a); }
fs.writeFileSync(process.argv[4], JSON.stringify(out)); console.log('oyuncu', out.length);
