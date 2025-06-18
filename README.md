# Simple EAN13 Generator

## Prerequisites
1. Install Python/Python3
2. Create a virtual environment
```bash
python3 -m venv venv
```
3. Start the virtual environment
```bash
source venv/bin/activate
```
4. Install the barcode package
```bash
pip install python-barcode
```

## How to Use
1. Prepare the code
Put the codes in barcode.txt, one per line
2. Start the virtual environment if you are not in
```bash
source venv/bin/activate
```
For other platforms, please refer to [Python's venv documentation](https://docs.python.org/3/library/venv.html#how-venvs-work).

3. Run the script
```bash
python3 barcode_ean13.py
```

## Changing the Barcode Format
You can change the barcode format by modifying line 17 in `barcode_ean13.py`:
```bash
EAN = barcode.get_barcode_class('barcode type you like')
```

For more information, visit the [python-barcode documentation](https://python-barcode.readthedocs.io/en/stable/getting-started.html?highlight=get_barcode_class#interactive-generating-an-svg).
