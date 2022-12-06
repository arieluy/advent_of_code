import os
from datetime import datetime, timedelta

SOLUTION_TEMPLATE = """use advent_of_code::Solution;
use std::str::Lines;

pub struct Day{0} {{}}

impl Solution for Day{0} {{

    fn part1(lines: Lines) {{
        
    }}

    fn part2(lines: Lines) {{
        
    }}
}}
"""

MAIN_TEMPLATE = "    aoc22::day{0}::Day{0}::run(args.part, args.use_sample_input);\n"

MOD_TEMPLATE = "pub mod day{0};\n"

def setup():
    day = (datetime.today() + timedelta(hours=3)).strftime("%d")
    string_day = 'day' + day

    path = os.path.join("aoc22", string_day)
    if os.path.exists(path):
        return

    os.makedirs(path)
    with open(os.path.join(path, "sample_input.txt"),'w'):
        pass

    with open(os.path.join(path, "input.txt"), 'w'):
        pass

    with open(os.path.join(path, "mod.rs"), 'w') as f:
        f.write(SOLUTION_TEMPLATE.format(day))
    
    with open("main.rs", 'r') as f:
        lines = f.readlines();
        lines[-2] = MAIN_TEMPLATE.format(day)
    
    with open("main.rs", 'w') as f:
        f.writelines(lines)

    with open("aoc22/mod.rs", 'a') as f:
        f.write(MOD_TEMPLATE.format(day))


if __name__ == "__main__":
    setup()
