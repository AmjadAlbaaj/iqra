# -*- coding: utf-8 -*-
import re
from . import tokens
from .errors import خطأاقرأ

class Token:
    def __init__(self, type_, value, line, column):
        self.type = type_
        self.value = value
        self.line = line
        self.column = column
    def __repr__(self):
        return f"Token({self.type}, {self.value}, {self.line}:{self.column})"

class Lexer:
    def __init__(self, source):
        self.source = source
        self.position = 0
        self.line = 1
        self.column = 1

    def tokenize(self):
        tokens_list = []
        while not self._is_at_end():
            char = self._peek()
            if char.isspace():
                self._advance_whitespace()
                continue
            if char.isdigit():
                tokens_list.append(self._number())
                continue
            if char.isalpha() or char == '_':
                tokens_list.append(self._identifier_or_keyword())
                continue
            if char in ('"', "'"):
                tokens_list.append(self._string())
                continue
            two_char = self._peek(2)
            if two_char in (tokens.EQ, tokens.NE, tokens.LE, tokens.GE):
                tokens_list.append(Token(two_char, two_char, self.line, self.column))
                self._advance(2)
                continue
            if char in (tokens.PLUS, tokens.MINUS, tokens.MULT, tokens.DIV,
                        tokens.LT, tokens.GT, '=', '(', ')', '{', '}', ',', ':', '[', ']'):
                tokens_list.append(Token(char, char, self.line, self.column))
                self._advance()
                continue
            raise خطأاقرأ(f"رمز غير معروف: {char}", self.line, self.column)
        return tokens_list

    def _is_at_end(self):
        return self.position >= len(self.source)

    def _peek(self, length=1):
        if self.position + length > len(self.source):
            return self.source[self.position:]
        return self.source[self.position:self.position+length]

    def _advance(self, n=1):
        for _ in range(n):
            if self._is_at_end():
                return
            if self.source[self.position] == '\n':
                self.line += 1
                self.column = 1
            else:
                self.column += 1
            self.position += 1

    def _advance_whitespace(self):
        while not self._is_at_end() and self._peek().isspace():
            self._advance()

    def _number(self):
        start_line, start_col = self.line, self.column
        num_str = ''
        while not self._is_at_end() and (self._peek().isdigit() or self._peek() == '.'):
            num_str += self._peek()
            self._advance()
        return Token('NUMBER', float(num_str), start_line, start_col)

    def _identifier_or_keyword(self):
        start_line, start_col = self.line, self.column
        ident = ''
        while not self._is_at_end() and (self._peek().isalnum() or self._peek() == '_'):
            ident += self._peek()
            self._advance()
        if ident in tokens.KEYWORDS:
            return Token(tokens.KEYWORDS[ident], ident, start_line, start_col)
        return Token('IDENT', ident, start_line, start_col)

    def _string(self):
        quote = self._peek()
        start_line, start_col = self.line, self.column
        self._advance()  # skip quote
        val = ''
        while not self._is_at_end() and self._peek() != quote:
            val += self._peek()
            self._advance()
        if self._is_at_end():
            raise خطأاقرأ("سلسلة نصية غير منتهية", start_line, start_col)
        self._advance()  # skip closing quote
        return Token('STRING', val, start_line, start_col)
