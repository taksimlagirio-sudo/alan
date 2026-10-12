// Rol pozisyonları: hücum takımı (Mavi) ile savunma takımı (Turuncu), kendi stilleri ve rolleriyle.
// Oyun sayıya kadar ya da Çekirdek savunma takımına geçtikten sonra 12 saniye daha (kontra görünsün) oynanır.
self.window = self;
const V = '?v=' + (self.location.search.match(/v=(\w+)/) || [, '1'])[1]; importScripts('core.js' + V, 'shot-table.js' + V, 'decide.js' + V, 'match.js' + V, 'shape.js' + V);
if (!self.AlanMatch || !self.AlanMatch.STYLES) throw new Error('match.js eski sürüm yüklendi (STYLES yok)');
const START = {
  kurulum: { title: 'Arkadan kurulum', why: 'Çekirdek Kurucu\'da, kendi yarısının derininde. Savunma takımı yerleşik.', role: 'Kurucu', x: 22, y: 25 },
  orta: { title: 'Orta sahada', why: 'Çekirdek orta sahada, Serbest oyuncuda. Savunma bloğunun önü.', role: 'Serbest', x: 48, y: 30 },
  kanat: { title: 'Kanatta, son üçte bir', why: 'Çekirdek Kanat\'ta, rakip yarıda çizgiye yakın. Savunma Kuyu önünde yerleşik.', role: 'Kanat', x: 68, y: 7 }
};
function play(q) {
  const M = self.AlanMatch, S0 = M.STYLES[q.atk], S1 = M.STYLES[q.def], st = START[q.start];
  const m = M.createMatch(q.seed, { tac: [S0.tac, S1.tac], roles: [S0.roles, S1.roles] });
  let h = m.ps.find(p => p.team === 0 && p.rh === st.role) || m.ps.find(p => p.team === 0 && p.role !== 'Bekçi');
  h.x = st.x; h.y = st.y; m.holder = h; m.ball = null; m.ch = .35; m.fl = null;
  // yerleşme: taşıyıcı sabit, iki takım şekline otursun (karar yok)
  for (let t = 0; t < 120; t++) { m.decT = 99; m.tick++; self.AlanShape.position(m); M.move(m); h.x = st.x; h.y = st.y; h.vx = h.vy = 0; }
  m.tick = 0; m.decT = 1; m.ra = [{}, {}]; m.events = []; m.st.steal = [0, 0];
  const frames = [], names = m.ps.map(p => p.name), teams = m.ps.map(p => p.team), rh = m.ps.map(p => p.rh || (p.role === 'Bekçi' ? 'Bekçi' : '')), rs = m.ps.map(p => p.rs || (p.role === 'Bekçi' ? 'Bekçi' : ''));
  let res = 'süre', lostT = null;
  for (let t = 0; t < 3000; t++) {
    try { M.step(m); } catch (e) { res = 'hata: ' + e.message; break; }
    const hd = m.holder, cp = M.corePos(m), bx = cp.x, by = cp.y, ph = hd ? hd.team : (m.ball ? m.ball.team : 0);
    frames.push([bx, by, hd ? m.ps.indexOf(hd) : -1, ph, m.ch, ...m.ps.flatMap(p => [Math.round(p.x * 10) / 10, Math.round(p.y * 10) / 10])]);
    if (m.score[0] > 0) { res = lostT == null ? 'SAYI' : 'SAYI (yeniden kazanıp)'; break; }
    if (m.score[1] > 0) { res = 'KONTRA SAYISI'; break; }
    if (lostT == null && hd && hd.team === 1) { lostT = m.tick; res = hd.role === 'Bekçi' ? 'Bekçi aldı' : 'kaybetti'; }
    if (lostT != null && m.tick - lostT >= 720) break;
  }
  const ra = m.ps.map(p => { const A = p.rh && m.ra[p.team][p.i]; return A ? { h: A.h, s: A.s } : null; });
  return { res, lostT, frames, names, teams, rh, rs, ra, mk: m.ps.map(p => p.markRef ? p.markRef.name : ''), events: m.events.slice().reverse().map(e => ({ t: e.t, text: e.text, team: e.team })), st: { pass: m.st.pass, passOk: m.st.passOk, shot: m.st.shot } };
}
self.onmessage = e => { const q = e.data; try { self.postMessage({ id: q.id, ok: true, r: play(q), title: START[q.start].title, why: START[q.start].why }); } catch (err) { self.postMessage({ id: q.id, ok: false, msg: String(err && err.message || err) }); } };
self.START = START;
