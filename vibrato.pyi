from collections.abc import Iterable
from collections.abc import Iterator

VIBRATO_VERSION: str

class Token:
    def end(self) -> int: ...
    def feature(self) -> str: ...
    def start(self) -> int: ...
    def surface(self) -> str: ...

class TokenIterator(Iterator[Token]):
    def __next__(self) -> Token: ...

class TokenList(Iterable[Token]):
    def __getitem__(self, index: int) -> Token: ...
    def __iter__(self) -> TokenIterator: ...
    def __len__(self) -> int: ...

class Vibrato:
    def __init__(
        self, dict_data: bytes, ignore_space: bool = False, max_grouping_len: int = 0
    ) -> None: ...
    @staticmethod
    def from_textdict(
        lex_data: str,
        matrix_data: str,
        char_data: str,
        unk_data: str,
        ignore_space: bool = False,
        max_grouping_len: int = 0,
    ) -> Vibrato: ...
    def tokenize(self, text: str) -> TokenList: ...
    def tokenize_to_surfaces(self, text: str) -> list[str]: ...
