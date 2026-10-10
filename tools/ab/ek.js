// A · tabloya eklenen iki ölçü. Ölçüler tanımdır, değerleri hafıza öğrenir (öğren-ab.js / tablo-kur.js); burada elle yazılmış ağırlık yok.
//  açıklık: rakip saha oyuncularının enine yayılımı (en büyük y − en küçük y) + hatlar arası mesafe (Çekirdek ile rakip Kuyu arasında kalan savunmacıların hücum yönündeki ardışık en büyük boşluğu). Birim: saha birimi.
//  boş arkadaş: Çekirdeğin önünde (hücum yönünde) olan ve pas hattı açık (decide.laneOk ≥ 0,5) saha oyuncusu arkadaş sayısı (0 | 1 | 2+). Çekirdeğin 2 birim yakınındaki arkadaş sayılmaz (Çekirdeği alacak olan odur).
// Düzeltme hücresi: uzaklık (<20 | 20–35 | 35–55 | 55+) × açıklık (3 dilim; sınırlar verinin üçte birlikleri) × boş arkadaş (3) = 36.
const UZ = [20, 35, 55];
function olc(g, src, team, x, y) { const dir = team === 0 ? 1 : -1, gx = team === 0 ? 100 : 0, L = g.AlanDecide.laneOk; let ymin = 99, ymax = -99; const ara = [];
  for (const p of src) { if (p.team === team || p.role === 'Bekçi') continue; ymin = Math.min(ymin, p.y); ymax = Math.max(ymax, p.y); if ((p.x - x) * dir > 0) ara.push((p.x - x) * dir); }
  ara.sort((a, b) => a - b); let hat = 0; for (let i = 1; i < ara.length; i++) hat = Math.max(hat, ara[i] - ara[i - 1]);
  let bos = 0; const at = { x, y }; for (const p of src) { if (p.team !== team || p.role === 'Bekçi') continue; if ((p.x - x) * dir <= 0 || Math.hypot(p.x - x, p.y - y) < 2) continue; if (L(src, team, at, p) >= .5) bos++; }
  return { uz: Math.hypot(gx - x, 25 - y), acik: (ymax - ymin) + hat, bos }; }
function hucre(f, ACK) { let iu = 0; while (iu < UZ.length && f.uz >= UZ[iu]) iu++; let ia = 0; while (ia < ACK.length && f.acik >= ACK[ia]) ia++; return (iu * 3 + ia) * 3 + Math.min(2, f.bos); }
function okuyucu(g, ek) { return (src, team, x, y) => ek.r[hucre(olc(g, src, team, x, y), ek.ACK)] || 0; }
module.exports = { olc, hucre, okuyucu, UZ };
