# pip install moviepy
from moviepy.editor import VideoFileClip 

videoClip = VideoFileClip("no1.mp4")
videoClip.write_gif("no1.gif")
