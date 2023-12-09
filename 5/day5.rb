require "pry"

class Almanac
  def initialize(path)
    @path = path

    @seeds = []
  end

  def walk_seeds(seeds, modes)
    results = seeds.map do |seed|
      output = nil

      prev_value = seed

      MODES.each do |mode|
        if key = modes[mode].keys.find { |k| k.include?(prev_value) }
          offset = prev_value - key.first

          value_range = modes[mode][key]
          value = value_range.first + offset
        else
          value = prev_value
        end
        raise "Bad value in #{output.to_s}" if value == 0

        output = value
        prev_value = value
      end

      output
    end

    results.min
  end

  def part_1
    modes = parse(read(@path))
    walk_seeds(@seeds, modes)
  end

  def range_intersection(a, b)
    return nil if (a.max < b.begin or b.max < a.begin)
    [a.begin, b.begin].max..[a.max, b.max].min
  end

  def part_2
    modes = parse(read(@path))

    first_mode_values = modes[MODES.first].values.sort { _1.first <=> _2.first }

    seed_ranges = @seeds.clone
      .each_slice(2)
      .map { |start, offset| (start..(start+offset-1)) }
      .sort { |x, y| x.first <=> y.first }
      .reduce([]) { |accum, range|
        first_mode_values.each do |mode_range|
          intersect = range_intersection(mode_range, range)
          accum << intersect if intersect
        end
        accum
      }
    threads = []
    seed_ranges.each do |range|
      threads << Thread.new { walk_seeds(range.to_a, modes) }
    end
    threads.each { |thr| thr.join }
    results = threads.map { |thr| thr.value }
    results.min
  end

  private

  def read(path)
    File.readlines(path).map(&:chomp!).compact
  end

  MODES = [
    "seed-to-soil",
    "soil-to-fertilizer",
    "fertilizer-to-water",
    "water-to-light",
    "light-to-temperature",
    "temperature-to-humidity",
    "humidity-to-location"
  ]

  def mode_str_to_sym(mode_str)
    mode_str.replace(/-/,'_').to_sym
  end

  def match_mode?(line)
    MODES.any? do |x|
      line.start_with?(x)
    end
  end

  def parse(lines)
    puts "#{lines.count} lines received."
    mode = nil

    modes = {}

    lines.each do |line|
      if line.start_with?("seeds: ")
        first, rest = line.split(":")
        @seeds = rest.split.map(&:to_i)
        next
      elsif match_mode?(line)
        mode = line.split.first
        modes[mode] = {}
        next
      elsif line.length == 0
        next
      elsif mode != nil && line.length > 0
        dest, source, range_len = line.split.map(&:to_i)

        modes[mode].merge!({
          (source..source+range_len) => (dest..dest+range_len)
        })
      else
        raise "Problem with input"
      end
    end
    modes
  end
end

# Only run the following code when this file is the main file being run
# instead of having been required or loaded by another file
if __FILE__==$0

  # Part 1
  sample_part_1 = Almanac.new("sample_input.txt").part_1
  raise "Invalid sample answer: got #{sample_part_1} expected 35" unless sample_part_1 == 35
  puts "part 1 sample: " + sample_part_1.to_s
  puts "part 1 actual: " + Almanac.new("input.txt").part_1.to_s

  # Part 2
  sample_part_2 = Almanac.new("sample_input.txt").part_2
  raise "Invalid sample answer: got #{sample_part_2} expected 46" unless sample_part_2 == 46
  puts "part 2 sample: " + sample_part_2.to_s
  puts "part 2 actual: " + Almanac.new("input.txt").part_2.to_s
end
