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
  print('const %s_FRAC_BITS: u32 = 32 - %s_LUT_BITS;' % (name, name))
  print('const %s_FRAC_MASK: u32 = (1 << %s_FRAC_BITS) - 1;' % (name, name))

def gen_lut(name, varname, bits, func):
  n = 1 << bits
  print('#[allow(clippy::excessive_precision, clippy::unreadable_literal)]')
  print('#[rustfmt::skip]')
  print('const %s_LUT_%s: [f32; %s_LUT_SIZE] = [' % (name, varname, name));
  for i in range(n):
    if i == 0:
      sys.stdout.write('    ')
    if i != 0 and i % 8 == 0:
      sys.stdout.write('\r\n    ')
    sys.stdout.write('%e,' % func(i))
  sys.stdout.write('\r\n];\r\n')

def cos_y(i, n):
  k = float(1 << n)
  return math.cos(float(i) * 2.0 * math.pi / k)

def cos_dy(i, n):
  scale = 1.0 / (1 << (32 - n))
  y0 = cos_y(i, n)
  y1 = cos_y(i + 1, n)
  return (y1 - y0) * scale

def main():
  n = 8
  gen_lut_hdr('COS', n)
  print()
  gen_lut('COS', 'Y', n, lambda i: cos_y(i, n))
  print()
  gen_lut('COS', 'DY', n, lambda i: cos_dy(i, n))

main()

