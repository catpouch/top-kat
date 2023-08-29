from top_kat import TDigest

td = TDigest(15.0)

td.merge_vec_unsorted(list(range(0,100)))

print(td.estimate_value(0.5))