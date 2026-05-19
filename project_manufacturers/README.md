# project_manufacturers

A small async CLI that queries the public NHTSA "get all manufacturers"
endpoint and prints every manufacturer whose name, common name, or country
matches a user-supplied keyword.

## What it does

- Sends a single HTTP GET to
  `https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json`.
- Decodes the JSON response.
- Iterates over `Results[]` and prints each entry whose `Mfr_Name`,
  `Mfr_CommonName`, or `Country` contains the keyword (case-sensitive
  substring match).

## Build & run

This crate ships a per-project `justfile`. From the repo root:

```bash
just project_manufacturers run BMW         # query manufacturers matching "BMW"
just project_manufacturers build           # cargo build
just project_manufacturers clippy          # cargo clippy -D warnings
just project_manufacturers fmt-check       # rustfmt in check mode
```

Or directly inside the crate:

```bash
cd project_manufacturers
just            # list available recipes
just run BMW
just watch BMW  # cargo-watch wrapper around `run`
```

The keyword is matched against multiple fields, so `just run Germany`
returns every German manufacturer, and `just run BMW` returns both `BMW`
and `BMW of North America, LLC`.

## Expected output

```
Keyword: BMW
Manufacturer: BMW OF NORTH AMERICA, LLC
Common Name: BMW
Country: UNITED STATES (USA)

Manufacturer: BAYERISCHE MOTOREN WERKE AG (BMW AG)
Common Name: BMW
Country: GERMANY
...
```

If nothing matches: `No manufacturers found matching '<keyword>'`.
