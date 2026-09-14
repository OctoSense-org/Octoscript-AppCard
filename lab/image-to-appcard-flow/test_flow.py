import copy
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

import flow


class FlowTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='flow project ')
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.doc = json.loads((Path(__file__).parent / 'examples/aircon.flow.json').read_text())
        self.manifest = self.root / 'flow.json'
        self.save()

    def save(self):
        self.manifest.write_text(json.dumps(self.doc))

    def test_paths_cannot_escape_project_or_follow_escape_symlink(self):
        for value in ('../outside', '/absolute', ''):
            with self.assertRaises(ValueError):
                flow.local(self.root, value)
        (self.root / 'escape').symlink_to(self.root.parent)
        with self.assertRaises(ValueError):
            flow.local(self.root, 'escape/anything')

    def test_manifest_requires_distinct_scenes_and_explicit_surfaces(self):
        for mutate in (lambda d: d['scenes'][1].update(id='1'),
                       lambda d: d['scenes'][0].update(surface='assistant'),
                       lambda d: d.update(scenes=d['scenes'][:7])):
            doc = copy.deepcopy(self.doc)
            mutate(doc)
            self.manifest.write_text(json.dumps(doc))
            with self.assertRaises(ValueError):
                flow.read_manifest(self.manifest, self.root)

    def test_plan_is_explicit_and_never_publishes(self):
        commands = flow.commands_for('integrate', self.doc, self.manifest, self.root, self.root / 'site', '/node path', sys.executable)
        self.assertEqual(commands[0]['argv'][0], '/node path')
        self.assertTrue(commands[0]['argv'][1].endswith('scripts/sync-service-wizard.mjs'))
        self.assertNotIn('publish', flow.STAGES)
        self.assertFalse((self.root / 'pipeline-output').exists())

    def test_missing_website_cannot_turn_check_into_other_cwd(self):
        with self.assertRaisesRegex(ValueError, 'requires --website'):
            flow.commands_for('web-test', self.doc, self.manifest, self.root, None, 'node', sys.executable)

    def test_failed_stage_stops_later_commands_and_retains_failed_receipt(self):
        self.doc['checks']['service-test'] = [{'argv': [sys.executable, '-c', 'raise SystemExit(7)']}]
        self.doc['checks']['web-test'] = [{'argv': [sys.executable, '-c', "open('should-not-exist','w').write('bad')"]}]
        self.save()
        with self.assertRaisesRegex(RuntimeError, 'service-test failed'):
            flow.run(['service-test', 'web-test'], self.doc, self.manifest, self.root, None, 'node', sys.executable)
        record = json.loads(next((self.root / 'pipeline-output/runs').glob('*/run.json')).read_text())
        self.assertEqual(record['status'], 'failed')
        self.assertEqual(record['stages'][0]['logs'][0]['exit_code'], 7)
        self.assertIn('web-test', record['not_run'])
        self.assertFalse((self.root / 'should-not-exist').exists())
        self.assertNotIn('accepted', record)

    def test_unconfigured_stage_fails_before_any_command(self):
        self.doc['checks']['web-test'] = []
        with patch('flow.subprocess.run') as execute:
            with self.assertRaises(ValueError):
                flow.run(['service-test', 'web-test'], self.doc, self.manifest, self.root, None, 'node', sys.executable)
            execute.assert_not_called()

    def test_status_fingerprint_detects_mapping_edits(self):
        before = flow.fingerprint(self.doc, self.manifest, self.root)
        model = self.root / self.doc['scenes'][0]['directory'] / 'mapped.json'
        model.parent.mkdir(parents=True)
        model.write_text('{}')
        self.assertNotEqual(before, flow.fingerprint(self.doc, self.manifest, self.root))

    def test_command_arguments_are_not_shell_evaluated(self):
        literal = 'literal $(touch PWNED) `touch ALSO_PWNED`'
        self.doc['checks']['service-test'] = [{'argv': [sys.executable, '-c', 'import sys; print(sys.argv[1])', literal]}]
        self.save()
        receipt = flow.run(['service-test'], self.doc, self.manifest, self.root, None, 'node', sys.executable)
        self.assertEqual(receipt['status'], 'completed')
        self.assertEqual(receipt['stages'][0]['status'], 'passed')
        self.assertIn('capture', receipt['not_run'])
        self.assertFalse((self.root / 'PWNED').exists())


if __name__ == '__main__':
    unittest.main()
