#!/usr/bin/env python3
"""Compile an SVD YAML tree into a CMSIS-SVD XML file.

Input is either:

* a directory (default: the directory containing this script) holding
  ``svd.yaml`` (device header) plus a ``peripherals/`` directory with one
  YAML file per peripheral. ``device.peripherals`` in ``svd.yaml`` is an
  ordered list of the peripheral file names and defines the order of the
  ``<peripheral>`` elements in the output.
* a single YAML file where the device header and peripherals are inline
  (``device.peripherals.peripheral`` is a list of mappings).

Within a YAML file the schema mirrors the SVD element tree one-to-one:

* each mapping key becomes a child element with the same name
* list items repeat the parent key as the element name
* keys starting with ``_`` become attributes of the enclosing element,
  minus the leading underscore (``_derivedFrom`` -> ``derivedFrom=""``)
* a key with no value becomes an empty (self-closing) element
"""

import argparse
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

import yaml


def build(node, tag):
    elem = ET.Element(tag)
    for key, value in node.items():
        if key.startswith('_'):
            elem.set(key[1:], str(value))
            continue
        if isinstance(value, dict):
            elem.append(build(value, key))
        elif isinstance(value, list):
            for item in value:
                if isinstance(item, dict):
                    elem.append(build(item, key))
                else:
                    e = ET.Element(key)
                    e.text = str(item)
                    elem.append(e)
        elif value is None:
            elem.append(ET.Element(key))
        else:
            e = ET.Element(key)
            e.text = str(value)
            elem.append(e)
    return elem


def load_yaml(path):
    with path.open(encoding='utf-8') as f:
        return yaml.safe_load(f)


def load_split(svd_dir):
    root = svd_dir / 'svd.yaml'
    if not root.is_file():
        sys.exit(f'error: {root} not found')
    doc = load_yaml(root)
    if not isinstance(doc, dict) or 'device' not in doc:
        sys.exit(f'error: {root} has no top-level "device" mapping')
    device = doc['device']
    refs = device.pop('peripherals', None)
    if not isinstance(refs, list) or not all(isinstance(r, str) for r in refs):
        sys.exit(f'error: {root} device.peripherals must be a list of file names')

    periph_dir = svd_dir / 'peripherals'
    if not periph_dir.is_dir():
        sys.exit(f'error: {periph_dir} not found')

    peripherals = []
    for ref in refs:
        path = periph_dir / ref
        if not path.is_file():
            sys.exit(f'error: {path} listed in {root} but not found')
        peripheral = load_yaml(path)
        if not isinstance(peripheral, dict) or 'name' not in peripheral:
            sys.exit(f'error: {path} must be a peripheral mapping with a "name"')
        peripherals.append(peripheral)

    on_disk = {p.name for p in periph_dir.iterdir()
               if p.is_file() and p.suffix in ('.yaml', '.yml')}
    orphans = sorted(on_disk - set(refs))
    if orphans:
        sys.exit(f'error: {periph_dir} contains files not listed in '
                 f'{root}: {", ".join(orphans)}')

    device['peripherals'] = {'peripheral': peripherals}
    return device


def load_inline(path):
    doc = load_yaml(path)
    if not isinstance(doc, dict) or 'device' not in doc:
        sys.exit(f'error: {path} has no top-level "device" mapping')
    return doc['device']


def main():
    script_dir = Path(__file__).resolve().parent
    parser = argparse.ArgumentParser(
        description='Compile an SVD YAML tree into a CMSIS-SVD XML file.'
    )
    parser.add_argument(
        'input',
        nargs='?',
        default=str(script_dir),
        help='SVd directory with svd.yaml and peripherals/ '
             '(default: directory containing this script), '
             'or a single YAML file with inline peripherals',
    )
    parser.add_argument(
        '-o',
        '--output',
        help='output XML file (default: <device name>.svd next to the input)',
    )
    args = parser.parse_args()

    input_path = Path(args.input).resolve()
    if input_path.is_dir():
        base_dir = input_path
        device = load_split(input_path)
    elif input_path.is_file():
        base_dir = input_path.parent
        device = load_inline(input_path)
    else:
        sys.exit(f'error: {input_path} not found')

    root = build(device, 'device')
    ET.indent(ET.ElementTree(root), space='  ')
    xml_bytes = ET.tostring(root, encoding='utf-8', xml_declaration=True)

    name = str(device.get('name') or input_path.stem)
    out_path = (
        Path(args.output).resolve()
        if args.output
        else base_dir / f'{name}.svd'
    )
    out_path.write_bytes(xml_bytes + b'\n')
    print(f'{input_path} -> {out_path}')


if __name__ == '__main__':
    main()
