const L=require("fs").readFileSync(process.argv[2],"utf8").split("\n").filter(l=>l.includes("ST "));const S={};
for(const l of L){const m=/ST (\d) (\d+) (\{.*\}) @/.exec(l); (S[m[2]]=S[m[2]]||{})[m[1]]=JSON.parse(m[3]);}
const fl=(x,p,o)=>{ if(x&&typeof x==='object'){ for(const k of Object.keys(x)) fl(x[k],p+'.'+k,o);} else o[p]=x; return o; };
for(const t of Object.keys(S).sort()){const a=S[t][0],b=S[t][1]; if(!a||!b)continue; const A=fl(a,'',{}),B=fl(b,'',{}); const f=[]; for(const k of new Set([...Object.keys(A),...Object.keys(B)])) if(JSON.stringify(A[k])!==JSON.stringify(B[k])) f.push(k+': '+JSON.stringify(A[k])+' | '+JSON.stringify(B[k]));
 console.log('tik',t,'fark',f.length); console.log(f.filter(k=>!/\._(otH|otF|kalH|otA)\./.test(k)).slice(0,+process.argv[3]||30).join('\n'));}
