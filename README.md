# 🐍 python-vibrato 🎤

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
vibrato uses `pyproject.toml`, so you also need to upgrade pip to version 19 or later.

```
$ pip install --upgrade pip
```

After setting up the environment, you can install vibrato as follows:

```
$ pip install git+https://github.com/daac-tools/python-vibrato
```

## Example Usage

python-vibrato does not contain model files.
To perform tokenization, follow [the document of Vibrato](https://github.com/daac-tools/vibrato) to download distribution models or train your own models beforehand.

Check the version number as shown below to use compatible models:

```python
>>> import vibrato
>>> vibrato.VIBRATO_VERSION
'0.5.0'

```

Examples:

```python
>>> import vibrato

>>> with open('tests/data/system.dic', 'rb') as fp:
...     tokenizer = vibrato.Vibrato(fp.read())

>>> tokens = tokenizer.tokenize('社長は火星猫だ')

>>> len(tokens)
5

>>> tokens[0]
Token { surface: "社長", feature: "名詞,普通名詞,一般,*" }

>>> tokens[0].surface()
'社長'

>>> tokens[0].feature()
'名詞,普通名詞,一般,*'

>>> tokens[0].start()
0

>>> tokens[0].end()
2

```

## Note for distributed models

The distributed models are compressed in zstd format. If you want to load these compressed models,
you must decompress them outside the API.

```python
>>> import vibrato
>>> import zstandard  # zstandard package in PyPI

>>> dctx = zstandard.ZstdDecompressor()
>>> with open('tests/data/system.dic.zst', 'rb') as fp:
...     with dctx.stream_reader(fp) as dict_reader:
...         tokenizer = vibrato.Vibrato(dict_reader.read())

```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
