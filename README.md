# ADT Bootstrap

## Compile


```Shell
cargo build
```
or

```Shell
cargo build --release
```

## Run
```Shell
cd ./cursive_cli/
cargo run
```

## Manual

Celý tento projekt funguje jako svazek řešení vybraných problémů se
zaměřením na haldy a stromy. 

Po spuštění se nám zobrazí hlavní menu se všemi vybranými problémy pro haldy a stromy.

## Halda

Aby struktura splňovala vlastnosti haldy, kde rodič ma větší hodnotu než jeho potomci, což znamená, že musí mít v kořeni prvek s nejvyšší hodnotou v z celé struktury (Max Heap). Opakem, kde je vždy rodič menší než potomek nazýváme Min Heap

### Heap Sort

Po spuštění se nám zobrazí dialogové okno. Aby nám program fungoval tak jak má, musíme dodržet následující formát vstupních parametrů:

Vstup:

*value1*;*value2*;*value3*;*value4*;....

Kde *valueX* je jakékoliv číslo. 

Následně se nám opět zobrazí dialogové okno kde budeme vidět náš vstup a seřazený výstup.

### Huffmanovo kódování

Huffmanovo kódování funguje na principu najít duplicitní znaky a následně jim přiřadit v závislosti na frekvenci ve vstupním řetězci kód. Poté vytvoří podle každého znaku ve vstupním řetězci jeho zašifrovanou kopii. 

## Binární strom

### Procházení binárního stromu

Vstup:

*value1*;*value2*;*value3*;*value4*;....

---

Následně se nám vypíše výstup procházení stromu.

POST ORDER - prochází uzly nejprve v levém podstromu, poté pravém podstromu, a nakonec rodičovský uzel. 

IN ORDER - od nejmenšího po největší

PRE ORDER - funguje na podobném principu jako algoritmus „prohledávání do hloubky“. Začne v kořeni stromu a postupuje dále do levého podstromu. Až s ním skončí přesune se na pravý podstrom.

---

### Sloučení 2 binárních stromů

Vstup:

*value1*;*value2*;*value3*;*value4*;....

*valueA*;*valueB*;*valueC*;*valueD*;....

---

Princip jak funguje sloučení 2 binárních stromů, je že pokud se na stejné pozici ve stromě A a B nachází nějaký prvek, jejich hodnoty se sečtou a přidají do nového stromu C. Avšak pokud se ve stromě A nachází uzel, který není na stejné pozici ve stromě B, přidá se jako nový uzel do stromu C.
