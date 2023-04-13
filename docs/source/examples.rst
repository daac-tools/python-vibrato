Example usage
=============

python-vibrato does not contain model files. To perform tokenization, follow `the document of
Vibrato <https://github.com/daac-tools/vibrato>`_ to download distribution models or train
your own models beforehand.

You can check the version number as shown below to use compatible models:

.. code-block:: python

   >>> import vibrato
   >>> vibrato.VIBRATO_VERSION
   '0.5.0'

Examples:

.. code-block:: python

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

The distributed models are compressed in zstd format. If you want to load these compressed models,
you must decompress them outside the API:

.. code-block:: python

   >>> import vibrato
   >>> import zstandard  # zstandard package in PyPI

   >>> dctx = zstandard.ZstdDecompressor()
   >>> with open('tests/data/system.dic.zst', 'rb') as fp:
   ...     with dctx.stream_reader(fp) as dict_reader:
   ...         tokenizer = vibrato.Vibrato(dict_reader.read())
