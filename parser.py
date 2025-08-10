# -*- coding: utf-8 -*-
from . import tokens
from .errors import خطأاقرأ
from . import ast_nodes as ast

class Parser:
    def __init__(self, tokens_list):
        self.tokens = tokens_list
        self.position = 0

    def parse(self):
        statements = []
        while not self._is_at_end():
            statements.append(self._statement())
        return ast.Program(statements)

    def _is_at_end(self):
        return self.position >= len(self.tokens)

    def _peek(self):
        return self.tokens[self.position]

    def _advance(self):
        if not self._is_at_end():
            self.position += 1

    def _match(self, *types):
        if self._is_at_end():
            return False
        if self._peek().type in types:
            self._advance()
            return True
        return False

    def _consume(self, type_, msg):
        if self._match(type_):
            return self.tokens[self.position-1]
        raise خطأاقرأ(msg)

    def _statement(self):
        tok = self._peek()
        if tok.type == 'PRINT':
            self._advance()
            expr = self._expression()
            return ast.PrintStatement(expr)
        # assignment: IDENT '=' expr
        if tok.type == 'IDENT':
            next_tok = None
            if self.position + 1 < len(self.tokens):
                next_tok = self.tokens[self.position + 1]
            if next_tok and next_tok.type == '=':
                # consume IDENT and '='
                self._advance()  # IDENT
                name = tok.value
                self._advance()  # '='
                expr = self._expression()
                return ast.VarAssign(name, expr)
        expr = self._expression()
        return ast.ExpressionStatement(expr)

    def _expression(self):
        return self._primary()

    def _primary(self):
        tok = self._peek()
        if tok.type == 'NUMBER':
            self._advance()
            return ast.NumberLiteral(tok.value)
        if tok.type == 'STRING':
            self._advance()
            return ast.StringLiteral(tok.value)
        if tok.type == 'IDENT':
            self._advance()
            return ast.VarReference(tok.value)
        raise خطأاقرأ(f"تعبير غير متوقع: {tok.type}")
