
Plonk Script is a specialized scripting language specifically designed for Plonkish arithmetic. It serves as a domain-specific language (DSL) with the ability to interface with a variety of systems that utilize Plonkish arithmetic, including halo2. Built on the Rhai programming language, Plonk Script adopts a syntax similar to Rust but without explicit typing, thereby reducing code redundancy.

## Motivation:

Our motivation for developing Plonk Script stems from three main issues:
1. Poor descriptiveness: The use of the raw halo2 API often leads to verbose and unclear code, making debugging and analysis difficult.
2. Lack of metaprogramming support: Existing solutions such as AirScript and Vamp-IR lack basic witness computation capabilities, necessitating the use of external scripts for secondary generation of scripts or input values.
3. Absence of analysis tools: Verifying the correctness of business logic constraints is often challenging.

## Expressiveness:

Plonk Script significantly improves code readability. For instance, a simple Fibonacci sequence coded in halo2 API spans over 200 lines. The equivalent Plonk Script, however, is a mere 31 lines, making it instantly comprehensible.

## Metaprogramming:

Plonk Script supports regular programming constructs like loops and conditionals for witness generation. It also allows for assignment and constraint notation using symbols like `<==`.