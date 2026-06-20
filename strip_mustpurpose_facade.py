import os
import re

def process_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # Change MustPurpose<T> to T
    content = re.sub(r'MustPurpose<([^>]+)>', r'\1', content)
    # in case of nested MustPurpose, but there shouldn't be

    with open(filepath, 'w') as f:
        f.write(content)

for root, _, files in os.walk('teaql-tool-context/src/'):
    for file in files:
        if file.endswith('.rs'):
            process_file(os.path.join(root, file))

