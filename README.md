<h1 align="center">szimg</h1>
<p align="center">Light weight image library which focus on plotting!</p>

# Overview

type `cargo run --example <example_name>` in your terminal to see these example images crafted by our library.

<table>
  <tr>
    <td align="center">
      <img
           src="./asset/mandlebrot.png"
           width="256"
           style="margin-bottom: -4px; border-radius: 8px;"
           alt="mandlebrot"
      />
    </td>
    <td align="center">
      <img
           src="./asset/julia_set.png"
           width="256"
           style="margin-bottom: -4px; border-radius: 8px;"
           alt="julia_set"
           />
    </td>
    <td align="center">
      <img
           src="./asset/barnsley_fern.png"
           width="256"
           style="margin-bottom: -4px; border-radius: 8px;"
           alt="barnsley_fern"
           />
    </td>
  </tr>
  <tr>
    <td align="center">
      <a href="./examples/mandlebrot.rs">mandlebrot</a>
    </td>
    <td align="center">
      <a href="./examples/julia.rs">julia_set</a>
    </td>
    <td align="center">
      <a href="./examples/barnsley_fern.rs">barnsley_fern</a>
    </td>
  </tr>
</table>



# Usage

To save a specified format image file, you need to prepare your data in Rust's raw multi-dimension array. That is because each channel data of your image file **must have same size**(both width and height). With raw array Rust compiler can easily check if your data have satisfied this constraint.

For example, if you want to save a PNG format file:

```rust
use szimg::png::save_png;

fn get_rgb_data() -> [[[u8; 256]; 256]; 3] {
  let R = [(0..=255).collect::<Vec<_>>().try_into().unwrap(); 256];
  let G = (0..=255).map(|x| [x; 256]).collect::<Vec<_>>().try_into().unwrap();
  let B = [[128; 256]; 256];

  [R, G, B]
}

fn main() {
  let data = get_rgb_data();
  save_png("rgb.png", data).unwrap();
}
```

You will get:

<p align="center"><img src="asset/rgb.png"></p>

For more exmaples you can check the test folder. In the near future the cargo doument will be supported as well.



# Roadmap

- [x] Netpbm
- [x] PNG
- [x] JPEG
- [ ] GIF
- [ ] BMP
- [ ] TIFF
- [ ] AVIF

All pull requests of other format images are welcome.




# License

[Apache-2.0 License](LICENSE)

Copyright (©) 2021 Sh-Zh-7

