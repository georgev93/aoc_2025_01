Time to complete: ~5 hours
Speed-optimized runtime: <=1.3ms (`perf stat [PROG]`)
Speed-optimized max heap: 316.8kB (`valgrind --trace-children=yes --tool=massif [PROG] && ms_print massif.out.* | head -40`)
Size-optimized executable size: 311kB
