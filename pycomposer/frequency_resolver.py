from typing import List
import subprocess
import sys


def chord(name: str) -> List[float]:
    """
    Get the frequencies of a chord
    """
    return __resolve(["chord", name])


def scale(name: str) -> List[float]:
    """
    Get the frequencies of a scale
    """
    return __resolve(["scale", name])


def __resolve(resolver_args: List[str]) -> List[float]:
    args = ["freq-resolver"] + __get_command() + resolver_args
    output = subprocess.check_output(args)
    return [float(f) for f in output.split()]


def __get_command():
    return list(filter(lambda a: a != "-c", sys.argv))
