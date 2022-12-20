require 'json'

def compare(left, right)
  if left.is_a?(Integer) && right.is_a?(Integer)
    return left <=> right
  elsif left.is_a?(Integer)
    return compare([left], right)
  elsif right.is_a?(Integer)
    return compare(left, [right])
  else
    left, right = left.to_enum, right.to_enum

    loop do
      return  0 if  done?(left) &&  done?(right)
      return -1 if  done?(left) && !done?(right)
      return  1 if !done?(left) &&  done?(right)

      result = compare(left.next, right.next)
      return result unless result.zero?
    end
  end
end

def done?(enum)
  enum.peek
  return false
rescue StopIteration
  return true
end

messages = File.readlines("input.txt", chomp: true).
                chunk(&:empty?).
                reject(&:first).
                collect(&:last).
                map { |packets| packets.collect { |p| JSON.parse(p) } }

comparisons = messages.collect { |(left, right)| compare(left, right) }
# pp(comparisons)

pp comparisons.collect.with_index { |c, i| c == 1 ? 0 : (i + 1) }.sum
