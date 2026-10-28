const { chromium } = require('/opt/node22/lib/node_modules/playwright');
(async () => { const b = await chromium.launch({ executablePath: '/opt/pw-browsers/chromium-1194/chrome-linux/chrome', args: ['--no-sandbox'] }).catch(async e => { console.log('başlatma', e.message.slice(0, 200)); return chromium.launch({ args: ['--no-sandbox'] }); });
  const p = await b.newPage(); const L = [];
  p.on('console', m => { const s = m.text(); if (/ALAN|HATA|paralel|BİTTİ|PD|İZ/.test(s)) console.log(new Date().toISOString().slice(11, 19), s.slice(0, 400)); });
  p.on('worker', w => { console.log('işçi açıldı', w.url().split('/').pop()); w.on('console', m => console.log(new Date().toISOString().slice(11, 19), '[' + w.url().split('?')[1] + ']', m.type(), m.text().slice(0, 500))); w.on('close', () => console.log(new Date().toISOString().slice(11, 19), 'işçi kapandı', w.url().split('?')[1])); });
  p.on('pageerror', e => console.log('SAYFA HATASI', e.message));
  await p.goto('http://localhost:8137/test.html?paralel=' + (process.env.N || 2) + '&tik=' + (process.env.TIK || 9000) + '&ek=' + encodeURIComponent(process.env.EK || ''));
  const t0 = Date.now(); while (Date.now() - t0 < (+process.env.SURE || 3000) * 1000) { const d = await p.evaluate(() => window.__durum).catch(() => null); if (d && (d.bitti || d.hata)) break; await new Promise(r => setTimeout(r, 5000)); }
  const d = await p.evaluate(() => window.__durum); console.log('SONUÇ', JSON.stringify({ bitti: d.bitti, hata: d.hata, bosluk: d.bosluk.filter(x => x.sn > 5), pd: d.pd }).slice(0, 3000)); require('fs').writeFileSync(process.env.HF || '/dev/null', d.H.join('\n')); require('fs').writeFileSync(process.env.WF || '/dev/null', (d.WN || []).join('\n')); await b.close(); })();
