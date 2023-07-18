
from .cython.lan import load_data as _load_data    
from .constants import TNOT_EXTENSION, ROLES, PROMOTION, SQUARES
from .utils import validate_filename
import numpy as np
__all__ = ("load_data")

def load_data(filename : str):
    # TODO move below into cython for efficiency? 
    filename = validate_filename(filename)
    data = _load_data(filename)
    return [np.stack([ROLES[x[:,0]], PROMOTION[x[:,1]], SQUARES[x[:,2]], SQUARES[x[:,3]]], axis=1) for (m,x) in data]
        
