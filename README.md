# Motywacja
Niniejszy projekt został przygotowany w celu zaprezentowania możliwości języka [Rust](https://www.rust-lang.org/) (głównie systemu makr oraz bogatej biblioteki rozszerzeń) w dziedzinie tworzenia
natywnych rozszerzeń dla języka Python (v3). Dodatkowymi zaletami są tu również:
- przenośność kodu bez konfiguracji - projekt powinien się kompilować bez problemu na wszystkich popularnych platformach,
- związłość kodu - Rust jest pod wieloma względami podobny do języka Python (składnia jest stosunkowo łatwa i ekspresywna),
- niezawodność kodu - zgodnie z założeniami języka, niemożliwym powinno być (w bezpiecznym podzbiorze języka) dopuszczenie do wystąpienia zachowania niezdefiniowanego (w szczególności wycieków pamięci)

# Kompilacja projektu
Poza posiadaniam zainstalowanego interpretera języka Python, niezbędny jest również kompilator języka Rust (najlepiej jest pobrać i zainstalować cały pakiet narzędzi z
[oficjalnego źródła](https://www.rust-lang.org/tools/install)).

Kroki:
- utworzyć wirtualne środowisko Pythona (i zainicjalizować je),
- zainstalować w tym środowisku rozszerzenie: Maturin (pip install maturin), więcej informacji [tutaj](https://github.com/PyO3/maturin),
- wywołać polecenie **maturin develop** (w katalogu projektu, tam gdzie znajduje się plik **pyproject.toml**) - biblioteka zostanie zbudowana i zainstalowana w 
uruchomionym wirtualnym środowisku Pythona. Można dodać flagę **--release** - kompilacja przebiegnie wtedy z włączonymi optymalizacjami.
- uruchomić skrypt **test/test.py** - powinno zostać wydrukowane podsumowanie dotyczące cząsteczki znajdującej się w dołączonym pliku .pdb oraz sekwencja aminokwasowa 
pierwszego łańcucha.

# O samym projekcie:
Projekt korzysta z rustowej biblioteki [PyO3](https://pyo3.rs/v0.16.4/). Rozszerzenie zawiera prosty parser plików .pdb. Plik **pdb_parser.pyi** zawiera sygnatury klas i funkcji wchodzących w skład rozszerzenia. Plik **test/test.py** zawiera
przykład użycia tego rozszerzenia.
