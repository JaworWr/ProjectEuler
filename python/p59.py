# Interactive this time!
import itertools
import string


with open("p59.txt") as f:
    msg = [int(x) for x in f.read().strip().split(",")]


def decrypt(key, message=None):
    if message is None:
        message = msg
    l = len(key)
    kc = [ord(c) for c in key]
    res = ""
    for i, code in enumerate(message):
        res += chr(code ^ kc[i % l])
    return res


def get_samples(l=50):
    keys = itertools.product(string.ascii_lowercase, repeat=3)
    keys = ("".join(t) for t in keys)
    samples = {}
    for k in keys:
        samples[k] = decrypt(k, msg[:l])
    return samples


def show_samples(l=50):
    """Show samples where we have only letters and spaces, to pick the one with English words only."""
    samples = get_samples(l=l)
    n = 0
    for k, v in samples.items():
        if not all(c.isalpha() or c == " " for c in v):
            continue
        print(k, v, sep="\t")
        n += 1
    print(n, "samples")


def ascii_sum(msg):
    return sum(map(ord, msg))
