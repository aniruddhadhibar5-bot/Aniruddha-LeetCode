# @param {Integer} n
# @return {Boolean}
def is_power_of_four(n)
  n > 0 && (n & (n - 1)) == 0 && (n & 0x55555555) != 0
end
