use pyo3::{exceptions::PyValueError, prelude::*, types::PyUnicode};

use hashbrown::HashMap;
use ouroboros::self_referencing;
use vibrato_rust::{
    dictionary::{LexType, WordIdx},
    tokenizer::worker::Worker,
    Dictionary, SystemDictionaryBuilder, Tokenizer,
};

/// Representation of a token.
#[pyclass]
struct Token {
    list: Py<TokenList>,
    index: usize,
}

#[pymethods]
impl Token {
    /// Return the surface of this token.
    ///
    /// :rtype: str
    fn surface(&self, py: Python) -> Py<PyUnicode> {
        self.list.borrow(py).tokens[self.index].0.clone_ref(py)
    }

    /// Return the start position (inclusive) in characters.
    ///
    /// :rtype: int
    fn start(&self, py: Python) -> usize {
        self.list.borrow(py).tokens[self.index].1
    }

    /// Return the end position (exclusive) in characters.
    ///
    /// :rtype: int
    fn end(&self, py: Python) -> usize {
        self.list.borrow(py).tokens[self.index].2
    }

    /// Return the feature of this token.
    ///
    /// :rtype: str
    fn feature(&self, py: Python) -> Py<PyUnicode> {
        let list = self.list.borrow(py);
        let word_idx = list.tokens[self.index].3;
        let vibrato = &mut *list.vibrato.borrow_mut(py);
        vibrato
            .feature_cache
            .raw_entry_mut()
            .from_key(&word_idx)
            .or_insert_with(|| {
                let token = vibrato
                    .wrapper
                    .borrow_tokenizer()
                    .dictionary()
                    .word_feature(word_idx);
                (word_idx, PyUnicode::new(py, token).into())
            })
            .1
            .clone_ref(py)
    }

    fn __str__(&self, py: Python) -> Py<PyUnicode> {
        self.surface(py)
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        let list = self.list.borrow(py);
        let surface = list.tokens[self.index].0.as_ref(py).to_str()?;
        let word_idx = list.tokens[self.index].3;
        let wrapper = &list.vibrato.borrow(py).wrapper;
        let feature = wrapper
            .borrow_tokenizer()
            .dictionary()
            .word_feature(word_idx);
        Ok(format!(
            "Token {{ surface: {:?}, feature: {:?} }}",
            surface, feature
        ))
    }
}

/// Iterator that returns :class:`Token`.
#[pyclass]
struct TokenIterator {
    list: Py<TokenList>,
    index: usize,
    len: usize,
}

#[pymethods]
impl TokenIterator {
    fn __next__(&mut self, py: Python) -> Option<Token> {
        if self.index < self.len {
            let index = self.index;
            self.index += 1;
            Some(Token {
                list: self.list.clone_ref(py),
                index,
            })
        } else {
            None
        }
    }
}

/// List of :class:`.Token` returned by the tokenizer.
#[pyclass]
struct TokenList {
    vibrato: Py<Vibrato>,
    tokens: Vec<(Py<PyUnicode>, usize, usize, WordIdx)>,
}

#[pymethods]
impl TokenList {
    fn __len__(&self) -> usize {
        self.tokens.len()
    }

    fn __getitem__(self_: PyRef<Self>, index: usize) -> PyResult<Token> {
        if index < self_.tokens.len() {
            Ok(Token {
                list: self_.into(),
                index,
            })
        } else {
            Err(PyValueError::new_err("list index out of range"))
        }
    }

    fn __iter__(self_: PyRef<Self>) -> TokenIterator {
        let len = self_.tokens.len();
        TokenIterator {
            list: self_.into(),
            index: 0,
            len,
        }
    }
}

#[self_referencing]
pub struct TokenizerWrapper {
    tokenizer: Tokenizer,
    #[borrows(tokenizer)]
    #[covariant]
    worker: Worker<'this>,
}

/// Python binding of Vibrato tokenizer.
///
/// Examples:
///     >>> import vibrato
///     >>> with open('path/to/system.dic', 'rb') as fp:
///     ...     dict_data = fp.read()
///     >>> tokenizer = vibrato.Vibrato(dict_data)
///     >>> tokens = tokenizer.tokenize('社長は火星猫だ')
///     >>> len(tokens)
///     5
///     >>> list(tokens)
///     [Token { surface: "社長", feature: "名詞,一般,*,*,*,*,社長,シャチョウ,シャチョー,," },
///      Token { surface: "は", feature: "助詞,係助詞,*,*,*,*,は,ハ,ワ,," },
///      Token { surface: "火星", feature: "名詞,一般,*,*,*,*,火星,カセイ,カセイ,," },
///      Token { surface: "猫", feature: "名詞,一般,*,*,*,*,猫,ネコ,ネコ,," },
///      Token { surface: "だ", feature: "助動詞,*,*,*,特殊・ダ,基本形,だ,ダ,ダ,," }]
///     >>> tokens[0].surface()
///     '社長'
///     >>> tokens[0].feature()
///     '名詞,一般,*,*,*,*,社長,シャチョウ,シャチョー,,'
///     >>> tokens[0].start()
///     0
///     >>> tokens[0].end()
///     2
///
/// :param dict_data: A byte sequence of the dictionary.
/// :param ignore_space: Ignores spaces from tokens. This option is for compatibility with MeCab.
///      Enable this if you want to obtain the same results as MeCab.
/// :param max_grouping_len: Specifies the maximum grouping length for unknown words. By default,
///      the length is infinity. This option is for compatibility with MeCab. Specifies the
///      argument with 24 if you want to obtain the same results as MeCab.
/// :type dict_data: bytes
/// :type ignore_space: bool
/// :type max_grouping_len: int
/// :rtype: vibrato.Vibrato
#[pyclass]
#[pyo3(text_signature = "(dict_data, /, ignore_space = False, max_grouping_len = 0)")]
struct Vibrato {
    wrapper: TokenizerWrapper,
    surface_cache: HashMap<WordIdx, Py<PyUnicode>>,
    feature_cache: HashMap<WordIdx, Py<PyUnicode>>,
}

#[pymethods]
impl Vibrato {
    #[new]
    #[pyo3(signature = (dict_data, /, ignore_space=false, max_grouping_len=0))]
    pub fn new(dict_data: &[u8], ignore_space: bool, max_grouping_len: usize) -> PyResult<Self> {
        let dict = Dictionary::read(dict_data).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let tokenizer = Tokenizer::new(dict)
            .ignore_space(ignore_space)
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .max_grouping_len(max_grouping_len);
        let wrapper = TokenizerWrapperBuilder {
            tokenizer,
            worker_builder: |tokenizer: &Tokenizer| tokenizer.new_worker(),
        }
        .build();
        Ok(Self {
            wrapper,
            surface_cache: HashMap::new(),
            feature_cache: HashMap::new(),
        })
    }

    /// Create a tokenizer from the text dictionary.
    ///
    /// :param lex_data: The content of ``lex.csv``.
    /// :param matrix_data: The content of ``matrix.def``.
    /// :param char_data: The content of ``char.def``.
    /// :param unk_data: The content of ``unk.def``.
    /// :param ignore_space: Ignores spaces from tokens. This option is for compatibility with
    ///     MeCab. Enable this if you want to obtain the same results as MeCab.
    /// :param max_grouping_len: Specifies the maximum grouping length for unknown words. By
    ///     default, the length is infinity. This option is for compatibility with MeCab.
    ///     Specifies the argument with 24 if you want to obtain the same results as MeCab.
    /// :type lex_data: str
    /// :type matrix_data: str
    /// :type char_data: str
    /// :type unk_data: str
    /// :type ignore_space: bool
    /// :type max_grouping_len: int
    /// :rtype: vibrato.Vibrato
    #[staticmethod]
    #[pyo3(
        text_signature = "(lex_data, matrix_data, char_data, unk_data, /, ignore_space = False, max_grouping_len = 0)"
    )]
    #[pyo3(signature = (lex_data, matrix_data, char_data, unk_data, /, ignore_space=false, max_grouping_len=0))]
    pub fn from_textdict(
        lex_data: &str,
        matrix_data: &str,
        char_data: &str,
        unk_data: &str,
        ignore_space: bool,
        max_grouping_len: usize,
    ) -> PyResult<Self> {
        let dict = SystemDictionaryBuilder::from_readers(
            lex_data.as_bytes(),
            matrix_data.as_bytes(),
            char_data.as_bytes(),
            unk_data.as_bytes(),
        )
        .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let tokenizer = Tokenizer::new(dict)
            .ignore_space(ignore_space)
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .max_grouping_len(max_grouping_len);
        let wrapper = TokenizerWrapperBuilder {
            tokenizer,
            worker_builder: |tokenizer: &Tokenizer| tokenizer.new_worker(),
        }
        .build();
        Ok(Self {
            wrapper,
            surface_cache: HashMap::new(),
            feature_cache: HashMap::new(),
        })
    }

    /// Tokenize a given text and return as a list of tokens.
    ///
    /// :param text: A text to tokenize.
    /// :type text: str
    /// :rtype: vibrato.TokenList
    #[pyo3(signature = (text, /))]
    fn tokenize(mut self_: PyRefMut<Self>, py: Python, text: &str) -> TokenList {
        self_.wrapper.with_worker_mut(|worker| {
            worker.reset_sentence(text);
            worker.tokenize();
        });
        let self_deref = &mut *self_;
        let tokens = self_deref
            .wrapper
            .borrow_worker()
            .token_iter()
            .map(|token| {
                // Surface strings need to be converted to Python strings immediately because those
                // strings are stored in the Worker.
                // On the other hand, the feature strings are stored in the Tokenizer, not the
                // Worker, so feature strings are converted as needed.
                let word_idx = token.word_idx();
                // Cache the string only when the token is contained in the dictionary.
                let surface = if word_idx.lex_type != LexType::Unknown {
                    self_deref
                        .surface_cache
                        .raw_entry_mut()
                        .from_key(&word_idx)
                        .or_insert_with(|| (word_idx, PyUnicode::new(py, token.surface()).into()))
                        .1
                        .clone_ref(py)
                } else {
                    PyUnicode::new(py, token.surface()).into()
                };
                let start = token.range_char().start;
                let end = token.range_char().end;
                let word_idx = token.word_idx();
                (surface, start, end, word_idx)
            })
            .collect();
        TokenList {
            vibrato: self_.into(),
            tokens,
        }
    }

    /// Tokenize a given text and return as a list of surfaces.
    ///
    /// :param text: A text to tokenize.
    /// :type text: str
    /// :rtype: list[str]
    #[pyo3(signature = (text, /))]
    fn tokenize_to_surfaces(&mut self, py: Python, text: &str) -> Vec<Py<PyUnicode>> {
        self.wrapper.with_worker_mut(|worker| {
            worker.reset_sentence(text);
            worker.tokenize();
        });
        self.wrapper
            .borrow_worker()
            .token_iter()
            .map(|token| {
                let word_idx = token.word_idx();
                if word_idx.lex_type != LexType::Unknown {
                    self.surface_cache
                        .raw_entry_mut()
                        .from_key(&word_idx)
                        .or_insert_with(|| (word_idx, PyUnicode::new(py, token.surface()).into()))
                        .1
                        .clone_ref(py)
                } else {
                    PyUnicode::new(py, token.surface()).into()
                }
            })
            .collect()
    }
}

#[pymodule]
fn vibrato(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vibrato>()?;
    m.add_class::<TokenIterator>()?;
    m.add_class::<TokenList>()?;
    m.add_class::<TokenIterator>()?;
    m.add_class::<Token>()?;
    m.add("VIBRATO_VERSION", vibrato_rust::VERSION)?;
    Ok(())
}
