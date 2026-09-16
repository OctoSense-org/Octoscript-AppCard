import json
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from core.studio_bridge import loads_row


class LoadsRowTest(unittest.TestCase):
    def test_strict_json_passes_through(self):
        self.assertEqual(loads_row('{"Hello":{"client_id":[1]}}'), {"Hello": {"client_id": [1]}})

    def test_trailing_commas_from_absent_optional_fields(self):
        row = ('{"WidgetSnapshot":{"query_id":[1],"build_id":[1],"widgets":['
               '{"id":"-","widget_type":"Root","visible":false,"x":0,"y":0,"width":0,"height":0,},'
               '{"id":"main_window","widget_type":"Window","visible":true,"width":406,"height":776,},]}}')
        widgets = loads_row(row)["WidgetSnapshot"]["widgets"]
        self.assertEqual([w["id"] for w in widgets], ["-", "main_window"])

    def test_still_rejects_garbage(self):
        with self.assertRaises(ValueError):
            loads_row("studio remote: invalid request json")


if __name__ == "__main__":
    unittest.main()
