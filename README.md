# Polynomial Surface fitting

I did this google search:
  * https://www.google.com/search?q=how+many+degrees+of+a+polynoimal+need+to+fit+a+relative+smooth+3d+surface

And found this:
  * https://stackoverflow.com/questions/54854675/how-to-fit-a-set-of-3d-data-points-using-a-third-or-higher-degree-of-polynomial

And asked ChatGPt-4:
  * [How to fit a set of 3D data points using a third or higher degree of polynomial surface regression?](https://chat.openai.com/share/1d6ab013-9bf9-4b30-90c8-bcfd08294e2b)

I also had ask ChatGPT-4:
  * [Generate a set of random 3d data points to represent a 3d surface in rust.](https://chat.openai.com/share/fc128866-2127-4de3-9c39-8783caf3a6b8)

And the result is what's here :)

## Build

```bash
cargo build
```

## Run

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/polynomial-surface-fitting`
VecStorage { data: [0.0001436649656282706, -0.018571507454307974, 0.01156923062935847, -0.0019180398436506353, 0.2266817668137218, 1.9850407470434854, -0.6863367321830653, -29.095735742737773, -46.18946398851716, 886.7108683582592], nrows: Dyn(10), ncols: Const }
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
