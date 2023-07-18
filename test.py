
from chess_elo_guesser.data_loader import tnot, lan
import time

data = lan.load_data("./data/test-(1).tnot")
print(data)
data = tnot.load_data("./data/test-(1).tnot")
print(data)

