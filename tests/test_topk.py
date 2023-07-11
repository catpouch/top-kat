from top_kat import TopK

data = ["{:02x}".format(x) for x in range(128)]

def test_push():
    topk = TopK(5, .99, .002)
    for i, x in enumerate(data):
        topk.push(x, i + 1)
    assert topk.top() == [("7f", 128), ("7e", 127), ("7d", 126), ("7c", 125), ("7b", 124)]

def test_clear():
    topk = TopK(5, .99, .002)
    for i, x in enumerate(data):
        topk.push(x, i + 1)
    assert topk.top() != []
    topk.clear()
    assert topk.top() == []

def test_capacity():
    topk = TopK(48, .99, .002)
    assert topk.capacity() == 48