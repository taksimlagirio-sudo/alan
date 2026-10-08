// Laboratuvar maçı (tarayıcısız): bir hücreden i'nci maçı oynatır, A takımı açısından özet satırı yazar.
// Laboratuvar sayfasıyla aynı düzen: tek sıralı maçlarda taraflar yer değiştirir, maç 5400 tik.
// Kullanım: node tools/lab.js <motor klasörü> <hücre> <i> [tik]
const { load } = require('./load');
const [dir, cell, iS, lenS] = process.argv.slice(2), i = +iS, len = +lenS || 5400, seed = 1000 + i * 7919;
const g = load(dir, seed), M = g.AlanMatch, S = M.STYLES;
const LIVE = [{ sistem: 'Alan', pres: 1, arkada: 1, blok: 'Düşük', genislik: 'Dar', tempo: .8, risk: .8, kazaninca: 'Kontra' }, { sistem: 'Adam adama', pres: 2, arkada: 2, blok: 'Yüksek', genislik: 'Geniş', tempo: .2, risk: .2, kazaninca: 'Dengeli' }];
const CELLS = { ayna: { tac: [{}, {}] }, canli: { tac: LIVE }, stiller: { tac: [S['Hücumcu'].tac, S['Savunmacı'].tac], roles: [S['Hücumcu'].roles, S['Savunmacı'].roles] } };
const c = CELLS[cell], sw = i % 2 === 1, A = sw ? 1 : 0, B = 1 - A, pick = v => v && (sw ? [v[1], v[0]] : v);
const m = M.createMatch(seed, { tac: pick(c.tac), roles: pick(c.roles) }); m.len = len;
const t0 = Date.now(); M.run(m); const st = m.st, two = k => [st[k][A], st[k][B]];
console.log(JSON.stringify({ dir, cell, i, ms: Date.now() - t0, g: [m.score[A], m.score[B]], shot: two('shot'), pass: two('pass'), passOk: two('passOk'), poss: two('poss'), steal: two('steal') }));
