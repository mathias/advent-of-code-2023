require "pry"

class Almanac
  def initialize(path)
    @path = path

    @seeds = []
    @seed_to_soil = {}
    @soil_to_fertilizer = {}
    @fertilizer_to_water = {}
    @water_to_light = {}
    @light_to_temperature = {}
    @temperature_to_humidity = {}
    @humidity_to_location = {}
  end

  def part_1
    parse(read(@path))

    output = @seeds.map do |seed|
      soil = map_next(:@seed_to_soil, seed)
      fertilizer = map_next(:@soil_to_fertilizer, soil)
      water = map_next(:@fertilizer_to_water, fertilizer)
      light = map_next(:@water_to_light, water)
      temperature = map_next(:@light_to_temperature, light)
      humidity = map_next(:@temperature_to_humidity, temperature)
      [seed, soil, fertilizer, water, light, temperature, humidity, map_next(:@humidity_to_location, humidity)]
    end
    output.map(&:last).min
  end

  private

  def map_next(hsh, value)
    output = self.instance_variable_get(hsh)[value]
    if output.nil?
      output = value
    end
    output
  end


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

  def match_mode?(line)
    MODES.any? do |x|
      line.start_with?(x)
    end
  end

  def mode_from(line)
    line.split.first.gsub(/-/, '_').to_sym
  end

  def parse(lines)
    puts "#{lines.count} lines received."
    mode = nil

    lines.each do |line|
      if line.start_with?("seeds: ")
        first, rest = line.split(":")
        @seeds = rest.split.map(&:to_i)
        next
      elsif match_mode?(line)
        mode = mode_from(line)
      elsif mode != nil && line.length == 0 # switching mode, keep only mappings we need
        if mode == :seed_to_soil
          @seed_to_soil.keep_if { |k, _v| @seeds.include?(k) }
          @seeds.each do |k|
            if !@seed_to_soil.has_key?(k)
              @seed_to_soil[k] = k
            end
          end
        else
          from, to = mode.to_s.split("_to_")
          prev_mode = MODES.find { |x| x.end_with?(from) }
          keys_from_prev = self.instance_variable_get("@#{mode_from(prev_mode)}").values

          self.instance_variable_get("@#{mode}").keep_if { |k, _v| keys_from_prev.include?(k) }

          keys_from_prev.each do |k|
            if !self.instance_variable_get("@#{mode}").has_key?(k)
              self.instance_variable_get("@#{mode}")[k] = k
            end
          end
        end
      elsif mode != nil && line.length > 0
        # Parse maps lines in:
        dest_start, source_start, range_length = line.split.map(&:to_i)

        (0..range_length-1).each do |i|
          self.instance_variable_get("@#{mode}").send("[]=", source_start + i, dest_start + i);
        end
      elsif line.length != 0
        raise "Not implemented yet"
      end
    end
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
  # TODO
end
