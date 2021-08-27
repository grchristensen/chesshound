#!/bin/sh
exec cargo test
exec cargo fmt -- --check
