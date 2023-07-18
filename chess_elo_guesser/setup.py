# setup.py

from setuptools import setup, find_packages
from Cython.Build import cythonize
import numpy as np

setup(
    name = "chess_elo_guesser",
    version = "0.0.1",
    ext_modules=cythonize(["data_loader/cython/lan_cython_loader.pyx", "data_loader/cython/tnot_cython_loader.pyx"]),
    include_dirs=[np.get_include()],
    packages=find_packages(),
)
