---
type: 'Task Contract'
title: 'format_greeting: texto de saludo para la pagina /greet/{name}'
description: 'Utilidad pura de texto: construye el saludo mostrado por la pagina /greet/{name}, con nombre vacio/blanco -> World, y capitalizando solo el primer caracter (UTF-8-safe).'
tags: ['rust', 'topcoat-app', 'utilidad', 'prueba-e2e-kdd-topcoat']

task: format-greeting
intent: "Construir el texto de saludo para un nombre de URL (posiblemente vacio), capitalizando solo el primer caracter."
language: rust
target: src/greeting.rs
signature: "fn format_greeting(name: &str) -> String"
test_command: "cargo test --test format_greeting"
budget:
  cyclomatic_max: 5
  nesting_max: 2
  lines_max: 15
  params_max: 1
tests: "tests_rs/format_greeting.rs"
tests_sha256: "050bdc1a83554bca71c7a703212644a268a1eea4bf6a1b9cf9d7a23f04ad401b"
touch_only: ['src/greeting.rs']
deps_allowed: []
forbids: ['network', 'subprocess', 'llm', 'unsafe']
---

# Contract: format_greeting

## Intent
La pagina `/greet/{name}` (`src/main.rs`, ya conectada al router y a esta
funcion) necesita el texto de saludo a partir del segmento de URL `name`, que
puede venir vacio o con solo espacios. La logica de formateo del texto se
extrae a una funcion pura testeable en aislamiento, sin depender del
contexto de request (`Cx`) de Topcoat.

## Interface
```
pub fn format_greeting(name: &str) -> String
```

## Invariants
- Si `name` recortado (trim) queda vacio, el resultado es `"Hello, World!"`.
- Si no, el resultado es `"Hello, {N}!"` donde `N` es `name` recortado con
  SOLO su primer caracter en mayuscula (el resto del string NO se modifica,
  ni siquiera a minusculas).
- Nunca panickea, incluyendo con caracteres UTF-8 multi-byte (acentos) como
  primer caracter.
- No usa `unsafe`.

## Examples
- `format_greeting("")` -> `"Hello, World!"`
- `format_greeting("   ")` -> `"Hello, World!"` (solo espacios)
- `format_greeting("ana")` -> `"Hello, Ana!"`
- `format_greeting("Bob")` -> `"Hello, Bob!"` (ya capitalizado, sin cambios)
- `format_greeting("  bob  ")` -> `"Hello, Bob!"` (recorta espacios primero)
- `format_greeting("mcCARTHY")` -> `"Hello, McCARTHY!"` (solo el primer char cambia)
- `format_greeting("école")` -> `"Hello, École!"` (primer char multi-byte)

## Do / Don't
- DO: recortar (`trim`) antes de decidir si esta vacio y antes de capitalizar.
- DO: capitalizar SOLO el primer caracter (`chars().next()` + `to_uppercase()`),
  dejar el resto del string exactamente como vino.
- DON'T: usar `.to_lowercase()` sobre el resto del string.
- DON'T: indexar el string por bytes (`&name[0..1]`) -- rompe con UTF-8
  multi-byte; iterar por `char`.
- DON'T: agregar dependencias nuevas al `Cargo.toml`.

## Tests
Los tests estan en `tests_rs/format_greeting.rs` -- son el oraculo congelado,
sellado por `tests_sha256`. Se escribieron ANTES de delegar la implementacion.

## Constraints
- PARAR y reportar si `intent` resulta imposible de cumplir sin violar
  `touch_only` (solo `src/greeting.rs`; `src/main.rs`/`src/lib.rs` ya estan
  conectados y no deberian necesitar cambios).
- PARAR y reportar si hiciera falta una dependencia externa (crate) para
  resolver la capitalizacion UTF-8-safe.
