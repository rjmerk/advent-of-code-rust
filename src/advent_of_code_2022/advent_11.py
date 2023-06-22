import sys
from dataclasses import dataclass
from time import time
from typing import List


INPUT = """Monkey 0:
  Starting items: 57
  Operation: new = old * 13
  Test: divisible by 11
    If true: throw to monkey 3
    If false: throw to monkey 2

Monkey 1:
  Starting items: 58, 93, 88, 81, 72, 73, 65
  Operation: new = old + 2
  Test: divisible by 7
    If true: throw to monkey 6
    If false: throw to monkey 7

Monkey 2:
  Starting items: 65, 95
  Operation: new = old + 6
  Test: divisible by 13
    If true: throw to monkey 3
    If false: throw to monkey 5

Monkey 3:
  Starting items: 58, 80, 81, 83
  Operation: new = old * old
  Test: divisible by 5
    If true: throw to monkey 4
    If false: throw to monkey 5

Monkey 4:
  Starting items: 58, 89, 90, 96, 55
  Operation: new = old + 3
  Test: divisible by 3
    If true: throw to monkey 1
    If false: throw to monkey 7

Monkey 5:
  Starting items: 66, 73, 87, 58, 62, 67
  Operation: new = old * 7
  Test: divisible by 17
    If true: throw to monkey 4
    If false: throw to monkey 1

Monkey 6:
  Starting items: 85, 55, 89
  Operation: new = old + 4
  Test: divisible by 2
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 7:
  Starting items: 73, 80, 54, 94, 90, 52, 69, 58
  Operation: new = old + 7
  Test: divisible by 19
    If true: throw to monkey 6
    If false: throw to monkey 0"""


def main():
    monkeys = parse_monkeys(INPUT)
    x = solve_x(monkeys)
    print(x)
    assert x == 28244037010


def solve_x(monkeys):
    common_divider = 1
    for d in [m.divisible_by for m in monkeys]:
        common_divider *= d

    for _n in range(10_000):
        for m in range(len(monkeys)):
            items_to_throw = []
            monkey = monkeys[m]
            while monkey.items:
                (target_monkey, item) = execute_round(monkey)
                items_to_throw.append((target_monkey, item % common_divider))

            for (target_monkey, item) in items_to_throw:
                monkeys[target_monkey].items.append(item)

    return calculate_monkey_business(monkeys)


def execute_round(monkey):
    item = monkey.items.pop(0)
    if monkey.operation == "multiply":
        item *= monkey.value
    elif monkey.operation == "add":
        item += monkey.value
    else:
        item *= item

    monkey.nr_inspects += 1
    if item % monkey.divisible_by == 0:
        target_monkey = monkey.if_true_throw_to_monkey
    else:
        target_monkey = monkey.if_false_throw_to_monkey

    return target_monkey, item


def calculate_monkey_business(monkeys):
    result = [m.nr_inspects for m in monkeys]
    result.sort()
    result.reverse()
    return result[0] * result[1]


@dataclass
class Monkey:
    items: List[int]
    operation: str
    value: int
    divisible_by: int
    if_true_throw_to_monkey: int
    if_false_throw_to_monkey: int
    nr_inspects: int = 0


def parse_operation(line):
    if line == '  Operation: new = old * old':
        return "square", 0
    op_str = line.split("old")[1]
    if op_str == "*":
        return "square", 0

    value = int(op_str[3:])

    if op_str.strip().startswith("*"):
        return "multiply", value
    if op_str.strip().startswith("+"):
        return "add", value


def parse_monkeys(s):
    result = []
    monkey_strs = s.split("\n\n")
    for monkey_str in monkey_strs:
        lines = monkey_str.splitlines()
        items_str = lines[1].split(":")[1]
        items = [int(s.strip()) for s in items_str.split(",")]
        (operation, value) = parse_operation(lines[2])
        divisible_by = int((lines[3].split("by")[1]).strip())
        a = len("    If true: throw to monkey ")
        b = len("    If false: throw to monkey ")

        if_true_throw_to_monkey = int(lines[4][a:])
        if_false_throw_to_monkey = int(lines[5][b:])
        result.append(Monkey(
            items=items,
            operation=operation,
            value=value,
            divisible_by=divisible_by,
            if_true_throw_to_monkey=if_true_throw_to_monkey,
            if_false_throw_to_monkey=if_false_throw_to_monkey,
            nr_inspects=0,
        ))
    return result


if __name__ == "__main__":
    t1 = time()
    main()
    t2 = time()
    print("Time elapsed: {} seconds".format(t2 - t1))
    sys.exit(0)
