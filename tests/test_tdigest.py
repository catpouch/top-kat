from top_kat import TDigest
from random import shuffle

def percent_diff(x, expected):
    return abs(expected - x) / expected

def test_unsorted_push():
    td = TDigest(10)
    vals = list(range(0,33))
    shuffle(vals)
    td.push_list_unsorted(vals)
    assert percent_diff(td.estimate_value(0.5), 16) < 0.05

def test_sorted_push():
    td = TDigest(10)
    vals = list(range(0,33))
    td.push_list_sorted(vals)
    assert percent_diff(td.estimate_value(0.5), 16) < 0.05

def test_estimation():
    td = TDigest(10)
    vals = list(range(0,33))
    td.push_list_sorted(vals)

    q = td.estimate_quantile(20)
    v = td.estimate_value(q)
    assert percent_diff(v, 20) < 0.05

    v = td.estimate_value(0.5)
    q = td.estimate_quantile(v)
    assert percent_diff(q, 0.5) < 0.05