file = File.open("input.txt")
elves_data = file.read

elves = elves_data.split("\n\n")

elves_with_calories = [] 

elves.each do | elve |
    calories = elve.split("\n").each.map(&:to_i)
    elves_with_calories << calories.sum
end


puts "the most performer:"
puts elves_with_calories.max

puts "the Top 3 performers:"
puts elves_with_calories.max(3).sum
