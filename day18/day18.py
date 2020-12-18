from operator import add, mul

def _evaluate(string, inx):
    value = None
    op = None
    
    while inx < len(string):
        c = string[inx]
        if c.isdigit():
            if value is None:
                value = int(c)
            else:
                value = op(value, int(c))
                op = None
        elif c == '+':
            op = add
        elif c == '*':
            op = mul
        elif c == ')':
            break
        elif c == '(':
            v, inx = _evaluate(string, inx+1)
            if value is None:
                value = v
            else:
                value = op(value, v)
                op = None
        elif c == ' ':
            pass
        else:
            raise Exception(f"unknown char {c}")
        #print(c, value, op)
        inx += 1
     
    return value, inx
                
def evaluate(string):
    result, inx = _evaluate(string, 0)
    assert inx >= len(string), f"{string} => {inx} < {len(string)}"
    return result
    

def main():
    total = 0
    with open('inputs/day18.txt', 'r') as f:
        for line in f:
            result = evaluate(line.strip())
            print(f"{line.strip()} = {result}")
            total += result
    print(total)
    
if __name__ == '__main__':
    #print(evaluate("1 + 2 * 3 + 4 * 5 + 6"))
    main()
