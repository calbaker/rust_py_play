# Useful Links
[PYO3 Python Classes in Rust](https://pyo3.rs/master/class.html#object-properties-using-pyo3get-set)  
[Calling Rust from Python using PyO3](https://saidvandeklundert.net/learn/2021-11-18-calling-rust-from-python-using-pyo3/)  

# How to use
1. Assuming you've cloned this repo and have Python3 (e.g. Anaconda) and Rust installed, run `pip install maturin`.
2. In the repo folder, run `maturin develop`.  This will also pip install the package.  
3. Fire up ipython or any interpreter.
4. Run the following commands:
   1. `import py_play`
   2. `import numpy as np`
   3. `my_struct = py_play.MyStruct(c=5, arr=np.arange(5)) # totally ok to pass in np.ndarray or list`
   4. `my_struct.b # get`
   5. `my_struct.mult # getter`
   6. `my_struct.to_json_str()`
