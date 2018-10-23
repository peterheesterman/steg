
# Implementation details

A place for thoughts and ideas about the way that `steg` should be developed.

### Taking input

Planning to use `clap` (command line argument parser) for this

### Organisation

- steg 
  - images
    - all the image algorithms
  - text
    - all the text algorithms
  - video
    - all the video algorithms

All implementations of algorithms (strategies) should live in different repos. 
