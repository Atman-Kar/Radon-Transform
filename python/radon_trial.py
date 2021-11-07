from radon_lib import Tissue
from PIL import Image, ImageOps, ImageEnhance, ImageFilter
import matplotlib.pyplot as plt
import numpy as np
import numpy.matlib
import numpy.fft


with Image.open("test3.png") as im:
	im = ImageOps.grayscale(im)
	pix = np.array(im)
	# pix = np.true_divide(pix, 256)
	# print(pix.shape)

t = Tissue(256)
for i in range(256):
	for j in range(256):
		t.insert_attenuation_value(pix[j][i], (i, j))

 
r = np.array(t.radon_transform())
f = np.uint8((r / np.amax(r) ) * 255)

# Recreate image without applying any filtering
im_bp = t.backprojection(r)
f_bp = np.uint8((im_bp / np.amax(im_bp) ) * 255)
im2 = Image.fromarray(f_bp.T, 'L')

# Apply ramp filter as shown in the math 
N = r.shape[0]

'''
Use an additional filter to subdue high frequency content
'''

ham = np.blackman(N)
ham = np.fft.fftshift(ham)
ham = np.matlib.repmat(ham, 180, 1)
ham = np.transpose(ham)
ramp = np.abs(np.linspace(-1, 1, N))
ramp = np.fft.fftshift(ramp)
ramp = np.matlib.repmat(ramp, 180, 1)
ramp = np.transpose(ramp)
r_fft = np.fft.fft(r, axis = 0)
r_fft = ramp * r_fft * ham
r_filt = np.real(np.fft.ifft(r_fft, axis =0))
im_filt = 0.5 * np.array(t.backprojection(r_filt))
f_filt = np.uint8((im_filt / np.amax(im_filt) ) * 255)
im3 = Image.fromarray(f_filt.T, 'L')


plt.subplot(1,4,1)
plt.imshow(im)
plt.title("Actual Image")


plt.subplot(1,4,2)
plt.imshow(f)
plt.title("Radon Transform of the Image")


plt.subplot(1,4,3)
plt.imshow(im2)
plt.title("Unfiltered BP of Image")

plt.subplot(1,4,4)
plt.imshow(im3)
plt.title("Filtered BP of Image")

plt.show()