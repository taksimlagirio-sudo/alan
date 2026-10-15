const {oku}=require(process.argv[2]+"/tools/kayit-oku.js");const J={},H={};
for(const f of process.argv.slice(3)){const k=oku(f);const N=k.N;const X=(t,i)=>k.P.x[t*N+i]/100,Y=(t,i)=>k.P.y[t*N+i]/100;
 for(let t=6;t<k.n;t+=6){const h=k.T.hold[t];if(h===255||k.oyuncular[h].takim!==0||k.oyuncular[h].rol==="Bekçi")continue;const lim=k.oyuncular.map((o,i)=>i).filter(i=>k.oyuncular[i].takim===0&&k.oyuncular[i].rol!=="Bekçi"&&i!==h);
  for(const i of lim){const ad=k.oyuncular[i].ad;if(ad!=="Ece"&&ad!=="Cansu"&&ad!=="Kaan")continue;const l=k.lane[(t/6)*N+i],d=Math.hypot(X(t,i)-X(t,h),Y(t,i)-Y(t,h));if(!(d<30&&l<50))continue;
   const g=(k.gorevler[k.P.job[t*N+i]]||"").replace(/ · tepki/g,"");J[ad+" · "+g]=(J[ad+" · "+g]||0)+1;
   const v=Math.hypot(k.P.vx[t*N+i],k.P.vy[t*N+i])/1000;const tg=Math.hypot(k.P.tx[t*N+i]/100-X(t,i),k.P.ty[t*N+i]/100-Y(t,i));const key=ad;const c=H[key]=H[key]||{n:0,dur:0,vardi:0};c.n++;if(v<.05)c.dur++;if(tg<1.5)c.vardi++;}}}
for(const [kk,v] of Object.entries(J).sort((a,b)=>b[1]-a[1]).slice(0,10))console.log(v,kk);
for(const [a,c] of Object.entries(H))console.log(a,"yakın-kapalı an",c.n,"· duruyor %"+(100*c.dur/c.n).toFixed(0),"· hedefine varmış (1,5 br) %"+(100*c.vardi/c.n).toFixed(0));
