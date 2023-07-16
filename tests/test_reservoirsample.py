from top_kat import UnstableReservoirSample

def test_unseeded():
    sample_u = UnstableReservoirSample(10)
    for i in range(100):
        sample_u.push(i)
    res = sample_u.reservoir()
    assert len(res) == 10

def test_seeded():
    sample_u_1 = UnstableReservoirSample(10, 727)
    for i in range(100):
        sample_u_1.push(i)
    res_1 = sample_u_1.reservoir()

    sample_u_2 = UnstableReservoirSample(10, 727)
    for i in range(100):
        sample_u_2.push(i)
    res_2 = sample_u_2.reservoir()

    sample_u_3 = UnstableReservoirSample(10, 728)
    for i in range(100):
        sample_u_3.push(i)
    res_3 = sample_u_3.reservoir()

    assert len(res_1) == 10
    assert len(res_2) == 10
    assert len(res_3) == 10
    assert res_1 == res_2
    assert res_1 != res_3