// İş dağıtıcı: her satırı bir JSON iş olan dosyayı N paralelle çalıştırır. Satır: {"out": "...", "args": [...], "env": {...}}  → node tools/kayit.js <args...> <out>
// Çıktısı zaten var olan (boş olmayan) iş atlanır; yarıda kalan iş yeniden başlatılınca kaldığı yerden devam eder. Kullanım: node tools/paralel.js <işler.jsonl> [N=4]
const fs = require('fs'), { spawn } = require('child_process'); const [f, nS] = process.argv.slice(2), N = +nS || 4;
const isler = fs.readFileSync(f, 'utf8').split('\n').filter(Boolean).map(JSON.parse).filter(j => !(fs.existsSync(j.out) && fs.statSync(j.out).size > 0));
let i = 0, acik = 0; const bas = () => { while (acik < N && i < isler.length) { const j = isler[i++]; acik++; const tmp = j.out + '.yaziliyor';
  const c = spawn('node', ['tools/kayit.js', ...j.args, tmp], { env: { ...process.env, ...(j.env || {}) }, stdio: ['ignore', 'pipe', 'pipe'] }); let o = ''; c.stdout.on('data', d => o += d); c.stderr.on('data', d => o += d);
  c.on('close', code => { acik--; if (code === 0 && fs.existsSync(tmp)) fs.renameSync(tmp, j.out); fs.writeFileSync(j.out.replace(/\.json\.gz$/, '.log'), o); bas(); if (!acik && i >= isler.length) fs.writeFileSync(f + '.BITTI', 'bitti'); }); } };
console.log('iş:', isler.length); if (!isler.length) fs.writeFileSync(f + '.BITTI', 'bitti'); bas();
