// Özelliğin kopyalı ölçümü (bugünkü motor): taban maç (iki takım aynı, herkes 10) oynanır; her ADIM tikte top birindeyse an kopyalanır.
// Her kolda A'nın (takım 0) saha oyuncularında tek özellik = değer; R zarla UFUK tik oynatılır. Aynı an, aynı zarlar: fark yalnız özellikten.
// Ölçülen (kol başına): A'nın pası / tutulan pası, şut (A / rakip), gol (A / rakip), A'nın topu kaybetmesi (top A'dayken rakibe geçiş), ufkun sonunda top kimde.
// Kullanım: node tools/oz-kopya2.js <motor> <özellik> <tohum> [ADIM=240] [R=4] [UFUK=240]   (KOL=4,10,16)
const fs = require('fs'), path = require('path'); const [dir, oz, sS, aS, rS, uS] = process.argv.slice(2), seed = +sS, ADIM = +aS || 240, R = +rS || 4, UFUK = +uS || 240, KOL = (process.env.KOL || '4,10,16').split(',').map(Number);
const BEKCI = ['refleks', 'yerlesim', 'hakimiyet'], OZ = ['hiz', 'ceviklik', 'guc', 'uzanma', 'alan', 'kontrol', 'surme', 'pas', 'gonderme', 'yukleme', 'tepki', 'gorus', 'yeralma', 'sogukkanlilik', 'kesme', ...BEKCI];
function rng(sd) { let a = sd >>> 0; return () => { a = (a + 0x6D2B79F5) | 0; let t = Math.imul(a ^ a >>> 15, 1 | a); t = t + Math.imul(t ^ t >>> 7, 61 | t) ^ t; return ((t ^ t >>> 14) >>> 0) / 4294967296; }; }
Math.random = rng(seed * 7 + 3); globalThis.window = globalThis; globalThis.self = globalThis;
for (const f of ['sabit-mat.js', 'core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js']) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const M = globalThis.AlanMatch, TAC = { sistem: 'Alan', pres: 1, arkada: 1, blok: 'Orta', genislik: 'Normal', tempo: .5, risk: .5, kazaninca: 'Dengeli' }, DIZ = { sav: '2-2-2', huc: '2-1-3' };
const m = M.createMatch(seed * 7919 + 17, { tac: [{ ...TAC }, { ...TAC }], len: 7200, diz: [DIZ, DIZ] });
for (const p of m.ps) { const a = {}; for (const k of OZ) a[k] = 10; a.taraf = (p.a && p.a.taraf) || 'sag'; p.a = a; } for (const t of [0, 1]) M.dizUygula(m, t, DIZ);
const bk = BEKCI.includes(oz);
function kol(snap, v, r) { const c = M.kopya(snap); for (const p of c.ps) if (p.team === 0 && bk === (p.role === 'Bekçi')) p.a = { ...p.a, [oz]: v }; c.r.s = (r * 2654435761 + seed * 97 + 12345) >>> 0;
  const st0 = JSON.parse(JSON.stringify(c.st)), g0 = c.score.slice(); let kay = 0, prevH = c.holder ? c.holder.team : -1;
  for (let t = 0; t < UFUK && !c.over; t++) { M.step(c); const h = c.holder; if (h) { if (prevH === 0 && h.team === 1) kay++; prevH = h.team; } }
  const d = k => c.st[k] ? [c.st[k][0] - (st0[k] ? st0[k][0] : 0), c.st[k][1] - (st0[k] ? st0[k][1] : 0)] : [0, 0];
  return { pas: d('pass')[0], pasOk: d('passOk')[0], sut: d('shot'), gol: [c.score[0] - g0[0], c.score[1] - g0[1]], kay, son: c.holder ? c.holder.team : -1 }; }
while (!m.over && m.tick < 7200) { M.step(m); if (m.tick % ADIM || !m.holder) continue; const snap = M.kopya(m), out = { oz, seed, t: m.tick, kim: m.holder.team, kol: {} };
  for (const v of KOL) out.kol[v] = Array.from({ length: R }, (_, r) => kol(snap, v, r)); console.log(JSON.stringify(out)); }
