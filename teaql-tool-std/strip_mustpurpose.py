import os
import re
import sys

def strip_mustpurpose(content):
    # Remove use teaql_tool_core::MustPurpose
    content = re.sub(r'use teaql_tool_core::\{([^}]*?)MustPurpose,([^}]*?)\};', r'use teaql_tool_core::{\1\2};', content)
    content = re.sub(r'use teaql_tool_core::\{([^}]*?), MustPurpose([^}]*?)\};', r'use teaql_tool_core::{\1\2};', content)
    content = re.sub(r'use teaql_tool_core::MustPurpose;\n', '', content)
    content = re.sub(r'use teaql_tool_core::\{MustPurpose, ([^}]*?)\};', r'use teaql_tool_core::{\1};', content)

    # Change MustPurpose<T> to T
    content = re.sub(r'MustPurpose<([^>]+)>', r'\1', content)

    # Remove MustPurpose::new( ... )
    # This is tricky due to nested parens, so we use a simple balanced parenthesis replacer
    
    def remove_new(text):
        res = ""
        i = 0
        while i < len(text):
            if text[i:].startswith("MustPurpose::new("):
                start = i + len("MustPurpose::new(")
                # find matching paren
                parens = 1
                j = start
                while j < len(text) and parens > 0:
                    if text[j] == '(':
                        parens += 1
                    elif text[j] == ')':
                        parens -= 1
                    j += 1
                res += text[start:j-1]
                i = j
            else:
                res += text[i]
                i += 1
        return res
        
    content = remove_new(content)
    
    # Handle .map(MustPurpose::new)
    content = content.replace(".map(MustPurpose::new)", "")
    
    # In case there's an empty import like use teaql_tool_core::{};
    content = re.sub(r'use teaql_tool_core::\{\s*};\n', '', content)
    
    return content

for arg in sys.argv[1:]:
    with open(arg, 'r') as f:
        content = f.read()
    new_content = strip_mustpurpose(content)
    with open(arg, 'w') as f:
        f.write(new_content)

