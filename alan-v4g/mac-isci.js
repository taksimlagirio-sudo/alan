// Maç işçisi: maçın bütün hesabı sayfanın dışında (sayfa sadece çizer, akıcı kalır). Beynin denemeleri bu işçinin kendi alt işçilerine dağıtılır.
self.window = self;
importScripts('core.js', 'shot-table.js', 'value-table.js', 'decide.js', 'match.js', 'shape.js', 'beyin.js', 'vs-table.js', 'pas-table.js', 'ogrenme.js');
const M = self.AlanMatch, B = self.AlanBeyin, NF = 34; let gen = 0, m = null, len = 5400, anlik = new Map(), havuz = null, buf = null;
function havuzKur(NW) { const ws = [], wait = new Map(); let id = 0, ok = true;
  for (let k = 0; k < NW; k++) { try { const w = new Worker('beyin-isci.js'); w.onmessage = e => { const f = wait.get(e.data.id); wait.delete(e.data.id); if (e.data.hata) { ok = false; } f && f(e.data.sonuc || null); }; w.onerror = () => { ok = false; for (const [i, f] of wait) { wait.delete(i); f(null); } }; ws.push(w); } catch (e) { ok = false; } }
  return { tabloGonder(v) { for (const w of ws) w.postMessage({ tip: 'tablo', v }); }, get ok() { return ok && ws.length > 0; }, isler(m, jobs) { const durum = B.paketle(m), parts = ws.map(() => []); jobs.forEach((j, k) => { if (k) parts[(k - 1) % ws.length].push(k); }); const res = new Array(jobs.length);
      if (!ok) return Promise.resolve(jobs.map(j => B.rollout(m, j))); const ps = parts.map((ix, w) => ix.length ? new Promise(r => { const i = ++id; wait.set(i, v => { if (v) v.forEach((x, q) => res[ix[q]] = x); else ix.forEach(q => res[q] = B.rollout(m, jobs[q])); r(); }); ws[w].postMessage({ id: i, durum, isler: ix.map(q => jobs[q]), ayar: { ...B.A } }); }) : null); if (jobs.length) res[0] = B.rollout(m, jobs[0]); return Promise.all(ps).then(() => res); } }; }
const yeniBuf = () => ({ kare: [], olay: [], karar: [] });
function statik(m) { return m.ps.map(p => ({ id: p.id, name: p.name, role: p.role, team: p.team, a: { ...p.a }, R: p.R, D: p.D, i: p.i })); }
function kaydet(m) { const f = new Float32Array(NF); m.ps.forEach((p, k) => { f[2 * k] = p.x; f[2 * k + 1] = p.y; }); const c = M.corePos(m); f[28] = c ? c.x : NaN; f[29] = c ? c.y : NaN; f[30] = m.holder ? m.ps.indexOf(m.holder) : -1; f[31] = m.holder ? m.ch : (m.ball ? m.ball.ch || 0 : 0); f[32] = m.score[0]; f[33] = m.score[1]; buf.kare.push([m.tick, f]);
  if (m.events.length && m.events[0] !== m._sonOlay) { let k = 0; while (k < m.events.length && m.events[k] !== m._sonOlay) k++; for (let j = k - 1; j >= 0; j--) buf.olay.push({ tick: m.tick, ...m.events[j] }); m._sonOlay = m.events[0]; }
  if (m._beyinSon && m._beyinSon.tick === m.tick - 1) buf.karar.push(m._beyinSon);
  if (m.tick % 60 === 0) anlik.set(m.tick, B.paketle(m)); }
function gonder(g, son) { postMessage({ tip: 'kare', gen: g, kare: buf.kare, olay: buf.olay, karar: buf.karar, tick: m.tick, son: !!son }); buf = yeniBuf(); }
async function calistir(g, ay) { B.A.bak = ay.acik; B.A.icHafif = ay.hizli; B.A.kosu = ay.kosu; B.A.uc = ay.uc; const isler = ay.acik && havuz && havuz.ok ? (mm, jobs) => havuz.isler(mm, jobs) : undefined;
  const ilerle = async () => { if (g !== gen) throw 'iptal'; gonder(g); await new Promise(r => setTimeout(r, 0)); }, kare = mm => { if (g !== gen) throw 'iptal'; kaydet(mm); };
  try { if (!ay.acik) { while (!m.over && m.tick < len) { M.step(m); kare(m); if (m.tick % 60 === 0) await ilerle(); } } else await B.oynaAsync(m, { isler, kare, ilerle }); } catch (e) { if (e === 'iptal') return; postMessage({ tip: 'hata', msg: String(e && e.stack || e) }); return; }
  if (g === gen) gonder(g, true); }
onmessage = e => { const d = e.data;
  if (d.tip === 'kur') { havuz = havuzKur(d.NW); return; }
  if (d.tip === 'tablo') { self.ALAN_VS.v = d.v; if (havuz) havuz.tabloGonder(d.v); return; }
  if (d.tip === 'yeni') { gen = d.gen; len = d.len; m = M.createMatch(d.tohum, { tac: d.tac, len });
    if (d.oz === 'rastgele') { let r0 = d.kadro * 2654435761 >>> 0; const R = () => (r0 = (Math.imul(r0, 1664525) + 1013904223) >>> 0) / 4294967296; for (const p of m.ps) for (const k of Object.keys(p.a)) p.a[k] = 6 + Math.floor(R() * 11); }
    anlik = new Map(); buf = yeniBuf(); kaydet(m); postMessage({ tip: 'basla', gen, ps: statik(m) }); calistir(gen, d.ay); return; }
  if (d.tip === 'degistir') { const s = anlik.get(d.snapT); if (!s) { postMessage({ tip: 'hata', msg: 'anlık görüntü yok: ' + d.snapT }); return; } gen = d.gen; m = B.ac(structuredClone(s)); m.tac[d.t] = d.tac; if (m.tacB) m.tacB[d.t] = { ...d.tac }; m._sonOlay = m.events[0]; for (const k of [...anlik.keys()]) if (k > d.snapT) anlik.delete(k); buf = yeniBuf(); calistir(gen, d.ay); } };
