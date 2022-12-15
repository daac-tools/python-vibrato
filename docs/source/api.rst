API reference
=============

.. autoclass:: vibrato.Vibrato
   :members:

.. autoclass:: vibrato.TokenList
   :special-members: __getitem__, __iter__, __len__

.. autoclass:: vibrato.TokenIterator
   :special-members: __next__

.. autoclass:: vibrato.Token
   :members:

.. data:: VIBRATO_VERSION
   :type: str
   :canonical: vibrato.VIBRATO_VERSION

   Indicates the version number of *vibrato* used by this wrapper. It can be used to check the
   compatibility of the model file.
