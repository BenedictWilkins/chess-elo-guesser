# setup.py

from setuptools import setup, find_packages, Extension
from Cython.Build import cythonize
import numpy as np

lan_extension = Extension("data_loader.cython.lan", ["data_loader/cython/lan_cython_loader.pyx"], include_dirs=[np.get_include()])
tnot_extension = Extension("data_loader.cython.tnot", ["data_loader/cython/tnot_cython_loader.pyx"], include_dirs=[np.get_include()])

setup(
    name = "chess_elo_guesser",
    version = "0.0.1",
    ext_modules=cythonize([lan_extension, tnot_extension]),
    include_dirs=[np.get_include()],
    packages=find_packages(),
    #package_data={'data_loader.cython': ['myExternalLib.so']},

)
