import py_play
import numpy as np
my_struct = py_play.MyStruct(c=5, arr=np.arange(5)) # totally ok to pass in np.ndarray or list
print(my_struct.c) # get
print(my_struct.arr)
print(my_struct.mult())
print(my_struct.to_json_str())

# Note: an index of arr cannot be set directly, but here's a workaround
my_struct.set_arr_i(0, 22)
print(my_struct.arr)

# testing out inheritance
try:
    class ChildStruct(py_play.MyStruct):
        def minus_one(self):
            self.arr = self.arr - 1
except:
    print("Python Classes can't inherit from Rust Structs. :(")