# mymodule.pyx (Cython module)
# Set the language_level directive to use Python 3 syntax
#cython: language_level=3
#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION

from libc.stdio cimport *                                                                

import numpy as np
cimport numpy as np

def load_data(filename):
    cdef FILE* cfile
    cdef np.uint16_t size
    cdef np.ndarray[np.uint16_t, ndim=1] data
    cdef np.ndarray[np.uint16_t, ndim=1] meta

    cfile = fopen(filename.encode(), "rb")
    meta = np.empty(3, dtype=np.uint16)
    # Read the RESULT, WELO, BELO, and  values
    fread(meta.data, sizeof(np.uint16_t), 3, cfile)
    # Read the size of the array
    fread(&size, sizeof(np.uint16_t), 1, cfile)
    # Allocate memory for the arrays
    data = np.empty(size, dtype=np.uint16)
    # Read the data array
    fread(data.data, sizeof(np.uint16_t), size, cfile)
    # Return the loaded data as a tuple of NumPy arrays

    return meta, data
