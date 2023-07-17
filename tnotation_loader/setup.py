# setup.py

from setuptools import setup
from Cython.Build import cythonize
import numpy as np

setup(
    name = "tnotation_loader",
    version = "0.0.1",
    ext_modules=cythonize("cython/tnotation_cython_loader.pyx"),
    include_dirs=[np.get_include()],
)
