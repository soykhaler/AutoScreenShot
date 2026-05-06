import time
import os
import mss
import cv2
import numpy as np
print("----------------------------")
print("AutoScreenshot by SoyKhaler")
print("----------------------------")
OUTPUT_FILE = "capture.jpg"
INTERVAL = 15
MONITOR = 0
with mss.mss() as sct:
    monitor = sct.monitors[MONITOR]
    print(f"Capturando frame cada {INTERVAL}s en {OUTPUT_FILE}...")
    while True:
        img = np.array(sct.grab(monitor))
        img_bgr = cv2.cvtColor(img, cv2.COLOR_BGRA2BGR)
        cv2.imwrite(OUTPUT_FILE, img_bgr)
        print("Actualizado:", OUTPUT_FILE)
        time.sleep(INTERVAL)
