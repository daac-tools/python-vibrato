# π python-vibrato π€

[Vibrato](https://github.com/daac-tools/vibrato) is a fast implementation of tokenization (or morphological analysis) based on the Viterbi algorithm.
This is a Python wrapper for Vibrato.

[![PyPI](https://img.shields.io/pypi/v/vibrato)](https://pypi.org/project/vibrato/)
[![Build Status](https://github.com/daac-tools/python-vibrato/actions/workflows/CI.yml/badge.svg)](https://github.com/daac-tools/python-vibrato/actions)
[![Documentation Status](https://readthedocs.org/projects/python-vibrato/badge/?version=latest)](https://python-vibrato.readthedocs.io/en/latest/?badge=latest)

## Installation

### Install pre-built package from PyPI

Run the following command:

```
$ pip install vibrato
```

### Build from source

You need to install the Rust compiler following [the documentation](https://www.rust-lang.org/tools/install) beforehand.
daachorse uses `pyproject.toml`, so you also need to upgrade pip to version 19 or later.

```
$ pip install --upgrade pip
```

After setting up the environment, you can install daachorse as follows:

```
$ pip install git+https://github.com/daac-tools/python-vibrato
```

## Example Usage

python-vibrato does not contain model files.
To perform tokenization, follow [the document of Vibrato](https://github.com/daac-tools/vibrato) to download distribution models or train your own models beforehand.

Check the version number as shown below to use compatible models:

```python
import vibrato
vibrato.VIBRATO_VERSION
#=> "0.3.3"
```

Examples:

```python
import vibrato

with open('path/to/system.dic', 'rb') as fp:
    dict_data = fp.read()
tokenizer = vibrato.Vibrato(dict_data)

tokens = tokenizer.tokenize('η€Ύι·γ―η«ζη«γ ')

len(tokens)
#=> 5

list(tokens)
#=> [Token { surface: "η€Ύι·", feature: "εθ©,δΈθ¬,*,*,*,*,η€Ύι·,γ·γ£γγ§γ¦,γ·γ£γγ§γΌ,," },
#    Token { surface: "γ―", feature: "ε©θ©,δΏε©θ©,*,*,*,*,γ―,γ,γ―,," },
#    Token { surface: "η«ζ", feature: "εθ©,δΈθ¬,*,*,*,*,η«ζ,γ«γ»γ€,γ«γ»γ€,," },
#    Token { surface: "η«", feature: "εθ©,δΈθ¬,*,*,*,*,η«,γγ³,γγ³,," },
#    Token { surface: "γ ", feature: "ε©εθ©,*,*,*,ηΉζ?γ»γ,εΊζ¬ε½’,γ ,γ,γ,," }]

tokens[0].surface()
#=> 'η€Ύι·'

tokens[0].feature()
#=> 'εθ©,δΈθ¬,*,*,*,*,η€Ύι·,γ·γ£γγ§γ¦,γ·γ£γγ§γΌ,,'

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
