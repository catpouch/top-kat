from top_kat import CountMinSketch

data = ["{:02x}".format(x) for x in range(128)]

def test_push():
    cms = CountMinSketch(.99, .002)
    for i, x in enumerate(data):
        cms.push(x, i)
    assert cms.get("00") == 0
    assert cms.get("3f") == 63
    assert cms.get("7f") == 127

# def test_union():
#     cms = CountMinSketch(.99, .002)
#     for i, x in enumerate(data):
#         cms.push(x, i)
#     cms.union_assign("00", 100)
#     cms.union_assign("7f", 10)
#     assert cms.get("00") == 100
#     assert cms.get("7f") == 127

def test_clear():
    cms = CountMinSketch(.99, .002)
    for i, x in enumerate(data):
        cms.push(x, i)
    assert cms.get("7f") == 127
    cms.clear()
    assert cms.get("7f") == 0