#!/usr/bin/env python3
import hashlib
import os
import pathlib
import shutil
import sys
import subprocess
from pathlib import Path


# There is no sane way to test them.
IGNORE_TESTS = [
    # Disable this if you are on a Mac and want to update the macos tests.
    # 'macos.tests',
    'coretext.tests',
    'directwrite.tests',
    'uniscribe.tests',
    'arabic-fallback-shaping.tests',
]

IGNORE_TEST_CASES = [
    # aots tests

    # in-house tests
    # --shaper=fallback is not supported.
    'simple_002',
    # `dfont` is not supported.
    'collections_001',
    'collections_002',
    'collections_003',
    # Face index out of bounds. ttf-parser doesn't permit this.
    'collections_006',
    # no `hhea` table.
    'indic_decompose_001',
    # Resource exhaustion tests with large outputs
    'morx_34_001',
    'morx_36_001',
    # ttf-parser uses different rounding, not a bug
    'fallback_positioning_001',

    # Requires support for the ltag table.
    'macos_017',

    # No MacRoman encoding support in ttf-parser
    'cmap_3_001',
    'cmap_3_002',
    'cmap_3_003',
    'cmap_3_004',
    'cmap_3_006',
    'cmap_3_008',
    'cmap_3_010',
    'cmap_3_011',
    'cmap_3_012',
    'cmap_3_013',
    'cmap_3_014',
    'cmap_3_017',
]


def update_font_path(tests_name, fontfile):
    if not fontfile.startswith('/'):
        fontfile = fontfile.replace('../fonts/', '')
        return f'tests/fonts/{tests_name}/{fontfile}'  # relative to the root dir
    # macos tests contain absolute paths
    else:
        return fontfile


# Converts `U+0041,U+0078` or `0041,0078` into `\u{0041}\u{0078}`
def convert_unicodes(unicodes):
    text = ''
    for (i, u) in enumerate(unicodes.split(',')):
        if i > 0 and i % 10 == 0:
            text += '\\\n             '

        if u.startswith("U+"):
            u = u[2:]

        text += f'\\u{{{u}}}'

    return text


def convert_test(hb_dir, hb_shape_exe, tests_name, file_name, idx, data, fonts):
    if file_name == 'emoji-clusters.tests':
        return ''  # There are a lot of these; let's skip them

    fontfile, options, unicodes, glyphs_expected = data.split(';')

    # MacOS tests contain hashes, remove them.
    if "@" in fontfile:
        fontfile, _ = fontfile.split('@')


    fontfile_rs = update_font_path(tests_name, fontfile)
    # Some fonts contain escaped spaces, remove them.
    fontfile = fontfile.replace('\\ ', ' ')
    fontfile_rs = fontfile_rs.replace('\\ ', ' ')

    unicodes_rs = convert_unicodes(unicodes)

    test_name = file_name.replace(
        '.tests', '').replace('-', '_') + f'_{idx:03d}'
    test_name = test_name.lower()

    options = options.replace('--shaper=ot', '')
    options = options.replace(
        ' --font-funcs=ft', '').replace('--font-funcs=ft', '')
    options = options.replace(
        ' --font-funcs=ot', '').replace('--font-funcs=ot', '')
    # we don't support font scaling
    options = options.replace('--font-size=1000', '')
    options = options.strip()

    # We have to actually run hb-shape instead of using predefined results,
    # because hb sometimes stores results for freetype and not for embedded OpenType
    # engine, which we are using.
    # Right now, it only affects 'text-rendering-tests'.
    if len(options) != 0:
        options_list = options.split(' ')
    else:
        options_list = []

    options_list.insert(0, str(hb_shape_exe))

    # Force OT functions, since this is the only one we support in rustybuzz.
    options_list.append('--font-funcs=ot')

    abs_font_path = hb_dir.joinpath('test/shape/data')\
        .joinpath(tests_name)\
        .joinpath('tests') \
        .joinpath(fontfile)

    options_list.append(str(abs_font_path))
    options_list.append(f'--unicodes={unicodes}')  # no need to escape it

    glyphs_expected = subprocess.run(options_list, check=True, stdout=subprocess.PIPE)\
        .stdout.decode()

    glyphs_expected = glyphs_expected.strip()[1:-1]  # remove leading and trailing whitespaces and `[..]`
    glyphs_expected = glyphs_expected.replace('|', '|\\\n         ')

    options = options.replace('"', '\\"')
    options = options.replace(' --single-par', '')

    if not fontfile.startswith('/'):
        fonts.add(os.path.split(fontfile_rs)[1])

    if test_name in IGNORE_TEST_CASES:
        return ''

    final_string = (f'#[test]\n'
                    f'fn {test_name}() {{\n'
                    f'    assert_eq!(\n'
                    f'        shape(\n'
                    f'            "{fontfile_rs}",\n'
                    f'            "{unicodes_rs}",\n'
                    f'            "{options}",\n'
                    f'        ),\n'
                    f'        "{glyphs_expected}"\n'
                    f'    );\n'
                    f'}}\n'
                    '\n')

    if file_name == 'macos.tests':
        final_string = '#[cfg(target_os = "macos")]\n' + final_string

    return final_string


def convert(hb_dir, hb_shape_exe, tests_dir, tests_name):
    files = sorted(os.listdir(tests_dir))
    files = [f for f in files if f.endswith('.tests')]

    fonts = set()

    rust_code = ('// WARNING: this file was generated by ../scripts/gen-shaping-tests.py\n'
                 '\n'
                 'use crate::shape;\n'
                 '\n')

    for file in files:
        if file in IGNORE_TESTS:
            continue

        path = tests_dir / file if file != 'macos.tests' else pathlib.Path(__file__).parent / file
        with open(path, 'r') as f:
            for idx, test in enumerate(f.read().splitlines()):
                # skip comments and empty lines
                if test.startswith('#') or len(test) == 0:
                    continue

                rust_code += convert_test(hb_dir, hb_shape_exe, tests_name,
                                          file, idx + 1, test, fonts)

    tests_name_snake_case = tests_name.replace('-', '_')
    with open(f'../tests/shaping/{tests_name_snake_case}.rs', 'w') as f:
        f.write(rust_code)

    return fonts


if len(sys.argv) != 2:
    print('Usage: gen-shaping-tests.py /path/to/harfbuzz-src')
    exit(1)

hb_dir = Path(sys.argv[1])
assert hb_dir.exists()

# Check that harfbuzz was built.
hb_shape_exe = hb_dir.joinpath('builddir/util/hb-shape')
if not hb_shape_exe.exists():
    print('Build harfbuzz first using:')
    print('    meson builddir')
    print('    ninja -Cbuilddir')
    exit(1)

used_fonts = []
font_files = []
test_dir_names = ['aots', 'in-house', 'text-rendering-tests']
for test_dir_name in test_dir_names:
    tests_dir = hb_dir / f'test/shape/data/{test_dir_name}/tests'

    dir_used_fonts = convert(hb_dir, hb_shape_exe, tests_dir, test_dir_name)
    for filename in dir_used_fonts:
        shutil.copy(
            hb_dir / f'test/shape/data/{test_dir_name}/fonts/{filename}',
            f'../tests/fonts/{test_dir_name}')
    used_fonts += dir_used_fonts

    font_files += os.listdir(hb_dir /
                             f'test/shape/data/{test_dir_name}/fonts')

# Check for unused fonts. Just for debugging.
# unused_fonts = sorted(list(set(font_files).difference(used_fonts)))
# if len(unused_fonts) != 0:
#     print('Unused fonts:')
#     for font in unused_fonts:
#         print(font)
