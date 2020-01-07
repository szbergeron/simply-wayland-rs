#!/bin/bash
clang -M wrapper.h | tr " " "\n" | grep wayland
