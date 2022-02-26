
use pyo3::prelude::*;
use ndarray::prelude::*;

#[pyclass]
struct MyStruct {
    arr0:Vec<i64>,
    arr1:Vec<i64>
}

#[pymethods]
impl MyStruct{
    fn mult(&self) -> PyResult<Vec<i64>>{
        assert_eq!(self.arr0.len(), self.arr1.len());
        let mut vec = Vec::new();
        for i in 0..self.arr0.len(){
            vec.push(self.arr0[i] * self.arr1[i]);
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
    // m.add_class(wrap_pyfunction!(MyStruct, m)?)?;
    Ok(())
}