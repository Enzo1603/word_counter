from word_counter import make_vec, make_generic

v = make_vec(1, 2)

print(v)
print([2 * e for e in v])

w = make_generic("f", 2.0)

print(w)
print([2 * e for e in w])
