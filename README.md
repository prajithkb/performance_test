# Performance tests 
This crate is my attempt to measure the cache size of my mac inspired by the article ["What every programmer should know about memory"](https://lwn.net/Articles/252125/).

## Results
It looks like my CPU with the following specification

```
Model Name:	MacBook Pro
Processor Name:	6-Core Intel Core i7
Processor Speed:	2.6 GHz
Number of Processors:	1
Total Number of Cores:	6
L2 Cache (per Core):	256 KB
L3 Cache:	12 MB
Hyper-Threading Technology:	Enabled
Memory:	16 GB
```

shows the following results

### Violin plot
![Violin plot](./bench_results/Size%20in%20Bytes%20(power%20of%202)/report/violin.svg)


### Line plot
![Line  plot](./bench_results/Size%20in%20Bytes%20(power%20of%202)/report/lines.svg)

Shows that at until `2^8` the lookups are constant as they stay within the CPU L2 Cache line (256 KB). 

The max call stack size is 8MB and that is less that 12 MB L3 Cache. So I could not test the scenario where lookups needed to be done from Memory. That is why we see just one step instead of two.

