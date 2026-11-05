// Maç izlerinden istatistik (JS ve Rust motoru için aynı ölçü): node sina/istatistik.js iz1.jsonl [iz2.jsonl …]
// Pas: atandan arkadaşa uçuş; sonuç ilk tutanla (hedef / başka arkadaş / rakip). Kayıp: sahibin takımı değişti.
const fs = require('fs');
const toplam = { mac: 0, gol: [0, 0], sut: 0, pas: 0, hedef: 0, arkadas: 0, rakip: 0, kayip: 0, sahip: [0, 0], tik: 0, bar: 0 };
for (const f of process.argv.slice(2)) {
  const L = fs.readFileSync(f, 'utf8').trim().split('\n').map(l => JSON.parse(l));
  toplam.mac++; let acik = null, sonT = null, bar = 0, gol = [0, 0];
  const takim = i => i < 0 ? null : (i < 7 ? 0 : 1);
  for (const [t, h, s0, s1, yeni, sut, e0, e1] of L) {
    toplam.tik++; bar += (e0 + e1) / 2; toplam.sut += sut;
    if (yeni && yeni[0] !== 'gönder' && yeni[1] >= 0 && yeni[2] >= 0 && yeni[2] !== yeni[1]) { if (acik) toplam.pas--; acik = { a: yeni[1], q: yeni[2] }; toplam.pas++; }
    if (h >= 0) {
      const ht = takim(h); toplam.sahip[ht]++;
      if (acik) { if (h === acik.q) toplam.hedef++; else if (ht === takim(acik.a)) toplam.arkadas++; else toplam.rakip++; acik = null; }
      if (sonT != null && sonT !== ht) toplam.kayip++; sonT = ht;
    }
    gol = [s0, s1];
  }
  toplam.bar += bar / L.length; toplam.gol[0] += gol[0]; toplam.gol[1] += gol[1];
}
const n = toplam.mac, p = toplam.pas || 1, s = toplam.sahip[0] + toplam.sahip[1] || 1;
console.log(JSON.stringify({ mac: n, tik: toplam.tik, gol: toplam.gol[0] + toplam.gol[1], gol_mac: +((toplam.gol[0] + toplam.gol[1]) / n).toFixed(2), pas: toplam.pas, hedefe: +(100 * toplam.hedef / p).toFixed(1), rakibe: +(100 * toplam.rakip / p).toFixed(1), sut: toplam.sut, kayip_dk: +(toplam.kayip / (toplam.tik / 1800)).toFixed(2), pas_dk: +(toplam.pas / (toplam.tik / 1800)).toFixed(2), sahiplik0: +(100 * toplam.sahip[0] / s).toFixed(1), bar_ort: +(toplam.bar / n).toFixed(3) }));
