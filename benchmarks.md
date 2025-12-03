
## Benchmark
-t all\
input: 4k jpg\
output: 4k jpeg
-----------------------------------------------
## Results
|  Metric  |  v1.0  |   Q100    | Final(Q90) |   Speedup    |
|:--------:|:------:|:---------:|:----------:|:------------:|
|  Total   | 10.44s |   2.59s   |   2.31s    | 4.52x (352%) |
| Process  | 1.99s  |   1.33s   |   1.33s    |  1.5x (50%)  |
|   Save   | 7.98s  |   1.13s   |   0.84s    | 9.5x (850%)  |
| ms/image | 497ms  |   123ms   |   110ms    |              |


-----------------------------------------------
## Logs
v1.0
 
    Performance: (Q100)
      Total:    10.44s
      Load:      0.14s
      Process:   1.99s
      Save:      7.96s  
-----------------------------------------------
Changes:
- Palette precast to f32


    Performance: (Q100)
      Total:     9.47s  (10% speedup)
      Load:      0.13s
      Process:   1.33s  (50% speedup)
      Save:      7.98s   
-----------------------------------------------
Changes:
- Palette precast to f32
- Palette lookup with k-d-tree (Kiddo)


    Performance: (Q100)
      Total:     9.38s  (11% speedup)
      Load:      0.13s
      Process:   1.24s  (60% speedup)
      Save:      8.03s   
-----------------------------------------------
Changes:
- Palette precast to f32
- Palette lookup with k-d-tree (Kiddo)
- Improved jpeg encoding with jpeg-encoder


    Performance: (Q100)
      Total:     5.85s  (78% speedup)
      Load:      0.14s
      Process:   1.31s  (52% speedup)
      Save:      4.40s  (81% speedup)
-----------------------------------------------
Changes:
- Palette precast to f32
- Palette lookup with k-d-tree (Kiddo)
- Replaced jpeg-encoder with turbojpeg


    Performance: (Q100)
      Total:  2.59s  (303% speedup)
      Load:      0.13s
      Process:   1.33s  (50% speedup)
      Save:   1.13s  (606% speedup)

    Performance: (Q90)
      Total:     2.31s  (352% speedup)
      Load:      0.13s
      Process:   1.33s  (50% speedup)
      Save:      0.84s  (850% speedup)
-----------------------------------------------