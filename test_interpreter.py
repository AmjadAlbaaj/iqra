# -*- coding: utf-8 -*-
import io
import sys
import builtins
import pytest
from iqra.lexer import Lexer
from iqra.parser import Parser
from iqra.interpreter import Interpreter

def run_code(source):
    lexer = Lexer(source)
    tokens = lexer.tokenize()
    parser = Parser(tokens)
    program = parser.parse()
    interp = Interpreter()
    return interp.interpret(program)

def test_print_output(capsys):
    run_code("اطبع 'مرحبا'")
    out, err = capsys.readouterr()
    assert "مرحبا" in out

def test_number_literal():
    result = run_code("اطبع 123")
    # The output is printed, not returned, so test via capsys
    assert result is None

def test_variable_assignment_and_reference(capsys):
    src = "س = 42\nاطبع س"
    run_code(src)
    out, _ = capsys.readouterr()
    assert "42" in out
