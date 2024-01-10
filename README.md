# Shift Carry File Encryptor/Decryptor

I Know this is maybe a stupid idea but i made a encryptor that do shift carry operations on every bytes on the file.

## Usage:

| Input | Direction | Nb of iterations | Output file *(optional)* | Inverse output *(optional, ***false by default***)* |
|-------|-----------|------------------|--------------------------|-----------------------------------------------------|
|**File path** or **String value**| *"left"* or *"right"* | 1-7 | A file path | *"true"* or "false" |

**Warning:**

*If `output_file` is not set then if the input is a file path then it replace it and if it's a string input then it just output it in the console...*

## Tip:

If you want to decrypt a file you need to do the inverse operations to encrypt _(logic)_

Ex:

To encrypt...:

`shift_carry_encryptor foo.txt left 2 bar.txt`

To decrypt...:

`shift_carry_encryptor bar.txt right 2 foo.txt`

## How it works

The **S**hift **C**arry **E**ncryptor*(SCE in short)* have 2 steps of bit manipulation per bytes in the input:

1. Invert the byte **If `Inverse Output` is set to `true` in the arguments!** *(e.g: `0110` -> `1001`)*
2. Shift Carry depending in the direction given *(e.g: Shift carry left 1 -> `1001` -> `0011` | Shift carry right 1 -> `1101` -> `1110`)*
