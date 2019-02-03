#!/usr/bin/env python3

"""
Generate lookup tables.
"""

import struct
import math
import sys

def gen_lut_hdr(name, bits):
  print('const %s_LUT_BITS: u32 = %d;' % (name, bits))
  print('const %s_LUT_SIZE: usize = 1 << %s_LUT_BITS;' % (name, name))
  print('const %s_LUT_MASK: usize = (1 << %s_LUT_BITS) - 1;' % (name, name))
  print('const %s_LUT_SCALE: f32 = (1 << (%s_LUT_BITS * 2)) as f32;' % (name, name))

def gen_lut(name, varname, bits, func):
  n = 1 << bits
  print('#[rustfmt::skip]')
  print('const %s_LUT_%s: [f32; %s_LUT_SIZE] = [' % (name, varname, name));
  for i in range(n):
    if i == 0:
      sys.stdout.write('    ')
    if i != 0 and i % 8 == 0:
      sys.stdout.write('\r\n    ')
    sys.stdout.write('%e,' % func(i))
  sys.stdout.write('\r\n];\r\n')

def pow0(i, n):
  k = 1.0 / (1 << n)
  x = float(i) * k
  return math.pow(2, x)

def pow1(i, n):
  k = 1.0 / (1 << n)
  x = float(i) * k * k
  return math.pow(2, x) - 1.0

def main():
  n = 7
  gen_lut_hdr('POW', n)
  print()
  gen_lut('POW', '0', n, lambda i: pow0(i, n))
  print()
  gen_lut('POW', '1', n, lambda i: pow1(i, n))

main()

