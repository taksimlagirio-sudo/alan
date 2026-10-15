const {oku}=require(process.argv[2]+"/tools/kayit-oku.js");
const C={yakinKapali:0,uzakAcik:0,uzakKapali:0,arkadas:0},CA={};let an=0;for(const f of process.argv.slice(3)){const k=oku(f);const N=k.N;const X=(t,i)=>k.P.x[t*N+i]/100,Y=(t,i)=>k.P.y[t*N+i]/100;
 for(let t=0;t<k.n;t+=6){const h=k.T.hold[t];if(h===255||k.oyuncular[h].takim!==0||k.oyuncular[h].rol==="Bekçi")continue;const lim=k.oyuncular.map((o,i)=>i).filter(i=>k.oyuncular[i].takim===0&&k.oyuncular[i].rol!=="Bekçi"&&i!==h);
  const ok=lim.filter(i=>{const l=k.lane[(t/6)*N+i];return l!==255&&l>=50&&Math.hypot(X(t,i)-X(t,h),Y(t,i)-Y(t,h))<30});if(ok.length)continue;an++;
  for(const i of lim){const l=k.lane[(t/6)*N+i],d=Math.hypot(X(t,i)-X(t,h),Y(t,i)-Y(t,h));C.arkadas++;if(d<30)C.yakinKapali++;else if(l>=50)C.uzakAcik++;else C.uzakKapali++;
   const ad=k.oyuncular[i].ad;const c=CA[ad]=CA[ad]||{yk:0,ua:0,uk:0};if(d<30)c.yk++;else if(l>=50)c.ua++;else c.uk++;}}}
console.log("Liman seçeneksiz an",an,"· arkadaşlar: yakın (<30) ama yolu kapalı %"+(100*C.yakinKapali/C.arkadas).toFixed(0)+" · uzak ve yolu açık %"+(100*C.uzakAcik/C.arkadas).toFixed(0)+" · uzak ve kapalı %"+(100*C.uzakKapali/C.arkadas).toFixed(0));
for(const [a,c] of Object.entries(CA)){const s=c.yk+c.ua+c.uk;console.log("  ",a.padEnd(6),"yakın-kapalı %"+(100*c.yk/s).toFixed(0),"uzak-açık %"+(100*c.ua/s).toFixed(0),"uzak-kapalı %"+(100*c.uk/s).toFixed(0))}
