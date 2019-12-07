def check?(num)
  numbers = Hash.new(0)
  digits = num.to_s.split('')
  digits.each_with_index do |digit, i|
    numbers[digit] += 1
    next if i == 0
    return false unless digit >= digits[i - 1]
  end
  numbers.values.any? { |v| v == 2 }
end

input = $<.first.split('-').map(&:to_i)
puts (input.first..input.last).sum { |num| check?(num) ? 1 : 0 }
