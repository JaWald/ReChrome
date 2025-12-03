## Benchmark

-----------------------------------------------
### Performed test
-t all\
input: 4k jpg\
output: 4k png
-----------------------------------------------
v1.0

    Performance:
      Total:     3.69s
      Load:      0.14s
      Process:   1.99s
      Save:      1.56s
-----------------------------------------------
Changes:
- Palette precast to f32


    Performance:
      Total:     3.05s  (21% increase)
      Load:      0.12s
      Process:   1.33s  (50% increase)
      Save:      1.58s
-----------------------------------------------
Changes:
- Palette precast to f32
- Palette lookup with k-d-tree (Kiddo)


    Performance:
      Total:     2.94s  (25% increase)
      Load:      0.13s
      Process:   1.24s  (60% increase)
      Save:      1.58s
-----------------------------------------------