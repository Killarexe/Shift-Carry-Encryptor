# Shift Carry File Encryptor/Decryptor

I Know this is maybe a stupid idea but i made a encryptor that do shift carry operations on every bytes on the file.

## Usage:

`shift_carry_encryptor <file_path|value> <left|right> <iterations> (output_file)`

## Tip:

If you want to decrypt a file you need to do the inverse operations to encrypt _(logic)_

Ex:

To encrypt...:

`shift_carry_encryptor foo.txt left 2 bar.txt`

To decrypt...:

`shift_carry_encryptor bar.txt right 2 foo.txt`
