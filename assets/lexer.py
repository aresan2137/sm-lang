import json
import re
import sys
import os

TOKEN_SPEC = [
    ('COMMENT', r'//.*'),
    ('KEY', r'\bfunc|class|enum|ret|var|if|else|new|pub|use\b'),
    ('CPP_BLOCK', r'\$\$\([\s\S]*?\)\$\$'),
    ('NUMBER', r'\d+(\.\d+)?'),
    ('STRING', r'"[^"]*"'),
    ('SCOPE', r'::'),
    ('SYMBOL', r'\+=|-=|\*=|/=|==|!=|<=|>=|\+\+|--|&&|\|\||[+\-*/%&|^<>!=]'),
    ('ASSIGN', r'='),
    ('MEMBER', r'\.'),
    ('BRACKET', r'[\(\)\[\]\{\}]'),
    ('SEMICOLON', r';'),
    ('COMMA', r','),
    ('VALUE', r'[A-Za-z_]\w*'),
    ('SKIP', r'[ \t]+'),
    ('NEWLINE', r'\n'),
    ('MISMATCH', r'.'),
]

def preprocess(code, current_file_path):
    base_dir = os.path.dirname(os.path.abspath(current_file_path))
    new_lines = []
    
    for line in code.splitlines():
        match = re.search(r'use\s+"([^"]+)"', line)
        if match:
            include_path = match.group(1)
            potential_paths = [
                os.path.join(base_dir, include_path),
                os.path.join(os.getcwd(), include_path),
                include_path
            ]
            
            found = False
            for path in potential_paths:
                full_path = os.path.normpath(path)
                if os.path.exists(full_path):
                    print(f"[*] Preprocessor: Including {full_path}")
                    with open(full_path, "r", encoding="utf-8") as f:
                        content = f.read()
                        new_lines.append(preprocess(content, full_path))
                    found = True
                    break
            
            if not found:
                print(f"[!] Preprocessor Error: Could not find file '{include_path}'")
                print(f"    Searched in: {potential_paths}")
        else:
            new_lines.append(line)
            
    return "\n".join(new_lines)

def tokenize(code):
    tokens = []
    line_num = 1
    tok_regex = '|'.join('(?P<%s>%s)' % pair for pair in TOKEN_SPEC)

    for mo in re.finditer(tok_regex, code):
        kind = mo.lastgroup
        value = mo.group()
        
        if kind == 'NUMBER':
            value = str(value)
        elif kind == "COMMENT" or kind == 'SKIP':
            continue
        elif kind == 'NEWLINE':
            line_num += 1
            continue
        elif kind == 'MISMATCH':
            continue

        tokens.append({
            "type": kind,
            "value": str(value),
            "line": line_num
        })
    return tokens

def main():
    if len(sys.argv) < 2:
        return

    input_file = sys.argv[1]

    try:
        with open(input_file, "r", encoding="utf-8") as f:
            raw_code = f.read()
        
        processed_code = preprocess(raw_code, input_file)
        
        tokens = tokenize(processed_code)

        with open("assets/in.json", "w", encoding="utf-8") as f:
            json.dump(tokens, f, indent=2)

    except Exception as e:
        print(f"Lexer error: {e}")

if __name__ == "__main__":
    main()