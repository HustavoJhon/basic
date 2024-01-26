#step 1: Install pytube
pip install pytube

from pytube import Playlist, Youtube

playlist = Playlist("URL of your youtube playlist here")
for video in playlist:
    print("Starting download of:")
    print("===")
    print(Youtube(video).title)
    Youtube(video).streams.get_highest_resolution().download('D:/Videos/')
    print("===")
