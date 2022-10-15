DIGITS = {
    0: "",
    1: "one",
    2: "two",
    3: "three",
    4: "four",
    5: "five",
    6: "six",
    7: "seven",
    8: "eight",
    9: "nine",
}

TENS = {
    10: "ten",
    11: "eleven",
    12: "twelve",
    13: "thirteen",
    14: "fourteen",
    15: "fifteen",
    16: "sixteen",
    17: "seventeen",
    18: "eighteen",
    19: "nineteen",
    20: "twenty",
    30: "thirty",
    40: "forty",
    50: "fifty",
    60: "sixty",
    70: "seventy",
    80: "eighty",
    90: "ninety",
}

def count_letters(i):
    res = 0
    hundrets = i // 100
    tens = i % 100
    if tens <= 9:
        res += len(DIGITS[tens])
    elif 10 <= tens <= 19:
        res += len(TENS[tens])
    else:
        ones = tens % 10
        tens -= ones
        res += len(TENS[tens]) + len(DIGITS[ones])
    if hundrets > 0:
        res += len("hundret") + len(DIGITS[hundrets])
        if tens > 0:
            res += len("and")
    return res

print(342, count_letters(342))
print(115, count_letters(115))

res = len("one") + len("thousand")
for i in range(1, 1000):
    res += count_letters(i)
print(res)
