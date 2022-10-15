from asyncore import file_dispatcher
from dataclasses import dataclass
from collections import Counter
from enum import IntEnum, auto
from typing import Any


CARDS = [str(x) for x in range(2, 10)] + list("TJQKA")
CARD_VALUES = {c: i for i, c in enumerate(CARDS)}

@dataclass
class Card:
    value: str
    suit: str

    @classmethod
    def from_str(cls, s):
        return cls(s[0], s[1])

    @property
    def num_value(self):
        return CARD_VALUES[self.value]


class HandType(IntEnum):
    HIGH_CARD = auto()
    ONE_PAIR = auto()
    TWO_PAIR = auto()
    THREE = auto()
    STRAIGHT = auto()
    FLUSH = auto()
    FULL = auto()
    FOUR = auto()
    STRAIGHT_FLUSH = auto()
    ROYAL = auto()


@dataclass(order=True)
class HandRank:
    type: HandType
    value: Any
    rest: list[int]

    @classmethod
    def from_cards(cls, cards: list[Card]):
        assert len(cards) == 5

        vals = [c.value for c in cards]
        numvals = sorted([c.num_value for c in cards])
        suits = [c.suit for c in cards]
        nvc = Counter(numvals)

        # royal flush
        flush = len(set(suits)) == 1
        if set(vals) == {"10", "J", "Q", "K", "A"} and flush:
            return cls(HandType.ROYAL, 1, [])
        
        # straight flush
        straight = True
        for i in range(len(cards) - 1):
            if numvals[i+1] - numvals[i] != 1:
                straight = False
                break
        
        if straight and flush:
            return cls(HandType.STRAIGHT_FLUSH, numvals[-1], [])
        
        # four
        try:
            m = next(v for v, c in nvc.items() if c >= 4)
            return cls(
                HandType.FOUR,
                m,
                [v for v in numvals if v != m],
            )
        except StopIteration:
            pass

        # full house
        try:
            m1 = next(v for v, c in nvc.items() if c >= 3)
            rest = list(set(numvals) - {m1})
            if len(rest) == 1:
                return cls(
                    HandType.FULL,
                    (m1, rest[0]),
                    [],
                )
        except StopIteration:
            pass

        # flush
        if flush:
            return cls(HandType.FLUSH, numvals[::-1], [])

        # straight
        if straight:
            return cls(HandType.STRAIGHT, numvals[::-1], [])
        
        # three
        try:
            m = next(v for v, c in nvc.items() if c >= 3)
            return cls(
                HandType.THREE,
                m,
                sorted([v for v in numvals if v != m])[::-1],
            )
        except StopIteration:
            pass

        # two pairs
        try:
            m1, m2 = sorted([v for v, c in nvc.items() if c >= 2])
            return cls(
                HandType.TWO_PAIR,
                (m2, m1),
                [v for v in numvals if v not in [m1, m2]],
            )
        except ValueError:
            pass

        # pair
        try:
            m = next(v for v, c in nvc.items() if c >= 2)
            return cls(
                HandType.ONE_PAIR,
                m,
                sorted([v for v in numvals if v != m])[::-1],
            )
        except StopIteration:
            pass

        # high card
        return cls(HandType.HIGH_CARD, 1, numvals[::-1])


def parse_line(line: str):
    cards = [Card.from_str(s) for s in line.split()]
    return cards[:5], cards[5:]


c = 0
with open("p54.txt") as f:
    for line in f:
        l, r = parse_line(line)
        hl = HandRank.from_cards(l)
        hr = HandRank.from_cards(r)
        c += 1 if hl > hr else 0
print(c)
    