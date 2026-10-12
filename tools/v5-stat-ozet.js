// v5-stat.js çıktılarını iki motor arasında karşılaştırır. Kullanım: node tools/v5-stat-ozet.js <klasör> <motorA> <motorB>
const fs = require('fs'), path = require('path'); const [dir, a, b] = process.argv.slice(2);
const oku = e => fs.readdirSync(dir).filter(f => f.startsWith(e + '-')).map(f => { try { return JSON.parse(fs.readFileSync(path.join(dir, f), 'utf8')); } catch (x) { return null; } }).filter(Boolean);
const A = oku(a), B = oku(b), sum = (R, f) => R.reduce((s, r) => s + f(r), 0), t2 = x => x[0] + x[1];
const sat = (ad, f, bic) => { const x = f(A), y = f(B); console.log(`| ${ad} | ${bic(x)} | ${bic(y)} |`); };
console.log(`| | ${a} (${A.length} maç) | ${b} (${B.length} maç) |\n|---|---|---|`);
const n1 = v => v.toFixed(2), pc = v => (v * 100).toFixed(0) + '%';
sat('sayı / maç', R => sum(R, r => t2(r.skor)) / R.length, n1);
sat('gönderme / maç', R => sum(R, r => t2(r.st.shot)) / R.length, n1);
sat('pas / maç', R => sum(R, r => t2(r.st.pass)) / R.length, n1);
sat('pas tutma', R => sum(R, r => t2(r.st.passOk)) / sum(R, r => t2(r.st.pass)), pc);
sat('fake / maç', R => sum(R, r => t2(r.st.fake || [0, 0])) / R.length, n1);
sat('fake\'te aldanan oldu', R => sum(R, r => t2(r.st.fakeOk || [0, 0])) / Math.max(1, sum(R, r => t2(r.st.fake || [0, 0]))), pc);
sat('top kapma / maç', R => sum(R, r => t2(r.st.steal)) / R.length, n1);
sat('ikili / maç', R => sum(R, r => t2(r.st.duel)) / R.length, n1);
sat('hesap süresi (toplam sn)', R => sum(R, r => r.sure), v => v.toFixed(0));
const es = A.map(x => [x, B.find(y => y.sc === x.sc && y.kay === x.kay)]).filter(([, y]) => y);
console.log(`\nAynı kurulumda süre oranı (${b} / ${a}): ${(sum(es, ([x, y]) => y.sure) / sum(es, ([x]) => x.sure)).toFixed(2)} · skor aynı kalan maç: ${es.filter(([x, y]) => x.skor + '' === y.skor + '').length}/${es.length}`);
console.log('\n| kurulum | ' + a + ' skor | ' + b + ' skor | ' + a + ' fake | ' + b + ' fake |\n|---|---|---|---|---|'); for (const [x, y] of es.sort((p, q) => p[0].sc - q[0].sc)) console.log(`| ${x.ad} | ${x.skor.join('–')} | ${y.skor.join('–')} | ${t2(x.st.fake || [0, 0])} | ${t2(y.st.fake || [0, 0])} |`);
