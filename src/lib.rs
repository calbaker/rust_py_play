// Things to figure out (TODO):
// - [x] pass stuff into Rust as i32, 64, and lists of these
// convert to ndarray and run sim
// use serde to serialize classes so that you can programmatically change all 
// arrays back to pyO3-compliant lists
// pip stuff out


use pyo3::prelude::*;
use serde::Serialize;
use ndarray::prelude::*;


#[pyclass] 
#[derive(Serialize)]
struct MyStruct {
    a:i64,
    #[pyo3(get, set)]
    b:i64,
    c:Array1<i64>,
}

#[pymethods]
impl MyStruct{
    #[new]
    fn __new__(a:i64, b:i64, c:Vec<i64>) -> Self {
        let c = Array::from_vec(c);
        MyStruct { a, b, c}
    }    
    #[getter]
    fn mult(&self) -> PyResult<Vec<i64>>{
        let mut vec = Vec::new();
        for i in self.a..self.b{
            vec.push(i * self.c[i as usize]);
        }
        Ok(vec)
    }
}


// Call the class using:
// import py_play
// import numpy as np
// my_struct = py_play.MyStruct(a=0, b=5, c=np.arange(5)) # totally ok to pass in np.ndarray
// my_struct.b # get
// my_struct.mult # getter

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