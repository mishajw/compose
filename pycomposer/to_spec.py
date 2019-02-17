import yaml


class ToSpec:
    """Implementors can be converted to specs"""

    def __init__(self, name: str, fields: dict) -> None:
        self.name = name
        self.fields = fields

    def to_dict(self) -> dict:
        """Convert to a python dictionary representation of the spec"""
        return dict(self.__to_dict_iter())

    def __to_dict_iter(self):
        yield "name", self.name
        for key in self.fields:
            value = self.fields[key]
            key = key.replace("_", "-")
            if value is None:
                continue
            if isinstance(value, ToSpec):
                value = value.to_dict()
            if isinstance(value, list) \
                    and all(isinstance(v, ToSpec) for v in value):
                value = [v.to_dict() for v in value]
            yield key, value

    def to_spec(self) -> str:
        """Convert to a string representation of the spec"""
        return yaml.dump(self.to_dict())
