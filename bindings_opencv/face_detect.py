import cv2
import timeit

start = timeit.default_timer()
# Load the cascade
face_cascade = cv2.CascadeClassifier('haarcascades/haarcascade_frontalface_alt.xml')
# Read the input image
img = cv2.imread('multifaces.jpeg')
# Convert into grayscale
gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
# Detect faces
faces = face_cascade.detectMultiScale(gray, 1.1, 4)
# Draw rectangle around the faces
for (x, y, w, h) in faces:
    cv2.rectangle(img, (x, y), (x+w, y+h), (255, 0, 0), 2)
# Display the output
stop = timeit.default_timer()
print('Time: ', stop - start)
cv2.imshow('img', img)
cv2.waitKey()