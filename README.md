# det-rs
Calculate the determinant of a square matrix using a Rust macro

This macro should not be used directly. Instead, it should be called inside a function that is used elsewhere.

## Usage
Calculate the area of a parallelogram:
```rust
pub fn find_orthonormal_4(a: &Vector2<f32>, b: &Vector2<f32>) -> f32 {
    det_copy!(a.x, a.y,
              b.x, b.y)
}
```

Calculate a 4D orthogonal vector using nalgebra:
```rust
pub fn find_orthonormal_4(a: &Vector4<f32>, b: &Vector4<f32>, c: &Vector4<f32>) -> Vector4<f32> {
    det_copy!(Vector4::x(), Vector4::y(), Vector4::z(), Vector4::w(),
              a.x,          a.y,          a.z,          a.w,
              b.x,          b.y,          b.z,          b.w,
              c.x,          c.y,          c.z,          c.w         )
}
```
Usually `det!` should be used in these cases, but strangely, it causes a type recursion error,
so we use `det_copy!` instead. Note, that the constructors (eg. `Vector4::x()`) are executed multiple times.
To avoid this, bind the vectors to a variable first and pass in the variable instead.


## Requirements
There are various macros for various types of values.

- `det_copy!`
  1. `T: Copy`
  2. `T: Mul<T, Output=T>`
  3. `T: Sub<T, Output=T>`
  4. `T: Add<T, Output=T>`
- `det_clone!`
  1. `T: Clone`
  2. `T: Mul<T, Output=T>`
  3. `T: Sub<T, Output=T>`
  4. `T: Add<T, Output=T>`
- `det!`
  1. `T: Mul<T, Output=T>`
  2. `for<'a> &'a T: Mul<T, Output=T>`
  3. `for<'a, 'b> &'a T: Mul<&'b T, Output=T>`
  4. `T: Sub<T, Output=T>`
  5. `T: Add<T, Output=T>`
