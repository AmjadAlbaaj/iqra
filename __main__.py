# -*- coding: utf-8 -*-
import sys
import os

# Support running as: python -m iqra  OR python __main__.py (inside iqra/)
if __name__ == "__main__" and __package__ is None:
    # when executed as a script, ensure package imports work
    sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
    from repl import repl_loop
else:
    from .repl import repl_loop

from .lexer import Lexer
from .parser import Parser
from .interpreter import Interpreter

def main():
    if len(sys.argv) == 1:
        repl_loop()
    else:
        filename = sys.argv[1]
        with open(filename, encoding='utf-8') as f:
            source = f.read()
        lexer = Lexer(source)
        tokens = lexer.tokenize()
        parser = Parser(tokens)
        program = parser.parse()
        interp = Interpreter()
        interp.interpret(program)

if __name__ == "__main__":
    main()
