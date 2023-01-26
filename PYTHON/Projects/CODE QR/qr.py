# Frist Install below Modules
# Pip install pyqrcode
# pip install pypng 

import pyqrcode
import png

from pyqrcode import QRCode 

import pyqrcode
import png 
from pyqrcode import QRCode 

# String which Reprecent The QR Code 
s = "www.facebook.com"

# Generate QR Code
url = pyqrcode.create(s) 

# Create Save The Svg File Naming "myqr.svg" 
url.svg("myqr.svg", scale = 8) 

# Create Save The Png File Naming "myqr.png"
url.png("myqr.png", scale = 6)
