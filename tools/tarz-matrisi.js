// Tarz matrisi: iki tarzı (taktik-ayar.js: T_kale, T_kontra, T_sahip, T_presci, T_dikine, T_kanat ya da tek ayarlar) aynı oyuncularla karşılaştırır.
// Her takımın enerji profili (barın ortalaması, dibe vurma, sprint payı, harcamanın türleri) ve sonucu (skor, şut, topa sahip olma, ikili, top kazanma).
// Amaç (TASARIM-enerji-ve-alan.md, enerjinin dinamiği): her tarzın artısı ve eksisi mekanikten çıkıyor mu, hiçbir harcama bedelsiz kalmıyor mu.
// Kullanım: node tools/tarz-matrisi.js <motor klasörü> <tarzA> <tarzB> <tohum> [tik] >> sonuc.jsonl   (tik varsayılan ALAN_MAC_LEN; kafa açık, canlıdaki gibi)
//           node tools/tarz-matrisi-ozet.js sonuc.jsonl
// A takımı tohumun tekliğine göre 0 ya da 1 (taraf dengelenir). Bütün özellikler 10 (OZ=20 ile 20). Bayrak: BAYRAK=ALAN_SPRINT_DEGER_ESKI,...
const fs = require('fs'), path = require('path'); const [dir, aAd, bAd, sS, lS] = process.argv.slice(2), seed = +sS;
const OZ = ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', 'refleks', 'yerlesim', 'hakimiyet', 'enerji'], OZD = +(process.env.OZ || 10);
const { V, VD, AYAR } = require('./taktik-ayar');
function rng(sd) { let a = sd >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
Math.random = rng(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis; globalThis.location = { search: '' };
if (process.env.BAYRAK) for (const b of process.env.BAYRAK.split(',')) globalThis[b] = true;
for (const f of ['sabit-mat.js', 'core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js']) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const M = globalThis.AlanMatch, LEN = +lS || globalThis.ALAN_MAC_LEN || 9000; if (!AYAR[aAd] || !AYAR[bAd]) throw new Error('ayar yok: ' + aAd + ' / ' + bAd);
const kur = ad => { const { _diz, ...t } = AYAR[ad]; return { tac: { ...V, tetik: V.tetik.slice(), ...t }, diz: _diz || VD }; };
const A = seed % 2, B = 1 - A, KA = kur(aAd), KB = kur(bAd), K = A === 0 ? [KA, KB] : [KB, KA], t0 = Date.now();
const m = M.createMatch(seed * 7919 + 17, { tac: K.map(k => k.tac), len: LEN, diz: K.map(k => k.diz), devre: true });
for (const p of m.ps) { const a = {}; for (const k of OZ) a[k] = OZD; a.taraf = (p.a && p.a.taraf) || 'sag'; p.a = a; } for (const t of [0, 1]) M.dizUygula(m, t, K[t].diz); if (M.kafaAc) M.kafaAc(m);
const T = [0, 1].map(() => ({ n: 0, en: 0, dip: 0, spr: 0, sahip: 0 }));
while (!m.over && m.tick < LEN) { M.step(m); if (m.holder) T[m.holder.team].sahip++;
  for (const p of m.ps) { if (p.role === 'Bekçi' || p.en == null) continue; const o = T[p.team]; o.n++; o.en += p.en; if (p.en < .2) o.dip++; if (Math.hypot(p.vx || 0, p.vy || 0) > M.spd(p) * 1.02) o.spr++; } }
const H = m.st.enH || [{}, {}], r3 = v => Math.round(v * 1000) / 1000, tik = Math.max(1, m.tick);
const takim = t => { const o = T[t], dk = o.n / (60 * (globalThis.ALAN_TPS || 30)) || 1; /* oyuncu-dakika */ const har = {}; for (const k of ['sprint', 'ivme', 'surme', 'fake', 'ikili', 'alan', 'sarj', 'dol']) har[k] = r3((H[t][k] || 0) / dk); return { bar: r3(o.en / (o.n || 1)), dip: r3(o.dip / (o.n || 1)), sprint: r3(o.spr / (o.n || 1)), harcama_dk: har, sahip: r3(o.sahip / tik), skor: m.score[t], sut: m.st.shot[t], ikili: m.st.duel[t], ikiliK: m.st.duelW[t], kazanma: m.st.steal[t], pas: m.st.pass[t], pasOk: m.st.passOk[t] }; };
console.log(JSON.stringify({ tohum: seed, tik: m.tick, A: aAd, B: bAd, a: takim(A), b: takim(B), sn: Math.round((Date.now() - t0) / 1000) }));
