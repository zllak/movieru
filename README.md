<h1 align="center">movieru</h1>
<p align="center">Video editing in Rust</p>
<div align="center">
    <a href="https://crates.io/crates/movieru">
        <img src="https://img.shields.io/crates/v/movieru.svg" />
    </a>
    <a href="https://docs.rs/movieru">
        <img src="https://docs.rs/movieru/badge.svg" />
    </a>
</div>

## Overview

`Movieru` is a video editing library. It allows splitting, concatenations,
compositing, inserting text, or custom effects.

It is backed by `ffmpeg`, so it supports all the most common audio and video
formats.
Only tested on Linux for now.

## Features

TODO

## Limitations

For now, there is no real attempt at having good performances. Reading and writing
from the `ffmpeg` binary is probably far from the optimal way to do it.
The focus for no is on functionality, supporting effects, composition.

## Contributing

TODO
