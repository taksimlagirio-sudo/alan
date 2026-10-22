// Kaydı motorun işçisinin yükleme sırasıyla tek çekirdekte oynatır; her tikin süresini ve hatayı yazar.
const fs = require('fs'), path = require('path'); const [dir, kf, LIM] = process.argv.slice(2); globalThis.window = globalThis; globalThis.self = globalThis;
const yuk = /importScripts\(([^)]*)\)/.exec(fs.readFileSync(path.join(dir, 'mac-isci.js'), 'utf8'))[1].split(',').map(s => s.trim().replace(/['"]/g, ''));
for (const f of yuk) (0, eval)(fs.readFileSync(path.join(dir, f), 'utf8') + '\n//# sourceURL=' + f);
const d = JSON.parse(fs.readFileSync(kf, 'utf8')).yeni, M = globalThis.AlanMatch; const m = M.createMatch(d.tohum, { tac: d.tac, len: d.len, diz: d.diz, devre: d.devre ?? false });
for (const t of [0, 1]) for (const p of m.ps) { if (p.team !== t) continue; const o = d.oyuncular[t][p.i]; if (o) { p.name = o.ad; p.a = { ...p.a, ...o.a }; } }
M.assignRoles(m, d.roller); for (const p of m.ps) { const r = d.roller[p.team] && d.roller[p.team][p.i - 1]; if (r && r.eg) p.eg = r.eg; } for (const t of [0, 1]) M.dizUygula(m, t, d.diz[t]); M.kafaAc && M.kafaAc(m);
const yavas = [], T = []; let T0 = Date.now();
try { while (!m.over && m.tick < +LIM) { const t = Date.now(); M.step(m); const dt = Date.now() - t; T.push(dt); if (dt > 400) { yavas.push([m.tick, dt]); console.log('yavaş tik', m.tick, dt + ' ms', m.holder ? m.holder.name : '-'); } if (m.tick % 600 === 0) console.log('tik', m.tick, 'skor', m.score.join('-'), ((Date.now() - T0) / 1000).toFixed(0) + ' sn'); } console.log("bitti", m.tick, m.score); fs.writeFileSync(process.env.TF || "/dev/null", T.join("\n")); }
catch (e) { console.log('HATA tik', m.tick, String(e.stack).slice(0, 1500)); }
