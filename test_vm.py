# -*- coding: utf-8 -*-
from iqra.vm import VM
from iqra.errors import خطأاقرأ

def test_vm_push_and_pop():
    vm = VM([], [])
    vm.push(10)
    assert vm.pop() == 10

def test_vm_unknown_instruction():
    vm = VM([("FOO",)], [])
    try:
        vm.run()
    except خطأاقرأ as e:
        assert "FOO" in str(e)
