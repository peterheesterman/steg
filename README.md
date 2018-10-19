
# Steg

A steganography tool.

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
steg -p ./payload -c ./carrier -s ./strategy.str
```

The below is a plan of the supported `payload` and `carrier` types and default `strategies` for use with each.


### Supported formats

 - [ ] text
    - [ ] utf8
 - [ ] images
    - [ ] png
    - [ ] jpeg
    - [ ] bmp
 - [ ] videos
    - [ ] avi
    - [ ] mp4


### Text default stratagies
  - x is done
  - :hammer: is being made
  - empty means not supported yet - make a PR!

| Supported     | Payload       | Carrier       | Default stratagy   |
| ------------- | ------------- | ------------- | ------------------ | 
| [ ]           | utf8          | utf8          | zero-width-spaces  |
| [ ]           | utf8          | image/png     |                    |
| [ ]           | utf8          | image/jpeg    |                    |
| [ ]           | utf8          | image/bmp     |                    |
| [ ]           | utf8          | video/avi     |                    |
| [ ]           | utf8          | video/mp4     |                    |


### Image default stratagies

| Supported     | Payload       | Carrier       | Default stratagy   |
| ------------- | ------------- | ------------- | ------------------ | 
| [:hammer:]    | png           | image/png     |                    |
| [ ]           | png           | image/jpeg    |                    |
| [ ]           | png           | image/bmp     |                    |
| [ ]           | png           | video/avi     |                    |
| [ ]           | png           | video/mp4     |                    |
| [ ]           | jpeg          | image/png     |                    |
| [ ]           | jpeg          | image/jpeg    |                    |
| [ ]           | jpeg          | image/bmp     |                    |
| [ ]           | jpeg          | video/avi     |                    |
| [ ]           | jpeg          | video/mp4     |                    |
| [ ]           | bmp           | image/png     |                    |
| [ ]           | bmp           | image/jpeg    |                    |
| [ ]           | bmp           | image/bmp     |                    |
| [ ]           | bmp           | video/avi     |                    |
| [ ]           | bmp           | video/mp4     |                    |


### Video default stratagies

| Supported     | Payload       | Carrier       | Default stratagy   |
| ------------- | ------------- | ------------- | ------------------ |
| [ ]           | avi           | image/avi     |                    |
| [ ]           | avi           | image/mp4     |                    |
| [ ]           | mp4           | image/avi     |                    |
| [ ]           | mp4           | image/mp4     |                    |

