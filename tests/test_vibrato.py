import os.path
import pathlib

import vibrato


def create_tokenizer():
    dict_path = pathlib.PurePath(__file__).parent / 'data'
    with open(dict_path / 'lex.csv', encoding='utf-8') as fp:
        lex_data = fp.read()
    with open(dict_path / 'matrix.def', encoding='utf-8') as fp:
        matrix_data = fp.read()
    with open(dict_path / 'char.def', encoding='utf-8') as fp:
        char_data = fp.read()
    with open(dict_path / 'unk.def', encoding='utf-8') as fp:
        unk_data = fp.read()
    return vibrato.Vibrato.from_textdict(lex_data, matrix_data, char_data, unk_data)


def test_tokenlist_index():
    tokenizer = create_tokenizer()
    tokens = tokenizer.tokenize('まぁ社長は火星猫だ')

    assert 'まぁ' == tokens[0].surface()
    assert '社長' == tokens[1].surface()
    assert 'は' == tokens[2].surface()
    assert '火星' == tokens[3].surface()
    assert '猫' == tokens[4].surface()
    assert 'だ' == tokens[5].surface()


def test_tokenlist_iter():
    tokenizer = create_tokenizer()
    tokens = tokenizer.tokenize('まぁ社長は火星猫だ')

    assert ['まぁ', '社長', 'は', '火星', '猫', 'だ'] == list(
        token.surface() for token in tokens
    )


def test_tokenlist_iter_positions():
    tokenizer = create_tokenizer()
    tokens = tokenizer.tokenize('まぁ社長は火星猫だ')

    assert [(0, 2), (2, 4), (4, 5), (5, 7), (7, 8), (8, 9)] == list(
        (token.start(), token.end()) for token in tokens
    )


def test_feature():
    tokenizer = create_tokenizer()
    tokens = tokenizer.tokenize('まぁ社長は火星猫だ')

    assert [
        '名詞,固有名詞,一般,*',
        '名詞,普通名詞,一般,*',
        '助詞,係助詞,*,*',
        '名詞,固有名詞,一般,*',
        '名詞,普通名詞,*,*',
        '助動詞,*,*,*',
    ] == list(token.feature() for token in tokens)


def test_tokenize_to_surfaces():
    tokenizer = create_tokenizer()
    assert ['まぁ', '社長', 'は', '火星', '猫', 'だ'] == tokenizer.tokenize_to_surfaces(
        'まぁ社長は火星猫だ'
    )
