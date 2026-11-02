#!/usr/bin/env python3
"""Write a synthetic v1.9 demo lump with scripted pseudo-random input.

Both engines play the same demo, so the input only has to exercise code:
players run, turn, strafe, fire, use and switch weapons.

usage: gen_demo.py OUT.lmp --episode E --map M [--skill S] [--players 1111]
         [--deathmatch D] [--respawn] [--fast] [--nomonsters] [--console N]
         [--tics N] [--seed N] [--scripted SPEC]

--scripted replaces the random input: every player stands still and only
presses BT_ATTACK, held for fire:A-B (tics A..B-1) and for single tics
press:T. SPEC is a comma-separated list, e.g. fire:0-900,press:1200.
"""
import argparse, random, struct

ap = argparse.ArgumentParser()
ap.add_argument("out")
ap.add_argument("--episode", type=int, default=1)
ap.add_argument("--map", type=int, default=1)
ap.add_argument("--skill", type=int, default=2)
ap.add_argument("--players", default="1000")
ap.add_argument("--deathmatch", type=int, default=0)
ap.add_argument("--respawn", action="store_true")
ap.add_argument("--fast", action="store_true")
ap.add_argument("--nomonsters", action="store_true")
ap.add_argument("--console", type=int, default=0)
ap.add_argument("--tics", type=int, default=2000)
ap.add_argument("--seed", type=int, default=1)
ap.add_argument("--scripted")
a = ap.parse_args()

attack = set()
for item in (a.scripted or "").split(","):
    if item.startswith("fire:"):
        lo, hi = map(int, item[5:].split("-"))
        attack.update(range(lo, hi))
    elif item.startswith("press:"):
        attack.add(int(item[6:]))
    elif item:
        raise SystemExit(f"bad --scripted item {item!r}")

rng = random.Random(a.seed)
pig = [1 if c == "1" else 0 for c in a.players.ljust(4, "0")[:4]]
out = bytearray([109, a.skill, a.episode, a.map, a.deathmatch, int(a.respawn),
                 int(a.fast), int(a.nomonsters), a.console] + pig)

class Bot:
    def __init__(self):
        self.turn = 0
        self.turn_left = 0
        self.fwd = 50
        self.side = 0
        self.mode_left = 0
    def cmd(self, t):
        if self.mode_left <= 0:
            self.mode_left = rng.randint(10, 70)
            self.fwd = rng.choice([50, 50, 50, 25, 0, -25])
            self.side = rng.choice([0, 0, 0, 24, -24, 40, -40])
            self.turn = rng.choice([0, 0, 1, -1, 2, -2, 6, -6])
        self.mode_left -= 1
        buttons = 0
        if rng.random() < 0.35:
            buttons |= 1                      # BT_ATTACK
        if t % 9 == 0 or rng.random() < 0.05:
            buttons |= 2                      # BT_USE
        if rng.random() < 0.02:
            buttons = (buttons & 3) | 4 | (rng.randint(0, 7) << 3)   # BT_CHANGE
        fwd = max(-127, min(127, self.fwd + rng.randint(-3, 3)))
        side = max(-127, min(127, self.side))
        turn = self.turn & 0xff
        return struct.pack("<bbBB", fwd, side, turn, buttons)

bots = [Bot() for _ in range(4)]
for t in range(a.tics):
    for i in range(4):
        if pig[i]:
            if a.scripted is not None:
                out += struct.pack("<bbBB", 0, 0, 0, 1 if t in attack else 0)
            else:
                out += bots[i].cmd(t)
out.append(0x80)
open(a.out, "wb").write(out)
