from top_kat import TopK
from random import choice

topk = TopK(20, .99, .002)

print(f"capacity: {topk.capacity()}")

keys = ["{:02x}".format(x) for x in range(256)]

for _ in range(1000000):
    topk.push(choice(keys), 1)

print(topk.top())

topk.clear()

print(f"cleared: {topk.top()}")

# print(keys)