class Atom:
    """Klasa Atom reprezentuje pojedynczy atom struktury cząsteczki."""
    name: str
    pos_x: float
    pos_y: float
    pos_z: float

    def __init__(self, name: str, pos_x: float, pos_y: float, pos_z: float) -> Atom: ...
    def __repr__(self) -> str: ...


class Residue:
    """Klasa Residue reprezentuje pojedynczy aminokwas struktury."""
    name: str
    sequence_number: int
    atoms: list[Atom]

    def __init__(self, name: str, sequence_number: int, atoms: list[Atom]) -> Residue: ...
    def __repr__(self) -> str: ...


class Chain:
    """Klasa Chain reprezentuje pojedynczy łańcych struktury."""
    name: str
    residues: list[Residue]

    def __init__(self, name: str, residues: list[Residue]) -> Chain: ...
    def __repr__(self) -> str: ...
    def __len__(self) -> int:
        """Zwraca długość łańcucha."""
    def get_sequence(self) -> list[str]:
        """Zwraca sekwencję aminokwasów łańcucha."""


class Structure:
    """Klasa Structure reprezentuje całą strukturę."""
    name: str
    chains: list[Chain]

    def __init__(self, file_path: str) -> Structure: ...
    def __repr__(self) -> str: ...
