# -*- coding: utf-8 -*-
from typing import Any
from . import ast_nodes as ast
from .errors import خطأاقرأ, push_context, pop_context
from . import tokens
import random

class _ReturnSignal(Exception):
    def __init__(self, value: Any):
        self.value = value

class Interpreter:
    def __init__(self, builtins: dict = None):
        self.frames = [{}]
        self.functions = {}
        self.builtins = {
            'ادخل': lambda prompt='': input(prompt),
            'عشوائي': lambda a=0, b=1: random.randint(int(a), int(b)),
            'اطبع': lambda *args: print(*args),
        }
        if builtins:
            self.builtins.update(builtins)

    def current_env(self):
        return self.frames[-1]

    def push_env(self):
        self.frames.append({})

    def pop_env(self):
        if len(self.frames) == 1:
            self.frames[-1].clear()
        else:
            self.frames.pop()

    def resolve_var(self, name: str):
        for env in reversed(self.frames):
            if name in env:
                return env[name]
        if name in self.functions:
            return self.functions[name]
        if name in self.builtins:
            return self.builtins[name]
        return None

    def store_var(self, name: str, val: Any):
        for env in reversed(self.frames):
            if name in env:
                env[name] = val
                return
        self.current_env()[name] = val

    def interpret(self, node: ast.AST):
        push_context(f"Node:{node.__class__.__name__}")
        try:
            method = getattr(self, f'visit_{node.__class__.__name__}', None)
            if method is None:
                raise خطأاقرأ(f'لا يوجد معالج للعقدة: {node.__class__.__name__}')
            return method(node)
        except خطأاقرأ:
            raise
        except Exception as e:
            raise خطأاقرأ(f"خطأ داخلي أثناء معالجة {node.__class__.__name__}: {e}", None, None, cause=e, include_traceback=True)
        finally:
            pop_context()

    def visit_Program(self, node: ast.Program):
        for stmt in node.statements:
            self.interpret(stmt)

    def visit_ExpressionStatement(self, node: ast.ExpressionStatement):
        return self.interpret(node.expression)

    def visit_PrintStatement(self, node: ast.PrintStatement):
        val = self.interpret(node.expression) if node.expression is not None else None
        printer = self.builtins.get('اطبع', print)
        if val is None:
            return printer()
        return printer(val)

    def visit_StringLiteral(self, node: ast.StringLiteral):
        return node.value

    def visit_NumberLiteral(self, node: ast.NumberLiteral):
        return node.value

    def visit_VarAssign(self, node: ast.VarAssign):
        val = self.interpret(node.expression)
        self.store_var(node.name, val)
        return val

    def visit_VarReference(self, node: ast.VarReference):
        val = self.resolve_var(node.name)
        if val is None:
            raise خطأاقرأ(f'متغير أو دالة غير معرفة: {node.name}')
        return val
