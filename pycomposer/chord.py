import itertools
from typing import NamedTuple, List

from pycomposer import Note
from pycomposer.scale import Scale, ABSTRACT_SCALES
from pycomposer.scale_index import ScaleIndex


class Chord(NamedTuple):
    """
    Chord defined by a finite list of notes.
    """

    notes: List[Note]

    @classmethod
    def major_scale_indexed(cls, note: Note, indices: List["ScaleIndex"]):
        """
        Creates a chord from indices in the major scale.

        The indices can be adjusted, making the major scale's note flat or sharp.
        """
        maj_scale = Scale(note, ABSTRACT_SCALES["maj"])
        return Chord([maj_scale[index] for index in indices])

    @classmethod
    def scale_indexed(cls, scale: Scale, index: int):
        """
        Creates a chord from alternate notes in a scale, starting at `index`.

        E.g. with index 3 resulting in [note 3, note 5, note 7] in `scale`.
        """
        notes = list(itertools.islice(scale.notes(), index - 1, index + 4))
        return Chord([notes[0], notes[2], notes[4]])

    def frequencies(self) -> List[float]:
        return [note.frequency() for note in self.notes]
