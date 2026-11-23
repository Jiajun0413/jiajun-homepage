"""Regression test for limiting the installed pdf2zh GUI to one provider."""

import runpy
import unittest
from pathlib import Path
from unittest.mock import patch

from pdf2zh_next import main as pdf2zh_main
from pdf2zh_next.config import translate_engine_model as engines
from pdf2zh_next.config.model import SettingsModel

LAUNCHER = Path(__file__).resolve().parents[1] / "deploy" / "pdf2zh_gui.py"


class GuiLauncherTest(unittest.TestCase):
    def test_term_choices_are_limited_without_breaking_translation(self):
        translation = engines.TRANSLATION_ENGINE_METADATA
        term = engines.TERM_EXTRACTION_ENGINE_METADATA
        translation_map = engines.TRANSLATION_ENGINE_METADATA_MAP
        term_map = engines.TERM_EXTRACTION_ENGINE_METADATA_MAP
        entries = translation.copy()
        translation_lookup = translation_map.copy()
        term_lookup = term_map.copy()

        def restore():
            translation[:] = entries
            term[:] = term_entries
            translation_map.clear()
            translation_map.update(translation_lookup)
            term_map.clear()
            term_map.update(term_lookup)

        term_entries = term.copy()
        self.addCleanup(restore)

        def verify():
            self.assertEqual([item.translate_engine_type for item in term], ["OpenAICompatible"])
            self.assertEqual(translation, entries)
            self.assertEqual(translation_map, translation_lookup)
            self.assertEqual(term_map, term_lookup)

            provider = engines.OpenAICompatibleSettings(
                openai_compatible_base_url="https://example.invalid/v1",
                openai_compatible_api_key="test-key",
                openai_compatible_model="test-model",
            )
            settings = SettingsModel(
                translate_engine_settings=provider,
                term_extraction_engine_settings=provider.model_copy(),
            )
            settings.validate_settings()
            settings.validate_settings()
            for configured in (settings.translate_engine_settings, settings.term_extraction_engine_settings):
                self.assertIsInstance(configured, engines.OpenAISettings)
                self.assertEqual(configured.openai_base_url, "https://example.invalid/v1")
                self.assertTrue(any(isinstance(configured, item.setting_model_type) for item in translation))

        with patch.object(pdf2zh_main, "cli", side_effect=verify) as cli:
            runpy.run_path(str(LAUNCHER), run_name="__main__")
        cli.assert_called_once_with()


if __name__ == "__main__":
    unittest.main()
