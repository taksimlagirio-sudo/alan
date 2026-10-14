# ALAN · zip42 ("zip44") ölçümü

Yöntem: birleşik kayıt, Liman'ın 4 maçı (senaryo 0–3), her ayar ayrı; duruş için ayrıca ev / hedef / gerçek konum izi. Her hücre 4 maç: yön gösterir.

## 1 · Karşı pres: hata düzeldi ✓

| | Olay / maç | Süre / olay | Ort. basan | 3 sn içinde geri kazanma |
|---|---|---|---|---|
| "her yerde, 3 sn" | 8,8 | **135 tik** (zip41: ~3) | 2,1 | %42 |
| "hiç" | 0 | – | – | %34 |
| varsayılan ("rakip yarısında, 2 sn") | 9,0 | 87 tik | 1,8 | %54 |

Pres artık süresine yakın sürüyor, ~2 kişi basıyor. "Hiç"e göre geri kazanma artıyor (%34 → %42). Varsayılanın %54 çıkması gürültüye işaret ediyor (kol başına ~45 kayıp); etkinin büyüklüğü daha çok maçla netleşir.

## 2 · Duruş: hâlâ tutmuyor ⚠ (kayıp nerede, ölçüldü)

Son bölge duruşunda iki kanadı çizgiye çekmek (y 3 / 47). Kanatların **ortadan uzaklığı**, top Liman'da ve son bölgedeyken (4 maç, maçın ilk yarısı):

| Senaryo | Varsayılan ev | Duruşlu ev | Seçilen hedef (tx, ty) | Gerçek konum |
|---|---|---|---|---|
| 0 | 12,6 | 19,1 | 12,4 | **5,4** |
| 1 | 14,7 | 20,1 | 17,0 | 12,1 |
| 2 | 13,1 | 19,0 | 16,1 | **6,7** |
| 3 | 13,5 | 19,4 | 16,3 | 14,1 |
| ortalama | 13,5 | **19,4** | **15,5** | **9,6** |

Ortada (top orta bölgede) dört sütun da ~14: etki doğru bölgede.

**Kayıp iki yerde:**
1. **Ev doğru açılıyor** (13,5 → 19,4; el payı ortalama 0,9). Sorun evde değil.
2. **Aday tartımı kaymanın üçte ikisini geri alıyor:** seçilen hedef evin değil, varsayılanın yakınında (15,5). Evden uzaklığın bedeli hâlâ boşluk ve pas değerine yeniliyor.
3. **Gerçek konum hedeften de içeride (9,6), hatta varsayılan evden de içeride.** Top son bölgedeyken kanatlar çoğu zaman hedeflerine değil içeri, Kuyu önüne gidiyor. Adaylar: son bölgede Kuyu çevresindeki halka adayları (`oppX ± r 11/19`), koşu penceresi, kontra koşucusu listesi ya da kovalama. Bunlar evin ve hedefin üstüne yazıyor.

**Öneri:**
- Elle yeri verilen oyuncu için evden uzaklığın bedeli, kendi bölgesinde boşluk değerini yenecek kadar olsun.
- Kuyu çevresi halka adaylarını ve koşu adaylarını, bu oyuncunun serbestliği ve el payıyla sınırlayın: elle çizgiye konmuş kanat, Kuyu önüne yalnız bir pas onu oraya çağırırsa gitsin.
- Ölçüt aynı: ellenen oyuncunun gerçek konumu kendi bölgesinde evine en az yarı yarıya uysun (bu örnekte ortadan uzaklık ~16'yı geçmeli; şu an 9,6).

## 3 · Geçiş eşiği 5: geçiş arttı, "git" hâlâ yok

| | Geçiş süresi (tik/maç) | "Git" (tik/maç) | Geçişte şut / maç |
|---|---|---|---|
| eşik 4 (varsayılan) | 200 | 0 | 0 |
| eşik 5 | 305 | 3 | 0,25 |

Eşik 5 geçişi %50 uzatıyor ama "git" neredeyse hiç olmuyor. Darboğaz eşik değil, varsayılan kural "önde sayıca üstünsek git": Liman geçişte topun önünde neredeyse hiç sayıca üstün olmuyor. Varsayılanı "eşitsek de git" yapmak ya da sayımı (topun önündeki bizden: taşıyıcı hariç; rakipten: Bekçi hariç) gözden geçirmek gerekebilir. Ölçülebilir.

## 4 · Sürme: örnek yetersiz
Kopyalı ölçüm (aynı an, Sürme 10 ve 14, 12 an × 2 zar): Liman'ın ilk kararları değişmiyor (iki kolda da %63 önüne pas, %25 sür); şut an başına 0,50 → 0,25 (8 deneme). Kararlar aynıysa ters etki "akıl sürmeyi abartıyor" değil, sürmenin kendisinde (ör. topla dönüş hızlanınca taşıyıcı topu daha uzun tutabiliyor, pres ve tempo farkı). Kesin değil; daha çok an gerekir.

## 5 · Hız
Aynı maçın ilk 1500 tiki, tek başına, sırayla: zip40 35,6 ms/tik, zip42 35,5 ms/tik (bulut makinesi yavaş; oran önemli). Fark yok. Design'ın kısa denemesindeki %20 hızlanma burada görünmedi; yavaşlama da yok.
