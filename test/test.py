from pdb_parser import Structure
import json

structure = Structure("7v39.pdb")
print(structure)

chain_a = structure.chains[0]
print(chain_a.get_sequence())

residue = chain_a.residues[0]
print(residue)

json_repr_str = structure.dump_to_json()
json_repr = json.loads(json_repr_str)

print(json_repr["name"])
