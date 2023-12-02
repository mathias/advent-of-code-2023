require "pry"

class Day
  LINE_REGEX = /(one|two|three|four|five|six|seven|eight|nine|[1-9])/

  def initialize(path)
    @path = path
  end

  def run
    parse(read(@path)).sum
  end

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
    lines.map do |line|
      matches = line.chomp.scan(LINE_REGEX)

      first = parse_word(matches[0][0])
      last = parse_word(matches[-1][0])

      raise if (first < 1 || first > 9 || last < 1 || last > 9)
      raise unless first.is_a?(Integer) && last.is_a?(Integer)

      (first * 10 + last)
    end
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
end

# Only run the following code when this file is the main file being run
# instead of having been required or loaded by another file
if __FILE__==$0
  puts Day.new("sample_input_2.txt").run
  puts Day.new("input.txt").run
end
