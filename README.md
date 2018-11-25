# cpal-record
Cross platform audio recording test

Just a quick test for Raspberry Pi audio recordings to compare quality using a variety of USB mic devices.

1. Setup Pi audio
sudo apt-get install rpi-update
sudo rpi-update
sudo apt-get install alsa-utils
sudo apt-get install mpg321
sudo apt-get install lame
sudo apt install libasound2-dev
arecord -l
arecord -d 5              # 5 second wav file on the above default device

2. More at http://iwearshorts.com/blog/raspberry-pi-setting-up-your-audio/ 
http://www.g7smy.co.uk/2013/08/recording-sound-on-the-raspberry-pi/  


To manually setup Cortana for voice activation by "Hey Cortana, start transcribing" see https://www.windowscentral.com/how-run-custom-commands-using-cortana-windows-10 

You should point that Start menu shortcut to the application at C:\Users\MY-USERNAME\github\cpal-record\target\debug\cpal-record.exe 
