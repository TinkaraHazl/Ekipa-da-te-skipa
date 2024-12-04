# Projektna naloga pri predmetu Programiranje 2 na FMF
## Delovanje projekta
Projekt implementira generator različnih zaporedij, predstavljenih spodaj, in strežnik, preko katerega lahko zahtevamo zaporedja.

## Zagon projekta
Projekt poženemo tako, da v terminalu najprej zaženemo register z ukazom `cargo run`. Nato v ločenem terminalui zaženemo generator z ukazom `cargo run -- -- IP_REGISTRA IP_GENERATORJA PORT`, kjer so privzete vrednosti `IP_REGISTRA=0.0.0.0`, `IP_GENERATORJA=127.0.0.1` in `PORT=9000`.
Za poizvedbo uporabimo Python program, ki sprejme poizvedbo za zaporedje v obliki JSON:
```python
import requests
def poizvedba
    body = {
            "range": {
                "from": _,
                "to": _,
                "step": _
            },
            "parameters": seznam parametrov,
            "sequences": seznam zaporedij oblike [{"name": _, "parameters": seznam parametrov, "sequences": seznam zaporedij}],
            }
      response = requests.post("URL/sequence/ImeZaporedja", json=body)
poizvedba()
```
Primer poizvedbe je prikazan v Python datoteki `test.py`.

## Implementirana zaporedja
- **Konstantno**
  vrne konstantno zaporedje.
  - Število parametrov: 1
  - Število zaporedij: 0
  
- **Arithmetično** vrne aritmetično zaporedje, kjer je prvi parameter začetni člen in drugi diferenca.
  - Število parametrov: 2
  - Število zaporedij: 0
- **Geometrijsko**
  vrne geometrijsko zaporedje, kjer prvi parameter določa začetni člen in drugi kvocient.
  - Število parametrov: 2
  - Število zaporedij: 0
- **Tribonacci**
   vrne zaporedje, kjer je vsak naslednji člen vsota prejšnjih treh. Parametri določajo prve tri začetne člene.
  - Število parametrov: 3
  - Število zaporedij: 0
- **Vsota dveh zaporedij**
  vrne zaporedje, kjer je i-ti člen vsota i-tih členov podanih zaporedij.
  - Število parametrov: 0
  - Število zaporedij: 2
- **Produkt dveh zaporedij**
  vrne zaporedje, kjer je i-ti člen produkt i-tih členov podanih zaporedij.
  - Število parametrov: 0
  - Število zaporedij: 2
- **Mix**
 vrne zaporedje, ki izmenjuje elemente prvega in drugega zaporedja. Prvi parameter poda, koliko členov prvega zaporedja je vključenih, preden preklopimo na elemente drugega zaporedja, drugi pa,  koliko členov drugega zaporedja bo vključenih, preden preklopimo na elemente prvega.
  - Število parametrov: 2
  - Število zaporedij: 2
- **Drop**
  vrne zaporedje, kjer je prvih k členov zaporedja izpuščenih, k določa parameter.
  - Število parametrov: 1
  - Število zaporedij: 1
- **Lahova števila**
  vrne zaporedje Lahovih števil $L(n, k)$, kjer je k podan parameter, n se pa veča s členom.
  - Število parametrov: 1
  - Število zaporedij: 0
- **Catalanova števila**
  vrne zaporedje Catalanovih števil.
  - Število parametrov: 0
  - Število zaporedij: 0
- **Sprememba baze zaporedja**
   spremeni številski sistem danega zaporedja, ki je podan s prvim parametrom, v številski sistem, ki je podan z drugim parametrom.
  - Število parametrov: 2
  - Število zaporedij: 1
- **Alikvotno zaporedje**
   vrne zaporedje, kjer je vsak naslednji člen vsota pravih deliteljev prejšnjega člena. Prvi člen je podan s parametrom.
  - Število parametrov: 1
  - Število zaporedij: 0
