import re
import sys
from typing import Any
from yaml import dump, Dumper

with open('defs/map.hs') as f:
    defs_lines = {i + 1: line for i, line in enumerate(f.readlines()) }
    
# Lines 82 - 156 contain Province names, in CamelCase
provinces: dict[str, dict[str, Any]] = {}
for lineno in range(82, 157):
    line = defs_lines[lineno]
    # Extract tne name in CamelCase
    if match := re.search(r'(?P<province>\w+$)', line):
        province: str = match['province']
        provinces[province] = {}
    else:
        print(f'Error on line {lineno}: expected province name', file=sys.stderr)
        exit(1)

for province, province_data in provinces.items():
    # Convert from CamelCase to Title Case
    name = re.sub(r'(?<=\w)(?=[A-Z])', ' ', province)
    # Lowercase the word 'of'
    name = re.sub(r'(?<= )Of(?= )', 'of', name)
    # Add the hyphen to Mid-Atlantic Ocean
    name = re.sub(r'(?<=Mid) ', '-', name)
    
    province_data['display_name'] = name

# Lines 163 - 237 contain province types
for lineno in range(163, 238):
    line = defs_lines[lineno]
    if match := re.match(r'provinceType (?P<province>\w+) = (?P<terrain>Inland|Coastal|Water)', line):
        province: str = match['province']
        terrain: str = match['terrain']
        provinces[province]['terrain'] = terrain
    else:
        print(f'Error on line {lineno}: expected terrain definition', file=sys.stderr)
        exit(1)

# Lines 242-316 contain adjacency information
for lineno in range(242, 317):
    line = defs_lines[lineno]
    if match := re.match(r'adjacency (?P<province>\w+) = \[(?P<neighbors>\w+(, \w+)*)]', line):
        province: str = match['province']
        neighbors: list[str] = match['neighbors'].split(', ')
        provinces[province]['neighbors'] = neighbors
    else:
        print(f'Error on line {lineno}: expected adjacency information', file=sys.stderr)
        exit(1)
        
# Lines 326-359 contain supply centers
for lineno in range(326, 360):
    line = defs_lines[lineno]
    if match := re.match(r'supplyCentre (?P<province>\w+)', line):
        province: str = match['province']
        provinces[province]['supply_center'] = True
    else:
        print(f'Error on line {lineno}: expected supply center', file=sys.stderr)
        exit(1)
        
# Lines 372-412 contain country information
for lineno in range(372, 413):
    line = defs_lines[lineno]
    if match := re.match(r'country (?P<province>\w+) = Just (?P<country>\w+)', line):
        province: str = match['province']
        country: str = match['country']
        provinces[province]['country'] = country
        
# Coasts I'll do by hand
provinces['StPetersburg']['coasts'] = {
    'North': ['Norway', 'BarentsSea'],
    'South': ['Livonia', 'GulfOfBothnia', 'Finland']
}
provinces['Spain']['coasts'] = {
    'North': ['Gascony', 'MidAtlanticOcean', 'Portugal'],
    'South': ['Portugal', 'MidAtlanticOcean', 'WesternMediterranean', 'GulfOfLyon', 'Marseilles']
}
provinces['Bulgaria']['coasts'] = {
    'East': ['Constantinople', 'BlackSea', 'Rumania'],
    'South': ['Constantinople', 'AegeanSea', 'Greece']
}

with open('defs/map.yaml', 'w') as f:
    dump(provinces, f, Dumper=Dumper)
