from .utils import validate_filename
from .cython.tnot import load_data as _load_data

__all__ = ("load_data",)

def load_data(filename : str):
    filename = validate_filename(filename)
    return _load_data(filename)
