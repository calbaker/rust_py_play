// Things to figure out (TODO):
// - [x] pass stuff into Rust as i32 and lists of these
// - [x] convert to ndarray and run sim
// - [x] use serde to serialize to pass EVERYTHING back to python if desired


use pyo3::prelude::*;
use serde::Serialize;
use ndarray::prelude::*;
// use std::collections::HashMap;

#[derive(Serialize)]


#[pyclass] 
struct MyStruct {
    #[pyo3(get, set)]
    c:i64,
    arr:Array1<i64>,
}

#[pymethods]
impl MyStruct{
    #[new]
    fn __new__(c:i64, arr:Vec<i64>) -> Self {
        let arr = Array::from_vec(arr); // this allows us to use the pass vec as an ndarray
        MyStruct { c, arr }
    }    

    fn mult(&self) -> PyResult<Vec<i64>>{
        Ok((&self.arr * self.c).to_vec())
    }

    fn to_json_str(&self) -> PyResult<String>{
        Ok(serde_json::to_string(self).unwrap())
    }
}


// Call the struct using:
// import py_play
// import numpy as np
// my_struct = py_play.MyStruct(c=5, arr=np.arange(5)) # totally ok to pass in np.ndarray or list
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