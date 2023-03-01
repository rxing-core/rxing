# rxing-cli
A command line interface for rxing supporting encoding and decoding of barcode data.

## Full documentation
`rxing-cli help`
`rxing-cli help encode`
`rxing-cli help decode`

## Instalation 
`cargo install rxing-cli`

## Example Encode
`rxing-cli test_image.jpg encode --width 500 --height 500 --data "Sample Data and TEST Data" qrcode`

## Example Decode
`rxing-cli test_image.jpg decode`

## Example Multi Barcode Decode
`rxing-cli test_image.jpg decode --decode-multi`