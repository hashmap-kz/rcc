# pub as_id: Rc<RefCell<Ident>>,
# as_id: Rc::new(RefCell::new(Ident::new("as".to_string(), 0))),
# break_ident            @ "break"
# idmap.insert("as".to_string(), Rc::clone(&keywords.as_id));

with open("kw.txt", "r") as f:
    lines = f.readlines()
    structs = ''
    maps = ''
    maps2 = ''
    cnt = 0
    for line in lines:
        line = line.strip()
        if line == '':
            continue
        content = line.split('@')
        ident = content[0].strip()
        name = content[1].strip()
        structs += 'pub ' + ident + ': Rc<RefCell<Ident>>,\n'
        maps += ident + ': Rc::new(RefCell::new(Ident::new(' + name + '.to_string(), ' + str(cnt) + '))),\n'
        maps2 += 'idmap.insert(' + name + '.to_string(), Rc::clone(&keywords.' + ident + '));\n'
        cnt += 1
    print(structs)
    print(maps)
    print(maps2)

