import math
import pathlib
import sys

from itertools import chain
from operator import lt, gt
from typing import Callable

sys.path.append(str(pathlib.Path(__file__).resolve().parents[3] / 'lib' / 'python'))

RatingMap = dict[str, tuple[int, int]]

def rule(workflow: str) -> tuple[str, Callable[[dict[str, int]], str]]:
  name, rule_spec = workflow[:-1].split("{")

  rules = []
  final_target = ""

  for rule in rule_spec.split(","):
    if len(rule) > 1 and rule[1] in {'<', '>'}:
      rating = rule[0]
      op = rule[1]
      val, target = rule[2:].split(":", 1)
      rules.append((rating, lt if op == "<" else gt, int(val), target))
    else:
      final_target = rule

  return (
    name,
    lambda r: next(chain(
      (target for rating, op, val, target in rules if op(r[rating], val)),
      (final_target, )
    ))
  )

def split_rule(workflow: str) -> tuple[str, Callable[[RatingMap], list[tuple[RatingMap, str]]]]:
  name, rule_spec = workflow[:-1].split("{")

  rules: list[tuple[str, str, int, str]] = []
  final_target = ""

  for rule in rule_spec.split(","):
    if len(rule) > 1 and rule[1] in {'<', '>'}:
      rating = rule[0]
      op = rule[1]
      val, target = rule[2:].split(":", 1)
      rules.append((rating, op, int(val), target))
    else:
      final_target = rule

  def results(r: RatingMap) -> list[tuple[RatingMap, str]]:
    splits: list[tuple[RatingMap, str]] = []

    for rating, op, val, target in rules:
      if op == "<" and r[rating][1] >= val:
        splits.append(({k: (r[k][0], val - 1) if k == rating else v for k, v in r.items()}, target))
        r = {k: (val, r[k][1]) if k == rating else v for k, v in r.items()}
      elif op == ">" and r[rating][0] <= val:
        splits.append(({k: (val + 1, r[k][1]) if k == rating else v for k, v in r.items()}, target))
        r = {k: (r[k][0], val) if k == rating else v for k, v in r.items()}
    else:
      splits.append((r, final_target))

    return splits

  return name, results

def run() -> None:
  with open('data19.txt') as f:
    input = f.read()

  rule_spec, part_spec = input.strip().split("\n\n")

  rules = {name: result for name, result in (rule(spec) for spec in rule_spec.splitlines())}
  parts = [
    {cat: int(rating) for cat, rating in (r.split("=") for r in spec.strip()[1:-1].split(","))}
    for spec in part_spec.splitlines()
  ]

  total_rating = 0
  for part in parts:
    result = "in"
    while result not in {"A", "R"}:
      result = rules[result](part)

    if result == "A":
      print(part)
      total_rating += sum(part.values())

  print(f"Rating sum for accepted parts: {total_rating}")

  split_rules = {name: result for name, result in (split_rule(spec) for spec in rule_spec.splitlines())}
  ratings: RatingMap = {"x": (1, 4000), "m": (1, 4000), "a": (1, 4000), "s": (1, 4000)}

  total_combos = 0
  stack: list[tuple[RatingMap, str]] = [(ratings, "in")]

  while stack:
    r, func = stack.pop()

    for split, target in split_rules[func](r):
      if target == "A":
        total_combos += math.prod(hi - lo + 1 for lo, hi in split.values())
      elif target != "R":
        stack.append((split, target))

if __name__ == '__main__':
  run()
  sys.exit(0)
