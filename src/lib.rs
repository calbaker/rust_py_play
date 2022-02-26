// Things to figure out (TODO):
// pass stuff into Rust as i32, 64, and lists of these
// convert to ndarray and run sim
// use serde to serialize classes so that you can programmatically change all 
// arrays back to pyO3-compliant lists
// pip stuff out


use pyo3::prelude::*;
use ndarray::prelude::*;

#[pyclass]
struct MyStruct {
    a:i64,
    b:i64,
    c:Vec<i64>,
}

#[pymethods]
impl MyStruct{
    #[new]
    fn __new__(a:i64, b:i64, c:Vec<i64>) -> Self {
        MyStruct { a, b, c }
    }    
    fn mult(&self) -> PyResult<Vec<i64>>{
        let mut vec = Vec::new();
        for i in self.a..self.b{
            vec.push(i * self.c[i as usize]);
        }
        Ok(vec)
    }
}


/// Formats the sum of two numbers as string.
#[pyfunction]
fn array_square(a: i64, b: i64) -> PyResult<Vec<i64>> {
    let mut vec = Vec::new();
    for i in a..b {
        vec.push(i.pow(3));
    }
    Ok(vec)
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_play(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(array_square, m)?)?;
    m.add_class::<MyStruct>()?;
    Ok(())
}