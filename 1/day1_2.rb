require "pry"

LINE_REGEX = /([1-9]|one|two|three|four|five|six|seven|eight|nine)/

def read(path)
  lines = []
  File.open(path, "r") do |f|
      f.each_line do |line|
        lines << line
      end
  end
  lines
end

def parse(lines)
  results = []
  lines.each do |line|
    matches = line.chomp.scan(LINE_REGEX)

    first = parse_word(matches[0][0])
    last = parse_word(matches[-1][0])

    raise if first < 1 || first > 9 || last < 0 || last > 9

    results << (first * 10 + last)
  end
  results
end

def parse_word(word)
  case word
    when "one"
      1
    when "two"
      2
    when "three"
      3
    when "four"
      4
    when "five"
      5
    when "six"
      6
    when "seven"
      7
    when "eight"
      8
    when "nine"
      9
    else
      word.to_i
  end
end

def add_numbers(list)
  list.reduce(0) { |acc, n| acc + n }
end

parse(read("input.txt")).each { |n| raise "incorrect number: #{n}" if n > 99 || n < 11 }

puts add_numbers(parse(read("sample_input_2.txt")))
puts add_numbers(parse(read("input.txt")))
