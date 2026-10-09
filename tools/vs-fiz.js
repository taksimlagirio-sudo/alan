// Tablonun damgası: tabloyu etkileyen fizik ve karar parametrelerinin özeti (core P/Q/Turn.TQ, decide D, durum özeti ST, gönderme tablosu ölçüsü).
// Kullanım: node tools/vs-fiz.js <motor> → kısa özet. Motor açılırken aynı hesap tabloyla uyuşmazsa laboratuvar durmalı.
const crypto = require('crypto'), { load } = require('./load'); const g = load(process.argv[2] || 'alan-v3g', 1);
const D = Object.fromEntries(Object.entries(g.AlanDecide.D).filter(([k]) => !k.startsWith('_') && k !== 'useVS'));
const src = JSON.stringify({ P: g.AlanCore.P, Q: g.AlanCore.Q, TQ: g.AlanCore.Turn && g.AlanCore.Turn.TQ, D, ST: g.AlanState.ST, SHT: { m: g.ALAN_SHT && g.ALAN_SHT.mouth, k: g.ALAN_SHT && g.ALAN_SHT.kr } });
console.log(crypto.createHash('sha256').update(src).digest('hex').slice(0, 16));
