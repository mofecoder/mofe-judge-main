import csv
import pathlib
import sys
from typing import Final

INDENT: Final[str] = ' ' * 4


mime_list = [
    ('C++', 'text/x-c++-src'),
    ('Java', 'text/x-java'),
    ('Python', 'text/x-python'),
    ('PyPy', 'text/x-python'),
    ('C#', 'text/x-csharp'),
    ('Rust', 'text/x-rustsrc'),
    ('Ruby', 'text/x-ruby'),
    ('Kotlin', 'text/x-kotlin'),
    ('Go', 'text/x-go'),
    ('Fortran', 'text/x-fortran'),
    ('Crystal', 'text/x-crystal'),
    ('Perl', 'text/x-perl'),
    ('Bash', 'text/x-sh'),
    ('C', 'text/x-c-src'),
]


def find_mime(name: str):
    for n, mime in mime_list:
        if n in name:
            return mime
    return 'text/plain'

def main():
    if len(sys.argv) != 2:
        print(f'Usage: {sys.argv[0]} [csv-file-name]')
        exit(1)

    path = pathlib.Path(sys.argv[1])
    if not path.exists():
        print('File not found.')
        exit(1)

    with open(path, 'r') as f:
        reader = csv.reader(f)

        output_rs = ''
        output_json = ''

        for lang in reader:
            output_rs += '\n'.join([
                INDENT * 1 + 'map.insert(',
                INDENT * 2 + f'"{lang[0]}".to_string(),',
                INDENT * 2 + "Command::new(",
                INDENT * 3 + f'"{lang[3] if lang[3] else ":"}",',
                INDENT * 3 + f'"{lang[4]}",',
                INDENT * 3 + f'"{lang[5]}"',
                INDENT * 2 + ')',
                INDENT * 1 + ');'
            ]) + '\n'
            output_json += '\n'.join([
                INDENT * 1 + '{',
                INDENT * 2 + f'"innerName": "{lang[0]}",',
                INDENT * 2 + f'"name": "{lang[1]}",',
                INDENT * 2 + f'"mime": "{find_mime(lang[1])}",',
                INDENT * 2 + f'"compilationCommand": "{lang[3] if lang[3] else ""}",',
                INDENT * 2 + f'"runCommand": "{lang[4]}"',
                INDENT * 1 + '},'
            ]) + '\n'

    base = open('./lang_cmd_base.rs', 'r').readlines()
    with open(f'{path.name}.rs', 'w') as f:
        for ln in base:
            f.write(ln)
            if '$INSERT_HERE$' in ln:
                f.write(output_rs)
    base = open('./lang_cmd_base_json', 'r').readlines()
    with open(f'{path.name}.json', 'w') as f:
        for ln in base:
            if '$INSERT_HERE$' in ln:
                f.write(output_json)
            else:
                f.write(ln)

    print('Complete')


if __name__ == '__main__':
    main()
