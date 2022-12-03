# frozen_string_literal: true
bags = File.read('input.txt').split("\n")

def calculate_value(character)
  case character
  when ('a'..'z')
    character.ord - 'a'.ord + 1
  when ('A'..'Z')
    character.ord - 'A'.ord + 27
  end
end

def part_1(bags)
  total_priorities = 0
  bags.each do |bag|
    pockets = bag.chars
                 .each_slice(bag.size / 2)
                 .map { |pockets| pockets.to_a }

    repeated = pockets[0] & pockets[1]
    total_priorities += calculate_value(repeated.first)
  end
  total_priorities
end

def part_2(bags)
  total_priorities = 0
  bags.each_slice(3) do |group_bags|
    bag_array = group_bags.to_a
    repeated = bag_array[0].chars & bag_array[1].chars & bag_array[2].chars
    total_priorities += calculate_value(repeated.first)
  end
  total_priorities
end

puts "Part 1 - total points #{part_1(bags)}"

puts "Part 2 - total points #{part_2(bags)}"
