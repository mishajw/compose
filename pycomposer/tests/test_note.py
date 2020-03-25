import unittest

from pycomposer import Note, AbstractNote


class TestAbstractNote(unittest.TestCase):
    def test_next(self):
        self.assertEqual(AbstractNote.A.next(), AbstractNote.As)

    def test_next_wrapped(self):
        self.assertEqual(AbstractNote.B.next(), AbstractNote.C)

    def test_prev(self):
        self.assertEqual(AbstractNote.A.prev(), AbstractNote.Gs)

    def test_prev_wrapped(self):
        self.assertEqual(AbstractNote.C.prev(), AbstractNote.B)


class TestNote(unittest.TestCase):
    def test_next(self):
        self.assertEqual(Note(AbstractNote.A.next(), 3), Note(AbstractNote.As, 3))

    def test_next_wrapped(self):
        self.assertEqual(Note(AbstractNote.B, 3).next(), Note(AbstractNote.C, 4))
