# Sumcheck Ingonyama

## What is the Sumcheck Protocol?

The **Sumcheck protocol** is a foundational interactive proof system in zero-knowledge cryptography. It allows a prover to convince a verifier that the sum of a multivariate polynomial  
$f(x_1, \dots, x_n)$ evaluated over the Boolean hypercube $\{0,1\}^n$ equals a claimed value $S$, without revealing the polynomial itself.

This protocol is a core building block in modern ZK systems, including:
- SNARKs (e.g., Plonk, Halo2)
- Folding schemes (Nova, HyperNova)
- Lookup arguments and other advanced primitives

The key idea is to reduce a high-dimensional sum into a sequence of univariate polynomial checks, making verification efficient.

---

## Implementation Details

This repository contains an implementation of the **multilinear Sumcheck protocol** over a custom 64-bit prime field using `ark-ff`.

- The polynomial is represented via its **Multilinear Extension (MLE)** over a lookup table of size $2^n$
- Includes:
  - Prover (generates round messages)
  - Verifier (checks correctness of the claim)
- Follows the classical interactive Sumcheck protocol used in ZK literature

---

## Correctness

The implementation has been manually verified on small instances (e.g., $n = 3$) against independently computed sums.

- The verifier **accepts valid proofs**
- The verifier **rejects tampered proofs**
- All computations are performed within the finite field
- Random field elements are used for verifier challenges

---

## System Configuration

Benchmarks were executed on:

- **CPU:** AMD Ryzen 5 5500U  
- **RAM:** 8 GB  
- **Architecture:** x86_64  
- **Execution Mode:** Single-threaded  

---

## CPU Benchmark Results (Single Iteration)

The following benchmarks measure the performance of the Sumcheck protocol for different input sizes.

---

### 1. n = 5 (32 points)
- **Table Creation Time:** 9.267 µs  
- **Protocol Execution Time:** 734.723 µs  

---

### 2. n = 8 (256 points)
- **Table Creation Time:** 39.022 µs  
- **Protocol Execution Time:** 31.141802 ms  

---

### 3. n = 10 (1024 points)
- **Table Creation Time:** 123.588 µs  
- **Protocol Execution Time:** 482.88468 ms  

---

### 4. n = 12 (4096 points)
- **Table Creation Time:** 491.849 µs  
- **Protocol Execution Time:** 7.635135158 s  

---

##  Note

- Table creation scales approximately **linearly** with input size  
- Protocol execution grows **exponentially**, consistent with $2^n$ evaluations  
- Benchmarks represent a **single iteration** of the protocol  
- Minor variations may occur due to randomness in input generation  

---

##  License

This project is licensed under the **MIT License**. See the `LICENSE` file for details.
