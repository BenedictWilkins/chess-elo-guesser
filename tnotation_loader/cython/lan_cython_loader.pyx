# mymodule.pyx (Cython module)
# Set the language_level directive to use Python 3 syntax
#cython: language_level=3
#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION

from libc.stdio cimport *
from libc.stdint cimport uint16_t
from libc.stdlib cimport realloc, free

import numpy as np
cimport numpy as np

def load_data(filename):
    cdef FILE* cfile

    cfile = fopen(filename.encode(), "rb")
    cdef long end
    cdef np.ndarray[np.uint16_t, ndim=1] meta
    cdef np.ndarray[np.uint16_t, ndim=2] data
   
    data_blocks = []  # List to store the data blocks

    cdef uint16_t size
    cdef uint16_t *arr_ptr = NULL
    cdef uint16_t i, v # index, temp value
    cdef uint16_t promotion

    cdef uint16_t role_mask = 14        # ((1 << (3 - 1 + 1)) - 1) << 1
    cdef uint16_t from_mask = 1008      # ((1 << (9 - 4 + 1)) - 1) << 4
    cdef uint16_t to_mask = 64512       # ((1 << (15 - 10 + 1)) - 1) << 10

    while True:
        meta = np.empty(3, dtype=np.uint16)
        # Read the RESULT, WELO, BELO, and  values
        end = fread(meta.data, sizeof(np.uint16_t), 3, cfile)
        if end == 0:
            break

        # Read the size of the array
        fread(&size, sizeof(uint16_t), 1, cfile)
        
        arr_ptr = <uint16_t *>realloc(arr_ptr, size * sizeof(uint16_t))
        fread(arr_ptr, sizeof(uint16_t), size, cfile)
        
        # Allocate data
        data = np.empty((size, 4), dtype=np.uint16) 
        # scan arr_ptr to convert the values into numpy arrays
        for i in range(size):
            v = arr_ptr[i]
            promotion = (v & 1)
            data[i, promotion] = (v & role_mask) >> 1 # role
            data[i, 1-promotion] = 0                  # promotion
            data[i, 2] = (v & from_mask) >> 4         # from
            data[i, 3] = (v & to_mask) >> 10          # to

        # Append the meta and data as a tuple to the data_blocks list
        data_blocks.append((meta, data))

    free(arr_ptr)

    # Return the list of data blocks
    return data_blocks


