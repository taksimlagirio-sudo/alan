// JS motorundan maç izi (her tik bir satır): istatistik.js hem bunu hem Rust motorunun (kos iz) çıktısını okur.
// Biçim (JSON dizi): [tik, taşıyıcı (m.ps sırası, yoksa -1), skor0, skor1, yeni uçuş ? [tür, atan, hedef] : 0, şut olayı 0/1, bar ort. 0, bar ort. 1]
// node sina/iz-js.js ../alan-v5 <kayit.json> <tik> > iz.jsonl
const path = require('path'), fs = require('fs');
const { M, kurMac } = require('./yukle')(path.resolve(process.argv[2]));
const m = M.kafaAc(kurMac(JSON.parse(fs.readFileSync(process.argv[3])).yeni)), N = +process.argv[4];
let sonFl = null, e0 = m.events[0]; const out = [];
const ort = t => { let s = 0, n = 0; for (const p of m.ps) if (p.team === t && p.en != null) { s += p.en; n++; } return n ? +(s / n).toFixed(4) : 1; };
while (m.tick < N) {
  const h0 = m.holder; M.step(m);
  let sut = 0; for (const e of m.events) { if (e === e0) break; if (e.type === 'gonder') sut = 1; } e0 = m.events[0];
  const fl = m.fl; let yeni = 0;
  if (fl && fl !== sonFl && fl.kind) yeni = [fl.kind, h0 ? m.ps.indexOf(h0) : -1, fl.q ? m.ps.indexOf(fl.q) : -1];
  sonFl = fl;
  out.push(JSON.stringify([m.tick, m.holder ? m.ps.indexOf(m.holder) : -1, m.score[0], m.score[1], yeni, sut, ort(0), ort(1)]));
}
process.stdout.write(out.join('\n') + '\n');
