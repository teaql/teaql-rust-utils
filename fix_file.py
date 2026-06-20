with open("teaql-tool-context/src/std_tools/file.rs", "r") as f:
    content = f.read()

content = content.replace(" {>", "> {")
content = content.replace(" >{", "> {")

with open("teaql-tool-context/src/std_tools/file.rs", "w") as f:
    f.write(content)
