import re

def filter_content(content):
    content.sort()
    ret = {}
    for e in content:
        if re.search(r'\.\w{2,10}$', e):
            ret[e] = 1
    return list(ret.keys())

template = ["wav", "tex", "arp", "msh", "rs"]

if __name__ == "__main__":
    input = 'character'
    fd  = open("./output/%s.txt" % input, encoding="utf-8")
    content = fd.readlines()
    fd.close()
    content = filter_content(content)

    outContent = []
    errContent = []
    unknowConten = []

    for p in content:
        p = p.strip()
        p = p.split(".")

        if len(p) != 2:
            if p in errContent:
                continue
            errContent.append(".".join(str(x) for x in p) + "\n")
            continue

        ext = p[1]
        isMatch = False

        for suffix in template:
            if ext.startswith(suffix):
                ext = suffix
                isMatch = True

        p = p[0] + "." + ext + "\n"
        if isMatch:
            if p in outContent:
                continue
            outContent.append(p)
        else:
            if p in unknowConten:
                continue
            unknowConten.append(p)

    fd = open("./output/%s.filter.txt" % input, "w+", encoding="utf-8")
    fd.writelines(outContent)
    fd.writelines("\n##############UNKNOW##############\n")
    fd.writelines(errContent)
    fd.writelines("\n##############ERROR##############\n")
    fd.writelines(errContent)
    fd.close()