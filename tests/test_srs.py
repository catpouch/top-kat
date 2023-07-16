from top_kat import SimpleRandomSample

def test_unseeded():
    sample_t = SimpleRandomSample(100, 20)
    sampled = 0
    for _ in range(100):
        r = sample_t.sample()
        print(r)
        if r:
            sampled += 1
    assert sampled == 20

def test_seeded():
    sample_t_1 = SimpleRandomSample(100, 20, 727)
    sampled_1 = []
    for _ in range(100):
        sampled_1.append(sample_t_1.sample())
    
    sample_t_2 = SimpleRandomSample(100, 20, 727)
    sampled_2 = []
    for _ in range(100):
        sampled_2.append(sample_t_2.sample())

    sample_t_3 = SimpleRandomSample(100, 20, 728)
    sampled_3 = []
    for _ in range(100):
        sampled_3.append(sample_t_3.sample())
    
    assert sum(sampled_1) == 20
    assert sum(sampled_2) == 20
    assert sum(sampled_3) == 20
    assert sampled_1 == sampled_2
    assert sampled_1 != sampled_3