# Ortak yükleyici: ANALIZ_OUT klasöründeki analiz.jsonl (alan-kos ... analiz) ve kararlar.txt (alan-kos ... kararlar)
import json,math,os
S=os.environ.get('ANALIZ_OUT', os.path.join(os.path.dirname(__file__),'..','out'))
K=[json.loads(l) for l in open(os.path.join(S,'analiz.jsonl'))]
AD=['Aras','Deniz','Mert','Ece','Cansu','Kaan','Tuna','Sur','Burç','Kale','Siper','Hendek','Mancınık','Ok']
ROL=['Bekçi','Bek','Bek','Orta','Orta','Kanat','Uç']*2
TK=[0]*7+[1]*7
def tk(t): return K[t-1]
def d(a,b,c,e): return math.hypot(a-c,b-e)
def sn(t): return f"{int(t/30)//60:02d}:{int(t/30)%60:02d}"
