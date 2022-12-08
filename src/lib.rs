use std::cell::RefCell;
use std::pin::Pin;

use pyo3::{exceptions::PyValueError, prelude::*, types::PyUnicode};

use hashbrown::HashMap;
use vibrato_rust::{tokenizer::worker::Worker, Dictionary, Tokenizer};

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
    /// :type out: str
    #[pyo3(text_signature = "($self, /)")]
    fn surface(&self, py: Python) -> Py<PyUnicode> {
        self.list.borrow(py).tokens[self.index].0.clone_ref(py)
    }

    /// Return the start position (inclusive) in characters.
    ///
    /// :type out: int
    #[pyo3(text_signature = "($self, /)")]
    fn start(&self, py: Python) -> usize {
        self.list.borrow(py).tokens[self.index].1
    }

    /// Return the end position (exclusive) in characters.
    ///
    /// :type out: int
    #[pyo3(text_signature = "($self, /)")]
    fn end(&self, py: Python) -> usize {
        self.list.borrow(py).tokens[self.index].2
    }

    /// Return the feature of this token.
    ///
    /// :type out: str
    #[pyo3(text_signature = "($self, index, /)")]
    fn feature(&self, py: Python) -> Py<PyUnicode> {
        self.list.borrow(py).tokens[self.index].3.clone_ref(py)
    }

    fn __str__(&self, py: Python) -> Py<PyUnicode> {
        self.surface(py)
    }

    fn __repr__(&self, py: Python) -> PyResult<String> {
        let list = self.list.borrow(py);
        let surface = list.tokens[self.index].0.as_ref(py).to_str()?;
        let feature = list.tokens[self.index].3.as_ref(py).to_str()?;
        Ok(format!(
            "Token {{ surface: {:?}, feature: {:?} }}",
            surface, feature
        ))
    }
}

/// Iterator of tokens.
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

/// Token list returned by the tokenizer.
#[pyclass]
struct TokenList {
    tokens: Vec<(Py<PyUnicode>, usize, usize, Py<PyUnicode>)>,
}

#[pymethods]
impl TokenList {
    fn __len__(&self) -> usize {
        self.tokens.len()
    }

    fn __getitem__(self_: Py<Self>, py: Python, index: usize) -> PyResult<Token> {
        if index < self_.borrow(py).tokens.len() {
            Ok(Token { list: self_, index })
        } else {
            Err(PyValueError::new_err("list index out of range"))
        }
    }

    fn __iter__(self_: Py<Self>, py: Python) -> TokenIterator {
        let len = self_.borrow(py).tokens.len();
        TokenIterator {
            list: self_,
            index: 0,
            len,
        }
    }
}

/// Python binding of Vibrato tokenizer.
///
/// Examples:
///     >>> import vibrato
///     >>> with open('path/to/system.dic', 'rb') as fp:
///     >>>     dict_data = fp.read()
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
/// :param ignore_space: Ignores spaces from tokens.
///                      This option is for compatibility with MeCab. Enable this if you want to
///                      obtain the same results as MeCab.
/// :param max_grouping_len: Specifies the maximum grouping length for unknown words.
///                          By default, the length is infinity.
///                          This option is for compatibility with MeCab. Specifies the argument
///                          with 24 if you want to obtain the same results as MeCab.
/// :type dict_data: bytes
/// :type ignore_space: bool
/// :type max_grouping_len: int
/// :type out: vibrato.Vibrato
#[pyclass]
#[pyo3(text_signature = "($self, dict_data, /, ignore_space = False, max_grouping_len = 0)")]
struct Vibrato {
    tokenizer: Pin<Box<Tokenizer>>,
    worker: Option<Worker<'static>>,
    surface_cache: RefCell<HashMap<String, Py<PyUnicode>>>,
    feature_cache: RefCell<HashMap<String, Py<PyUnicode>>>,
}

impl Vibrato {
    fn init_worker(&mut self) {
        if self.worker.is_some() {
            return;
        }
        // Safety: The tokenizer is pinned and will continue to live until the Vibrato struct is
        // removed.
        let tokenizer: Pin<&'static Tokenizer> = unsafe {
            std::mem::transmute::<Pin<&Tokenizer>, Pin<&'static Tokenizer>>(Pin::as_ref(
                &self.tokenizer,
            ))
        };
        self.worker.replace(tokenizer.get_ref().new_worker());
    }
}

#[pymethods]
impl Vibrato {
    #[new]
    #[args(ignore_space = "false", max_grouping_len = "0")]
    pub fn new(dict_data: &[u8], ignore_space: bool, max_grouping_len: usize) -> PyResult<Self> {
        let dict = Dictionary::read(dict_data).map_err(|e| PyValueError::new_err(e.to_string()))?;
        let tokenizer = Tokenizer::new(dict)
            .ignore_space(ignore_space)
            .map_err(|e| PyValueError::new_err(e.to_string()))?
            .max_grouping_len(max_grouping_len);
        Ok(Self {
            tokenizer: Box::pin(tokenizer),
            worker: None,
            surface_cache: RefCell::new(HashMap::new()),
            feature_cache: RefCell::new(HashMap::new()),
        })
    }

    /// Tokenize a given text and return as a list of tokens.
    ///
    /// :param text: A text to tokenize.
    /// :type text: str
    /// :type out: vibrato.TokenList
    #[pyo3(text_signature = "($self, text, /)")]
    fn tokenize(&mut self, py: Python, text: &str) -> TokenList {
        self.init_worker();
        let worker = self.worker.as_mut().unwrap();
        worker.reset_sentence(text);
        worker.tokenize();
        let surface_cache = &mut self.surface_cache.borrow_mut();
        let feature_cache = &mut self.feature_cache.borrow_mut();
        TokenList {
            tokens: worker
                .token_iter()
                .map(|token| {
                    let surface = surface_cache
                        .raw_entry_mut()
                        .from_key(token.surface())
                        .or_insert_with(|| {
                            (
                                token.surface().to_string(),
                                PyUnicode::new(py, token.surface()).into(),
                            )
                        })
                        .1
                        .clone_ref(py);
                    let feature = feature_cache
                        .raw_entry_mut()
                        .from_key(token.feature())
                        .or_insert_with(|| {
                            (
                                token.feature().to_string(),
                                PyUnicode::new(py, token.feature()).into(),
                            )
                        })
                        .1
                        .clone_ref(py);
                    let start = token.range_char().start;
                    let end = token.range_char().end;
                    (surface, start, end, feature)
                })
                .collect(),
        }
    }
}

#[pymodule]
fn vibrato(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vibrato>()?;
    m.add_class::<TokenList>()?;
    m.add_class::<Token>()?;
    Ok(())
}
