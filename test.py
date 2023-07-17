
from tnotation_loader import load_data
import time



data = load_data("./data/lichess_db_standard_rated_2013-01.tnot")
print(len(data))
data = load_data("./data/test.tnot")
print(len(data))

print(data)