# Markajcı Kuyu'da süresini harcayıp şut anında çıkıyor (217468, hepsi açık, 15fc7dd-0f739b9-0f15c40)
- 5049 golü: Mert, Mancınık'ı 6 sn 2–3 br'den tutuyor (S7k/S7j), 5009'da kale dibinde (2,29); 5049 şut anında "Kuyu alanından çıkıyor" → Mancınık 15 br boş, gol.
- 2034 golü: Cansu, Mancınık'ı 2 br'den tutuyor; 1954'te "Kuyu alanından çıkıyor", bırakıyor; 2014'te Deniz de çıkıyor → gol.
- Liman "Kuyu alanından çıkıyor" tik: hiz2 yok 130, hiz2 ilk 283, bu tur 387. Markajcının kale dibinde (x<8, y 20–40) tik: 197 / 21 / 327.
Neden: S7j (adamla kale arası 1.6 br) ve S7k (adamı bırakma) hücumcu Kuyu kenarında dururken markajcıyı Kuyu'nun içine koyuyor;
Kuyu süresi (mac/kuyu.rs, Q.kuyu_sn) tehlike yokken harcanıyor, şut anında zorunlu çıkış. Markaj yeri Kuyu süresini hesaba katmıyor:
şut yakın değilken Kuyu sınırında beklemek, süreyi asıl ana saklamak gerekir (kuyu_girer markaj hedefinde bu değeri tartmıyor).
