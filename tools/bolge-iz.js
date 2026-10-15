// Bölge savunmacısının adam seçiminin girdileri, maçı birebir yeniden oynatarak: her tikte seçilen oyuncuların atandığı rakip,
// evlerine en yakın rakipler (uzaklık / o rakip için yarıçap), ve rakibin kime gittiği.
// Kullanım: node tools/bolge-iz.js <motor> <senaryo> <t0> <t1> <ad,ad>
const { kur } = require('./kurulum'); const [dir, sS, a, b, adS] = process.argv.slice(2), ADS = adS.split(',');
const anchor = "      if (bp) { taken.add(bp); mark.set(bp, q); }\n    }";
const { m, adim, g } = kur(dir, +sS, 0, { 'shape.js': [[anchor, anchor + " if (self.__bz && !m._lite) self.__bz(m, dt, free, mark, base, atk, q => S.zoneR + ok(free[0], 'yeralma') * S.zoneOk + S.zoneDng * (dMax > 0 ? Math.min(1, dng(q) / dMax) : 0), markedRef);"]] });
const R = []; g.__bz = (mm, dt, free, mark, base, atk, rad, markedRef) => { if (mm !== m || dt !== 0 || mm.tick < +a || mm.tick > +b) return;
  const row = { t: mm.tick, o: {} }; for (const ad of ADS) { const p = free.find(x => x.name === ad); if (!p) { row.o[ad] = '(bölgede değil)'; continue; } const bs = base.get(p), q = mark.get(p);
    const ys = atk.map(o => ({ ad: o.name, d: Math.hypot(o.x - bs.x, o.y - bs.y), r: rad(o), sw: markedRef.has(o) })).sort((u, v) => u.d - v.d).slice(0, 2);
    row.o[ad] = (q ? q.name : '—') + ' · ' + ys.map(y => `${y.ad} ${y.d.toFixed(1)}/${y.r.toFixed(1)}${y.sw ? '(arkada tutuyor)' : ''}`).join(', '); } R.push(row); };
while (m.tick <= +b) adim();
let son = ''; for (const r of R) { const k = ADS.map(a => r.o[a].split(' · ')[0]).join('|'); const ch = k !== son; son = k; if (ch) console.log(`${(r.t / 60).toFixed(2)} ${ADS.map(a => a + ': ' + r.o[a]).join('  ||  ')}`); }
