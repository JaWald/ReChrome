## Benchmark

-----------------------------------------------
### Performed test
-t all\
input: 4k jpg\
output: 4k jpeg at 100% quality
-----------------------------------------------
v1.0

    Performance:
      Total:    10.44s
      Load:      0.14s
      Process:   1.99s
      Save:      7.96s  
-----------------------------------------------
Changes:
- Palette precast to f32


    Performance:
      Total:     9.47s  (10% speedup)
      Load:      0.13s
      Process:   1.33s  (50% speedup)
      Save:      7.98s   
-----------------------------------------------
Changes:
- Palette precast to f32
- Palette lookup with k-d-tree (Kiddo)


    Performance:
      Total:     9.38s  (11% speedup)
      Load:      0.13s
      Process:   1.24s  (60% speedup)
      Save:      8.03s   
-----------------------------------------------
Changes:
- Palette precast to f32
- Palette lookup with k-d-tree (Kiddo)
- Improved jpeg encoding with jpeg-encoder


    Performance:
      Total:     5.85s  (78% speedup)
      Load:      0.14s
      Process:   1.31s  (52% speedup)
      Save:      4.40s  (81% speedup)
-----------------------------------------------