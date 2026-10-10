// 8. madde (A / B / C değerlendirmesi) ortak parçaları. Motor dosyalarına dokunulmaz; bu süreçte yüklenirken üç küçük kanca eklenir:
//  1. beyin.js · globalThis.__NOKARAR: top sahibinin araması kapalı (C maçları: top sahibi ilk bakışla oynar, savunmanın beyni açık kalır)
//  2. beyin.js · iş alanı j.sona (B): oynatma hücum bitene kadar sürer; ilk komuttan sonra top sahibi beynin ilk bakışıyla (decide) oynar, top rakibe geçince durur
//  3. decide.js · globalThis.__VSEK: tablo okumasına eklenen öğrenilmiş düzeltme (A: blok açıklığı ve boştaki arkadaş)
const fs = require('fs'), path = require('path'), v8 = require('v8');
function rngF(s) { let a = s >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
function yama(src, a, b) { if (!src.includes(a)) throw new Error('kanca yeri yok: ' + a.slice(0, 60)); return src.replace(a, b); }
function yukle(dir, seed = 1) {
  Math.random = rngF(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
  for (const f of ['core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js', 'beyin.js', 'vs-table.js', 'pas-table.js', 'ogrenme.js']) { let s = fs.readFileSync(path.join(dir, f), 'utf8');
    if (f === 'beyin.js') {
      s = yama(s, 'if (h && m.decT <= 1 && !(h.stun > m.tick)', 'if (h && !globalThis.__NOKARAR && m.decT <= 1 && !(h.stun > m.tick)');
      s = yama(s, "c._pmSig = !(j.tip === 'karar' || j.tip === 'karar2'); }", "c._pmSig = !(j.tip === 'karar' || j.tip === 'karar2') || !!j.sona; }");
      s = yama(s, 'if (c._dur) break;', 'if (c._dur) break; if (j.sona && c.holder && c.holder.team !== j.team) break;');
    }
    if (f === 'decide.js') { s = yama(s, 'function vsRead(T, src, team, x, y, tw) {', 'function vsRead(T, src, team, x, y, tw) { const v0 = vsRead0(T, src, team, x, y, tw); return globalThis.__VSEK ? v0 + globalThis.__VSEK(src, team, x, y, v0) : v0; }\n  function vsRead0(T, src, team, x, y, tw) {'); }
    (0, eval)(s + '\n//# sourceURL=' + f); }
  return globalThis;
}
// Herkes 10, karışık taktikler (önceki testlerle aynı altı stil)
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const tacOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] });
function taktik(seed) { let r0 = (seed ^ 0x9e3779b9) >>> 0; const R = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296, k = Object.keys(STY), a = k[Math.floor(R() * k.length)], b = k[Math.floor(R() * k.length)]; return { ad: [a, b], tac: [tacOf(STY[a]), tacOf(STY[b])] }; }
// Maç ölçüleri (her tikten sonra çağrılır): kendi yarıda topla geçen süre, uzaktan gönderme, pas, sayı
function olcuKur() { return { kendiYari: 0, top: 0, gon: 0, gon35: 0, gonUz: [], _fl: null }; }
function olcuAdim(m, o) { const h = m.holder; if (h) { o.top++; if (h.team === 0 ? h.x < 50 : h.x > 50) o.kendiYari++; }
  if (m.fl && m.fl !== o._fl) { o._fl = m.fl; if (m.fl.kind === 'gönder' && m.ball && m.ball.from) { const f = m.ball.from, gx = f.team === 0 ? 100 : 0, d = Math.hypot(gx - f.x, 25 - f.y); o.gon++; if (d >= 35) o.gon35++; o.gonUz.push(+d.toFixed(1)); } } }
function olcuSon(m, o) { return { skor: m.score, sayi: m.score[0] + m.score[1], kendiYariSn: +(o.kendiYari / 60).toFixed(1), topSn: +(o.top / 60).toFixed(1), gon: o.gon, gon35: o.gon35, pas: m.st.pass[0] + m.st.pass[1], pasOk: m.st.passOk[0] + m.st.passOk[1], gonUz: o.gonUz }; }
const kaydet = (f, o) => fs.writeFileSync(f, v8.serialize(o)), oku = f => v8.deserialize(fs.readFileSync(f));
module.exports = { yukle, taktik, olcuKur, olcuAdim, olcuSon, kaydet, oku, rngF };
