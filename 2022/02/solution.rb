# frozen_string_literal: true

plays = File.read('input.txt').split("\n")

scores = [1, 2, 3]

def score_difference(opponent_score, my_score)
  if opponent_score % 3 + 1 == my_score
    my_score + 6
  elsif opponent_score == my_score % 3 + 1
    my_score
  else
    my_score + 3
  end
end

part1_points = plays
               .map(&:split)
               .map { |opponent, mine| [scores[(opponent.ord - 65) % 3], scores[(mine.ord - 88) % 3]] }
               .map { |opponent_score, my_score| score_difference(opponent_score, my_score) }
               .sum

puts "Part 1 - total points #{part1_points}"

def what_to_do(my_play, opponent_play)
  case my_play
  when 'X' then 'CAB'.chars[opponent_play.ord - 65]
  when 'Y' then opponent_play
  when 'Z' then 'BCA'.chars[opponent_play.ord - 65]
  else raise ''
  end
end

part2_points = plays
               .map(&:split)
               .map { |opponent, mine| [scores[(opponent.ord - 65) % 3], scores[(what_to_do(mine, opponent).ord - 65) % 3]] }
               .map { |opponent_score, my_score| score_difference(opponent_score, my_score) }
               .sum

puts "Part 2 - total points #{part2_points}"
