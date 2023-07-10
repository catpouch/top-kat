from top_kat import HyperLogLog
import pickle
# from random import randint

f = open("benchmarks/hll_data", "rb")

data = pickle.load(f)

hll1 = HyperLogLog(0.00408)
hll2 = HyperLogLog(0.00408)

hll_union = HyperLogLog(0.00408)
hll_intersect = HyperLogLog(0.00408)

for x in data[:len(data)//2]:
    hll1.push(x)
    hll_union.push(x)
    hll_intersect.push(x)

for x in data[len(data)//2:]:
    hll2.push(x)

print(f"1 cardinality: {hll1.len()}")
print(f"2 cardinality: {hll2.len()}")

hll_union.union(hll2)

print(f"union cardinality: {hll_union.len()}")

hll_intersect.intersect(hll2)

print(f"intersect cardinality: {hll_intersect.len()}")

print(f"empty?: {hll1.is_empty()}")

hll1.clear()

print(f"cleared cardinality: {hll1.len()}")
print(f"empty?: {hll1.is_empty()}")

# data = []

# for _ in range(100000):
#     val = randint(0,1000000)
#     for _ in range(randint(1,100)):
#         data.append(val)

# f = open("benchmarks/hll_data", "wb")
# pickle.dump(data, f)
# f.close()