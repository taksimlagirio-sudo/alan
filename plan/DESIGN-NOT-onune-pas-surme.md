# ALAN · Önüne pas ve topla koşma (kullanıcı gözlemi)

Claude Code'dan Design'a.

**Kullanıcı:** "Gerçekten güzel atılan önüne pas yok; ya seker ya farklı bir açıya gider. Oyuncular topu sürerek kontrol etmiyor; hep bir duruyor ya da tek pas – tek şut. Koşarken topu alıp koşmaya devam etme olmuyor."

**Kaynak:** zip61, 16 maç (karşılaştırma zip50 ve zip58: aynı tablo; yani yeni bir bozulma değil, baştan böyle). Araç: `tools/alis.js`.

## 1 · Önüne pas: yönü doğru, zamanlaması değil
| | Önüne | Ayağa (pas) | Kenardan |
|---|---|---|---|
| Pas sayısı (16 maç) | 1147 | 311 | 309 |
| Alıcı aldı | %84 | %90 | %72 |
| Arada sekme / çarpma | %0 | %0 | %1 |
| Alındığı yer ile hedef (`to`) arası | 8,2 | 4,7 | 15,1 |

Önüne pasta alış noktası, hedefe göre:
- **Yana sapma küçük:** ort. 1,4 birim; 3'ten çok yana kayan yalnız %9.
- **İleri / geri sapma büyük:** %44'ünde top hedeften 3+ birim **önce** alınıyor, %34'ünde 3+ birim **ötede**. Hedef noktasında (±3) alınan yalnız ~%22.
- Pas süresi ort. 36 tik.

Yani "sekme" değil, **buluşma**: alıcı ya hedefe erken varıp bekliyor / geri geliyor, ya da top önünden geçip gidiyor ve alıcı peşinden koşuyor. Kullanıcının "farklı açıya gidiyor" dediği bu.

**Muhtemel sebep:** Önüne pasın hedefi bir **nokta**; alıcı o noktaya koşuyor (orada duruyor), top da o noktaya gönderiliyor. İkisinin aynı anda varması ayrı ayrı hesaplanıyor (alıcının varış süresi, topun varış süresi) ve tutmuyor.

**Öneri (tek kural):** Önüne pas bir **buluşma** olsun: hedef, alıcının **koşu yolu üzerinde** topla aynı tikte vardığı nokta (alıcının bugünkü hızı ve yönüyle). Alıcı o noktada durmaz, koşusunu sürdürür; topu koşarken alır. Pası veren bu noktayı seçer; akıl aynı hesabı yapar. Görüş ve Pas bu hesabın doğruluğunu belirleyebilir (gizli katsayı değil: "iyi pasçı buluşmayı tutturur").

## 2 · Koşarak alınca duruyor
Önüne pasta alıcının hızı (birim / tik):
| Alış anı | 10 tik sonra | 30 tik sonra |
|---|---|---|
| 0,32 | **0,17** | 0,26 |

- Alıcıların %92'si topu koşarak alıyor, ama hız 10 tikte yarıya iniyor; **%32'si ilk 20 tikte duruyor**. Koşarak alıp koşmaya devam eden yalnız %53.
- İlk karar ort. **16 tik** sonra geliyor (koşarak alınan 877 önüne pas). İlk karar: sür %26 · fake %25 · tut %14 · 40 tik içinde karar yok %35 (hemen pas / şut).
- Taşıyıcı (Bekçi hariç) topla: koşuyor (> 0,15) %57, yavaş %23, duruyor %20. Ortanca hız topla 0,19, topsuz 0,29 (en yüksek ikisinde de 0,36: topla koşmanın kendisinde sınır yok, oyuncu yavaşlıyor).

**Muhtemel sebepler:**
1. Alış anından ilk karara kadar (16 tik) taşıyıcının bir hedefi yok: zip56'dan beri topu alanın koşu planı siliniyor, yeni plan ilk kararla geliyor. Arada yavaşlıyor.
2. İlk kararda fake (%25) ve tut (%14) yüksek: ikisi de durmak demek. Kontra / boş alan varken "sür" daha değerli olmalı.

**Öneri:**
- **Alışta koşu sürsün:** Topu koşarak alan oyuncu ilk kararına kadar geliş yönünde koşmaya devam etsin, topu önüne alarak (ilk dokunuş koşu yönüne; Kontrol ne kadar iyi alacağını belirler). Plan silinmesin, ilk kararla değişsin.
- **İlk karar daha erken:** Koşarak alınan topta ilk karar alış tikinde (ya da 2–3 tik içinde) gelsin; bugünkü 16 tik bir hamle boyu.
- Sürmenin değeri: önünde boş alan olan taşıyıcıda "sür" ile "fake / tut" farkı açılmalı; bunu kalibrasyonla ölçebilirim (sürdükten sonra aklın beklediği ile olan).

## 3 · Kontrol tablosunda izlenecek (yeni satırlar)
- Önüne pasın hedefin ±3 içinde alınma payı ↑ (bugün ~%22).
- Koşarak alıp koşmaya devam eden ↑ (bugün %53); ilk 20 tikte duran ↓ (%32).
- Alıştan ilk karara kadar tik ↓ (16).
- Taşıyıcı topla koşuyor payı ↑ (%57).
- Sayı, pas tutma ve top kaybı kötüleşmesin.
