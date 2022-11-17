import pyautogui 
import time 
# from PIL import Image
# import webbrowser 

#url = webbrowser.open("https://i.postimg.cc/ZRtPHmt3/68747470733a2f2f6d656469612e67697068792e636f6d2f6d656469612f57556c706c634d704f43456d5447427442572f67.gif")

time.sleep(3)
for i in range(10):
    im1 = pyautogui.screenshot()
    im1.save('./awifu.gif')
    # pyautogui.typewrite()
    pyautogui.press("enter")
