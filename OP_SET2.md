
## Run on MacOS Apple M1 11/22/2024 automerge "op\_set2" branch

```
bestiary                             fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ load                                            │               │               │               │         │
│  ├─ ./benches/embark.automerge     283 ms        │ 311.5 ms      │ 295.2 ms      │ 298 ms        │ 10      │ 10
│  │                                 max alloc:    │               │               │               │         │
│  │                                   124.2 MB    │ 124.2 MB      │ 124.2 MB      │ 124.2 MB      │         │
│  │                                 alloc:        │               │               │               │         │
│  │                                   1829232     │ 1829232       │ 1829232       │ 1829232       │         │
│  │                                   331.9 MB    │ 331.9 MB      │ 331.9 MB      │ 331.9 MB      │         │
│  │                                 dealloc:      │               │               │               │         │
│  │                                   1829232     │ 1829232       │ 1829232       │ 1829232       │         │
│  │                                   501.2 MB    │ 501.2 MB      │ 501.2 MB      │ 501.2 MB      │         │
│  │                                 grow:         │               │               │               │         │
│  │                                   571440      │ 571440        │ 571440        │ 571440        │         │
│  │                                   169.2 MB    │ 169.2 MB      │ 169.2 MB      │ 169.2 MB      │         │
│  ╰─ ./benches/moby-dick.automerge  325.1 ms      │ 337.4 ms      │ 331.4 ms      │ 331.3 ms      │ 9       │ 9
│                                    max alloc:    │               │               │               │         │
│                                      440.9 MB    │ 440.9 MB      │ 440.9 MB      │ 440.9 MB      │         │
│                                    alloc:        │               │               │               │         │
│                                      1245858     │ 1245858       │ 1245858       │ 1245858       │         │
│                                      402.5 MB    │ 402.5 MB      │ 402.5 MB      │ 402.5 MB      │         │
│                                    dealloc:      │               │               │               │         │
│                                      1245858     │ 1245858       │ 1245858       │ 1245858       │         │
│                                      732.2 MB    │ 732.2 MB      │ 732.2 MB      │ 732.2 MB      │         │
│                                    grow:         │               │               │               │         │
│                                      623         │ 623           │ 623           │ 623           │         │
│                                      329.7 MB    │ 329.7 MB      │ 329.7 MB      │ 329.7 MB      │         │
╰─ save                                            │               │               │               │         │
   ├─ ./benches/embark.automerge     29.57 ms      │ 59.17 ms      │ 33.36 ms      │ 34.21 ms      │ 88      │ 88
   │                                 max alloc:    │               │               │               │         │
   │                                   8.076 MB    │ 8.076 MB      │ 8.076 MB      │ 8.076 MB      │         │
   │                                 alloc:        │               │               │               │         │
   │                                   2193        │ 2193          │ 2193          │ 2193          │         │
   │                                   17.27 MB    │ 17.27 MB      │ 17.27 MB      │ 17.27 MB      │         │
   │                                 dealloc:      │               │               │               │         │
   │                                   2193        │ 2193          │ 2193          │ 2193          │         │
   │                                   18.63 MB    │ 18.63 MB      │ 18.63 MB      │ 18.63 MB      │         │
   │                                 grow:         │               │               │               │         │
   │                                   514         │ 514           │ 514           │ 514           │         │
   │                                   1.351 MB    │ 1.351 MB      │ 1.351 MB      │ 1.351 MB      │         │
   ╰─ ./benches/moby-dick.automerge  50.07 ms      │ 130.2 ms      │ 52.4 ms       │ 53.81 ms      │ 56      │ 56
                                     max alloc:    │               │               │               │         │
                                       7.272 MB    │ 7.272 MB      │ 7.272 MB      │ 7.272 MB      │         │
                                     alloc:        │               │               │               │         │
                                       92          │ 92            │ 92            │ 92            │         │
                                       6.305 MB    │ 6.305 MB      │ 6.305 MB      │ 6.305 MB      │         │
                                     dealloc:      │               │               │               │         │
                                       92          │ 92            │ 92            │ 92            │         │
                                       12.47 MB    │ 12.47 MB      │ 12.47 MB      │ 12.47 MB      │         │
                                     grow:         │               │               │               │         │
                                       42          │ 42            │ 42            │ 42            │         │
                                       6.168 MB    │ 6.168 MB      │ 6.168 MB      │ 6.168 MB      │         │

edit_trace               fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ edit_trace_many_tx    10.06 s       │ 10.06 s       │ 10.06 s       │ 10.06 s       │ 1       │ 1
│                        max alloc:    │               │               │               │         │
│                          286.5 MB    │ 286.5 MB      │ 286.5 MB      │ 286.5 MB      │         │
│                        alloc:        │               │               │               │         │
│                          62407332    │ 62407332      │ 62407332      │ 62407332      │         │
│                          36.61 GB    │ 36.61 GB      │ 36.61 GB      │ 36.61 GB      │         │
│                        dealloc:      │               │               │               │         │
│                          61644606    │ 61644606      │ 61644606      │ 61644606      │         │
│                          36.66 GB    │ 36.66 GB      │ 36.66 GB      │ 36.66 GB      │         │
│                        grow:         │               │               │               │         │
│                          2307587     │ 2307587       │ 2307587       │ 2307587       │         │
│                          328.6 MB    │ 328.6 MB      │ 328.6 MB      │ 328.6 MB      │         │
╰─ edit_trace_single_tx  4.436 s       │ 4.436 s       │ 4.436 s       │ 4.436 s       │ 1       │ 1
                         max alloc:    │               │               │               │         │
                           121 MB      │ 121 MB        │ 121 MB        │ 121 MB        │         │
                         alloc:        │               │               │               │         │
                           18163431    │ 18163431      │ 18163431      │ 18163431      │         │
                           3.06 GB     │ 3.06 GB       │ 3.06 GB       │ 3.06 GB       │         │
                         dealloc:      │               │               │               │         │
                           17971268    │ 17971268      │ 17971268      │ 17971268      │         │
                           3.11 GB     │ 3.11 GB       │ 3.11 GB       │ 3.11 GB       │         │
                         grow:         │               │               │               │         │
                           233360      │ 233360        │ 233360        │ 233360        │         │
                           91.05 MB    │ 91.05 MB      │ 91.05 MB      │ 91.05 MB      │         │

load_save                    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ load                                    │               │               │               │         │
│  ├─ BigPaste               94.95 µs      │ 614.1 µs      │ 98.26 µs      │ 112.2 µs      │ 100     │ 100
│  │                         max alloc:    │               │               │               │         │
│  │                           81.76 KB    │ 81.76 KB      │ 81.76 KB      │ 81.76 KB      │         │
│  │                         alloc:        │               │               │               │         │
│  │                           118         │ 118           │ 118           │ 118           │         │
│  │                           126 KB      │ 126 KB        │ 126 KB        │ 126 KB        │         │
│  │                         dealloc:      │               │               │               │         │
│  │                           118         │ 118           │ 118           │ 118           │         │
│  │                           192.3 KB    │ 192.3 KB      │ 192.3 KB      │ 192.3 KB      │         │
│  │                         grow:         │               │               │               │         │
│  │                           34          │ 34            │ 34            │ 34            │         │
│  │                           66.36 KB    │ 66.36 KB      │ 66.36 KB      │ 66.36 KB      │         │
│  ├─ DeepHistory            26.5 ms       │ 34.05 ms      │ 27.84 ms      │ 28.07 ms      │ 100     │ 100
│  │                         max alloc:    │               │               │               │         │
│  │                           15.11 MB    │ 15.11 MB      │ 15.11 MB      │ 15.11 MB      │         │
│  │                         alloc:        │               │               │               │         │
│  │                           194901      │ 194901        │ 194901        │ 194901        │         │
│  │                           34.13 MB    │ 34.13 MB      │ 34.13 MB      │ 34.13 MB      │         │
│  │                         dealloc:      │               │               │               │         │
│  │                           194901      │ 194901        │ 194901        │ 194901        │         │
│  │                           50.04 MB    │ 50.04 MB      │ 50.04 MB      │ 50.04 MB      │         │
│  │                         grow:         │               │               │               │         │
│  │                           71111       │ 71111         │ 71111         │ 71111         │         │
│  │                           15.91 MB    │ 15.91 MB      │ 15.91 MB      │ 15.91 MB      │         │
│  ├─ MapsInMaps             3.439 ms      │ 4.773 ms      │ 3.501 ms      │ 3.6 ms        │ 100     │ 100
│  │                         max alloc:    │               │               │               │         │
│  │                           4.404 MB    │ 4.404 MB      │ 4.404 MB      │ 4.404 MB      │         │
│  │                         alloc:        │               │               │               │         │
│  │                           10108       │ 10108         │ 10108         │ 10108         │         │
│  │                           4.39 MB     │ 4.39 MB       │ 4.39 MB       │ 4.39 MB       │         │
│  │                         dealloc:      │               │               │               │         │
│  │                           10108       │ 10108         │ 10108         │ 10108         │         │
│  │                           7.848 MB    │ 7.848 MB      │ 7.848 MB      │ 7.848 MB      │         │
│  │                         grow:         │               │               │               │         │
│  │                           88          │ 88            │ 88            │ 88            │         │
│  │                           3.457 MB    │ 3.457 MB      │ 3.457 MB      │ 3.457 MB      │         │
│  ╰─ PoorlySimulatedTyping  22.16 ms      │ 27.54 ms      │ 23.83 ms      │ 24.04 ms      │ 100     │ 100
│                            max alloc:    │               │               │               │         │
│                              14.65 MB    │ 14.65 MB      │ 14.65 MB      │ 14.65 MB      │         │
│                            alloc:        │               │               │               │         │
│                              194891      │ 194891        │ 194891        │ 194891        │         │
│                              30.61 MB    │ 30.61 MB      │ 30.61 MB      │ 30.61 MB      │         │
│                            dealloc:      │               │               │               │         │
│                              194891      │ 194891        │ 194891        │ 194891        │         │
│                              46.63 MB    │ 46.63 MB      │ 46.63 MB      │ 46.63 MB      │         │
│                            grow:         │               │               │               │         │
│                              80111       │ 80111         │ 80111         │ 80111         │         │
│                              16.02 MB    │ 16.02 MB      │ 16.02 MB      │ 16.02 MB      │         │
╰─ save                                    │               │               │               │         │
   ├─ BigPaste               170.9 µs      │ 331.7 µs      │ 174.1 µs      │ 195.1 µs      │ 100     │ 100
   │                         max alloc:    │               │               │               │         │
   │                           379.8 KB    │ 379.8 KB      │ 379.8 KB      │ 379.8 KB      │         │
   │                         alloc:        │               │               │               │         │
   │                           38          │ 38            │ 38            │ 38            │         │
   │                           369.5 KB    │ 369.5 KB      │ 369.5 KB      │ 369.5 KB      │         │
   │                         dealloc:      │               │               │               │         │
   │                           37          │ 37            │ 37            │ 37            │         │
   │                           418.2 KB    │ 418.2 KB      │ 418.2 KB      │ 418.2 KB      │         │
   │                         grow:         │               │               │               │         │
   │                           23          │ 23            │ 23            │ 23            │         │
   │                           56.42 KB    │ 56.42 KB      │ 56.42 KB      │ 56.42 KB      │         │
   ├─ DeepHistory            4.749 ms      │ 7.061 ms      │ 4.924 ms      │ 5.132 ms      │ 100     │ 100
   │                         max alloc:    │               │               │               │         │
   │                           4.225 MB    │ 4.225 MB      │ 4.225 MB      │ 4.225 MB      │         │
   │                         alloc:        │               │               │               │         │
   │                           47          │ 47            │ 47            │ 47            │         │
   │                           5.429 MB    │ 5.429 MB      │ 5.429 MB      │ 5.429 MB      │         │
   │                         dealloc:      │               │               │               │         │
   │                           46          │ 46            │ 46            │ 46            │         │
   │                           5.786 MB    │ 5.786 MB      │ 5.786 MB      │ 5.786 MB      │         │
   │                         grow:         │               │               │               │         │
   │                           34          │ 34            │ 34            │ 34            │         │
   │                           392.8 KB    │ 392.8 KB      │ 392.8 KB      │ 392.8 KB      │         │
   ├─ MapsInMaps             2.129 ms      │ 3.296 ms      │ 2.175 ms      │ 2.258 ms      │ 100     │ 100
   │                         max alloc:    │               │               │               │         │
   │                           731.3 KB    │ 731.3 KB      │ 731.3 KB      │ 731.3 KB      │         │
   │                         alloc:        │               │               │               │         │
   │                           730         │ 730           │ 730           │ 730           │         │
   │                           1.235 MB    │ 1.235 MB      │ 1.235 MB      │ 1.235 MB      │         │
   │                         dealloc:      │               │               │               │         │
   │                           729         │ 729           │ 729           │ 729           │         │
   │                           1.537 MB    │ 1.537 MB      │ 1.537 MB      │ 1.537 MB      │         │
   │                         grow:         │               │               │               │         │
   │                           721         │ 721           │ 721           │ 721           │         │
   │                           342.5 KB    │ 342.5 KB      │ 342.5 KB      │ 342.5 KB      │         │
   ╰─ PoorlySimulatedTyping  1.423 ms      │ 2.914 ms      │ 1.554 ms      │ 1.65 ms       │ 100     │ 100
                             max alloc:    │               │               │               │         │
                               1.052 MB    │ 1.052 MB      │ 1.052 MB      │ 1.052 MB      │         │
                             alloc:        │               │               │               │         │
                               47          │ 47            │ 47            │ 47            │         │
                               1.714 MB    │ 1.714 MB      │ 1.714 MB      │ 1.714 MB      │         │
                             dealloc:      │               │               │               │         │
                               46          │ 46            │ 46            │ 46            │         │
                               1.76 MB     │ 1.76 MB       │ 1.76 MB       │ 1.76 MB       │         │
                             grow:         │               │               │               │         │
                               27          │ 27            │ 27            │ 27            │         │
                               53.43 KB    │ 53.43 KB      │ 53.43 KB      │ 53.43 KB      │         │

map                    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ decreasing_put                    │               │               │               │         │
│  ├─ 100              513.3 µs      │ 1.531 ms      │ 529.1 µs      │ 556.8 µs      │ 100     │ 100
│  ├─ 1000             5.363 ms      │ 7.105 ms      │ 5.452 ms      │ 5.577 ms      │ 100     │ 100
│  ╰─ 10000            55.99 ms      │ 62.95 ms      │ 57.32 ms      │ 57.65 ms      │ 52      │ 52
├─ increasing_put                    │               │               │               │         │
│  ├─ 100              651 µs        │ 1.076 ms      │ 671 µs        │ 694.8 µs      │ 100     │ 100
│  ├─ 1000             7.435 ms      │ 9.262 ms      │ 7.593 ms      │ 7.759 ms      │ 100     │ 100
│  ╰─ 10000            77.66 ms      │ 81.51 ms      │ 79.23 ms      │ 79.44 ms      │ 38      │ 38
├─ repeated_increment                │               │               │               │         │
│  ├─ 100              1.939 ms      │ 3.432 ms      │ 1.984 ms      │ 2.078 ms      │ 100     │ 100
│  ╰─ 1000             138.1 ms      │ 221.4 ms      │ 142.9 ms      │ 146 ms        │ 21      │ 21
╰─ repeated_put                      │               │               │               │         │
   ├─ 100              997.7 µs      │ 1.854 ms      │ 1.016 ms      │ 1.031 ms      │ 100     │ 100
   ├─ 1000             48.32 ms      │ 53.78 ms      │ 49.77 ms      │ 50.13 ms      │ 60      │ 60
   ╰─ 10000            4.416 s       │ 4.416 s       │ 4.416 s       │ 4.416 s       │ 1       │ 1

range         fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ range                    │               │               │               │         │
│  ╰─ 100000  18.86 ms      │ 22.8 ms       │ 19.31 ms      │ 19.54 ms      │ 100     │ 100
╰─ range_at                 │               │               │               │         │
   ╰─ 100000  18.93 ms      │ 95.08 ms      │ 19.64 ms      │ 20.54 ms      │ 100     │ 100

sync                 fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ sync                            │               │               │               │         │
   ├─ ManyTx(10000)  1.749 µs      │ 118.8 ms      │ 1.874 µs      │ 1.19 ms       │ 100     │ 100
   ╰─ OneTx(10000)   582.6 ns      │ 79.55 ms      │ 624.6 ns      │ 796.1 µs      │ 100     │ 100
```
