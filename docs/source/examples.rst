Example usage
=============

python-vibrato does not contain model files. To perform tokenization, follow `the document of
Vibrato <https://github.com/daac-tools/vibrato>`_ to download distribution models or train
your own models beforehand.

You can check the version number as shown below to use compatible models:

.. code-block:: python

   >>> import vibrato
   >>> vibrato.VIBRATO_VERSION
   '0.3.3'

Examples:

.. code-block:: python

   >>> import vibrato

   >>> with open('path/to/system.dic', 'rb') as fp:
   >>>     dict_data = fp.read()
   >>> tokenizer = vibrato.Vibrato(dict_data)

   >>> tokens = tokenizer.tokenize('社長は火星猫だ')

   >>> len(tokens)
   5

   >>> list(tokens)
   [Token { surface: "社長", feature: "名詞,一般,*,*,*,*,社長,シャチョウ,シャチョー,," },
    Token { surface: "は", feature: "助詞,係助詞,*,*,*,*,は,ハ,ワ,," },
    Token { surface: "火星", feature: "名詞,一般,*,*,*,*,火星,カセイ,カセイ,," },
    Token { surface: "猫", feature: "名詞,一般,*,*,*,*,猫,ネコ,ネコ,," },
    Token { surface: "だ", feature: "助動詞,*,*,*,特殊・ダ,基本形,だ,ダ,ダ,," }]

   >>> tokens[0].surface()
   '社長'

   >>> tokens[0].feature()
   '名詞,一般,*,*,*,*,社長,シャチョウ,シャチョー,,'

   >>> tokens[0].start()
   0

   >>> tokens[0].end()
   2
