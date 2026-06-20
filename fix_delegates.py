import os
import re

def fix_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    lines = content.split('\n')
    out_lines = []
    in_res_block = False
    
    for line in lines:
        if 'delegate_res_' in line:
            in_res_block = True
            out_lines.append(line)
            continue
        
        if in_res_block:
            if line.strip() == '}':
                in_res_block = False
                out_lines.append(line)
                continue
            
            # extract inner type of Result<T> or teaql_tool_core::Result<T>
            m = re.search(r'->\s*(?:teaql_tool_core::)?Result<(.*?)>(;?)$', line)
            if m:
                inner = m.group(1)
                semi = m.group(2)
                line = re.sub(r'->\s*(?:teaql_tool_core::)?Result<.*?>;?$', f'-> {inner}{semi}', line)
        
        out_lines.append(line)

    with open(filepath, 'w') as f:
        f.write('\n'.join(out_lines))

for root, _, files in os.walk('teaql-tool-context/src/'):
    for file in files:
        if file.endswith('.rs'):
            fix_file(os.path.join(root, file))

