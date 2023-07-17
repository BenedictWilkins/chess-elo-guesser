

TNOTATION_EXTENSION = ".tnot"

def load_data(filename : str):
    from .tnotation_cython_loader import load_data as _load_data
    import pathlib 
    path = pathlib.Path(filename)
    path = path.expanduser().resolve().absolute()
    if not path.exists():
        raise FileNotFoundError(f"File: {str(path)} does not exist.")
    if path.suffix != TNOTATION_EXTENSION:
        raise ValueError(f"File: {str(path)} does not have the correct file extension {TNOTATION_EXTENSION}")   
    return _load_data(str(path))
