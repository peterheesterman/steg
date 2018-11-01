
# Implementation details

A place for thoughts and ideas about the way that `steg` should be developed.

### Taking input

Planning to use `clap` (command line argument parser) for this

### Organisation

- steg 
  - images
    - all the ways of hiding images
  - text
    - all the ways of hiding text
  - video
    - all the ways of hiding video

All implementations of algorithms (strategies) should live in different repos. 

Each strategy should have a `hide` and a `reveal` function with `detech` and `analyse` functions to come later on if they seem like they might be helpful.

It should be easy to switch out and use other peoples code to add strategies to `steg` in the future. This may mean wrapping their code in another module or using is directly. It will have to been seen which ends up being easier.


### Should i be writing adapters not stand alone things
Reduce for a format:
  - Payload -> byte array
  - byte array -> payload

Construct from a package:
  - byte array -> package with carrier type
  - package with carrier type -> byte array
