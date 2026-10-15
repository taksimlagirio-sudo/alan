const {oku}=require(process.argv[2]+"/tools/kayit-oku.js");
const R={};for(const f of process.argv.slice(3)){const k=oku(f);const N=k.N;const X=(t,i)=>k.P.x[t*N+i]/100,Y=(t,i)=>k.P.y[t*N+i]/100;
 for(const T of [0,1]){const lim=k.oyuncular.map((o,i)=>i).filter(i=>k.oyuncular[i].takim===T&&k.oyuncular[i].rol!=="Bekçi");
 for(let t=0;t<k.n-120;t+=6){const h=k.T.hold[t];if(h===255||k.oyuncular[h].takim!==T||k.oyuncular[h].rol==="Bekçi")continue;let n=0;for(const i of lim){if(i===h)continue;const l=k.lane[(t/6)*N+i];const d=Math.hypot(X(t,i)-X(t,h),Y(t,i)-Y(t,h));if(l!==255&&l>=50&&d<30)n++}
  let kay=0;for(let u=t+1;u<=t+120;u++){const q=k.T.hold[u];if(q!==255&&k.oyuncular[q].takim!==T){kay=1;break}}const b=Math.min(n,3);const r=R[b]=R[b]||[0,0];r[0]++;r[1]+=kay;}}}
for(const [b,r] of Object.entries(R))console.log("açık seçenek",b==="3"?"3+":b,"· an",r[0],"· 2 sn içinde top rakibe geçti %"+(100*r[1]/r[0]).toFixed(0));
