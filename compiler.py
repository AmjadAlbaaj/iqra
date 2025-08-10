# -*- coding: utf-8 -*-
from . import ast_nodes as ast
from .errors import خطأاقرأ

class Compiler:
    def __init__(self):
        self.code = []
        self.consts = []
        self.varnames = []

    def compile(self, node):
        method = getattr(self, f'compile_{node.__class__.__name__}', None)
        if not method:
            raise خطأاقرأ(f"لا يوجد مترجم لهذه العقدة: {node.__class__.__name__}")
        method(node)

    def add_const(self, value):
        if value in self.consts:
            return self.consts.index(value)
        self.consts.append(value)
        return len(self.consts) - 1

    def compile_Program(self, node: ast.Program):
        for stmt in node.statements:
            self.compile(stmt)
