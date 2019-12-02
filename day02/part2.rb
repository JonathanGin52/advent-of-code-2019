input = $<.first.split(',').map(&:to_i)

def correct?(numbers, noun, verb)
  instruction_pointer = 0
  numbers[1] = noun
  numbers[2] = verb

  numbers.size.times do
    a, b, c, d = numbers[instruction_pointer..instruction_pointer + 3]
    case a
    when 1
      numbers[d] = numbers[b] + numbers[c]
    when 2
      numbers[d] = numbers[b] * numbers[c]
    when 99
      return numbers.first == 19690720
    end
    instruction_pointer += 4
  end

  false
end

(0...99).each do |noun|
  (0..99).each do |verb|
    if correct?(input.dup, noun, verb)
      puts [noun, verb].join(' ')
      puts 100 * noun + verb
    end
  end
end
