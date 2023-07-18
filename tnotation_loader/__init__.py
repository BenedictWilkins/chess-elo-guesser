
from .constants import TNOT_EXTENSION, ROLES, PROMOTION, SQUARES

__all__ = ("load_tnot_data", "load_lan_data", "TNOT_EXTENSION", "PGN_EXTENSION", "ROLES", "PROMOTION", "SQUARES")


def load_tnot_data(filename : str):
    from .tnot_cython_loader import load_data as _load_data
    import pathlib 
    path = pathlib.Path(filename)
    path = path.expanduser().resolve().absolute()
    if not path.exists():
        raise FileNotFoundError(f"File: {str(path)} does not exist.")
    if path.suffix != TNOT_EXTENSION:
        raise ValueError(f"File: {str(path)} does not have the correct file extension {TNOT_EXTENSION}")   
    return _load_data(str(path))


def load_lan_data(filename : str):
    # TODO move below into cython for efficiency? 
    from .lan_cython_loader import load_data as _load_data    
    import numpy as np
    data = _load_data(filename)
    return [np.stack([ROLES[x[:,0]], PROMOTION[x[:,1]], SQUARES[x[:,2]], SQUARES[x[:,3]]], axis=1) for (m,x) in data]
        

