import pathlib 
from .constants import TNOT_EXTENSION

def validate_filename(filename : str):
    path = pathlib.Path(filename)
    path = path.expanduser().resolve().absolute()
    if not path.exists():
        raise FileNotFoundError(f"File: {str(path)} does not exist.")
    if path.suffix != TNOT_EXTENSION:
        raise ValueError(f"File: {str(path)} does not have the correct file extension {TNOT_EXTENSION}")   
    return str(path)