## Bevezetés

##### Az ergonomikus billentyűzetek világában sok „normál” billentyűzetek ismeretében

##### furcsának tűnő megoldás van. Általában egy billentyűzeten a gombok nem egy

##### oszlopban, hanem egy kis eltolással találhatóak meg, viszont ha a billentyűket egymás

##### alá helyezzük sokkal rövidebb úton érhetőek el a gombok, ezzel csökken az újnak a

##### fáradása. Hasonlóan érdekes megoldás a billentyűzet „kettőbe vágása”, ez is a

##### kényelmet segíti, a természetes csuklópozíció megtartásával. A feladat egy ilyen

##### billentyűzet megtervezése.

##### A tervezés első fázisa a kiosztás megtervezése. Minél kevesebb gombunk van annál

##### kényelmesebb lesz gépelni, viszont időbe telhet a kiosztás megszokása. Ebben a

##### projektben egy 36 gombos kiosztás lett megvalósítva, ami pont arra elég, hogy az angol

##### ABC betűi elférjenek rajta és a végén maradjon még 1-2 gomb. Jogosan felmerülhet,

##### hogy hogyan érjük el a többi karaktert, például a számokat? Erre rétegeket

##### használhatunk, amiket erre dedikált gombokkal érhetünk el, ezek lenyomásval a

##### billentyű másik karaktert jelez a cél eszközön, hasonlóan mintha egy módosító gombot


##### használnánk. Felmerülhet még az is, hogy a módosító gombokat hogyan érjük el.

##### Többféle megoldás létezik, a legegyszerűbb talán a gombok lenyomásának idejét

##### figyelni, és a hosszan lenyomott gombokat egy adott pozícióban módosító gombként

##### kezelni.

## PCB Tervezés

### Alkatrészek

##### Fontos meghatározni az alkatrészeket mielőtt nekiállunk a PCB tervezésének, hogy

##### konkrét footprinteket (az alkatrészek lábainak az elhelyezkedése a modellben) tudjunk

##### használni. A legfontosabb alkatrészek a billentyűkapcsolók, hogy regisztrálni tudjuk a

##### billentyűlenyomásokat, a mikrokontroller, ami feldolgozza az alkatrészekből érkező

##### jeleket. Kevésbé fontos alkatrészek az enkóderek és a kijelzők.

##### Mikrokontroller választásnál két fontos szempontom volt, legyen elég GPIO lába, legyen

##### Bluetooth támogatása. Végül az nrf5284 0 - re esett a választás, ami megfelelt ezeknek a

##### követelményeknek.

##### Billentyű kapcsolókat kétféle módon tudunk beszerelni az áramkörünkbe. Vagy direktbe

##### beforrasztjuk őket, vagy használunk úgynevezett „hotswap socket”eket, amik

##### egyszerűbbé teszik a billentyűk cseréit.

### Billentyűmátrix

##### Legtöbb esetben a mikrontrollerünknek kevesebb GPIOja van mint ahány gombja van a

##### billentyűzetünknek. Ez elsőre problémának tűnik, viszont van rá egy egyszerű megoldás.

##### A kapcsolókat egy mátrixba kötjük, az alábbi módon.

###### ábra 1 :Billentyűmátrix


##### Így a kontrollernek csak annyi lába kell hogy legyen, mint ahány oszlopunk és sorunk

##### van. Az oszlopokon küldünk egy aktív jelet, amit a sorokon kiolvasunk, vagy fordítva,

##### ezzel megállapíthatjuk, hogy melyik gomb lett lenyomva, mivel az zárja az áramkört.

##### Viszont így felmerül az úgynevezett „ghosting” problémája.

###### ábra 2 : Ghosting

##### Ahogy a képen is látszik a probléma, ha a képen látható három gomb le van nyomva,

##### akkor a kontroller a C3 gombot is regisztrálja, pedig az nincsen lenyomva.

##### Erre megoldás, ha diódákat rakunk az áramkörünkbe a gombok és a sorok köze, feltéve,

##### hogy az oszlopokon küldjuk a kimenete jelen és a sorokon várjuk a bemenetit. Ekkor a

##### kép szerinti C2 gombból nem folyik áram a C3 gombba, így a 3as bemeneten nem

##### figyelünk meg jelet.


### Billentyű elosztás

##### A billentyűk elosztásának megtervezéséhez egy Ergogen nevű projektet használtam, ami

##### egy magyar fejelsztő projektje. A program egy yaml fájlból generál KiCad fájlokat, de akár

##### házat is tervezhetünk vele a billentyűzetünknek. Tervezés közben megadhatjuk a

##### gombok pozícióját, egymáshoz relatívan, a perifériák elhelyezkedését és ami a

##### legfontosabb megtudjuk adni az oszlopokhoz rendelt lábakat a mikrokontrolleren.

###### ábra 3 : Ergogen kimenet

### PCB tervezés

##### Az ergogenben elkészítettek terveket importálva kapunk egy kezdetleges NYÁK tervet,

##### ahol az összeköttetéseket és a végső elhelyezéseket fejezhetjük be.

##### A tervezés közben próbáltam figyelni rá, hogy a terv forgatható legyen, tehát mindkét

##### oldala ugyanúgy működjön, ne kelljen kettő féle prototípust legyártani. Amikor kézhez

##### kaptam akkor viszont észrevettem, hogy ez nem sikerült teljesen. Az enkóder és a kijelző

##### nem működik a megforgatott oldalon, ezt egy későbbi verzióban javítani tervezem. Ez

##### megoldható úgy, hogy a hozzájuk vezető kapcsolatot nem kötjük végig, hagyunk neki

##### kettő utat amit az összeszerelésnél összekötünk ónnal.


## Firmware

##### A firmwarrel kapcsolatos elvárások elég sztenderdek, képesnek kell lennie a perifériák

##### kezelésére, a kommunikáció helyes megvalósítására. Ahhoz hogy ezek szinkron

##### tudjanak működni, valamilyen asszinkron megoldásra lenne szükség. Példának a ZMK

##### nevű billentyűzet firmwaret használtam, ami ezt a problémát a Zephyr RTOS

##### használatával oldja meg.

##### A megvalósítás nyelvére a C/C++ lett volna az első logikus választás, viszont érdekelt,

##### hogy milyen más lehetőség vannak és hogyan lehet ezeket kihasználni. Egy ideje már ki

##### akartam próbálni, milyen Rustban beágyazott fejleszteni, a biztonsági funkciói miatt, így

##### gondoltam a projekt keretein belül megismerkedek vele.

### Rust

##### A Rust egy modern programozási nyelv, ami egyre nagyobb teret kezd hódítani magának,

##### egyre több cég, köztük a Google is kezdi preferálni C++ helyett, a biztonsági funkciói és a

##### fejlesztés ideje miatt, ami elméletben töredéke az eddig használt nyelveknek.

#### Borrow Checker és előnyei

##### A legtöbb memóriabiztos nyelv Garbage Collector segítségével oldja meg a memória

##### címek kezelését, viszont a Rust egy új megközelítést alkalmaz. Ezt Borrow Checkernek

##### nevezik, ami futás idő helyett már fordítás közben szigorú szabályokat ad meg a

##### fejlesztőnek, a kóddal kapcsolatban. A fordítás ennek következtében lassabb lesz

##### viszont a kész programunk sokkal kisebb helyet foglal, és gyorsabban is lefut, mintha

##### futás időben kezelnénk a memória hibákat.

###### ábra 5 : PCB render ábra 4 : Összeszerelt billentyűzet


##### Három fő koncepcióra bontható a működése

- Tulajdonlás

##### Minden objektumnak egyszerre csak egy tulajdonosa lehet, mint az alábbi példában

##### is látszik miután s2 átveszi a tulajdont s1 felett azután nem érhetjük el az

##### objektumot s1-en keresztül.

1. fn main() {
2. let s1 = String::from("Hello");
3. let s2 = s1; // s1 tulajdonjoga átkerül s2-höz, s1 többé nem érvényes
4.
5. // println!("{}", s1); // Ez a sor hibaüzenetet eredményezne, mert s1 már nem érvényes
6. println!("{}", s2); // s2 érvényes és használható
7. }
    - Kölcsönzés

##### Minden objektum kölcsönözhető, viszont egyszerre csak egy helyen, ha azt meg akarjuk

##### változtatni, mutábilisan (írható) akarjuk átadni. Viszont ha immutábilisan (csak

##### olvasható) akarjuk átadni, akkor több helyen is kölcsönözhetjük.

1. // This function takes ownership of a box and destroys it
2. fn eat_box_i32(boxed_i32: Box<i32>) {
3. println!("Destroying box that contains {}", boxed_i32);
4. }
5.
6. // This function borrows an i
7. fn borrow_i32(borrowed_i32: &i32) {
8. println!("This int is: {}", borrowed_i32);
9. }
10.
11. fn main() {
12. // Create a boxed i32 in the heap, and a i32 on the stack
15. let boxed_i32 : i32 = Box::new( 5 );
16. let stacked_i32 : i32 = 6 ;
17.
18. // Borrow the contents of the box. Ownership is not taken,
19. // so the contents can be borrowed again.
20. borrow_i32(&boxed_i32);
21. borrow_i32(&stacked_i32);
22.
23. {
24. // Take a reference to the data contained inside the box
25. let _ref_to_i32: &i32 = &boxed_i32;
26.
27. // Error!
28. // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
29. eat_box_i32(boxed_i32);
31.
32. // Attempt to borrow `_ref_to_i32` after inner value is destroyed
33. borrow_i32(_ref_to_i32);
34. // `_ref_to_i32` goes out of scope and is no longer borrowed.
35. }
36.
37. // `boxed_i32` can now give up ownership to `eat_box_32` and be destroyed
38. eat_box_i32(boxed_i32);
39. }
40.


- Élettartam

##### Az élettartamok számon tartják a referenciák érvényességét, így ellenőrzi, hogy

##### mindig érvényesek maradjanak. Erre lehet bevezetni paramétereket is, amikkel

##### tudjuk jelezni, hogy egy-egy referenciának meddig kell életben maradnia. Erre itt egy

##### példa.

1. fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // az ‘a paraméter megadja, hogy a
2. if x.len() > y.len() { // függvény által visszaadott referencia
3. x // élettartama megegyezik a
4. } else { // paraméterekével
5. y
6. }
7. }

##### Ezekkel a megoldásokkal a nyelv eléri, hogy majdnem teljesen kizárja a memória

##### szivárgásos hibákat, a null referenciákat és az adatverseny problémákat.

#### Beágyazott fejlesztés

##### A nyelv előbb említett funkció ideálissá teszik beágyazott alkalmazásra, viszont van egy

##### nagy hátránya. Lévén, hogy modern, friss nyelv még nincs akkora támogatása, mint

##### jobban elterjedt nyelveknek. Ez megnehezítheti nagyban az életünkent, viszont

##### szerencsére egyre többen használják és támogatjak, így egyre több crate (így hivják a

##### libraryket) és eszköz van amik segítenek a fejlesztés folyamataiban. A legfontosabb ilyen

##### eszköz a nyelv fő fordító-mindenes eszköze a cargo. Ez kezeli az összes

##### dependenciánkat, a fordítás közben tippeket ad a hibák feloldására, és ami a

##### legfontosabb külső debugger segítségével képes írni a kontrollerünkre, debugolni és

##### logolni a futást.

##### Az előbb említett cratek közül kettőt említenék meg amik fontos szerepet játszanak a

##### projekt megvalósításában.

- **Embassy**

##### Az embassy projekt célja, hogy platformfüggetlen aszinkron kódot tudjunk írni

##### beágyazott rendszerekre, megtartva a Rust modelljének az előnyeit. Az aszinkron

##### működés egy ilyen projektben nagyon fontos, mivel egyszerre több I/O műveletet

##### végzünk akár, miközben a perifériákat is le kell kezelnünk. A platformfüggetlenség

##### nem olyan fontos a projektben, viszont a modularítását elősegíti, későbbiekben

##### nem kell újraírni a kódbázist, ha más kontrollerre szeretnénk váltani.

- **probe-rs**

##### Ez egy eszköz/könyvtár csomag rustban való beágyazott fejlesztéshez, lehetőve

##### teszi, hogy a cargon keresztül flasheljük, futtasuk a kódot a chipen. A kód

##### szempontjából lehetőséget ad a logolás egyszerű megvalósítására, külső

##### debuggereken keresztül.


### Tervezés

##### A tervezésnél fontos volt figyelembe venni a modularitást, mivel a két oldal több fajta

##### konfigurációban is tud működni, nem lenne érdemes minden konfigurációt külön

##### lefejleszteni, ha váltani szeretnénk köztuk csak a main fájlunkat kell módosítani.

#### Struktúra

##### A struktúra felosztását úgy kellett megoldani, hogy feladatkörönként elkülönüljenek az

##### elemek, hogy a párhuzamos futás közben ne kelljen köztük erőforrást megosztani.

##### A fő megvalósítandó elemek maga a billentyűzet perifériák kezelő egysége és a

##### kommunikációt kezelő egységek. Érdemes arra is figyelni, hogy ezeknek csináljunk egy

##### wrappert, hogy könnyen lehessen váltani USB és Bluetooth kommunikáció között.

### Megvalósítás

#### Billentyűzet funkciók

##### A billentyűzet struktúrája:

1. pub struct Keyboard<
2. In : InputPin,
3. Out : OutputPin
4. > {
5. input_pins: [In ; 4 ],
6. output_pins: [Out ; 5 ],
7. key_states: [[KeyState; 4 ]; 5 ],
8. key_map: KeyMap,
9. report : KeyboardReport,
10. should_report : bool,
11. }

###### A fő osztály ami összefogja a billentyűzet funkcióit

1. pub(crate) struct KeyState {
2. pressed : bool,
3. pub changed : bool,
4. pressed_time : Option<Instant>,
5. debounce_counter : DebounceCounter,
6. last_tick : u32,
7. }

###### Egy-egy gomb állapotát kezelő osztály


##### Debounce

##### A debouncing algoritmus feladata hogy a kontakt zajt kiszürje, tehát egy billentyű

##### lenyomást tényleg egyként regisztráljon. Ennek a megvalósításra az alábbi algoritmust

##### implementeltám, amit a ZMK billentyűzet firmware is alkalmaz.

1. /* Step 1: Update the integrator based on the input signal. Note that the
2. integrator follows the input, decreasing or increasing towards the limits as
3. determined by the input state (0 or 1). */
4.
5. if (input == 0 )
6. {
7. if (integrator > 0 )
8. integrator--;
9. }
10. else if (integrator < MAXIMUM)
11. integrator++;
12.
13. /* Step 2: Update the output state based on the integrator. Note that the
14. output will only change states if the integrator has reached a limit, either
15. 0 or MAXIMUM. */
16.
17. if (integrator == 0 )
18. output = 0 ;
19. else if (integrator >= MAXIMUM)
20. {
21. output = 1 ;
22. integrator = MAXIMUM; /* defensive code if integrator got corrupted */
23. }
24.

##### Az alább pedig látható az implementációm, ahol a DebounceCounter osztály kezeli az

##### integrátorra vonatkozó szabályokat a fenti példából.

1. pub(crate) fn debounce_keys(key_state : &mut KeyState,pin_state : bool) - > bool {
2. let cur_tick = Instant::now().as_ticks() as u32;
3. let elapsed = (cur_tick - key_state.last_tick) as u16;
4. if elapsed > 0 {
5. if pin_state == key_state.pressed {
6. key_state.debounce_counter.decrement_counter(elapsed);
7. }
8. else {
9. if key_state.debounce_counter. 0 < DEBOUNCE_THRESHOLD * 10 {
10. key_state.debounce_counter.increment_counter(elapsed);
11. } else {
12. key_state.last_tick = cur_tick;
13. key_state.debounce_counter.reset();
14. return true
15. }
16. }
17. }
18. false
19. }
20.


##### Matrix Polling

##### A gombok lekezeléséhez így már megvan minden eszközünk, már csak meg kell

##### valósítani azt. Itt egyszerűen csak egyenként jelet adunk a kimeneteknek, ebben az

##### esetben az oszlopoknak, aztán végig nézzük a sorokat, hogy van e rajtuk jel és

##### elvégezzük a debouncingot, hogy kiszűrjuk a hamis jeleket.

1. async fn scan(&mut self) {
2. for (out_idx, out_pin) in self.output_pins.iter_mut().enumerate() {
3. out_pin.set_high().ok();
4. Timer::after_micros( 1 ).await;
5. for (in_idx, in_pin) in self.input_pins.iter_mut().enumerate() {
6. let current = in_pin.is_high().ok().unwrap_or_default();
7. let changed = debounce_keys(&mut self.key_states[out_idx][in_idx], current);
8.
9. if changed {
10. self.key_states[out_idx][in_idx].toggle_key();
11. }
12.
13. self.key_states[out_idx][in_idx].changed = changed;
14. }
15. out_pin.set_low().ok();
16. }
17. }
18.

##### Végül az üzenet elküldése előtt ki olvassuk a billentyűk állapotát és megalkotjuk a report

##### tömbünket, amit ha nem üres ki küldhetünk a választott I/O eszközön keresztül.

#### Kommunikáció

##### USB

##### Az USBn történő kommunikációnál az első feladatunk a kapcsolat felépítése a

##### céleszközzel, egy HID (Human Interface Device) descriptor generálásával jeleznünk kell

##### felé az eszközünk típusát, a tőle elvárt kommunikációt. Ezután fenn kell tartanunk a

##### kapcsolatot és szabadon tudunk a megadott paraméterekkel üzeneteket küldeni.

##### Bluetooth

##### A két oldal közti kommunikáció megvalósításának az első lépése a két oldal párosítása.

##### Ezt minden indításnál meg kell tennünk, ha nem használunk belső tárhelyet.

