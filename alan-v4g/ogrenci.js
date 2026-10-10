// Öğrenci: arka planda beynin kendi maçlarını oynar ve hafızayı (durum değeri tablosu) günceller. Kusursuz okuma (karar ve algı kanallarında Okuma 20): tablo kusursuz, kusur oyuncuda.
self.window = self;
importScripts('core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js', 'beyin.js', 'vs-table.js', 'pas-table.js', 'ogrenme.js');
self.ALAN_OKX = { eq: ['karar', 'kararHizi', 'rakipModel', 'gonder', 'kontrol', 'kenar', 'algi', 'yerlesim'], mid: 20 };
let calis = false, tacs = null, tur = 0, kos = false;
onmessage = e => { const d = e.data;
  if (d.tip === 'basla') { if (d.v) { self.ALAN_VS.v = d.v; self.ALAN_VS.n = d.n; } tacs = d.tacs; tur = d.tur || 0; calis = true; if (!kos) dongu(); }
  else if (d.tip === 'dur') calis = false; else if (d.tip === 'devam') { calis = true; if (!kos) dongu(); } };
async function dongu() { kos = true; const M = self.AlanMatch, B = self.AlanBeyin, O = self.AlanOgren;
  while (calis) { const seed = 1 + Math.floor(Math.random() * 1e9), r = () => tacs[Math.floor(Math.random() * tacs.length)], m = M.createMatch(seed, { len: 5400, tac: [r(), r()] }); O.basla(m);
    while (!m.over && calis) { let p = B.planla(m); while (p) p = p.bitir(p.jobs.map(j => B.rollout(m, j))) || null; M.step(m); O.adim(m); if (m.tick % 240 === 0) await new Promise(res => setTimeout(res, 0)); }
    if (m.over) { tur++; postMessage({ tip: 'mac', v: self.ALAN_VS.v, n: self.ALAN_VS.n, tur, skor: m.score }); } }
  kos = false; }
