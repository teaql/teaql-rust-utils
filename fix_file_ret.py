with open("teaql-tool-context/src/std_tools/file.rs", "r") as f:
    lines = f.readlines()

out = []
for line in lines:
    if "pub fn read_" in line or "pub fn list_" in line:
        line = line.replace("Result<String>", "teaql_tool_core::Result<teaql_tool_core::MustPurpose<String>>")
        line = line.replace("Result<Vec<u8>>", "teaql_tool_core::Result<teaql_tool_core::MustPurpose<Vec<u8>>>")
        line = line.replace("Result<Vec<PathBuf>>", "teaql_tool_core::Result<teaql_tool_core::MustPurpose<Vec<PathBuf>>>")
    elif "pub fn write_" in line or "pub fn mkdir" in line or "pub fn delete_" in line or "pub fn copy" in line or "pub fn rename" in line:
        line = line.replace("Result<()>", "teaql_tool_core::Result<teaql_tool_core::MustAuditAs<()>>")
        line = line.replace("Result<u64>", "teaql_tool_core::Result<teaql_tool_core::MustAuditAs<u64>>")
    elif "pub fn exists" in line or "pub fn is_file" in line or "pub fn is_dir" in line:
        line = line.replace("-> bool {", "-> teaql_tool_core::MustPurpose<bool> {")
    out.append(line)

with open("teaql-tool-context/src/std_tools/file.rs", "w") as f:
    f.writelines(out)
