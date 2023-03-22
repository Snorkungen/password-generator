# Password Generator

This program takes a master string and key strings. Using theese inputs the program generates deterministically a password.

Why? Well the reasoning is that using this somehow create a statless password manager I.E a password manager that doesn't keep a record of password in any format ecrypted or not.

How? The idea is to have program that generates the same output given the same output. This program takes the master and key inputs and generates a password. How would a person use this? Let's say someone needed a new password for the website "example.com". They would then use the program by using a master string which could be a pass phrase and then use "example.com" as the key.

Issues, i assume this solution is less secure than any other statefull password manager. Due to the passwords generated not being truly random. Thus leading to vulnerabilities i can't think of.

## Example

```shell
cargo run -- --master master --key key
```

```shell
WtNVywZZG0~H4W`[VA[wv8Zyyt
```
