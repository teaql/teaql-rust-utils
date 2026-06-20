import os
import re
import sys

def process_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    if 'delegate!' not in content and 'ContextFileFacade' not in content:
        return

    # Add macro imports
    content = content.replace('use crate::macros::{define_context_facade, delegate};', 'use crate::macros::*;')
    content = content.replace('use crate::macros::define_context_facade;', 'use crate::macros::*;')

    # Fix file.rs manual implementations
    if 'file.rs' in filepath:
        lines = content.split('\n')
        out_lines = []
        for line in lines:
            if 'pub fn read_' in line or 'pub fn exists' in line or 'pub fn is_' in line or 'pub fn list_' in line:
                # read methods
                if 'Result<' in line:
                    line = line.replace('Result<', 'teaql_tool_core::Result<teaql_tool_core::MustPurpose<') + '>'
                else:
                    # exists, is_file
                    parts = line.split('-> ')
                    if len(parts) == 2:
                        ret_type = parts[1].strip().strip('{')
                        line = line.replace('-> ' + ret_type, f'-> teaql_tool_core::MustPurpose<{ret_type}>')
            elif 'pub fn write_' in line or 'pub fn mkdir' in line or 'pub fn delete_' in line or 'pub fn copy' in line or 'pub fn rename' in line:
                # write methods
                if 'Result<' in line:
                    line = line.replace('Result<', 'teaql_tool_core::Result<teaql_tool_core::MustAuditAs<') + '>'
            
            # fix return values inside methods
            if 'FileTool.' in line:
                if 'FileTool.read_' in line or 'FileTool.exists' in line or 'FileTool.is_' in line or 'FileTool.list_' in line:
                    if '.map(' not in line:
                        if 'Result' in out_lines[-1]:
                            line = line.replace('FileTool.', 'FileTool.').rstrip() + '.map(teaql_tool_core::MustPurpose::new)'
                        else:
                            line = line.replace('FileTool.', 'teaql_tool_core::MustPurpose::new(FileTool.').rstrip() + ')'
                elif 'FileTool.write_' in line or 'FileTool.mkdir' in line or 'FileTool.delete_' in line or 'FileTool.copy' in line or 'FileTool.rename' in line:
                    if '.map(' not in line:
                        if 'Result' in out_lines[-1]:
                            line = line.replace('FileTool.', 'FileTool.').rstrip() + '.map(teaql_tool_core::MustAuditAs::new)'
            out_lines.append(line)
        content = '\n'.join(out_lines)
        with open(filepath, 'w') as f:
            f.write(content)
        return

    # parse delegate! block
    # A file can have multiple methods in delegate! block
    # We will split it
    def replacer(match):
        tool_expr = match.group(1)
        methods_str = match.group(2)
        
        methods = [m.strip() for m in methods_str.split(';') if m.strip()]
        
        # Decide which methods go to which macro
        # For cmd, email: write -> audit
        # For kv: read -> purpose
        # For others: comment
        
        comment_methods = []
        res_comment_methods = []
        purpose_methods = []
        res_purpose_methods = []
        audit_methods = []
        res_audit_methods = []
        
        for m in methods:
            is_res = '-> Result<' in m or '-> teaql_tool_core::Result<' in m
            is_write = False
            is_read = False
            
            if 'cmd.rs' in filepath or 'email.rs' in filepath:
                is_write = True
            elif 'kv.rs' in filepath:
                is_read = True
            
            if is_write:
                if is_res:
                    res_audit_methods.append(m)
                else:
                    audit_methods.append(m)
            elif is_read:
                if is_res:
                    res_purpose_methods.append(m)
                else:
                    purpose_methods.append(m)
            else:
                if is_res:
                    res_comment_methods.append(m)
                else:
                    comment_methods.append(m)
        
        res = ""
        if comment_methods:
            res += "delegate_comment! { " + tool_expr + ",\n        " + ";\n        ".join(comment_methods) + "\n    }\n    "
        if res_comment_methods:
            res += "delegate_res_comment! { " + tool_expr + ",\n        " + ";\n        ".join(res_comment_methods) + "\n    }\n    "
        if purpose_methods:
            res += "delegate_purpose! { " + tool_expr + ",\n        " + ";\n        ".join(purpose_methods) + "\n    }\n    "
        if res_purpose_methods:
            res += "delegate_res_purpose! { " + tool_expr + ",\n        " + ";\n        ".join(res_purpose_methods) + "\n    }\n    "
        if audit_methods:
            res += "delegate_audit! { " + tool_expr + ",\n        " + ";\n        ".join(audit_methods) + "\n    }\n    "
        if res_audit_methods:
            res += "delegate_res_audit! { " + tool_expr + ",\n        " + ";\n        ".join(res_audit_methods) + "\n    }\n    "
            
        return res.strip()

    content = re.sub(r'delegate!\s*\{\s*(.*?),\s*(.*?)\s*\}', replacer, content, flags=re.DOTALL)

    with open(filepath, 'w') as f:
        f.write(content)

for root, _, files in os.walk('teaql-tool-context/src/'):
    for file in files:
        if file.endswith('.rs'):
            process_file(os.path.join(root, file))

