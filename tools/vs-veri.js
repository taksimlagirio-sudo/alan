// Durum değeri tablosu (Bölüm 9) · veri üretimi. Bir maç oynatır, her 10 tikte Çekirdeğe sahip takımın durum hücresini (AlanState.idx) ve sayıları kaydeder.
// Kurallar (TASARIM.md Bölüm 9): taktik %80 tutarlı çekirdek (altı tarz, pres ±1, blok ±1, üçte birinde komşu sistem) + %20 rastgele (çoğu çekirdeğe karşı);
// özellikler 6–16 rastgele; karar/algı kanallarında kusursuz okuma (Okuma 20), fiziksel kanallarda (tepki, ikili, Bekçi) oyuncunun kendi değeri.
// Kullanım: node tools/vs-veri.js <motor> <i> <tur> [tablo.js] → JSON satırı
const fs = require('fs'), { load } = require('./load');
const [dir, iS, turS, tabPath] = process.argv.slice(2), i = +iS, tur = +turS || 0, seed = 20000 + i * 7919 + tur * 1000003;
let r0 = seed ^ 0x9e3779b9; const R = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296, pick = a => a[Math.floor(R() * a.length)], ri = (a, b) => a + Math.floor(R() * (b - a + 1));
const KARAR = ['karar', 'kararHizi', 'rakipModel', 'gonder', 'kontrol', 'kenar', 'algi', 'yerlesim'];
globalThis.ALAN_OKX = { eq: KARAR, mid: 20 }; // kusursuz algı: sadece karar/algı kanalları
const g = load(dir, seed), M = g.AlanMatch, D = g.AlanDecide.D, S = g.AlanState;
if (tabPath) { (0, eval)(fs.readFileSync(tabPath, 'utf8')); D.useVS = true; }
const STY = { Dengeli: ['Alan', 1, 1, 'Orta', 'Normal', .5, .5, 'Dengeli'], Sabırlı: ['Alan', 1, 1, 'Orta', 'Geniş', .2, .2, 'Yerleş'], Dikine: ['Alan', 1, 1, 'Orta', 'Normal', .8, .8, 'Kontra'], Kontra: ['Alan', 0, 1, 'Düşük', 'Dar', .8, .8, 'Kontra'], 'Ön alan': ['Adam adama', 3, 1, 'Yüksek', 'Geniş', .5, .5, 'Dengeli'], 'Kuyu önü': ['Alan', 0, 2, 'Düşük', 'Dar', .2, .2, 'Yerleş'] };
const BL = ['Düşük', 'Orta', 'Yüksek'], SIS = ['Alan', 'Adam adama', 'Kenara sıkıştır'], KOM = { Alan: 'Kenara sıkıştır', 'Adam adama': 'Kenara sıkıştır', 'Kenara sıkıştır': null };
const tacOf = a => ({ sistem: a[0], pres: a[1], arkada: a[2], blok: a[3], genislik: a[4], tempo: a[5], risk: a[6], kazaninca: a[7] });
function core() { const n = pick(Object.keys(STY)), t = tacOf(STY[n]); t.pres = Math.max(0, Math.min(3, t.pres + ri(-1, 1))); t.blok = BL[Math.max(0, Math.min(2, BL.indexOf(t.blok) + ri(-1, 1)))];
  if (R() < 1 / 3) t.sistem = KOM[t.sistem] || pick(['Alan', 'Adam adama']); return { ad: n, t }; }
function rnd() { return { ad: 'rastgele', t: { sistem: pick(SIS), pres: ri(0, 3), arkada: ri(0, 2), blok: pick(BL), genislik: pick(['Dar', 'Normal', 'Geniş']), tempo: pick([.2, .5, .8]), risk: pick([.2, .5, .8]), kazaninca: pick(['Yerleş', 'Dengeli', 'Kontra']) } }; }
let A, B; const u = R(); if (u < .8) { A = core(); B = core(); } else if (u < .97) { A = rnd(); B = core(); if (R() < .5) [A, B] = [B, A]; } else { A = rnd(); B = rnd(); }
const KEYS = ['okuma', 'aktarim', 'tutus', 'kesme', 'yogunluk', 'hiz', 'cesaret', 'surme'];
const m = M.createMatch(seed, { tac: [A.t, B.t], len: 5400 });
for (const p of m.ps) for (const k of KEYS) p.a[k] = ri(6, 16);
const s = [], gl = []; let sc = [0, 0];
while (!m.over && m.tick < 5400) { M.step(m);
  if (m.score[0] !== sc[0] || m.score[1] !== sc[1]) { gl.push([m.tick, m.score[0] !== sc[0] ? 0 : 1]); sc = m.score.slice(); }
  const h = m.holder; if (h && m.tick % 10 === 0) { const tr = m.winTeam === h.team && (m.tick - (m.winT ?? -1e9)) < 180; s.push([m.tick, h.team, S.idx(m.ps, h.team, h.x, h.y, tr)]); } }
console.log(JSON.stringify({ i, tur, tac: [A, B], score: m.score, g: gl, s }));
