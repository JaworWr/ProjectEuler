import argparse
from concurrent.futures import process
from pathlib import Path
import re
import shutil
from typing import Any, Iterable, Sequence, Union


PROBLEM_RANGE = range(1, 101)
ROOT = Path(__file__).parent.resolve()


def get_parser():
    parser = argparse.ArgumentParser()
    parser.add_argument("input", type=Path)
    return parser


def get_problem_numbers(path: Union[str, Path]):
    if isinstance(path, Path):
        path = path.stem

    numbers = map(int, re.findall(r"\d+", path))
    return list(numbers)


def copy_directory(input_root: Path, dir, exclude: Sequence[Any] = None):
    input_root = input_root.resolve()
    path = input_root / dir
    if exclude is not None:
        exclude = {(path / p).resolve() for p in exclude}
    else:
        exclude = []
    
    for x in path.glob("**/*"):
        if not x.is_file():
            continue
        if x in exclude:
            continue
        numbers = get_problem_numbers(x)
        if any(n not in PROBLEM_RANGE for n in numbers):
            continue
        x_rel = x.relative_to(input_root)
        x_target = ROOT / x_rel
        x_target.parent.mkdir(exist_ok=True, parents=True)
        shutil.copy(x, x_target)


def check_mod_rs_line(line: str):
    if not re.match(r"mod\s+p\d+;$", line.strip()):
        return True
    numbers = get_problem_numbers(line)
    if all(n in PROBLEM_RANGE for n in numbers):
        return True
    else:
        return False


def process_mod_rs(input_root: Path, file):
    input_root = input_root.resolve()
    (ROOT / file).parent.mkdir(exist_ok=True, parents=True)
    with open(input_root / file) as f, open(ROOT / file, "w") as out:
        lines = map(str.strip, f)
        lines = filter(check_mod_rs_line, lines)
        for line in lines:
            print(line, file=out)


if __name__ == "__main__":
    args = get_parser().parse_args()
    for dir in ["cpp", "fortran", "julia", "python"]:
        copy_directory(args.input, dir)
    copy_directory(args.input, "rust", exclude=["src/problems/mod.rs"])
    process_mod_rs(args.input, "rust/src/problems/mod.rs")
