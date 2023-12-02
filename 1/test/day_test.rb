require "minitest/autorun"
require_relative '../day1_2'

class DayTest < Minitest::Test
  def setup
    @day = Day.new("fake.txt")
  end

  def day
    @day
  end

  def parse_first(day, str)
    day.parse([str]).first
  end

  def parse(str)
    @day.parse([str]).first
  end

  def test_some_examples
    input_lines = ["two1nine", "eightwothree", "abcone2threexyz", "xtwone3four",
                  "4nineeightseven2", "zoneight234", "7pqrstsixteen"]
    assert_equal(281, day.parse(input_lines).sum)

    assert_equal(14, day.parse(["1six7396484"]).first)
    assert_equal(19, day.parse(["1ninehgqtjprgnpkchxdkctzk"]).first)
    assert_equal(79, day.parse(["sevenmpsmstdfivebtnjljnlnpjrkhhsninefour9"]).first)
    assert_equal(42, day.parse(["pppmfmnfourtworxrqrfhbgx8vvxgrjzhvqmztltwo"]).first)
    assert_equal(99, parse_first(day, " 9eightctkdnnllnine"))
    assert_equal(46, parse_first(day, "4rlqzthlhkxvzhcm6"))
    assert_equal(17, parse_first(day, "tb17"))
    assert_equal(88, parse_first(day, "m8t"))
    assert_equal(11, parse_first(day, "1dgschj"))
    assert_equal(71, parse("seven443six8three31"))
  end
end
