# 🐍 python-vibrato 🎤

[Vibrato](https://github.com/daac-tools/vibrato) is a fast implementation of tokenization (or morphological analysis) based on the Viterbi algorithm.
This is a Python wrapper for Vibrato.

## Installation

### Build from source

You need to install the Rust compiler following [the documentation](https://www.rust-lang.org/tools/install) beforehand.
daachorse uses `pyproject.toml`, so you also need to upgrade pip to version 19 or later.

```
$ pip install --upgrade pip
```

After setting up the environment, you can install daachorse as follows:

```
$ pip install .
```

## Example Usage

python-vibrato does not contain model files.
To perform tokenization, follow [the document of Vibrato](https://github.com/daac-tools/vibrato) to download distribution models or train your own models beforehand.

```python
import vibrato

with open('path/to/system.dic', 'rb') as fp:
    dict_data = fp.read()
tokenizer = vibrato.Vibrato(dict_data)

tokens = tokenizer.tokenize('社長は火星猫だ')

len(tokens)
#=> 5

list(tokens)
#=> [Token { surface: "社長", feature: "名詞,一般,*,*,*,*,社長,シャチョウ,シャチョー,," },
#    Token { surface: "は", feature: "助詞,係助詞,*,*,*,*,は,ハ,ワ,," },
#    Token { surface: "火星", feature: "名詞,一般,*,*,*,*,火星,カセイ,カセイ,," },
#    Token { surface: "猫", feature: "名詞,一般,*,*,*,*,猫,ネコ,ネコ,," },
#    Token { surface: "だ", feature: "助動詞,*,*,*,特殊・ダ,基本形,だ,ダ,ダ,," }]

tokens[0].surface()
#=> '社長'

tokens[0].feature()
#=> '名詞,一般,*,*,*,*,社長,シャチョウ,シャチョー,,'

tokens[0].start()
#=> 0

tokens[0].end()
#=> 2
```

## Documentation

Use the help function to show the API reference.

```python
import vibrato
help(vibrato)
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
