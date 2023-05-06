# ray_tracer

Working through _The Ray Tracer Challenge_ book, by Jamis Buck,
implementing a 3D Renderer.

This Ray Tracer is the first program I have written in Rust, so the
module structure of the program is probably not the best,
and there are a lot of `foo.clone()` calls to keep the memory handling
and borrow checking as simple as possible.

The follow commands download the source code,
build and run the program, then display the
PPM image output file on a Linux system:

    $ git clone https://github.com/simoc/ray_tracer.git
    $ cd ray_tracer
    $ cargo build --release
    $ ./target/release/ray_tracer > a.ppm
    $ display a.ppm

## Completed Chapters

- [x] Chapter 1 - Tuples, Points, and Vectors
- [x] Chapter 2 - Drawing on a Canvas
- [x] Chapter 3 - Matrices
- [x] Chapter 4 - Matrix Transformations
- [x] Chapter 5 - Ray-Sphere Intersections
- [x] Chapter 6 - Light and Shading
- [x] Chapter 7 - Making a Scene
- [x] Chapter 8 - Shadows
- [x] Chapter 9 - Planes
- [x] Chapter 10 - Patterns
- [x] Chapter 11 - Reflection and Refraction (several tests fail with probable floating point accuracy problem)
- [x] Chapter 12 - Cubes
- [ ] Chapter 13 - Cylinders
- [ ] Chapter 14 - Groups
- [ ] Chapter 15 - Triangles
- [ ] Chapter 16 - Constructive Solid Geometry (CSG)
- [ ] Chapter 17 - Next Steps
- [ ] Appendix 1 - Rendering the Cover Image

