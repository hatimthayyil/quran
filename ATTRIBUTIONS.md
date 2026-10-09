# Attributions

## Tanzil Quran Text

The Quranic text is the Tanzil Quran Text, Version 1.1, by the Tanzil Project
(<https://tanzil.net>). It is licensed under Creative Commons Attribution 3.0,
with the Tanzil terms of use: verbatim copies only, the source clearly
indicated, and a link to tanzil.net. Licence: <https://tanzil.net/docs/text_license>.

| File                                | Text type    | sha256                                                             |
| ----------------------------------- | ------------ | ------------------------------------------------------------------ |
| `data/tanzil/quran-uthmani.xml`      | Uthmani      | `203f0f1bf3158b1e5be4ab9f8f6870e570aab6d9a626fe6192a70b75d4afe0fd` |
| `data/tanzil/quran-simple-clean.xml` | Simple Clean | `140787b7c399b11d54bfa72c237075916084232efa8a4cddf704cc8a2a74b1fe` |

Both were downloaded on 2026-10-09 without the download form's options, so
they have no pause marks and no sajdah signs:

```text
https://tanzil.net/pub/download/index.php?quranType=uthmani&outType=xml&agree=true
https://tanzil.net/pub/download/index.php?quranType=simple-clean&outType=xml&agree=true
```

How this project complies:

- The files are stored and embedded in the binary byte for byte, with their
  copyright block. Tests pin each file's sha256.
- Output prints the original bytes. The Basmala of surahs 2 to 114 is the
  file's own `bismillah` attribute, printed as given; nothing is stripped or
  normalised.
- `quran --license` prints the Tanzil copyright block verbatim with a link to
  <https://tanzil.net>; `quran --help` and `README.md` name Tanzil.

Updates: <https://tanzil.net/updates/>.
