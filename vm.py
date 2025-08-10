# -*- coding: utf-8 -*-
from .errors import خطأاقرأ

class VM:
    def __init__(self, code, consts, globals_vars=None, builtins=None, enable_jit=False):
        self.code = code
        self.consts = consts
        self.stack = []
        self.vars = globals_vars if globals_vars is not None else {}
        self.builtins = builtins if builtins is not None else {}
        self.enable_jit = enable_jit
        self.ip = 0

    def push(self, val):
        self.stack.append(val)

    def pop(self):
        return self.stack.pop()

    def run(self):
        while self.ip < len(self.code):
            instr = self.code[self.ip]
            self.ip += 1
            op = instr[0]
            if op == 'PUSH_CONST':
                idx = instr[1]
                self.push(self.consts[idx])
            elif op == 'LOAD_VAR':
                name = instr[1]
                if name in self.vars:
                    self.push(self.vars[name])
                elif name in self.builtins:
                    self.push(self.builtins[name])
                else:
                    raise خطأاقرأ(f'متغير غير معروف: {name}')
            elif op == 'STORE_VAR':
                name = instr[1]
                val = self.pop()
                self.vars[name] = val
            elif op == 'PRINT':
                val = self.pop()
                print(val)
            elif op == 'POP_TOP':
                self.pop()
            else:
                raise خطأاقرأ(f"تعليمة غير معروفة: {op}")
