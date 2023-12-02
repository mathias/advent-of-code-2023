require "pry"

class Day
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
    replacements = {
      "one" => "o1e",
      "two" => "t2o",
      "three" => "t3e",
      "four" => "f4r",
      "five" => "f5e",
      "six" => "s6x",
      "seven" => "s7n",
      "eight" => "e8t",
      "nine" => "n9e",
    }

    lines.map do |line|
      line.chomp!

      replacements.each do |key, value|
        line.gsub!(key, value)
      end

      line.gsub!(/\D/, '') # remove any non-digits

      first = line[0].to_i
      last = line[-1].to_i

      raise "First digit out of bounds: #{first}" if (first < 1 || first > 9)
      raise "Last diigit out of bounds: #{last}" if (last < 1 || last > 9)

      (first * 10 + last)
    end
  end
end

# Only run the following code when this file is the main file being run
# instead of having been required or loaded by another file
if __FILE__==$0
  puts Day.new("sample_input_2.txt").run
  puts Day.new("input.txt").run
end
