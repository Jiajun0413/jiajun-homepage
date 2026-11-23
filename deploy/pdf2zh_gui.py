#!/usr/bin/env python3
"""Run the pdf2zh GUI with OpenAICompatible as the only visible channel.

pdf2zh 2.10.2 maps a selected "OpenAICompatible" translation setting to the
internal "OpenAI" engine on validate, so the internal dispatch loops over the
full TRANSLATION_ENGINE_METADATA registry. Only the term-extraction dropdown
must be narrowed to the OpenAICompatible metadata; leaving every registry
intact keeps the second validate on the transformed settings working.
"""
from pdf2zh_next.config import translate_engine_model as engines
from pdf2zh_next.main import cli

_ENGINE = "OpenAICompatible"


def main() -> None:
    engines.TERM_EXTRACTION_ENGINE_METADATA[:] = [
        m for m in engines.TERM_EXTRACTION_ENGINE_METADATA if m.translate_engine_type == _ENGINE
    ]
    cli()


if __name__ == "__main__":
    main()