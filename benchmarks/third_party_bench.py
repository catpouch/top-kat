from hyperloglog import HyperLogLog
import pickle

f = open("benchmarks/data", "rb")

data = pickle.load(f)

hll = HyperLogLog(0.00408)
for x in data:
    hll.add(x)

print(len(hll))