#!/bin/bash

cat output.h | grep -v "typedef float _Float32;" | grep -v "typedef double _Float64;" | grep -v "typedef double _Float32x;" | grep -v "typedef long double _Float64x" > stripped.h
