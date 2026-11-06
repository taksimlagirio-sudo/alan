#!/usr/bin/env python3
"""Derleyicinin ad önerileriyle birebir token yeniden adlandırması (yorum ve metin dışında; MIMARI.md 3.10).
kullanım: adlandir.py <ad_liste.tsv> <esleme_cikti.tsv> [--uygula]
          adlandir.py --hazir sina/ad-esleme.tsv     hazır eşlemeyi uygular (eski adlarla yazılmış bir dalı güncellemek için)
ad_liste.tsv: tür, eski, öneri, dosya, satır (derleyicinin non_snake_case/... uyarılarından; JSON çıktısıyla).
Bir ad yalnız yeni adı hiçbir yerde token olarak geçmiyorsa, serde alanı / girdi JSON anahtarı / biçim dizgisinde
yakalanan ad / wasm dışa verimi değilse değişir: böylece yeniden adlandırma birebir (anlam değişmez)."""
import re,glob,sys,collections
import os
KOK=os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
KM=os.environ.get('ALAN_KM','')  # ek girdi JSON klasörü (sahneler)
DOSYALAR=[f for d in ('motor/src','motor/tests','kos','wasm','katman') for f in glob.glob(f'{KOK}/{d}/**/*.rs',recursive=True) if '/target/' not in f]
KW=set('as break const continue crate else enum extern false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type unsafe use where while async await dyn abstract become box do final macro override priv typeof unsized virtual yield try union gen'.split())
ID=re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
def parcala(s):
    """(tur, bas, son) parçaları: 'kod', 'yorum', 'metin', 'karakter'"""
    i=0;n=len(s);out=[];kb=0
    while i<n:
        c=s[i]
        if s.startswith('//',i):
            j=s.find('\n',i); j=n if j<0 else j
            out.append(('kod',kb,i)); out.append(('yorum',i,j)); i=kb=j; continue
        if s.startswith('/*',i):
            d=1;j=i+2
            while j<n and d:
                if s.startswith('/*',j): d+=1;j+=2
                elif s.startswith('*/',j): d-=1;j+=2
                else: j+=1
            out.append(('kod',kb,i)); out.append(('yorum',i,j)); i=kb=j; continue
        m=re.match(r'b?r(#*)"',s[i:]) if c in 'rb' and (i==0 or not (s[i-1].isalnum() or s[i-1]=='_')) else None
        if m:
            h=m.group(1); j=s.find('"'+h,i+len(m.group(0)))+1+len(h)
            out.append(('kod',kb,i)); out.append(('metin',i,j)); i=kb=j; continue
        if c=='"' or (c=='b' and s.startswith('b"',i) and (i==0 or not (s[i-1].isalnum() or s[i-1]=='_'))):
            j=i+(2 if c=='b' else 1)
            while s[j]!='"':
                j+=2 if s[j]=='\\' else 1
            j+=1
            out.append(('kod',kb,i)); out.append(('metin',i,j)); i=kb=j; continue
        if c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",s[i:])
            if m and not (m.group(0).startswith("'") and re.match(r"'[A-Za-z_][A-Za-z0-9_]*[^']",s[i:]) and not m):
                j=i+len(m.group(0)); out.append(('kod',kb,i)); out.append(('karakter',i,j)); i=kb=j; continue
        if c.isalpha() or c=='_':
            m=ID.match(s,i); i=m.end(); continue
        i+=1
    out.append(('kod',kb,n))
    return [p for p in out if p[1]<p[2]]
def _yap(s,a,b,r):
    out=[];k=a
    for m in ID.finditer(s,a,b):
        out.append(s[k:m.start()]); out.append(r(_M(m,a))); k=m.end()
    out.append(s[k:b]); return out
class _M:
    def __init__(s,m,a): s.m=m; s.a=a
    def start(s): return s.m.start()-s.a
    def group(s,i): return s.m.group(i)
metinler={}
for f in DOSYALAR:
    s=open(f).read(); metinler[f]=(s,parcala(s))
HAZIR=sys.argv[1]=='--hazir'
if HAZIR:
    esle=dict(l.split('\t')[:2] for l in open(sys.argv[2]) if l.strip())
    sys.argv.append('--uygula')
# bütün kod tokenleri
tok=collections.Counter(); bicim=set(); dizgi_tok=set()
for f,(s,pp) in metinler.items():
    for t,a,b in pp:
        if t=='kod':
            for m in ID.finditer(s,a,b):
                if a<m.start() and s[m.start()-1:m.start()]=="'" : continue
                tok[m.group(0)]+=1
        elif t=='metin':
            for m in re.finditer(r'\{([A-Za-z_][A-Za-z0-9_]*)[}:]',s[a:b]): bicim.add(m.group(1))
            for m in ID.finditer(s[a:b]): dizgi_tok.add(m.group(0))
# serde tipleri: derive'ında Serialize/Deserialize olan struct/enum'ların alan ve varyant adları
serde=set()
for f,(s,pp) in metinler.items():
    for m in re.finditer(r'#\[derive\(([^)]*)\)\][^{;]*?(?:struct|enum)\s+(\w+)[^{;]*\{',s):
        if not re.search(r'\b(Serialize|Deserialize)\b',m.group(1)): continue
        d=1;j=m.end()
        while d:
            if s[j]=='{': d+=1
            elif s[j]=='}': d-=1
            j+=1
        govde=s[m.end():j-1]
        for a in re.finditer(r'(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?([A-Za-z_]\w*)\s*[:({,]',govde): serde.add(a.group(1))
        for a in re.finditer(r'(?m)^\s*([A-Za-z_]\w*)\s*,?\s*$',govde): serde.add(a.group(1))
# makroyla kurulan serde tipleri: gövdesinde Serialize/Deserialize geçen macro_rules'ın çağrılarındaki adlar
for f,(s,pp) in metinler.items():
    for m in re.finditer(r'macro_rules!\s*(\w+)',s):
        ad=m.group(1); j=s.find('{',m.end()); d=1; k=j+1
        while d:
            if s[k]=='{': d+=1
            elif s[k]=='}': d-=1
            k+=1
        if not re.search(r'Serialize|Deserialize',s[j:k]): continue
        for f2,(s2,_) in metinler.items():
            for c in re.finditer(re.escape(ad)+r'!\s*[{(\[]',s2):
                d=1; k2=c.end()
                while d:
                    if s2[k2] in '{([': d+=1
                    elif s2[k2] in '})]': d-=1
                    k2+=1
                serde.update(ID.findall(s2[c.end():k2]))
# girdi JSON'larının anahtarları (sahne, veri)
import json as _j
def _yuru(o):
    if isinstance(o,dict):
        for k,v in o.items(): serde.add(k); _yuru(v)
    elif isinstance(o,list):
        for v in o: _yuru(v)
for jf in glob.glob(KOK+'/**/*.json',recursive=True)+(glob.glob(KM+'/*.json') if KM else []):
    if '/target/' in jf: continue
    try: _yuru(_j.load(open(jf)))
    except Exception: pass
# wasm'ın dışa verdikleri
dis=set(re.findall(r'extern\s+"C"\s+fn\s+(\w+)',''.join(s for f,(s,_) in metinler.items())))
if not HAZIR:
    # eşleme
    oner=collections.defaultdict(set); tur=collections.defaultdict(set)
    for l in open(sys.argv[1]):
        k,o,y,_,_=l.rstrip('\n').split('\t'); oner[o].add(y); tur[o].add(k)
    ELLE_HARIC={'KOP','UA','engage','oyala'}  # aynı token başka türde de (sabit/alan, tip/alan) kullanılıyor
    esle={};neden=collections.Counter();atla=[]
    for o,ys in oner.items():
        if len(ys)!=1: neden['farklı öneriler']+=1; atla.append((o,'farklı öneriler',','.join(ys)));continue
        y=next(iter(ys))
        r=None
        if o in ELLE_HARIC: r='başka türde de kullanılıyor'
        elif o in serde: r='serde alanı'
        elif o in bicim: r='biçim dizgisinde'
        elif o in dis: r='wasm dışa verimi'
        elif y in KW or y in tok: r='yeni ad zaten var'
        elif o in dizgi_tok and False: r='dizgide'
        if r: neden[r]+=1; atla.append((o,r,y)); continue
        esle[o]=y
    # aynı yeni ada iki eski
    say=collections.Counter(esle.values())
    for o,y in list(esle.items()):
        if say[y]>1: del esle[o]; neden['çakışan yeni ad']+=1; atla.append((o,'çakışan yeni ad',y))
    print('eski token:',len(oner),'uygulanacak:',len(esle),dict(neden))
    with open(sys.argv[2],'w') as f:
        for o,y in sorted(esle.items()): f.write(f'{o}\t{y}\t{",".join(sorted(tur[o]))}\n')
    with open(sys.argv[2]+'.atla','w') as f:
        for a in sorted(atla): f.write('\t'.join(a)+'\n')
if '--uygula' in sys.argv:
    deg=0
    for f,(s,pp) in metinler.items():
        parca=[]
        for t,a,b in pp:
            if t!='kod': parca.append(s[a:b]); continue
            def r(m):
                k=a+m.start()
                if k>0 and (s[k-1] in "'#" or s[k-1].isdigit()): return m.group(0)
                return esle.get(m.group(0),m.group(0))
            parca.append(ID.sub(r,s,0) if False else ''.join(_yap(s,a,b,r)))
        t=''.join(parca)
        if t!=s: open(f,'w').write(t); deg+=1
    print('değişen dosya:',deg)
