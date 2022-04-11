
# Bech32m encoding and decoding tool

## Getting the binary
### Windows
Download bech32m.exe from [latest release](https://github.com/B4rtosek/PA193/releases).  
You can verify the file by downloading bech32m.exe.gpg and running `gpg --verify .\bech32m.exe.gpg`
### Linux
Download bech32m from [latest release](https://github.com/B4rtosek/PA193/releases).  
You can verify the file by downloading bech32m.gpg and running `gpg --verify .\bech32m.exe.gpg`

## Using the tool
### Decoding
#### Decode bech32m to hex
 ```
$ .\bech32m.exe decode --input bc1xt2wg4d5ape8zhcf3s4seqx85pdcleyhc89eys54m78p5s
32d4e455b4e872715f098c2b0c80c7a05b8fe497c1cb924295
```
#### Decode bech32m to base64
 ```
$ .\bech32m.exe decode --input bc1xt2wg4d5ape8zhcf3s4seqx85pdcleyhc89eys54m78p5s --output-format base64
MtTkVbTocnFfCYwrDIDHoFuP5JfBy5JClQ==
```

#### Decode bech32m to binary
 ```
$ .\bech32m.exe decode --input bc1xt2wg4d5ape8zhcf3s4seqx85pdcleyhc89eys54m78p5s --output-format binary
00110010110101001110010001010101101101001110100001110010011100010101111100001001100011000010101100001100100000001100011110100000010110111000111111100100100101111100000111001011100100100100001010010101
 ```
 
### Encoding
#### Encode bech32m from hex
 ```
$ .\bech32m.exe encode --input 32d4e455b4e872715f098c2b0c80c7a05b8fe497c1cb924295 --hrp bc
bc1xt2wg4d5ape8zhcf3s4seqx85pdcleyhc89eys54m78p5s
```
#### Encode bech32m from base64
 ```
$ .\bech32m.exe encode --input MtTkVbTocnFfCYwrDIDHoFuP5JfBy5JClQ== --hrp bc --input-format base64
bc1xt2wg4d5ape8zhcf3s4seqx85pdcleyhc89eys54m78p5s
```

#### Encode bech32m from binary
 ```
$ .\bech32m.exe encode --input 00110010110101001110010001010101101101001110100001110010011100010101111100001001100011000010101100001100100000001100011110100000010110111000111111100100100101111100000111001011100100100100001010010101 --hrp bc --input-format binary
bc1xt2wg4d5ape8zhcf3s4seqx85pdcleyhc89eys54m78p5s
```
