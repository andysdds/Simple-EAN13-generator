import barcode
from barcode.writer import ImageWriter, SVGWriter
from barcode import generate
import os

def generate_barcode(data):
    # Create svg folder if it doesn't exist
    if not os.path.exists('svg'):
        os.makedirs('svg')
        
    options = {
        "module_width": 0.25,
        "module_height": 9,
        "write_text": True,
        "quiet_zone": 5,
    }
    EAN = barcode.get_barcode_class('ean13-guard')
    ean = EAN(data)
    svg_path = os.path.join('svg', data)
    svg = ean.save(svg_path, options=options)
    
    # Read and modify SVG file
    with open(f"{svg_path}.svg", 'r') as file:
        lines = file.readlines()
    
    # Remove line containing &gt;
    lines = [line for line in lines if "&gt;" not in line]
    
    # Write back to file
    with open(f"{svg_path}.svg", 'w') as file:
        file.writelines(lines)
    
    return svg
    
if __name__ == "__main__":
    with open('barcode.txt', 'r') as file:
        for line in file:
            barcode_text = line.strip()
            generate_barcode(barcode_text)



    