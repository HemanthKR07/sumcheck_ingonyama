# Sumcheck Ingonyama

## CPU Benchmark Results (Single Iteration)

The following benchmarks measure the performance of the Sumcheck protocol implementation on a CPU for different input sizes.

---

### 1.  n = 5 (32 points)
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

## 📌 Notes
- Table creation scales relatively linearly with input size.
- Protocol execution shows exponential growth, consistent with the increase in evaluation domain size.
- These benchmarks represent a **single iteration** of the Sumcheck protocol.
- As the input table is randomly generated, there is minute difference every time we test for the same input size.

---
