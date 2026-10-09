# quran

Print Quranic text for a reference query.

```sh
quran 2:255          # one ayah
quran 2:12-16        # a range; also 2:282-3:5 across surahs
quran 2:255,3:18     # several references, in that order
quran 2:{1,5,10-12}  # a set
quran '*:1'          # ayah 1 of every surah; quote * in zsh
quran 112            # a whole surah, with its Basmala on the first line
quran --text simple-clean 1:1
```

Each ayah prints as `S:A<TAB>text`. Errors go to stderr with a stable code.
The grammar is in `ai/specs/reference-grammar/spec.md`.

## Install

```sh
nix profile add /hatimthayyil/code/quran#quran
nix profile upgrade quran
```

## Text

Tanzil Quran Text (Uthmani and Simple Clean, Version 1.1) by the Tanzil
Project, <https://tanzil.net>, CC BY 3.0, embedded verbatim. See
`ATTRIBUTIONS.md` and `quran --license`.
