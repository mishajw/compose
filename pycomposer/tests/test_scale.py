import itertools
import unittest

from pycomposer import Note, AbstractNote
from pycomposer.chord import Scale
from pycomposer.scale import AbstractScale


class TestScale(unittest.TestCase):
    def test_notes(self):
        scale = Scale(Note(AbstractNote.C, 5), AbstractScale([2, 2, 1, 2, 2, 2, 1]))
        self.assertEqual(
            list(itertools.islice(scale.notes(), 8)),
            [
                Note(AbstractNote.C, 5),
                Note(AbstractNote.D, 5),
                Note(AbstractNote.E, 5),
                Note(AbstractNote.F, 5),
                Note(AbstractNote.G, 5),
                Note(AbstractNote.A, 5),
                Note(AbstractNote.B, 5),
                Note(AbstractNote.C, 6),
            ],
        )
