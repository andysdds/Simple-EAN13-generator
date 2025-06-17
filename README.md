# Simple EAN13 generator

## Pre-requirement
#### 1. Install python/python3
#### 2, Create a virtual environment
```bash
python3 -m venv venv
```
#### 3. Install barcode package
```bash
pip install python-barcode
```
## How to use
#### 1. Perpare the code
##### Put the code in barcode.txt line by line 
#### 1. Start a virtual environment
```bash
source venv/bin/activate
```
#### 2. Run
```bash
python3 barcode_ean13.py
```
## Change it to different formet
#### You can change any format you want from python-barcode 

`barcode_ean13.py line 17`
```bash
EAN = barcode.get_barcode_class('barcode type you like')
```