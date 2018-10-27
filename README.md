
# Steg

Command line steganography tool.


### Terms
  - `payload` - the secret message or content to be hidden
  - `carrier` - the entity in which you are trying to hide a `payload`
  - `package` - the result of hiding the `payload` in the `carrier`
  - `encoding density` - the normalised proportion of the package that the paylaod has affected
  - `channel` - describes the type of carrier in which the `payload` will be hidden
  - `scheme` - the way in which a `payload` will be hidden in the specific type of `channel`
  - `stratagy` - refers to the combination of a `channel` and a `scheme`


### Usage

```
cargo install steg (you must have [cargo](https://crates.io/install) installed)

// Hide something
steg hide -p ./payload.png -c ./carrier.png -o ./output.png

// Reveal something that was hidden
steg reveal -c ./output.png

```

The below is a plan of the supported `payload` and `carrier` types and default `strategies` for use with each.


### Key
  - :heavy_check_mark: is done
  - :hammer: is being made
  - empty means not supported yet - make a PR!


### Supported formats

 - [ ] text
    - [ ] utf8
 - [ ] images
    - :hammer: png
    - [ ] jpeg
    - [ ] bmp
 - [ ] videos
    - [ ] avi
    - [ ] mp4


### Text default stratagies

| Supported     | Payload       | Carrier       | Default stratagy               |
| ------------- | ------------- | ------------- | ------------------------------ |
| :hammer:      | utf8          | utf8          | [whitespace_text_steganography](https://crates.io/crates/whitespace_text_steganography) |
| :hammer:      | utf8          | image/png     | [lsb_text_png_steganography](https://crates.io/crates/lsb_text_png_steganography) |
|               | utf8          | image/jpeg    |                                |
|               | utf8          | image/bmp     |                                |
|               | utf8          | video/avi     |                                |
|               | utf8          | video/mp4     |                                |


### Image default stratagies

| Supported          | Payload       | Carrier       | Default stratagy      |
| ------------------ | ------------- | ------------- | --------------------- | 
| :heavy_check_mark: | png           | image/png     | [lsb_png_steganography](https://crates.io/crates/lsb_png_steganography) |
|                    | png           | image/jpeg    |                       |
|                    | png           | image/bmp     |                       |
|                    | png           | video/avi     |                       |
|                    | png           | video/mp4     |                       |
|                    | jpeg          | image/png     |                       |
|                    | jpeg          | image/jpeg    |                       |
|                    | jpeg          | image/bmp     |                       |
|                    | jpeg          | video/avi     |                       |
|                    | jpeg          | video/mp4     |                       |
|                    | bmp           | image/png     |                       |
|                    | bmp           | image/jpeg    |                       |
|                    | bmp           | image/bmp     |                       |
|                    | bmp           | video/avi     |                       |
|                    | bmp           | video/mp4     |                       |


### Video default stratagies

| Supported     | Payload       | Carrier       | Default stratagy   |
| ------------- | ------------- | ------------- | ------------------ |
|               | avi           | image/avi     |                    |
|               | avi           | image/mp4     |                    |
|               | mp4           | image/avi     |                    |
|               | mp4           | image/mp4     |                    |
