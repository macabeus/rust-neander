# rust-neander
> Do you want the easiest way to learning virtual machine and assembly? You've found it!

**rust-neander** is a virtual machine that runs assembly Neander and Neander+!

They are very didactic hypothetical machines. It is useful if you want to learn more about how CPU works and how to write a simple assembly code, because Neander and Neander+ was developed in order to be simple for beginners. You will learn these languages and hypothetical machines in few minutes!

## Neander hypothetic machine

- memory size: 256
- 8-bit to data width and addresses
- datas are represented using complement two (if the bit 7 is turn on, the number is negative)
- one accumalator with 8-bit (`AC`)
- one program counter with 8-bit (`PC`)
- one status register with 2 condition codes: negative (`N`) and zero (`Z`)

### Neander+ hypothetic machine

This machine is the same of the above plus:

- 256 registers to input data (accessible with the `IN` operator)

## Assembly

Both assembly have few operators. Neander+ is compatible with Neander and it has a little more operators.

### Neander

| Hex code    | Instruction   | Description                                                            |
|-------------|---------------|------------------------------------------------------------------------|
| `00 ... 1F` | `NOP`         | No operation                                                           |
| `10 ... 1F` | `STA address` | Store the value from `AC` at the address                               |
| `20 ... 2F` | `LDA address` | Load at `AC` the value from the address                                |
| `30 ... 3F` | `ADD address` | Add the `AC` plus the value from the address                           |
| `40 ... 4F` | `OR address`  | Use the `OR` binary operator with `AC` and the value from the address  |
| `50 ... 5F` | `AND address` | Use the `AND` binary operator with `AC` and the value from the address |
| `60 ... 6F` | `NOT`         | Use the `NOT` binary operator with `AC`                                |
| `80 ... 8F` | `JMP value`   | Load at `PC` the value                                                 |
| `90 ... 9F` | `JN value`    | If `AC` is a negative value, load at `PC` the value                    |
| `A0 ... AF` | `JZ value`    | If `AC` is zero, load at `PC` the value                                |
| `F0 ... FF` | `HLT`         | Finish the execution                                                   |

If it tries to execute an nonexistent operator, such as `7A`, it will raise an error.

Example to sum 2 and 3:

| Address | Hex code |
| ------- | -------- |
| 00      | 20       |
| 01      | 05       |
| 02      | 30       |
| 03      | 06       |
| 04      | FF       |
| 05      | 02       |
| 06      | 03       |

### Neander+

Neander+ has five more operators than Neander: `SUB`, `JNZ`, `IN`, `OUT`, `LDI`. The remaining operators are the same as the ones Neander has.

Neander+ was *inspirated* in [Neander-X](http://www.dcc.ufrj.br/~gabriel/neander.php)

| Hex code    | Instruction   | Description                                   |
|-------------|---------------|-----------------------------------------------|
| `70 ... 7F` | `SUB address` | Sub the `AC` with the value from the address  |
| `B0 ... BF` | `JNZ value`   | If `AC` is not zero, load at `PC` the value   |
| `C0 ... CF` | `IN index`    | Load at `AC` the value the input at the index |
| `D0 ... DF` | `OUT`         | Copy the `AC` at the out                      |
| `E0 ... EF` | `LDI value`   | Store at `AC` the value                       |

Example to duplicate the value at first position of the input and output the result:

| Address | Hex code |
| ------- | -------- |
| 00      | C0       |
| 01      | 00       |
| 02      | 10       |
| 03      | 07       |
| 04      | 30       |
| 05      | 07       |
| 06      | FF       |
| 07      | 00       |

## How to execute

To execute your code (for example, that you wrote at `ndp/example.ndp` file), you can execute it using:

```
> cargo run ndp/example.ndp
```

And, to set the inputs used in `IN` instruction:

```
> cargo run ndp/example.ndp -i="02 0A"
```
