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
        "font_size": 8,
        "center_text":True,
        "text_distance": 0.1,
        "text_font": "Arial"
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
    
    # Add letter-spacing to text elements
    lines = [line.replace('text-anchor:middle;', f'text-anchor:middle;letter-spacing:{options["text_distance"]}mm;') for line in lines]
    
    # issue : https://github.com/WhyNotHugo/python-barcode/issues/104
    # I use replace here,
    lines = [line.replace('text-anchor:middle;', f'text-anchor:middle;font-family:{options["text_font"]};') for line in lines]
        
    # Write back to file
    with open(f"{svg_path}.svg", 'w') as file:
        file.writelines(lines)
    
    return svg
    
if __name__ == "__main__":
    with open('barcode.txt', 'r') as file:
        for line in file:
            barcode_text = line.strip()
            generate_barcode(barcode_text)



    