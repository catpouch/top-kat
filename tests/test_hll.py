from top_kat import HyperLogLog
from datetime import datetime

data = list(range(10))

def test_push():
    hll = HyperLogLog(0.00408)
    for x in data:
        hll.push(x)
    assert round(hll.len()) == len(data)

def test_object_push():
    hll = HyperLogLog(0.00408)
    for i in range(10):
        hll.push(datetime(2023, 7, 13, i + 10, 52, 30, 100))
    assert round(hll.len()) == 10

def test_empty():
    hll = HyperLogLog(0.00408)
    assert hll.is_empty() == True
    hll.push("a")
    assert hll.is_empty() == False

def test_union():
    hll1 = HyperLogLog(0.00408)
    hll2 = HyperLogLog(0.00408)
    for x in data:
        hll1.push(x)
        hll2.push(x + 10)
    assert round(hll1.len()) == len(data)
    assert round(hll2.len()) == len(data)
    hll1.union(hll2)
    assert round(hll1.len()) == 2 * len(data)

def test_intersect():
    hll1 = HyperLogLog(0.00408)
    hll2 = HyperLogLog(0.00408)
    for x in data:
        hll1.push(x)
        hll2.push(x + 5)
    assert round(hll1.len()) == len(data)
    assert round(hll2.len()) == len(data)
    hll1.intersect(hll2)
    assert round(hll1.len()) == len(data) // 2

def test_clear():
    hll = HyperLogLog(0.00408)
    for x in data:
        hll.push(x)
    assert round(hll.len()) == len(data)
    assert hll.is_empty() == False
    hll.clear()
    assert hll.is_empty() == True