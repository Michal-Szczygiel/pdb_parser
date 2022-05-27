from pdb_parser import Structure

structure = Structure("7v39.pdb")
print(structure)

chain_a = structure.chains[0]
print(chain_a.get_sequence())

residue = chain_a.residues[0]
print(residue)
