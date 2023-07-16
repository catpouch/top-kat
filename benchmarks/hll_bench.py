from top_kat import HyperLogLog
import pickle
# from random import randint

f = open("benchmarks/hll_data", "rb")

data = pickle.load(f)

hll = HyperLogLog(0.00408)

for x in data:
    hll.push(x)

print(hll.len())

# data = []

# for _ in range(100000):
#     val = randint(0,1000000)
#     for _ in range(randint(1,100)):
#         data.append(val)

# f = open("benchmarks/hll_data", "wb")
# pickle.dump(data, f)
# f.close()