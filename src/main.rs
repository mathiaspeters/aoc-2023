fn main() {
    std::env::args().skip(1).for_each(|arg| match arg.as_str() {
        "1" => aoc::day01::day1(),
        "2" => aoc::day02::day2(),
        "3" => aoc::day03::day3(),
        "4" => aoc::day04::day4(),
        "5" => aoc::day05::day5(),
        "6" => aoc::day06::day6(),
        "7" => aoc::day07::day7(),
        "8" => aoc::day08::day8(),
        "9" => aoc::day09::day9(),
        "10" => aoc::day10::day10(),
        "11" => aoc::day11::day11(),
        "12" => aoc::day12::day12(),
        "13" => aoc::day13::day13(),
        "14" => aoc::day14::day14(),
        "15" => aoc::day15::day15(),
        "16" => aoc::day16::day16(),
        "17" => aoc::day17::day17(),
        "18" => aoc::day18::day18(),
        "19" => aoc::day19::day19(),
        "20" => aoc::day20::day20(),
        "21" => aoc::day21::day21(),
        "22" => aoc::day22::day22(),
        "23" => aoc::day23::day23(),
        "24" => aoc::day24::day24(),
        "25" => aoc::day25::day25(),
        _ => println!("Invalid input: {}", arg),
    });
}