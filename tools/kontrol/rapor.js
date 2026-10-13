// kontrol.sh çıktısından tek sayfalık rapor. Kullanım: node tools/kontrol/rapor.js <çıktı klasörü>
const fs = require('fs'), path = require('path'), OUT = process.argv[2], oku = f => { try { return fs.readFileSync(path.join(OUT, f), 'utf8'); } catch (e) { return ''; } };
const YENI = oku('yeni.txt').trim(), ONCEKI = oku('onceki.txt').trim(), mean = a => a.length ? a.reduce((s, x) => s + x, 0) / a.length : NaN;
const json = d => fs.existsSync(path.join(OUT, d)) ? fs.readdirSync(path.join(OUT, d)).filter(f => f.endsWith('.json')).map(f => { try { return { f, j: JSON.parse(oku(path.join(d, f))) }; } catch (e) { return null; } }).filter(Boolean) : [];
const ozet = [], OK = '✓', UY = '⚠', HATA = '✗', satir = (d, ad, n) => ozet.push(`| ${d} | ${ad} | ${n} |`);
// 1 · tarayıcı
const tr = oku('tarayici.txt'), hz = tr.match(/hazır olana kadar (\d+) sn/), labH = (tr.split('--- lab')[1] || '').split('\n').filter(l => /console:|pageerror:|http [45]/.test(l) && !/favicon|404 \(File not found\)/.test(l));
satir(hz ? OK : HATA, 'Oyun açılıyor (kadro → taktik → canlı)', hz ? `maç ${hz[1]} sn'de hazır` : 'maç hazır olmadı');
satir(/\d+ \/ \d+ maç/.test(tr) && !labH.length ? OK : UY, 'Laboratuvar açılıyor ve çalışıyor', labH.length ? labH.slice(0, 2).join(' · ') : 'hata yok');
// 2 · donma
const dn = fs.existsSync(path.join(OUT, 'donma')) ? fs.readdirSync(path.join(OUT, 'donma')).map(f => oku('donma/' + f)) : [], hv = dn.map(t => +((t.match(/en uzun hareketsiz (\d+) tik/) || [])[1] ?? NaN)), ya = dn.map(t => +((t.match(/en yavaş adım (\d+) ms/) || [])[1] ?? NaN)), ys = dn.map(t => +((t.match(/0,5 sn üstü adım (\d+)/) || [])[1] ?? NaN));
const hvM = Math.max(...hv); satir(isNaN(hvM) ? HATA : hvM < 120 ? OK : HATA, 'Donma yok (4 maç, Liman–Demir Blok)', isNaN(hvM) ? 'ölçülemedi' : `en uzun hareketsizlik ${hvM} tik (${(hvM / 60).toFixed(1)} sn)`);
// 3 · istatistik ve hız
const S = json('stat'), Y = S.filter(x => x.f.startsWith('yeni-')).map(x => x.j), O = S.filter(x => x.f.startsWith('onceki-')).map(x => x.j);
const sy = mean(Y.map(r => r.sure)), so = mean(O.map(r => r.sure)), oran = sy / so;
satir(oran > 1.15 ? UY : OK, 'Hız (aynı kuyrukta, maç başına hesap)', `${so.toFixed(0)} → ${sy.toFixed(0)} sn (${oran >= 1 ? '+' : ''}${((oran - 1) * 100).toFixed(0)}%) · en yavaş adım ${Math.max(...ya)} ms · 0,5 sn üstü adım ${mean(ys).toFixed(0)}/maç`);
// 4 · hızlandırmalar ve YZ bayrağı (dosyadan)
const dosya = (d, f) => { try { return fs.readFileSync(path.join(d, f), 'utf8'); } catch (e) { return ''; } }, say = (t, s) => t.split(s).length - 1;
const hzY = ['core.js', 'decide.js', 'match.js', 'shape.js'].map(f => [f, say(dosya(YENI + '/alan-v5', f), 'hız:'), say(dosya(ONCEKI + '/alan-v5', f), 'hız:')]), eksik = hzY.filter(([, y, o]) => y < o);
satir(eksik.length ? UY : OK, 'Hızlandırmalar yerinde', eksik.length ? 'eksik: ' + eksik.map(([f, y, o]) => `${f} ${o}→${y}`).join(', ') : hzY.map(([f, y]) => `${f} ${y}`).join(' · '));
const sh = dosya(YENI + '/alan-v5', 'shape.js'), cn = dosya(YENI, 'canli.html'), yzS = /m\.tac\[t\]\s*&&\s*m\.tac\[t\]\.yz/.test(sh), yzC = /i === 1 \? \{ \.\.\.x, yz: true \}/.test(cn);
satir(yzS && yzC ? OK : UY, 'YZ bayrağı: skora göre taktik yalnız rakipte', `motorda koşul ${yzS ? 'var' : 'YOK'} · canlı maç rakibe ${yzC ? 'veriyor' : 'VERMİYOR'}`);
// 5 · taktik etkisi (yeni sürüm, Liman–Saat Kulesi, 2 tohum)
const TK = json('taktik'), tk = ad => TK.filter(x => x.f.startsWith(ad + '-')).map(x => x.j.sv.T[0]), tm = (ad, k) => mean(tk(ad).map(x => x[k]));
const et = (ad, a, b, k, kosul, bic) => { const x = tm(a, k), y = tm(b, k), ok = kosul(x, y); satir(isNaN(x) || isNaN(y) ? HATA : ok ? OK : UY, ad, bic(x, y)); return ok; };
et('Blok etkisi: presçi (düşük → yüksek)', 'blokDusuk', 'blokYuksek', 'pj', (x, y) => y >= 1.3 * x, (x, y) => `${x.toFixed(2)} → ${y.toFixed(2)} kişi`);
et('Şiddet etkisi: ikili (0 → 1)', 'siddet0', 'siddet1', 'duel', (x, y) => y >= 1.3 * x, (x, y) => `${x.toFixed(1)} → ${y.toFixed(1)} / maç`);
et('Tempo etkisi: top yerinde bekliyor (0,2 → 0,8)', 'tempo02', 'tempo08', 'dur', (x, y) => y <= .85 * x, (x, y) => `%${(100 * x).toFixed(0)} → %${(100 * y).toFixed(0)}`);
et('Genişlik etkisi: takım genişliği (dar → geniş)', 'genDar', 'genGenis', 'gen', (x, y) => y >= x + 3, (x, y) => `${x.toFixed(1)} → ${y.toFixed(1)} birim`);
// çıktı
const t2 = x => x[0] + x[1], M = [['Sayı / maç', r => t2(r.skor), 2], ['Gönderme / maç', r => t2(r.st.shot), 1], ['Pas / maç', r => t2(r.st.pass), 0], ['Pas tutma %', r => 100 * t2(r.st.passOk) / t2(r.st.pass), 1], ['Top kapma / maç', r => t2(r.st.steal), 1], ['İkili / maç', r => t2(r.st.duel), 1], ['Top el değiştirme / maç', r => r.sv.degis, 1], ['Taşıyıcıya basan (3 birim içinde)', r => r.sv.pres, 2], ['Son bölgede boş hücumcu', r => r.sv.bos ?? 0, 2], ['Hesap süresi (sn)', r => r.sure, 0]];
console.log(`# ALAN · kontrol raporu\n\nYeni: \`${YENI}\` · Önceki: \`${ONCEKI}\` · ${oku('baslangic.txt').trim()} – ${oku('bitis.txt').trim()}\n\n| | Kontrol | Sonuç |\n|---|---|---|\n${ozet.join('\n')}\n`);
console.log(`## Oyun nasıl değişti (16 kurulum, 1'er tohum, önceki → yeni)\n\n| | Önceki | Yeni | Fark |\n|---|---|---|---|`);
for (const [ad, f, d] of M) { const o = mean(O.map(f)), y = mean(Y.map(f)), r = (y - o) / Math.abs(o || 1); console.log(`| ${ad} | ${o.toFixed(d)} | ${y.toFixed(d)} | ${(y - o >= 0 ? '+' : '') + (y - o).toFixed(d)}${Math.abs(r) > .25 ? ' ' + UY : ''} |`); }
console.log(`\n## Taktik değişkenleri (yeni sürüm, Liman–Saat Kulesi, 2 tohum; Liman'ın değerleri)\n\n| Değişken | Presçi | İkili | Top bekliyor | Genişlik | Pas | Sayı (Liman–rakip) |\n|---|---|---|---|---|---|---|`);
for (const ad of ['varsayilan', 'blokDusuk', 'blokYuksek', 'siddet0', 'siddet1', 'tempo02', 'tempo08', 'genDar', 'genGenis']) { const r = TK.filter(x => x.f.startsWith(ad + '-')).map(x => x.j); if (!r.length) continue; const t = r.map(x => x.sv.T[0]); console.log(`| ${ad} | ${mean(t.map(x => x.pj)).toFixed(2)} | ${mean(t.map(x => x.duel)).toFixed(1)} | %${(100 * mean(t.map(x => x.dur))).toFixed(0)} | ${mean(t.map(x => x.gen)).toFixed(1)} | ${mean(t.map(x => x.pas)).toFixed(0)} | ${mean(r.map(x => x.skor[0])).toFixed(1)}–${mean(r.map(x => x.skor[1])).toFixed(1)} |`); }
console.log(`\n## Donma ayrıntısı\n\n${dn.map(t => '- ' + t.trim()).join('\n')}\n\n## Tarayıcı\n\n\`\`\`\n${tr.trim()}\n\`\`\`\n\nNot: Taktik satırları 2'şer maçtan; büyük etkiler güvenilir, küçük farklar gürültü olabilir. Eşikler: donma < 2 sn; hız ≤ %15 yavaşlama; taktik etkisi blok ve şiddet ×1,3, tempo ×0,85, genişlik +3 birim. ${UY} işareti tabloda %25'ten büyük değişimi gösterir.`);
