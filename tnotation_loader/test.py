
from tnotation_loader_iter import load_data 
import time

t0 = time.perf_counter()
data = load_data("/home/ben/Documents/repos/chess-elo-guesser/parser/test.bin")
t1 = time.perf_counter()
print("TIME:", t1-t0, "COUNT:", len(data))