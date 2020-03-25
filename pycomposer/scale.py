import itertools
from typing import NamedTuple, Iterable, List, Dict

from pycomposer import Note
from pycomposer.scale_index import ScaleIndex


class Scale(NamedTuple):
    """
    Scale defined by a base note, and a list of steps in notes (measured in semitones).
    """

    base: Note
    abstract_scale: "AbstractScale"

    def __getitem__(self, item: "ScaleIndex"):
        index = item.index - 1
        return next(itertools.islice(self.notes(), index, index + 1)) + item.step_adjustment

    def notes(self) -> Iterable[Note]:
        """
        Creates an infinite generator of ascending notes in the scale, starting at `base`.
        """
        note = self.base
        for step in itertools.cycle(self.abstract_scale.steps):
            yield note
            note += step

    def frequencies(self, num: int = 7) -> List[float]:
        return [note.frequency() for note in itertools.islice(self.notes(), 0, num)]


class AbstractScale(NamedTuple):
    """
    Scale without a base note, defined by a list of steps in notes (measured in semitones).
    """

    steps: List[int]

    @classmethod
    def safe(cls, steps: List[int]) -> "AbstractScale":
        """
        Creates the abstract scale, checking that the scale wraps the octave.
        """
        assert sum(steps) == 12, f"Expected scale to loop with 12 notes, instead found {sum(steps)}"
        return AbstractScale(steps)


ABSTRACT_SCALES: Dict[str, AbstractScale] = {
    "maj": AbstractScale.safe([2, 2, 1, 2, 2, 2, 1]),
    "min": AbstractScale.safe([2, 1, 2, 2, 1, 2, 2]),
    "chromatic": AbstractScale.safe([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
    "harmonic-min": AbstractScale.safe([2, 1, 2, 2, 1, 3, 1]),
    # TODO: Fix.
    # "melodic-min": AbstractScale.safe([2, 1, 2, 2, 2, 2, 2]),
    # "super-locrian": AbstractScale.safe([1, 2, 1, 2, 2, 2, 2, 2]),
}
