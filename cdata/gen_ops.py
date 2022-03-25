# punct_map_3.insert(">>=", T::T_RSHIFT_EQUAL);
# %:%:   @  T_SHARP_SHARP

with open("ops_no_digr.txt", "r") as f:
    lines = f.readlines()
    for line in lines:
        line = line.strip()
        if line == '':
            continue
        content = line.split('@')
        op = content[0].strip()
        en = content[1].strip()
        # print('punct_map.insert(\"' + op + '\", ' + 'T::' + en + ');')
        print(en + ', // ' + op)


