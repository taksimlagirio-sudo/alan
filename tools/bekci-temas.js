// Bekçi teması: maçı yeniden oynatır, gerçek maçın Çekirdeğine (aklın kafadaki kopyaları değil) rakip Bekçi her değdiğinde kaydeder:
// göreli hız (vr), kapasite (cap; gürültülü capE), tutma ihtimali, hazırlık (haz: Çekirdek yola çıkalı / tepki), sonuç (tuttu / sekti / deldi), sekmeden sonraki yön, sonra sayı oldu mu.
// Kullanım: node tools/bekci-temas.js <motor> <senaryo> <çıktı.json>
const fs = require('fs'); const { kur } = require('./kurulum'); const [dir, sS, out] = process.argv.slice(2);
const HOOK = "if (self.__tem && self.__m && b === self.__m.ball && best.role === 'Bekçi' && best.team !== b.team) self.__tem.push({ t: self.__m.tick, bek: best.team, gon: !!b.gon, bt: b.tp ?? b.t, vr: T.vr, cap: T.cap, capE, s: T.s, P, haz: T.haz, sp: hyp(b.vx, b.vy), bx: b.x, by: b.y, px: best.x, py: best.y, pvx: best.vx || 0, pvy: best.vy || 0 }); ";
const { M, m, adim } = kur(dir, +sS, 0, { 'core.js': [
  ["if (T.vr <= capE) { b.done = 'tutuldu';", HOOK + "if (T.vr <= capE) { b.done = 'tutuldu';"],
  ["return { p: best, took: false, P };", "if (self.__tem && self.__m && b === self.__m.ball && best.role === 'Bekçi' && best.team !== b.team) { const e = self.__tem[self.__tem.length - 1]; e.thru = thru; e.ovx = b.vx; e.ovy = b.vy; e.ox = b.x; e.oy = b.y; } return { p: best, took: false, P };"],
  ["return { cap: Math.max(.08, cap * (p.role", "return { haz, cap: Math.max(.08, cap * (p.role"]] });
const g = globalThis; g.__tem = []; g.__m = m; const skor = [];
const LEN = m.len || 7200; while (!m.over && m.tick < LEN) { adim(); skor.push(m.score[0] + m.score[1]); }
const S = g.AlanCore, W = (g.ALAN_SAHA && g.ALAN_SAHA.W) || 120, H = (g.ALAN_SAHA && g.ALAN_SAHA.H) || 60, MO = S.MOUTH;
for (const e of g.__tem) {
  e.sonuc = e.ovx == null ? 'tuttu' : e.thru ? 'deldi' : 'sekti';
  const gx = e.bek === 0 ? 0 : W; // Bekçi'nin kendi Kuyu'su
  if (e.ovx != null) { const dx = gx - e.ox; const tt = Math.abs(e.ovx) > 1e-6 ? dx / e.ovx : -1; const yy = e.oy + e.ovy * tt; e.kuyuya = tt > 0 && Math.abs(yy - H / 2) <= MO + 1; e.kuyuYaklas = (gx - e.ox) * e.ovx > 0; }
  // 60 tik içinde skor arttı mı (temas tikinden sonra)
  const i = e.t; e.sayi = (skor[Math.min(skor.length - 1, i + 60)] || 0) > (skor[Math.max(0, i - 1)] || 0);
}
fs.writeFileSync(out, JSON.stringify({ sc: +sS, skor: m.score, tem: g.__tem }));
console.log(JSON.stringify({ sc: +sS, skor: m.score, temas: g.__tem.length }));
