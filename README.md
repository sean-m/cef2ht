```
$ ./cef2ht -h
It might be faster than Python

Usage: cef2ht [OPTIONS]

Options:
  -g, --gunzip    Expect stdin to be gzip compressed
  -p, --preserve  Preserve event in rawEvent key
  -h, --help      Print help
  -V, --version   Print version


$ time cargo run --release -- -g -p < 10k.cef.gz > /dev/null
    Finished `release` profile [optimized] target(s) in 0.06s
     Running `/home/sean/Projects/cef2ht/target/release/cef2ht -g -p`

real    0m0.245s
user    0m0.193s
sys     0m0.034s
$ time cargo run --release -- -g -p < 1m.cef.gz > /dev/null
    Finished `release` profile [optimized] target(s) in 0.04s
     Running `/home/sean/Projects/cef2ht/target/release/cef2ht -g -p`

real    0m10.857s
user    0m10.549s
sys     0m0.308s
```