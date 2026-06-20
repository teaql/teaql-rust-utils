with open("teaql-tool-context/src/macros.rs", "r") as f:
    content = f.read()

content = content.replace("teaql_tool_core::$ret", "teaql_tool_core::MustPurpose<$ret>")

with open("teaql-tool-context/src/macros.rs", "w") as f:
    f.write(content)
