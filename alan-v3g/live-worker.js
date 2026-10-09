// Canlı maç iş parçacığı: maçı sayfadan bağımsız oynatır (karar anlarındaki ağır hesap ekranı dondurmaz), her karede bir anlık görüntü gönderir.
// Teşhis: adım hata verirse ya da oyuncular 3 sn boyunca kıpırdamazken saat ilerlerse, o anın tam durumu sayfaya gönderilir (sayfa saklar; aynı an yeniden oynatılıp sebep bulunur).
self.window = self;
const V = '?v=' + (self.location.search.match(/v=(\w+)/) || [, '1'])[1]; importScripts('core.js' + V, 'shot-table.js' + V, 'decide.js' + V, 'match.js' + V, 'shape.js' + V);
let m = null, playing = true, speed = 1, acc = 0, last = Date.now(), seed = 0, tac0 = null, err = null, still = 0, lastPos = null, diagSent = false;
const PK = ['x', 'y', 'vx', 'vy', 'tx', 'ty', 'team', 'role', 'rh', 'rs', 'name', 'job', 'press', 'sprint', 'noTouch', 'ca', 'i', 'id', 'R', 'D', 'a', 'slot'];
function snap() { const ps = m.ps.map(p => { const o = {}; for (const k of PK) o[k] = p[k]; return o; }); const b = m.ball ? { x: m.ball.x, y: m.ball.y, vx: m.ball.vx, vy: m.ball.vy, alive: m.ball.alive, team: m.ball.team, ch: m.ball.ch, done: m.ball.done } : null;
  return { ps, hi: m.holder ? m.ps.indexOf(m.holder) : -1, ball: b, ch: m.ch, tick: m.tick, len: m.len, over: m.over, score: m.score, events: m.events.slice(0, 40), lastDec: m.lastDec, st: m.st, tac: m.tac, err }; }
function diag(why) { if (diagSent) return; diagSent = true; const D = m.ps.map(p => ({ id: p.id, name: p.name, team: p.team, role: p.role, x: p.x, y: p.y, vx: p.vx, vy: p.vy, tx: p.tx, ty: p.ty, job: p.job, stun: p.stun, slowT: p.slowT, ca: p.ca, press: p.press, manRef: p.manRef ? p.manRef.id : null }));
  self.postMessage({ diag: { why, seed, tac: tac0, tick: m.tick, holder: m.holder ? m.holder.id : null, ball: m.ball ? { x: m.ball.x, y: m.ball.y, vx: m.ball.vx, vy: m.ball.vy, done: m.ball.done, team: m.ball.team } : null, ch: m.ch, decT: m.decT, score: m.score, ps: D, events: m.events.slice(0, 15).map(e => e.t + ' ' + e.text) } }); }
self.onmessage = e => { const q = e.data;
  if (q.type === 'new') { seed = q.seed; tac0 = JSON.parse(JSON.stringify(q.tac)); m = self.AlanMatch.createMatch(q.seed, { tac: q.tac, len: q.len }); acc = 0; last = Date.now(); err = null; still = 0; lastPos = null; diagSent = false; self.postMessage(snap()); }
  else if (q.type === 'tac' && m) { m.tac[q.t][q.k] = q.v; tac0 = JSON.parse(JSON.stringify(m.tac)); }
  else if (q.type === 'speed') speed = q.v;
  else if (q.type === 'play') playing = q.v; };
setInterval(() => { if (!m) return; const now = Date.now(), dt = Math.min(250, now - last); last = now; if (!playing || m.over) return; acc += dt * 60 / 1000 * speed; let n = 0;
  while (acc >= 1 && n < 8 * speed) { try { self.AlanMatch.step(m); } catch (e) { err = (e && e.message || String(e)) + ' · ' + String(e && e.stack || '').split('\n').slice(0, 3).join(' | '); diag('hata: ' + err); acc = 0; break; } acc -= 1; n++; }
  if (acc > 8) acc = 8;
  if (n) { const pos = m.ps.reduce((s, p) => s + p.x * 3.1 + p.y * 7.3, 0); if (lastPos != null && Math.abs(pos - lastPos) < .01) still += n; else still = 0; lastPos = pos; if (still >= 180) diag('donma: oyuncular 3 sn kıpırdamadı'); self.postMessage(snap()); } }, 16);
