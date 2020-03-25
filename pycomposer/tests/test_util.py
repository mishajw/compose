import unittest

from pycomposer import util, AbstractNote, Note
from pycomposer.chord import Chord
from pycomposer.scale import Scale, AbstractScale


class TestUtil(unittest.TestCase):
    def test_note(self):
        self.assertEqual(util.note("a4"), Note(AbstractNote.A, 4))
        self.assertEqual(util.note("f#6"), Note(AbstractNote.Fs, 6))

    def test_chord(self):
        self.assertEqual(
            util.chord("c4 maj"),
            Chord([Note(AbstractNote.C, 4), Note(AbstractNote.E, 4), Note(AbstractNote.G, 4)]),
        )

    def test_chord_scale_indexed(self):
        self.assertEqual(
            util.chord("c4 maj 3"),
            Chord([Note(AbstractNote.E, 4), Note(AbstractNote.G, 4), Note(AbstractNote.B, 4)]),
        )

    def test_scale(self):
        self.assertEqual(
            util.scale("c4 chromatic"), Scale(Note(AbstractNote.C, 4), AbstractScale([1] * 12)),
        )
