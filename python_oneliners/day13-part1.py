print((lambda t, ids: min(((i - int(t) % i, i*(i - int(t) % i)) for i in (int(i) for i in ids.split(',')
                                                                          if i != 'x')), key=lambda x: x[0]))(*open('../inputs/day13.txt'))[1])
