"""The OpenHarmony instrument maps an ArkUI dump into makepad's inspection shapes."""
import json
import unittest

from core import ohos_instrument as oi

TREE = {"attributes": {"type": "root", "bounds": "[0,0][1320,2760]"}, "children": [
    {"attributes": {"type": "Column", "id": "beauty_0", "bounds": "[0,146][1320,2669]"}, "children": [
        {"attributes": {"type": "Text", "id": "beauty_0_0", "bounds": "[81,84][900,185]", "text": "The Daily",
                        "enabled": "true", "visible": "true"}, "children": []},
        {"attributes": {"type": "Checkbox", "id": "beauty_0_1", "bounds": "[0,0][65,65]", "checked": "true"}, "children": []},
    ]},
]}


class Shapes(unittest.TestCase):
    def setUp(self):
        self.nodes = oi.flatten(TREE, 3.25)

    def test_tree_dump_is_makepads_w3_lines_in_logical_pixels(self):
        dump = oi.tree_dump(self.nodes, 7)["dump"].split("\n")
        self.assertEqual(dump[0], "W3 4")
        self.assertEqual(dump[1], "0 -1 - root 0 0 406 849")
        self.assertEqual(dump[3], "2 1 beauty_0_0 Text 25 26 252 31")

    def test_query_returns_every_node_with_that_id(self):
        q = oi.query(self.nodes, "beauty_0_0", 7)
        self.assertEqual(q["query"], "id:beauty_0_0")
        self.assertEqual(q["rects"], ["2 beauty_0_0 Text 25 26 252 31"])
        self.assertEqual(oi.query(self.nodes, "missing", 7)["rects"], [])

    def test_snapshot_carries_text_and_control_state(self):
        widgets = oi.snapshot(self.nodes, 7)["widgets"]
        by_id = {w["id"]: w for w in widgets}
        self.assertEqual(by_id["beauty_0_0"]["text"], "The Daily")
        self.assertTrue(by_id["beauty_0_1"]["checked"])
        self.assertTrue(by_id["-"]["visible"])
        self.assertAlmostEqual(by_id["beauty_0"]["width"], 406.15, places=1)

    def test_bounds_parse_and_reject_garbage(self):
        self.assertEqual(oi.parse_bounds("[1,2][3,4]"), (1, 2, 3, 4))
        self.assertIsNone(oi.parse_bounds("nope"))


if __name__ == "__main__":
    unittest.main()
