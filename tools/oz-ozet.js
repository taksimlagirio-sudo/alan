// Özellik ölçümü özeti (kayıtlardan): taban (herkes 10) ile Liman'ın saha oyuncularında (Bekçi özelliğinde Bekçi'sinde) tek özellik 6 / 14.
// Her özellik için Liman'ın genel sonucu ve mekanik göstergeleri; 6 → 10 → 14 tek yönlü mü.
// Kullanım: node tools/oz-ozet.js <klasör>   (dosyalar taban-<sc>.json.gz, <özellik>-<6|14>-<sc>.json.gz)
const fs = require('fs'), path = require('path'); const { oku } = require('./kayit-oku'); const dir = process.argv[2];
const A = ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', 'refleks', 'yerlesim', 'hakimiyet'];
const AD = { hiz: 'Hız', ceviklik: 'Çeviklik', guc: 'Güç', uzanma: 'Uzanma', alan: 'Alan gücü', kontrol: 'Kontrol', surme: 'Sürme', pas: 'Pas', gonderme: 'Gönderme', yukleme: 'Yükleme', tepki: 'Tepki', gorus: 'Görüş', yeralma: 'Yer alma', sogukkanlilik: 'Soğukkanlılık', kesme: 'Kesme', refleks: 'Refleks (B)', yerlesim: 'Yerleşim (B)', hakimiyet: 'Alan hakimiyeti (B)' };
// tek maçtan Liman (takım 0) göstergeleri
function olc(k) { const { N, n, T, P, oyuncular: O, st } = k, sonra = t => { for (const x of k.sahip) if (x.t > t && x.kim != null) return x; return null; };
  let hiz = [], chS = 0, chN = 0, hold0 = 0, hold = 0; for (let i = 0; i < N; i++) { if (O[i].takim !== 0 || O[i].rol === 'Bekçi') continue; const a = []; for (let t = 0; t < n; t++) { const q = t * N + i; a.push(Math.hypot(P.vx[q], P.vy[q]) / 1000); } a.sort((x, y) => y - x); hiz.push(a[Math.floor(a.length * .02)]); }
  for (let t = 0; t < n; t++) { const h = T.hold[t]; if (h === 255) continue; hold++; if (O[h].takim === 0) { hold0++; chS += T.ch[t] / 200; chN++; } }
  let bP = 0, bOk = 0, aP = 0, aOk = 0; for (const p of k.pas) { if (p.kim == null || O[p.kim].takim !== 0 || p.tur === 'gönder') continue; const s = sonra(p.t); if (!s) continue; let nd = 1e9; const q = p.t * N + p.kim; for (let j = 0; j < N; j++) if (O[j].takim === 1 && O[j].rol !== 'Bekçi') nd = Math.min(nd, Math.hypot(P.x[p.t * N + j] - P.x[q], P.y[p.t * N + j] - P.y[q]) / 100); const ok = s.takim === 0 ? 1 : 0; if (nd < 3) { bP++; bOk += ok; } else { aP++; aOk += ok; } }
  return { gf: k.skor[0] - k.skor[1], at: k.skor[0], ye: k.skor[1], pasTut: st.passOk[0] / Math.max(1, st.pass[0]), pas: st.pass[0], gon: st.shot[0], gonYe: st.shot[1], kapma: st.steal[0], kapYe: st.steal[1], ikiliK: st.duelW ? st.duelW[0] / Math.max(1, st.duel[0]) : null, sahip: hold0 / Math.max(1, hold), hiz: hiz.reduce((s, x) => s + x, 0) / hiz.length, sarj: chN ? chS / chN : null, baskiTut: bP ? bOk / bP : null, rahatTut: aP ? aOk / aP : null, bN: bP };
}
const ort = (R, k) => { const a = R.map(r => r[k]).filter(x => x != null && !Number.isNaN(x)); return a.length ? a.reduce((s, x) => s + x, 0) / a.length : null; };
const yukle = on => fs.readdirSync(dir).filter(f => f.startsWith(on) && f.endsWith('.json.gz')).map(f => olc(oku(path.join(dir, f))));
const T0 = yukle('taban-'); const G = [['gf', 'Gol farkı', 2], ['at', 'Atılan', 2], ['ye', 'Yenen', 2], ['pasTut', 'Pas tutma', 3], ['baskiTut', 'Baskıda pas', 3], ['kapma', 'Kapma', 1], ['kapYe', 'Kapılan', 1], ['sahip', 'Sahiplik', 3], ['hiz', 'Koşu hızı', 3], ['sarj', 'Şarj', 3], ['gon', 'Gönderme', 1]];
console.log(`Taban maç: ${T0.length}\n`); console.log('| Özellik | ' + G.map(g => g[1] + ' (6 / 10 / 14)').join(' | ') + ' |'); console.log('|---|' + G.map(() => '---').join('|') + '|');
for (const a of A) { const R6 = yukle(a + '-6-'), R14 = yukle(a + '-14-'); if (!R6.length && !R14.length) continue;
  const h = G.map(([k, , d]) => { const x = ort(R6, k), y = ort(T0, k), z = ort(R14, k); if (x == null || y == null || z == null) return '-'; const f = v => (k === 'pasTut' || k === 'baskiTut' || k === 'sahip') ? (100 * v).toFixed(0) : v.toFixed(d); const yon = (y - x) * (z - y) >= 0 ? '' : ' ⚠'; return `${f(x)} / ${f(y)} / ${f(z)}${yon}`; });
  console.log(`| ${AD[a]} (n ${R6.length}+${R14.length}) | ${h.join(' | ')} |`); }
console.log('\n⚠: 6→10→14 yönü dönüyor (tek yönlü değil ya da gürültü). Gol farkı ve kapma gürültülü; mekanik göstergeler (hız, pas tutma, şarj) daha az.');
