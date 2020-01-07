#!/bin/bash
grep -E "\#define WL\w* [0-9]{1,}" | sed -E 's/(\#define) ([0-9a-zA-Z_]*) ([0-9]*)/pub const \2: u32 = \3\;/' | sort | uniq
