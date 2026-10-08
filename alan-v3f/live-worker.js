// Canlı maç iş parçacığı: maçı sayfadan bağımsız oynatır (karar anlarındaki ağır hesap ekranı dondurmaz), her karede bir anlık görüntü gönderir.
self.window = self;
const V = '?v=' + (self.location.search.match(/v=(\w+)/) || [, '1'])[1]; importScripts('core.js' + V, 'shot-table.js' + V, 'decide.js' + V, 'match.js' + V, 'shape.js' + V);
let m = null, playing = true, speed = 1, acc = 0, last = Date.now();
const PK = ['x', 'y', 'vx', 'vy', 'tx', 'ty', 'team', 'role', 'rh', 'rs', 'name', 'job', 'press', 'sprint', 'noTouch', 'ca', 'i', 'id', 'R', 'D', 'a', 'slot'];
function snap() { const ps = m.ps.map(p => { const o = {}; for (const k of PK) o[k] = p[k]; return o; }); const b = m.ball ? { x: m.ball.x, y: m.ball.y, vx: m.ball.vx, vy: m.ball.vy, alive: m.ball.alive, team: m.ball.team, ch: m.ball.ch, done: m.ball.done } : null;
  return { ps, hi: m.holder ? m.ps.indexOf(m.holder) : -1, ball: b, ch: m.ch, tick: m.tick, len: m.len, over: m.over, score: m.score, events: m.events.slice(0, 40), lastDec: m.lastDec, st: m.st, tac: m.tac }; }
self.onmessage = e => { const q = e.data;
  if (q.type === 'new') { m = self.AlanMatch.createMatch(q.seed, { tac: q.tac, len: q.len }); acc = 0; last = Date.now(); self.postMessage(snap()); }
  else if (q.type === 'tac' && m) m.tac[q.t][q.k] = q.v;
  else if (q.type === 'speed') speed = q.v;
  else if (q.type === 'play') playing = q.v; };
setInterval(() => { if (!m) return; const now = Date.now(), dt = Math.min(250, now - last); last = now; if (!playing || m.over) return; acc += dt * 60 / 1000 * speed; let n = 0; while (acc >= 1 && n < 8 * speed) { self.AlanMatch.step(m); acc -= 1; n++; } if (acc > 8) acc = 8; if (n) self.postMessage(snap()); }, 16);
