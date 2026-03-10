# 🦀 Rust — Fiche Examen Complète

> **VS Code** : `Ctrl+Shift+V` pour la preview Markdown | `Ctrl+F` pour chercher

---

## Table des matières
- [Types & Variables](#types--variables)
- [Fonctions & Blocs](#fonctions--blocs)
- [match & Patterns](#match--patterns)
- [String vs &str](#string-vs-str)
- [Ownership](#ownership)
- [Lifetimes](#lifetimes)
- [Struct](#struct)
- [Enum](#enum)
- [Option\<T\>](#optiont)
- [Result\<T,E\> & Erreurs](#resultte--erreurs)
- [Traits](#traits)
- [Derive — Traits courants](#derive--traits-courants)
- [Serde (JSON)](#serde-json)
- [Génériques](#génériques)
- [Itérateurs](#itérateurs)
- [Collections](#collections)
- [Closures](#closures)
- [Threads & Concurrence](#threads--concurrence)
- [Smart Pointers](#smart-pointers)
- [Async / Await & Tokio](#async--await--tokio)
- [Conversions](#conversions)
- [Modules & Visibilité](#modules--visibilité)
- [Tests](#tests)
- [Divers](#divers)
- [Pattern TP — Bot d'arène](#pattern-tp--bot-darène)
- [Récap — Quand utiliser quoi](#récap--quand-utiliser-quoi)

---

## Types & Variables

```rust
// Entiers : i8/u8  i16/u16  i32/u32  i64/u64  i128/u128  isize/usize
let a: i32  = -42;
let b: u64  = 100u64;        // suffixe de type
let n: usize = 3;            // pour les index / tailles
let pi: f64 = 3.14;
let ok: bool = true;
let c: char = 'é';           // Unicode, quotes simples

// Mutabilité
let x = 1;                   // immutable par défaut
let mut x = 1;               // mutable
x = 4;

// Shadowing (nouvelle variable, type peut changer)
let size = "hello";
let size = 2;                // shadowing OK

// Constante (type obligatoire, évaluée à la compilation)
const MAX: u32 = 100_000;

// Tuple
let t: (i32, bool, &str) = (7, true, "hi");
let (n, flag, s) = t;        // destructuration
let first = t.0;             // accès par index

// Array (taille fixe connue à la compilation)
let a: [u8; 3] = [1, 2, 3];
let zeros: [i32; 4] = [0; 4]; // [valeur; taille]
let [first, second, ..] = a;  // destructuration partielle
```

---

## Fonctions & Blocs

```rust
// Fonction : type retour après ->
// Pas de ; sur la dernière ligne = valeur de retour
fn add(a: i32, b: i32) -> i32 {
    a + b          // équivalent à return a + b;
}

fn nothing(v: i32) {         // pas de -> = retourne ()
    dbg!(v);
}

// Bloc = expression avec valeur
let y = {
    let two = 2;
    two + 3        // valeur retournée par le bloc (pas de ;)
};

// if est une expression
let val = if cond { 10 } else { 5 };

// Boucles
for i in 0..5  { }           // 0,1,2,3,4
for i in 0..=5 { }           // 0,1,2,3,4,5
while n < 10 { n += 1; }

let result = loop {
    if done { break 42; }    // loop peut retourner une valeur
};

// Affichage
println!("{}", val);
println!("{:?}", val);       // Debug
println!("{val}");           // capture directe (Rust 2021+)
dbg!(val);                   // debug avec fichier + numéro de ligne
eprintln!("erreur: {val}");  // stderr
```

---

## match & Patterns

```rust
// match basique (exhaustif !)
let result = match n {
    0       => "zero",
    1 | 2   => "un ou deux",      // OU logique
    3..=9   => "entre 3 et 9",    // range inclusive
    n if n < 0 => "négatif",      // guard
    _       => "grand",            // wildcard obligatoire si non exhaustif
};

// Destructuration dans match
match point {
    (x, 0) => println!("sur axe x: {x}"),
    (0, y) => println!("sur axe y: {y}"),
    (x, y) => println!("{x},{y}"),
}

// Binding avec @
match n {
    x @ 1..=10 => println!("capturé: {x}"),
    _ => {}
}

// if let — un seul variant, plus concis qu'un match
if let Some(x) = maybe {
    println!("{x}");
}

// if let avec mutation
if let Some(x) = &mut maybe {
    *x += 1;
}

// let else — early return si le pattern ne matche pas
let Some(x) = maybe else { return; };
// x est disponible ici

// while let
while let Some(v) = stack.pop() {
    println!("{v}");
}
```

---

## String vs &str

```rust
// &str : vue sur données statiques (slice, non modifiable)
let s: &str = "hello";           // &'static str

// String : allouée sur le heap, modifiable
let mut owned: String = String::from("hello");
owned.push_str(", world");
owned.push('!');

// Conversions
let owned: String = s.to_string();
let owned: String = s.to_owned();
let slice: &str   = &owned;      // deref coercion String → &str
let slice         = &owned[0..3];// slice d'une sous-chaîne

// format! crée une nouvelle String
let s = format!("{} + {} = {}", a, b, a + b);

// Méthodes utiles
s.len();           s.is_empty();
s.contains("hi");  s.starts_with("he");  s.ends_with("lo");
s.trim();          s.to_uppercase();     s.to_lowercase();
s.replace("a","b");
s.split(' ').collect::<Vec<&str>>();
s.parse::<i32>()?;               // turbofish pour désambiguïser
```

---

## Ownership

### Les 3 règles
1. Chaque valeur a **un seul propriétaire**.
2. Quand le propriétaire sort du scope, la valeur est **libérée**.
3. Il ne peut y avoir qu'**un seul propriétaire à la fois**.

```rust
// Move : ownership transféré, s invalide après
let s = String::from("hi");
let t = s;            // s déplacé vers t
// dbg!(s);          // ❌ ERREUR : s a été déplacé

// Clone : copie explicite et coûteuse
let t = s.clone();    // s toujours valide

// Copy : copie implicite pour les types scalaires (i32, f64, bool, char…)
let x: i32 = 42;
let y = x;            // copié, x toujours valide
```

### Emprunts (Borrows)

```rust
// &T = référence immuable (lecture seule)
fn len(s: &String) -> usize { s.len() }
len(&s);                           // passer une référence

// &mut T = référence mutable (exclusive)
fn push(s: &mut String) { s.push('!'); }
push(&mut s);

// Règles :
// ✅ Plusieurs &T simultanément
// ✅ UN seul &mut T à la fois
// ❌ &T et &mut T en même temps
```

---

## Lifetimes

```rust
// Annotation quand le compilateur ne peut pas inférer
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct contenant une référence
struct Excerpt<'a> {
    text: &'a str,   // Excerpt ne survit pas à la donnée référencée
}

// 'static : vit toute la durée du programme
let s: &'static str = "je suis statique";

// Élision (le compilateur applique ces règles automatiquement) :
// Règle 1 : chaque &param reçoit sa propre lifetime
// Règle 2 : 1 seul param & → sa lifetime est assignée au retour
// Règle 3 : &self → sa lifetime est assignée au retour
fn first_word(s: &str) -> &str {  // élision OK, pas besoin d'annoter
    s.split_whitespace().next().unwrap_or("")
}

// T: 'static = T ne contient pas de références non-'static
fn spawn(data: impl Send + 'static) { ... }
```

---

## Struct

```rust
// Struct nommé
struct User {
    id:     u32,
    name:   String,
    active: bool,
}

let u = User { id: 1, name: String::from("Ada"), active: true };
let u2 = User { id: 2, ..u };     // struct update syntax (move les champs restants)

// Tuple struct
struct Color(u8, u8, u8);
let red = Color(255, 0, 0);
let r = red.0;

// Struct unitaire
struct Unit;

// impl : méthodes et fonctions associées
impl User {
    // Fonction associée (pas de self) = constructeur
    pub fn new(name: String) -> Self {
        Self { id: 0, name, active: true }
    }
    // Méthode lecture
    pub fn greet(&self) {
        println!("Hi {}", self.name);
    }
    // Méthode écriture
    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

// Appels
let u = User::new(String::from("Bob")); // fn associée
u.greet();                              // méthode

// derive : implémentations automatiques
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Point { x: i32, y: i32 }

println!("{:?}", Point { x: 1, y: 2 }); // Debug
let p2 = p1.clone();                     // Clone
```

---

## Enum

```rust
// Enum simple
enum Direction { North, South, East, West }
let d = Direction::North;

// Enum avec données (types somme)
enum Message {
    Quit,                          // unitaire
    Move { x: i32, y: i32 },      // struct-like
    Write(String),                 // tuple-like
    Color(u8, u8, u8),
}

// match sur enum avec données
match msg {
    Message::Quit             => println!("quit"),
    Message::Move { x, y }   => println!("{x},{y}"),
    Message::Write(text)      => println!("{text}"),
    Message::Color(r, g, b)   => println!("{r}{g}{b}"),
}
```

---

## Option\<T\>

```rust
// Définition
enum Option<T> { Some(T), None }

let x: Option<i32> = Some(42);
let y: Option<i32> = None;

// Méthodes essentielles
x.unwrap()                    // valeur ou PANIC si None
x.unwrap_or(0)                // valeur ou défaut
x.unwrap_or_default()         // valeur ou Default::default()
x.expect("message d'erreur")  // unwrap avec message custom
x.map(|v| v * 2)              // transforme le Some, None reste None
x.and_then(|v| Some(v + 1))   // flatMap
x.filter(|v| *v > 0)          // None si prédicat faux
x.or(Some(0))                 // autre Option si None
x.is_some()                   // bool
x.is_none()                   // bool

// Pattern matching
match x {
    Some(v) => println!("{v}"),
    None    => println!("rien"),
}

// if let (plus concis pour un seul cas)
if let Some(v) = x {
    println!("{v}");
}

// let else (early return)
let Some(v) = x else { return; };
```

---

## Result\<T,E\> & Erreurs

```rust
// Définition
enum Result<T, E> { Ok(T), Err(E) }

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { return Err("division par zéro".to_string()); }
    Ok(a / b)
}

// match sur Result
match divide(10.0, 2.0) {
    Ok(r)  => println!("{r}"),
    Err(e) => eprintln!("{e}"),
}

// ? — propagation automatique (retourne Err si Err)
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("f.txt")?; // propage io::Error
    let n: i32  = content.trim().parse()?;           // propage ParseIntError
    Ok(())
}

// Méthodes utiles
r.unwrap()
r.expect("message")
r.unwrap_or(default)
r.map(|v| v * 2)
r.map_err(|e| format!("{e}"))
r.ok()          // Result → Option
r.is_ok()  r.is_err()

// thiserror — définir ses propres erreurs
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("fichier introuvable: {0}")]
    NotFound(String),
    #[error("erreur I/O")]
    Io(#[from] std::io::Error),     // ? convertit auto io::Error → AppError
    #[error("parsing: {0}")]
    Parse(#[from] std::num::ParseIntError),
}

// anyhow — pour les applications (pas besoin de définir chaque erreur)
use anyhow::{Context, Result};

fn read_config() -> Result<String> {
    let content = std::fs::read_to_string("config.toml")
        .context("impossible de lire config.toml")?;
    Ok(content)
}

// Box<dyn Error> — sans crate externe
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let n: i32 = "42".parse()?;
    Ok(())
}
```

---

## Traits

```rust
// Définir un trait
trait Summary {
    fn summary(&self) -> String;                  // méthode requise
    fn default_impl(&self) -> String {            // méthode avec implémentation par défaut
        String::from("(lecture en cours...)")
    }
}

// Implémenter un trait pour un type
struct Post { title: String }

impl Summary for Post {
    fn summary(&self) -> String {
        format!("Post: {}", self.title)
    }
}

// Trait bound — générique avec contrainte
fn notify<T: Summary>(item: &T) { println!("{}", item.summary()); }
fn notify(item: &impl Summary) { ... }  // syntaxe équivalente plus courte

// where — plusieurs contraintes, plus lisible
fn print_all<T>(items: &[T])
where T: Display + Clone + Debug
{ for item in items { println!("{item}"); } }

// Retourner un trait (dispatch STATIQUE, type unique)
fn make_post() -> impl Summary { Post { title: "...".into() } }

// Box<dyn Trait> — dispatch DYNAMIQUE (types hétérogènes)
let items: Vec<Box<dyn Summary>> = vec![
    Box::new(Post { title: "A".into() }),
    Box::new(Article { ... }),
];
for item in &items { println!("{}", item.summary()); }

// Trait object-safe + Send (requis pour multi-thread)
trait Strategy: Send {
    fn next_move(&self, state: &GameState) -> Option<(i8, i8)>;
}
let s: Box<dyn Strategy> = Box::new(NearestResourceStrategy);

// Supertrait (héritage de contrats)
trait Printable: fmt::Display + fmt::Debug {
    fn print(&self) { println!("{self:?}"); }
}

// Types associés (évite un paramètre générique)
trait Parser {
    type Output;
    fn parse(&self, s: &str) -> Self::Output;
}
```

---

## Derive — Traits courants

| Derive | Effet | Quand l'utiliser |
|--------|-------|-----------------|
| `Debug` | Affichage `{:?}` | Toujours — indispensable |
| `Clone` | `.clone()` explicite | Quand on a besoin de dupliquer |
| `Copy` | Copie implicite bit-à-bit | Types légers (scalaires, petits structs) |
| `PartialEq` | `== !=` | Comparaisons |
| `Eq` | Égalité totale (après PartialEq) | HashMaps, assertions |
| `Hash` | Clé de HashMap/HashSet | Besoin de HashMap |
| `Default` | `Type::default()` | Valeurs initiales |
| `PartialOrd / Ord` | Tri, `<` `>` | sort(), BTreeMap |
| `Serialize` | → JSON/TOML/… | serde |
| `Deserialize` | ← JSON/TOML/… | serde |

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Point { x: i32, y: i32 }

let p = Point::default();        // Point { x: 0, y: 0 }
println!("{:?}", p);             // Debug
let p2 = p.clone();              // Clone
assert_eq!(p, p2);               // PartialEq
```

---

## Serde (JSON)

```toml
# Cargo.toml
[dependencies]
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    host: String,
    port: u16,
}

// Désérialiser (JSON → struct)
let raw = r#"{"host":"127.0.0.1","port":8080}"#;
let cfg: Config = serde_json::from_str(raw)?;

// Sérialiser (struct → JSON)
let out = serde_json::to_string(&cfg)?;
let pretty = serde_json::to_string_pretty(&cfg)?;

// Enum avec tag discriminant (pattern du TP)
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum ServerMsg {
    Hello { agent_id: Uuid, tick_ms: u64 },
    State { tick: u64, width: u16, height: u16 },
}
// → {"type":"Hello","data":{"agent_id":"...","tick_ms":100}}
```

---

## Génériques

```rust
// Fonction générique
fn first<T: Clone>(items: &[T]) -> Option<T> {
    items.first().cloned()
}

// Struct générique
struct Pair<T> { left: T, right: T }

impl<T: Display + PartialOrd> Pair<T> {
    fn max(&self) -> &T {
        if self.left > self.right { &self.left } else { &self.right }
    }
}

// Monomorphisation : le compilateur génère une version
// spécialisée par type → performance optimale, zéro coût runtime

// Newtype pattern : type distinct sans coût runtime
struct Meters(f64);
struct Seconds(f64);
fn speed(d: Meters, t: Seconds) -> f64 { d.0 / t.0 }
// speed(t, d) → ❌ ERREUR de compilation (types inversés protégés)
```

---

## Itérateurs

```rust
// 3 façons d'itérer sur une collection
for n in v.iter()      { }   // &T     — emprunt lecture
for n in v.iter_mut()  { }   // &mut T — emprunt écriture
for n in v.into_iter() { }   // T      — consomme la collection

// Chaîne de combinateurs (LAZY : rien ne s'exécute tant qu'on ne consomme pas)
let total: i32 = values.iter()
    .filter(|&&n| n % 2 == 0)
    .map(|&n| n * n)
    .sum();

// collect → matérialise en collection
let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();
let set: HashSet<i32>  = v.into_iter().collect();
```

| Méthode | Rôle |
|---------|------|
| `map(\|x\| ...)` | Transformer chaque élément |
| `filter(\|x\| ...)` | Garder si prédicat vrai |
| `filter_map(\|x\| ...)` | map + filter (retourne Option) |
| `flat_map(\|x\| ...)` | map puis aplatir |
| `fold(init, \|acc, x\| ...)` | Réduction en une valeur |
| `sum()` / `product()` | Somme / produit |
| `count()` | Nombre d'éléments |
| `any(\|x\| ...)` / `all(\|x\| ...)` | Existentiel / universel |
| `find(\|x\| ...)` | Premier satisfaisant → `Option` |
| `position(\|x\| ...)` | Index du premier → `Option` |
| `min_by_key(\|x\| k)` / `max_by_key` | Min/max par critère |
| `enumerate()` | `(index, &elem)` |
| `zip(other)` | Paires d'éléments |
| `take(n)` / `skip(n)` | Limiter / ignorer |
| `chain(other)` | Concaténer deux itérateurs |
| `cloned()` / `copied()` | `&T → T` |
| `collect::<Vec<_>>()` | Consommer → collection |

---

## Collections

```rust
use std::collections::{HashMap, HashSet, BTreeMap, VecDeque, BinaryHeap};

// ── Vec<T> — tableau dynamique ──────────────────────────────────
let mut v: Vec<i32> = Vec::new();
let mut v = vec![1, 2, 3];
v.push(4);      v.pop();         // ajouter / retirer à la fin
v.len();        v.is_empty();
v[0];           v.get(0);        // get → Option (pas de panic)
v.contains(&3); v.sort();        v.reverse();
v.retain(|&x| x > 2);           // supprimer si prédicat faux
v.dedup();                       // supprimer les doublons consécutifs
v.extend([4, 5]);

// ── HashMap<K, V> ───────────────────────────────────────────────
let mut map: HashMap<&str, i32> = HashMap::new();
map.insert("Alice", 12);
map.get("Alice");                // → Option<&V>
map.contains_key("Alice");
map.remove("Alice");
// entry API (insérer si absent, puis modifier)
map.entry("Bob").or_insert(0);
*map.entry("Bob").or_insert(0) += 1;
for (k, v) in &map { println!("{k}:{v}"); }

// Créer depuis itérateur
let map: HashMap<_, _> = keys.iter().zip(vals).collect();

// ── HashSet<T> ──────────────────────────────────────────────────
let mut set: HashSet<&str> = HashSet::new();
set.insert("rust");
set.contains("rust");
set.union(&set2);
set.intersection(&set2);

// ── BTreeMap / BTreeSet — triés ─────────────────────────────────
let mut bt: BTreeMap<i32, &str> = BTreeMap::new();
bt.insert(10, "Alice");          // itération dans l'ordre des clés

// ── BinaryHeap — file de priorité (max en tête) ─────────────────
let mut heap = BinaryHeap::from(vec![3, 1, 7, 2]);
heap.pop();                      // → Some(7)
```

---

## Closures

```rust
// Syntaxe : |args| expression
let square  = |x: i32| x * x;
let add     = |a, b| a + b;      // types inférés

// Capture de l'environnement
let limit = 10;
let over = |v: &i32| *v > limit; // emprunte limit (&limit)

// move : transfert l'ownership dans la closure
let name = String::from("Alice");
let greet = move || println!("Hi {name}");
// name n'est plus valide ici

// ─── Traits de closure (hiérarchie : Fn ⊂ FnMut ⊂ FnOnce) ──────
// Fn     : peut être appelée plusieurs fois,  n'écrit pas sa capture (&self)
// FnMut  : peut être appelée plusieurs fois,  modifie sa capture (&mut self)
// FnOnce : appelée une seule fois, consomme sa capture (self)

// Accepter une closure en paramètre
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
fn apply_mut<F: FnMut()>(mut f: F) { f(); f(); }
fn apply_once<F: FnOnce()>(f: F)   { f(); }

// Exemple
let mut counter = 0;
let mut inc = || { counter += 1; };  // FnMut (modifie counter)
inc(); inc();

let text = String::from("consommé");
let once = move || { let t = text; println!("{t}"); }; // FnOnce
once();  // text consommé, ne peut pas rappeler once()

// fn nommée implémente les 3 traits
fn double(x: i32) -> i32 { x * 2 }
apply(double, 5);
apply(|x| x * 2, 5);  // équivalent
```

---

## Threads & Concurrence

```rust
use std::{thread, sync::{Arc, Mutex, RwLock, mpsc}};

// ── thread::spawn ────────────────────────────────────────────────
let handle = thread::spawn(move || {
    println!("dans le thread");
    42                             // valeur de retour
});
let result = handle.join().unwrap(); // attend la fin, récupère la valeur

// ── mpsc::channel — Multi-Producer Single-Consumer ───────────────
let (tx, rx) = mpsc::channel();

for i in 0..3 {
    let tx = tx.clone();           // cloner le sender pour chaque thread
    thread::spawn(move || {
        tx.send(i * 10).unwrap();
    });
}
drop(tx);                          // fermer le dernier sender original
for val in rx { println!("{val}"); } // boucle jusqu'à fermeture du canal

// Non-bloquant
match rx.try_recv() {
    Ok(v)  => println!("{v}"),
    Err(_) => println!("pas de message"),
}

// ── Arc<Mutex<T>> — partage mutable entre threads ────────────────
let counter = Arc::new(Mutex::new(0));
let c = Arc::clone(&counter);
thread::spawn(move || {
    let mut val = c.lock().unwrap();
    *val += 1;
});  // val droppé ici → unlock automatique

// ── RwLock — plusieurs lecteurs OU un seul écrivain ──────────────
let lock = Arc::new(RwLock::new(vec![1, 2, 3]));
{
    let mut w = lock.write().unwrap();
    w.push(4);
}  // unlock écriture
let r1 = lock.read().unwrap();
let r2 = lock.read().unwrap(); // OK : deux lectures simultanées

// ── Arc<Mutex<Receiver<T>>> — partager un Receiver entre N threads ─
let (tx, rx) = mpsc::channel::<Task>();
let shared_rx: Arc<Mutex<Receiver<Task>>> = Arc::new(Mutex::new(rx));

for _ in 0..N {
    let rx = Arc::clone(&shared_rx);
    let result_tx = result_tx.clone();
    thread::spawn(move || loop {
        let task = rx.lock().unwrap().recv().unwrap(); // attend une tâche
        // traiter la tâche...
        let _ = result_tx.send(result);
    });
}
```

---

## Smart Pointers

| Type | Usage principal | Thread-safe |
|------|----------------|-------------|
| `Box<T>` | Heap, ownership unique | ✅ |
| `Rc<T>` | Ownership partagé mono-thread | ❌ |
| `Arc<T>` | Ownership partagé multi-thread | ✅ |
| `RefCell<T>` | Mutabilité intérieure, vérifié au runtime | ❌ |
| `Cell<T>` | Mutabilité intérieure pour types Copy | ❌ |
| `Mutex<T>` | Verrou exclusif (1 accès à la fois) | ✅ |
| `RwLock<T>` | Multi-lecture OU 1 écriture | ✅ |
| `Weak<T>` | Référence non-propriétaire (évite cycles) | — |

```rust
// Box — valeur sur le heap, types récursifs
let b = Box::new(42);
let v = *b;                        // déréférencement

enum List { Cons(i32, Box<List>), Nil }  // sans Box, taille infinie → erreur

// Rc — mono-thread, plusieurs propriétaires
use std::rc::Rc;
let a = Rc::new(String::from("hi"));
let b = Rc::clone(&a);             // incrémente le compteur (pas un clone de String)
println!("{}", Rc::strong_count(&a)); // 2

// Arc — multi-thread, même API que Rc
use std::sync::Arc;
let shared = Arc::new(Mutex::new(data));
let clone  = Arc::clone(&shared);

// RefCell — emprunts vérifiés au RUNTIME (panic si violation)
use std::cell::RefCell;
let cell = RefCell::new(10);
{
    let mut b = cell.borrow_mut(); // emprunt mutable
    *b += 5;
}  // borrow_mut droppé ici
println!("{}", cell.borrow());     // emprunt immutable

// Règle pratique :
// mono-thread  → Rc + RefCell
// multi-thread → Arc + Mutex  (ou Arc + RwLock si lectures dominantes)
```

---

## Async / Await & Tokio

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
// async fn → retourne un impl Future (LAZY : rien ne s'exécute sans .await)
async fn fetch(url: &str) -> String {
    reqwest::get(url).await.unwrap()
        .text().await.unwrap()
}

// Point d'entrée async
#[tokio::main]
async fn main() {
    let result = fetch("https://...").await;
}

// tokio::spawn — tâche concurrente légère
// Contrainte : la closure doit être Send + 'static → utiliser move
let data = String::from("hello");
let handle = tokio::spawn(async move {
    println!("{data}");
    42
});
let result = handle.await.unwrap();

// Parallélisme avec join!
let (a, b) = tokio::join!(task_a(), task_b());

// tokio::sync::mpsc (version async)
use tokio::sync::mpsc;
let (tx, mut rx) = mpsc::channel(32);  // borné (backpressure)
tokio::spawn(async move {
    tx.send("message").await.unwrap();
});
while let Some(v) = rx.recv().await { }

// Timeout
use tokio::time::{timeout, Duration};
match timeout(Duration::from_secs(2), slow_operation()).await {
    Ok(v)  => println!("{v}"),
    Err(_) => println!("timeout !"),
}

// select! — première future prête (annule les autres)
tokio::select! {
    val = rx.recv()    => { println!("{val:?}"); }
    _ = shutdown_sig() => { break; }
}

// Tableau récap sync vs async
// | Concept    | Sync              | Async (Tokio)         |
// |------------|-------------------|-----------------------|
// | Thread     | std::thread       | tokio::spawn          |
// | Coût/unité | ~8 Mo             | ~quelques centaines o |
// | Channel    | std::sync::mpsc   | tokio::sync::mpsc     |
// | Verrou     | Mutex             | tokio::sync::Mutex    |
// | Usage      | CPU-bound         | I/O-bound             |

// Test async
#[tokio::test]
async fn test_async() {
    let r = my_async_fn().await;
    assert_eq!(r, 42);
}
```

---

## Conversions

```rust
// as — entre types primitifs (PEUT tronquer silencieusement !)
let x: i32 = 300;
let y: u8  = x as u8;    // 44 (300 % 256, pas d'erreur !)
let z: f64 = x as f64;   // OK
let code   = 'A' as u32; // 65

// From / Into — conversions sûres (Into dérivé automatiquement de From)
let s: String = String::from("hello");
let s: String = "hello".into();     // Into via impl From<&str> for String

// Implémenter From pour ses propres types
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}
let f: Fahrenheit = Celsius(100.0).into(); // From donne Into gratuitement

// Astuce : utiliser Into dans les signatures pour plus de flexibilité
fn greet(name: impl Into<String>) {
    let name: String = name.into();
    println!("Hello, {name}!");
}
greet("Alice");               // &str
greet(String::from("Bob"));   // String

// TryFrom / TryInto — conversion faillible
use std::convert::TryFrom;

impl TryFrom<i32> for EvenNumber {
    type Error = String;
    fn try_from(v: i32) -> Result<Self, Self::Error> {
        if v % 2 == 0 { Ok(EvenNumber(v)) }
        else { Err(format!("{v} n'est pas pair")) }
    }
}
let n = EvenNumber::try_from(4)?;

// parse() — &str vers n'importe quel type qui implémente FromStr
let n: i32  = "42".parse()?;
let n       = "42".parse::<i32>()?;  // turbofish pour désambiguïser

// AsRef — accepter &str, String, &Path… sans copie
fn open_file(path: impl AsRef<std::path::Path>) {
    let path = path.as_ref();
}
open_file("/tmp/test.txt");           // &str
open_file(String::from("/home"));     // String

// Type alias
type Result<T>  = std::result::Result<T, AppError>;
type UserMap    = HashMap<String, Vec<String>>;
type SharedState = Arc<Mutex<GameState>>;
```

---

## Modules & Visibilité

```rust
// Structure de fichiers
// src/main.rs       → point d'entrée binaire
// src/lib.rs        → point d'entrée bibliothèque
// src/auth.rs       → module (ou src/auth/mod.rs)
// src/auth/oauth.rs → sous-module

// Déclarer et charger un module
mod auth;              // charge src/auth.rs
pub mod config;        // visible de l'extérieur

// Réexporter proprement
pub use auth::login;   // les utilisateurs peuvent faire use mon_crate::login

// Chemins
crate::helper();       // racine du crate
super::login("Bob");   // module parent
self::call_local();    // module courant (rarement nécessaire)

// Visibilité
pub fn public() {}              // accessible partout
fn private() {}                 // privé au module (défaut)
pub(crate) fn internal() {}     // visible dans le crate seulement
pub(super) fn parent_only() {}  // visible dans le module parent

pub struct Token {
    pub value: String,          // champ public
    created_at: u64,            // champ privé
}

// use — raccourcir les chemins
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::state::{GameState, SharedState};
```

---

## Tests

```rust
// Tests unitaires — dans le même fichier, accès aux privés
#[cfg(test)]
mod tests {
    use super::*;      // importer tout le module parent

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);       // égalité
        assert_ne!(add(2, 3), 99);      // inégalité
        assert!(add(2, 3) > 0);         // condition bool
    }

    #[test]
    #[should_panic(expected = "division par zéro")]
    fn test_div_zero() {
        divide(1.0, 0.0).unwrap();      // doit paniquer avec ce message
    }

    #[test]
    fn test_float() {
        let r = divide(10.0, 3.0).unwrap();
        assert!((r - 3.333).abs() < 0.001);  // float : comparer avec tolérance
    }

    #[test]
    #[ignore]           // non lancé par défaut (tests lents, réseau…)
    fn slow_test() { }

    #[tokio::test]      // test async
    async fn test_async() {
        let r = my_async_fn().await;
        assert_eq!(r, 42);
    }
}
```

```bash
cargo test                    # tous les tests
cargo test test_add           # un test spécifique (filtre par nom)
cargo test -- --nocapture     # afficher les println! dans les tests
cargo test -- --ignored       # lancer les tests #[ignore]
```

---

## Divers

### Surcharge d'opérateurs

```rust
use std::ops::Add;

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
// Autres traits : Sub, Mul, Div, Neg, Index, Not, BitAnd, BitOr…
```

### Drop — Destructeur (RAII)

```rust
impl Drop for Connection {
    fn drop(&mut self) {
        println!("connexion {} fermée", self.id);
    }
}
// Destruction automatique à la fin du scope
// Ordre : inverse de la déclaration (LIFO)
// drop(val) libère manuellement avant la fin du scope
```

### Macros utiles

```rust
todo!("à implémenter")        // panic — code non écrit
unimplemented!()              // panic — fonctionnalité absente délibérément
unreachable!("jamais ici")   // panic — branche impossible
panic!("message d'erreur")   // arrêt immédiat du programme
```

### unsafe

```rust
let ptr: *const i32 = &x;       // créer un pointeur brut (safe)
unsafe {
    println!("{}", *ptr);        // déréférencer (unsafe)
}
// 5 super-pouvoirs unsafe :
// 1. Déréférencer *const T / *mut T
// 2. Appeler une fn unsafe
// 3. Accéder à une variable static mut
// 4. Implémenter un trait unsafe
// 5. Accéder à un champ d'union
```

### Tracing (logs structurés)

```rust
use tracing::{info, warn, error, debug, instrument};
tracing_subscriber::fmt::init(); // activer les logs

info!("serveur démarré");
debug!(port = 8080, "configuration chargée");
warn!(remaining = 3, "espace disque faible");
error!("connexion DB échouée");

#[instrument]                    // crée un span automatique
async fn handle(id: u32) {
    info!("traitement");         // log inclut le contexte du span
}

// Filtrage par variable d'environnement
// RUST_LOG=debug cargo run
// RUST_LOG=my_crate=debug,tokio=warn cargo run
```

---

## Pattern TP — Bot d'arène

### state.rs — État partagé entre threads

```rust
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use crate::protocol::ServerMsg;

#[derive(Debug, Clone)]
pub struct ResourceInfo {
    pub resource_id: Uuid,
    pub x: u16, pub y: u16,
    pub expires_at: u64,
    pub value: u32,
}

#[derive(Debug, Clone)]
pub struct AgentInfo {
    pub id: Uuid,
    pub name: String, pub team: String,
    pub score: u32,
    pub x: u16, pub y: u16,
}

pub struct GameState {
    pub agent_id: Uuid,
    pub tick: u64,
    pub position: (u16, u16),
    pub map_size: (u16, u16),
    pub goal: u32,
    pub obstacles: Vec<(u16, u16)>,
    pub resources: Vec<ResourceInfo>,
    pub agents: Vec<AgentInfo>,
    pub team_scores: HashMap<String, u32>,
}

impl GameState {
    pub fn new(agent_id: Uuid) -> Self {
        Self {
            agent_id,
            tick: 0,
            position: (0, 0),
            map_size: (64, 48),
            goal: 0,
            obstacles: vec![],
            resources: vec![],
            agents: vec![],
            team_scores: HashMap::new(),
        }
    }

    pub fn update(&mut self, msg: &ServerMsg) {
        if let ServerMsg::State { tick, width, height, goal, obstacles, resources, agents } = msg {
            self.tick = *tick;
            self.map_size = (*width, *height);
            self.goal = *goal;
            self.obstacles = obstacles.clone();

            // Mettre à jour ma position depuis la liste des agents
            if let Some(me) = agents.iter().find(|(id, ..)| *id == self.agent_id) {
                self.position = (me.4, me.5);
            }

            // Convertir les resources
            self.resources = resources.iter().map(|r| ResourceInfo {
                resource_id: r.0,
                x: r.1, y: r.2,
                expires_at: r.3,
                value: r.4,
            }).collect();

            // Convertir les agents
            self.agents = agents.iter().map(|a| AgentInfo {
                id: a.0,
                name: a.1.clone(), team: a.2.clone(),
                score: a.3,
                x: a.4, y: a.5,
            }).collect();
        }

        if let ServerMsg::PowResult { resource_id, .. } = msg {
            self.resources.retain(|r| r.resource_id != *resource_id);
        }
    }
}

// Type alias pour faciliter le partage entre threads
pub type SharedState = Arc<Mutex<GameState>>;

pub fn new_shared_state(agent_id: Uuid) -> SharedState {
    Arc::new(Mutex::new(GameState::new(agent_id)))
}
```

### miner.rs — Pool de threads mineurs

```rust
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver};
use uuid::Uuid;
use crate::pow::pow_search;

#[derive(Debug, Clone)]
pub struct MineRequest {
    pub seed: String,
    pub tick: u64,
    pub resource_id: Uuid,
    pub agent_id: Uuid,
    pub target_bits: u8,
}

#[derive(Debug, Clone)]
pub struct MineResult {
    pub tick: u64,
    pub resource_id: Uuid,
    pub nonce: u64,
}

pub struct MinerPool {
    s: std::sync::mpsc::Sender<MineRequest>,
    r: std::sync::mpsc::Receiver<MineResult>,
}

impl MinerPool {
    pub fn new(n: usize) -> Self {
        let (req_tx, req_rx) = channel();
        let (res_tx, res_rx) = channel();

        // Partager le Receiver entre N threads
        let shared_rx: Arc<Mutex<Receiver<MineRequest>>> =
            Arc::new(Mutex::new(req_rx));

        for _ in 0..n {
            let rx     = Arc::clone(&shared_rx);
            let res_tx = res_tx.clone();

            thread::spawn(move || loop {
                // Attendre le prochain challenge
                let req = rx.lock().unwrap().recv().unwrap();

                // Chercher un nonce (boucle infinie par batch de 100_000)
                loop {
                    let start = rand::random::<u64>();
                    if let Some(nonce) = pow_search(
                        &req.seed, req.tick,
                        req.resource_id, req.agent_id,
                        req.target_bits, start, 100_000,
                    ) {
                        let _ = res_tx.send(MineResult {
                            tick: req.tick,
                            resource_id: req.resource_id,
                            nonce,
                        });
                        break;
                    }
                }
            });
        }

        MinerPool { s: req_tx, r: res_rx }
    }

    pub fn submit(&self, req: MineRequest) {
        let _ = self.s.send(req);
    }

    // Non-bloquant : retourne None s'il n'y a pas encore de résultat
    pub fn try_recv(&self) -> Option<MineResult> {
        self.r.try_recv().ok()
    }
}
```

### strategy.rs — Trait + dispatch dynamique

```rust
use crate::state::GameState;

// Trait object-safe + Send pour utilisation multi-thread
pub trait Strategy: Send {
    fn next_move(&self, state: &GameState) -> Option<(i8, i8)>;
}

pub struct NearestResourceStrategy;

impl Strategy for NearestResourceStrategy {
    fn next_move(&self, state: &GameState) -> Option<(i8, i8)> {
        // Trouver ma position
        let me = state.agents.iter()
            .find(|a| a.id == state.agent_id)?;

        // Trouver la ressource la plus proche (distance Manhattan)
        let target = state.resources.iter()
            .min_by_key(|r| {
                (r.x as i16 - me.x as i16).abs() +
                (r.y as i16 - me.y as i16).abs()
            })?;

        // Calculer la direction (-1, 0 ou 1)
        let dx = (target.x as i16 - me.x as i16).signum() as i8;
        let dy = (target.y as i16 - me.y as i16).signum() as i8;

        Some((dx, dy))
    }
}

// Dans main.rs — Box<dyn Strategy> permet de changer de stratégie
// sans modifier le reste du code (dispatch dynamique)
// let strategy: Box<dyn Strategy> = Box::new(NearestResourceStrategy);
// if let Some((dx, dy)) = strategy.next_move(&state) { ... }
```

### main.rs — Boucle principale (pattern)

```rust
// Lancer le thread lecteur WebSocket
let state_clone = Arc::clone(&state);
let (msg_tx, msg_rx) = mpsc::channel::<ServerMsg>();
thread::spawn(move || {
    loop {
        let raw = ws_reader.read_message().unwrap();
        let msg: ServerMsg = serde_json::from_str(&raw.to_string()).unwrap();
        state_clone.lock().unwrap().update(&msg);
        let _ = msg_tx.send(msg);
    }
});

// Boucle principale
let pool     = MinerPool::new(4);
let strategy: Box<dyn Strategy> = Box::new(NearestResourceStrategy);

loop {
    // Traiter tous les messages reçus (non-bloquant)
    while let Ok(msg) = msg_rx.try_recv() {
        match msg {
            ServerMsg::PowChallenge { tick, seed, resource_id, target_bits, .. } => {
                let state_guard = state.lock().unwrap();
                pool.submit(MineRequest {
                    seed, tick, resource_id,
                    agent_id: state_guard.agent_id,
                    target_bits,
                });
            }
            ServerMsg::Win { team } => {
                println!("Gagnant : {team}");
                return;
            }
            _ => {}
        }
    }

    // Envoyer les nonces trouvés
    if let Some(result) = pool.try_recv() {
        let msg = ClientMsg::PowSubmit {
            tick: result.tick,
            resource_id: result.resource_id,
            nonce: result.nonce,
        };
        ws_writer.send(tungstenite::Message::Text(
            serde_json::to_string(&msg).unwrap()
        )).unwrap();
    }

    // Se déplacer selon la stratégie
    let state_guard = state.lock().unwrap();
    if let Some((dx, dy)) = strategy.next_move(&state_guard) {
        let msg = ClientMsg::Move { dx, dy };
        ws_writer.send(tungstenite::Message::Text(
            serde_json::to_string(&msg).unwrap()
        )).unwrap();
    }
    drop(state_guard); // libérer le lock avant de dormir

    thread::sleep(Duration::from_millis(100));
}
```

---

## Récap — Quand utiliser quoi

### Ownership & Partage

| Besoin | Solution |
|--------|----------|
| Partager des données entre threads | `Arc<Mutex<T>>` |
| Partager en lecture seule multi-thread | `Arc<T>` ou `Arc<RwLock<T>>` |
| Plusieurs propriétaires mono-thread | `Rc<T>` |
| Muter depuis une référence immuable | `RefCell<T>` ou `Cell<T>` |
| Partager un `Receiver` entre N threads | `Arc<Mutex<Receiver<T>>>` |
| Stocker des types hétérogènes | `Box<dyn Trait>` |
| Valeur optionnelle | `Option<T>` |
| Opération faillible | `Result<T, E>` |
| Propager une erreur sans boilerplate | `?` |
| Erreurs typées (bibliothèque) | `thiserror` |
| Erreurs ergonomiques (application) | `anyhow` |

### Concepts Ownership — Résumé visuel

| Code | Effet |
|------|-------|
| `let t = s;` (String) | **Move** — s invalide après |
| `let t = s.clone();` | **Clone** — s toujours valide |
| `let y = x;` (i32) | **Copy** — x toujours valide |
| `&s` | Emprunt **immutable** |
| `&mut s` | Emprunt **mutable** (exclusif) |
| `move \|\| { s }` | Move de s **dans** la closure |
| `Arc::clone(&a)` | Clone du pointeur (atomique) |
| `drop(val)` | Libération **anticipée** |

### Traits importants à connaître

| Trait | Rôle |
|-------|------|
| `Send` | La valeur peut traverser les threads |
| `Sync` | La référence `&T` peut être partagée entre threads |
| `Clone` | Duplication explicite |
| `Copy` | Duplication implicite (marqueur) |
| `Debug` | Affichage `{:?}` |
| `Display` | Affichage `{}` |
| `From / Into` | Conversion infaillible |
| `TryFrom / TryInto` | Conversion faillible |
| `Iterator` | Itération paresseuse |
| `Drop` | Destructeur custom |
| `Deref` | `*` et coercions automatiques |

### Pièges fréquents

> ⚠️ `recv()` sur un `Receiver` dont tous les `Sender` sont droppés → `Err`. Utiliser `.recv().ok()` ou gérer l'erreur.

> ⚠️ Tenir un `MutexGuard` pendant un `send()` peut causer un **deadlock**. Dropper le guard avant d'envoyer.

> ⚠️ `tokio::spawn` exige `Send + 'static`. Toujours utiliser `move` et éviter les références empruntées.

> ⚠️ `as` peut **tronquer silencieusement** : `300u16 as u8 == 44`. Préférer `From`/`TryFrom` pour les types non-primitifs.

> ⚠️ Un `lock().unwrap()` peut **paniquer** si le mutex est "empoisonné" (thread précédent a paniqué en tenant le lock).

---

*Fiche générée depuis le cours Rust (181 pages) + TP arène multi-thread*
