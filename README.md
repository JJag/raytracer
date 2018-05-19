## Introduction

Simple raytracer written in Rust. Based on ["Ray Tracing in One Weekend"](http://in1weekend.blogspot.com/2016/01/ray-tracing-in-one-weekend.html) by Peter Shirley

### Running the code

Easiest way to run is via `cargo run` command:
```bash
cargo run --release $WIDTH $HEIGHT $RAYS_PER_PIXEL > image.ppm
```

Alternatively you can use [imgcat](https://github.com/eddieantonio/imgcat) to display images directly in your terminal:
```bash
cargo run --release $WIDTH $HEIGHT $RAYS_PER_PIXEL | imgcat
```

### Example images

![image 1](https://raw.githubusercontent.com/JJag/raytracer/master/images/image1.png)
![image 2](https://raw.githubusercontent.com/JJag/raytracer/master/images/image2.png)
