# -*- coding: utf-8 -*-
from .lexer import Lexer
from .parser import Parser
from .interpreter import Interpreter
from .errors import خطأاقرأ

def repl_loop():
    print("مرحبًا بك في REPL للغة اقرأ. اكتب ':quit' للخروج.")
    interp = Interpreter()
    buffer = []
    while True:
        try:
            line = input(">>> " if not buffer else "... ")
        except EOFError:
            break
        if line.strip() == ':quit':
            break
        if not line.strip():
            if buffer:
                source = '\n'.join(buffer)
                try:
                    lexer = Lexer(source)
                    toks = lexer.tokenize()
                    parser = Parser(toks)
                    program = parser.parse()
                    interp.interpret(program)
                except خطأاقرأ as e:
                    print("خطأ:", e)
                buffer = []
            continue
        buffer.append(line)
