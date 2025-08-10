# -*- coding: utf-8 -*-
from iqra.stdlib import text, date, files
import os

def test_text_join_and_split():
    s = text.join(["a","b"], "-")
    assert s == "a-b"
    parts = text.split(s, "-")
    assert parts == ["a", "b"]

def test_date_today_and_now():
    today = date.today()
    now = date.now()
    assert isinstance(today, str)
    assert isinstance(now, str)

def test_files_read_write(tmp_path):
    p = tmp_path / "file.txt"
    files.write(str(p), "مرحبا")
    content = files.read(str(p))
    assert "مرحبا" in content
    assert files.exists(str(p))
    assert os.path.basename(str(p)) in files.listdir(str(tmp_path))
