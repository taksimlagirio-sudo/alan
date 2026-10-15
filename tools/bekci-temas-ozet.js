// bekci-temas.js çıktılarının özeti (yalnız göndermeler)
const fs = require('fs'), { oku } = require('./kayit-oku.js'), [dTem, dKay, ad] = process.argv.slice(2);
const T = []; let sayiTop = 0, sayiTemasli = 0, gonTop = 0;
for (const f of fs.readdirSync(dTem).filter(f => f.startsWith(ad + '-') && f.endsWith('.json'))) { const d = JSON.parse(fs.readFileSync(dTem + '/' + f)); T.push(...d.tem.filter(e => e.gon));
  const k = oku(dKay + '/' + d.sc + '.json.gz'); k.olay.forEach((o, oi) => { if (o.tur !== 'gonder') return; gonTop++; let s = false; for (let j = oi + 1; j < k.olay.length; j++) { const p = k.olay[j]; if (p.tur === 'fake') continue; s = p.tur === 'sayi' && p.takim === o.takim; break; } if (!s) return; sayiTop++;
    const sn = k.olay.find((p, j) => j > oi && p.tur === 'sayi').t; if (d.tem.some(e => e.gon && e.t >= o.t && e.t <= sn)) sayiTemasli++; }); }
const n = T.length, say = a => a.length, p = (a, b) => b ? (100 * a / b).toFixed(0) + '%' : '–', o = (a, k) => a.length ? (a.reduce((s, e) => s + e[k], 0) / a.length).toFixed(2) : '–';
console.log(`gönderme ${gonTop} · sayı ${sayiTop} · bunlardan Bekçi'nin değdiği ${sayiTemasli} (${p(sayiTemasli, sayiTop)}); değmeden giren ${sayiTop - sayiTemasli}`);
console.log(`Bekçi'nin göndermeye teması: ${n}`);
for (const s of ['tuttu', 'sekti', 'deldi']) { const a = T.filter(e => e.sonuc === s); console.log(`  ${s}: ${a.length} (${p(a.length, n)}) · ardından 1 sn'de sayı ${say(a.filter(e => e.sayi))}`); }
const S = T.filter(e => e.sonuc === 'sekti');
console.log(`sekenler: Kuyu'ya doğru (çizgiyi ağızda keser) ${say(S.filter(e => e.kuyuya))} · Kuyu yönüne ama ağız dışına ${say(S.filter(e => e.kuyuYaklas && !e.kuyuya))} · dışarı ${say(S.filter(e => !e.kuyuYaklas))}`);
console.log(`  Kuyu'ya doğru sekip sayı olan ${say(S.filter(e => e.kuyuya && e.sayi))} · dışarı sekip sayı olan ${say(S.filter(e => !e.kuyuYaklas && e.sayi))}`);
console.log(`\ngöreli hız / kapasite / tutma ihtimali / hazırlık (ortalama):`);
for (const [ad2, a] of [['tuttu', T.filter(e => e.sonuc === 'tuttu')], ['sekti', S], ['deldi', T.filter(e => e.sonuc === 'deldi')], ['sayıya giden', T.filter(e => e.sayi && e.sonuc !== 'tuttu')]]) console.log(`  ${ad2.padEnd(13)} n ${String(a.length).padStart(3)} · vr ${o(a, 'vr')} · cap ${o(a, 'cap')} · P ${o(a, 'P')} · haz ${o(a, 'haz')} · Çekirdek yola çıkalı ${o(a, 'bt')} tik`);
const hz = [[0, .5], [.5, .99], [.99, 2]]; console.log('\nhazırlığa göre tutma:'); for (const [a, b] of hz) { const x = T.filter(e => e.haz >= a && e.haz < b); console.log(`  haz ${a}–${b > 1 ? 1 : b}: ${x.length} temas · tuttu ${p(x.filter(e => e.sonuc === 'tuttu').length, x.length)} · ardından sayı ${p(x.filter(e => e.sayi).length, x.length)}`); }
const vb = [[0, 2], [2, 2.6], [2.6, 9]]; console.log('göreli hıza göre tutma:'); for (const [a, b] of vb) { const x = T.filter(e => e.vr >= a && e.vr < b); console.log(`  vr ${a}–${b}: ${x.length} temas · tuttu ${p(x.filter(e => e.sonuc === 'tuttu').length, x.length)} · ardından sayı ${p(x.filter(e => e.sayi).length, x.length)} · ort. cap ${o(x, 'cap')}`); }
