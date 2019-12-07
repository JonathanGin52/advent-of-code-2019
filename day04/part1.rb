def check?(num)
  doubles = false
  digits = num.to_s.split('')
  digits[1..].each.with_index(1) do |digit, i|
    return false unless digit >= digits[i - 1]
    doubles = true if digit == digits[i - 1]
  end
  doubles
end

input = $<.first.split('-').map(&:to_i)
puts (input.first..input.last).sum { |num| check?(num) ? 1 : 0 }
