// Kuyu alanı ve Bekçi koruması: bir kaydı tam maç oynatır, kuralların izlerini sayar. Kullanım: [BAYRAK=ALAN_KUYU_ESKI,ALAN_BEKCI_KORUMA_ESKI] node mac.js <motor> <kayit.json>
const fs = require('fs'), path = require('path'); const [dir, kf] = process.argv.slice(2); globalThis.window = globalThis; globalThis.self = globalThis;
for (const kv of (process.env.BAYRAK || '').split(',').filter(Boolean)) globalThis[kv] = true;
const ms_ = fs.readFileSync(path.join(dir, 'mac-isci.js'), 'utf8'), ii = /importScripts\(([^;]*)\)/.exec(ms_)[1], aa = /\[([^\]]*)\]/.exec(ii), yuk = (aa ? aa[1] : ii).split(',').map(x => x.trim().replace(/['"]/g, '')).filter(x => x.endsWith('.js'));
for (const f of yuk) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const d = JSON.parse(fs.readFileSync(kf, 'utf8')).yeni, M = globalThis.AlanMatch, K = window.AlanCore; const m = M.createMatch(d.tohum, { tac: d.tac, len: d.len, diz: d.diz, devre: d.devre ?? false });
for (const t of [0, 1]) for (const p of m.ps) { if (p.team !== t) continue; const o = d.oyuncular[t][p.i]; if (o) { p.name = o.ad; p.a = { ...p.a, ...o.a }; } }
M.assignRoles(m, d.roller); for (const p of m.ps) { const r = d.roller[p.team] && d.roller[p.team][p.i - 1]; if (r && r.eg) p.eg = r.eg; } for (const t of [0, 1]) M.dizUygula(m, t, d.diz[t]); M.kafaAc && M.kafaAc(m);
const R = 14, KX = t => t === 0 ? 0 : 120, icinde = (p, t) => Math.hypot(p.x - KX(t), p.y - 30) < R; /* t: Kuyu'nun takımı (kendi Kuyu'su) */
const Z = [0, 1].map(() => ({ hucIc: 0, savIc: 0, savIcN: 0, giris: 0, kal: [], son: 0, sut: 0, sutIc: 0, gol: 0, golIc: 0, xg: 0, pas: 0, pasOk: 0, bek: { al: 0, kayip5: 0, birak: [], yakin: 0, korT: 0 } }));
const ic = new Map(); let fl0 = m.fl, onceki = null, sut = null, bekAl = null, maxMs = 0, pasA = null; const OL = [];
const T0 = Date.now();
while (!m.over) { const s0 = m.score.slice(), h0 = m.holder, t1 = Date.now(); M.step(m); maxMs = Math.max(maxMs, Date.now() - t1); const h = m.holder, b = m.ball;
  for (const p of m.ps) { if (p.role === 'Bekçi') continue; const rk = 1 - p.team, inOpp = icinde(p, rk), inOwn = icinde(p, p.team), z = Z[p.team], k0 = ic.get(p) || null;
    if (inOpp) z.hucIc++; if (inOwn) z.savIc++; const nowIn = inOpp ? 'o' : inOwn ? 's' : null;
    if (nowIn && (!k0 || k0.w !== nowIn)) { ic.set(p, { w: nowIn, t0: m.tick }); z.giris++; } else if (!nowIn && k0) { z.kal.push(m.tick - k0.t0); ic.delete(p); } }
  for (const p of m.ps) if (p.sonuk && !p._sayildi) { p._sayildi = true; Z[p.team].son++; OL.push(`${(m.tick / 30).toFixed(0)} sn: ${p.name} söndü (${icinde(p, p.team) ? 'kendi' : 'rakip'} Kuyu alanında)`); } else if (!p.sonuk) p._sayildi = false;
  { const at = h || (b ? { team: b.team, x: b.x } : null); if (at) { const dt = 1 - at.team, n = m.ps.filter(p => p.team === dt && p.role !== 'Bekçi' && icinde(p, dt)).length, ofYari = (at.team === 0 ? at.x : 120 - at.x) > 60; if (ofYari) { Z[dt].savIcN += n; Z[dt].savIcT = (Z[dt].savIcT || 0) + 1; } } }
  if (m.fl && m.fl !== fl0) { fl0 = m.fl; const at = onceki; if (at && m.fl.kind === 'gönder' && b && b.shotPk != null) { const z = Z[at.team]; z.sut++; z.xg += b.shotPk; const inn = icinde(at, 1 - at.team); if (inn) z.sutIc++; sut = { t: at.team, inn }; } else if (at && ['pas', 'önüne', 'kenardan'].includes(m.fl.kind)) { Z[at.team].pas++; pasA = at.team; } }
  if (m.score[0] !== s0[0] || m.score[1] !== s0[1]) { const t = m.score[0] !== s0[0] ? 0 : 1; Z[t].gol++; if (sut && sut.t === t && sut.inn) Z[t].golIc++; OL.push(`${(m.tick / 30).toFixed(0)} sn: GOL ${t ? 'Kale Duvarı' : 'Liman'} ${m.score.join('-')}${sut && sut.t === t ? (sut.inn ? ' (Kuyu alanının içinden)' : ' (alan dışından)') : ''}`); sut = null; }
  if (h && h !== h0) { if (pasA != null) { if (h.team === pasA) Z[pasA].pasOk++; pasA = null; }
    if (bekAl && h !== bekAl.p) { const z = Z[bekAl.p.team].bek; z.birak.push(m.tick - bekAl.t0); if (h.team !== bekAl.p.team && m.tick - bekAl.t0 <= 150) z.kayip5++; bekAl = null; }
    if (h.role === 'Bekçi') { bekAl = { p: h, t0: m.tick }; Z[h.team].bek.al++; } onceki = h; }
  if (bekAl && h === bekAl.p && h._korT > m.tick) { const z = Z[h.team].bek; z.korT++; if (m.ps.some(q => q.team !== h.team && Math.hypot(q.x - h.x, q.y - h.y) < 3)) z.yakin++; }
  if (h && h !== bekAl?.p && !h0) {} }
const av = a => a.length ? +(a.reduce((s, x) => s + x, 0) / a.length / 30).toFixed(2) : null, mx = a => a.length ? +(Math.max(...a) / 30).toFixed(1) : null;
const out = Z.map((z, t) => ({ takim: t ? 'Kale Duvarı' : 'Liman', gol: z.gol, golAlanIci: z.golIc, sut: z.sut, sutAlanIci: z.sutIc, xg: +z.xg.toFixed(2), pasIsabet: z.pas ? +(z.pasOk / z.pas).toFixed(3) : null, alan: { rakipAlanindaOyuncuSn: +(z.hucIc / 30).toFixed(0), kendiAlanindaOyuncuSn: +(z.savIc / 30).toFixed(0), giris: z.giris, ortKalis: av(z.kal), enUzun: mx(z.kal), sonme: z.son, kendiAlanindaOrtSavunmaci: z.savIcT ? +(z.savIcN / z.savIcT).toFixed(2) : null }, bekci: { topuAldi: z.bek.al, ortTutma: av(z.bek.birak), kayip5sn: z.bek.kayip5, korumadaRakip3brIcinde: z.bek.korT ? +(z.bek.yakin / z.bek.korT).toFixed(2) : null } }));
console.log(JSON.stringify({ bayrak: process.env.BAYRAK || '-', skor: m.score, tik: m.tick, sn: Math.round((Date.now() - T0) / 1000), enYavasTikMs: maxMs, T: out, olay: OL }));
