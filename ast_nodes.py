# -*- coding: utf-8 -*-
from dataclasses import dataclass
from typing import List, Optional, Any, Tuple

class AST:
    """Base class for AST nodes (marker)."""
    pass

@dataclass
class Program(AST):
    statements: List[Any]

@dataclass
class ExpressionStatement(AST):
    expression: Any

@dataclass
class PrintStatement(AST):
    expression: Optional[Any] = None

@dataclass
class StringLiteral(AST):
    value: str

@dataclass
class NumberLiteral(AST):
    value: float

@dataclass
class BooleanLiteral(AST):
    value: bool

@dataclass
class ListLiteral(AST):
    elements: List[Any]

@dataclass
class DictLiteral(AST):
    pairs: List[Tuple[Any, Any]]

@dataclass
class BinaryOp(AST):
    left: Any
    op: str
    right: Any

@dataclass
class UnaryOp(AST):
    op: str
    operand: Any

@dataclass
class VarAssign(AST):
    name: str
    expression: Any

@dataclass
class VarReference(AST):
    name: str

@dataclass
class Index(AST):
    collection: Any
    index: Any

@dataclass
class SetItem(AST):
    collection: Any
    index: Any
    expression: Any

@dataclass
class Slice(AST):
    collection: Any
    start: Optional[Any]
    stop: Optional[Any]

@dataclass
class IfStatement(AST):
    condition: Any
    true_statements: List[Any]
    false_statements: List[Any]

@dataclass
class WhileStatement(AST):
    condition: Any
    body: List[Any]

@dataclass
class FunctionDef(AST):
    name: str
    params: List[str]
    body: List[Any]

@dataclass
class FunctionCall(AST):
    name: Any
    args: List[Any]

@dataclass
class ReturnStatement(AST):
    expression: Optional[Any]
