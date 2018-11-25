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


TODO: Amazon file-by-file API
1. Transcribe to OGG or MPE

2. Upload to S3

3. Notify API to transcribe:

{
    "TranscriptionJobName": "paul.audiotest",
    "LanguageCode": "en-US",
    "MediaSampleRateHertz": 16000,
    "MediaFormat": "mp3",
    "Media": {
        "MediaFileUri": "https://s3.eu-central-1.amazonaws.com/paul.audiotest2/marcus1.mp3"
    }
}

4. Parse response:

{
    "TranscriptionJob": {
        "TranscriptionJobName": "paul.audiotest",
        "TranscriptionJobStatus": "COMPLETED",
        "LanguageCode": "en-US",
        "MediaSampleRateHertz": 16000,
        "MediaFormat": "mp3",
        "Media": {
            "MediaFileUri": "https://s3.eu-central-1.amazonaws.com/paul.audiotest2/marcus1.mp3"
        },
        "Transcript": {
            "TranscriptFileUri": "https://s3.us-east-2.amazonaws.com/aws-transcribe-us-east-2-prod/702229248381/paul.audiotest/eca43350-6a07-400f-9cc0-56b3579dfa1b/asrOutput.json?X-Amz-Security-Token=FQoGZXIvYXdzEAAaDHtoxwo%2FDU3dJfBe6yK3Ay%2FXR4N2vnSY9m1CtbB7qJgiaR8nTCOs3IOtwkUYM61nOPSFyi0hTTez7RBct%2BknQ9WeDcdzY0Cb9eDEFf79%2FLdpBke9RGSIFo9jFH97EStB9iRRBV9MytOLjkn7SZI4StDDuIOafvElKyGD4WdZ6J%2BPOj5NBBNABiwOEDcsCuFkHdqJ4KBSjVJ2UxN%2BwT%2FS5tp5YqOVbNniK6Qot9oNlVjq%2FvZRbDLV1om3gklBtAMPf2fw3%2BAw6ci8Xf%2FNuGoOhJmuvIopsNainyIp4XQxw7N2Joz7pmy29nAQ%2FhD8swTjENKOC%2BGREhxS4KHEYDNvb0VXHNM3okvmT6uRQzzstpgj5Pxh%2FB9Q2OlPWSWdnbVSoB7OIyCZtiQmA5HXHrTXh8Irj0Td6SO7lFt80rQcaPen1Kyzmy0Dynier5cjamtKgnKBC5wfXayzrAq76dGBR%2B1nNmTrhmR6FV8kgH6Zzze61YA%2BUxtSFVMMQaekfuG4Of6dTCMgCuxYZmt13c0dItg2zNrMih4STEmwu8saND7LfnXkqU1XBxU7J6Bw%2FY13aOlouO3QJaItgMf9Jkw2ca3GN3Lq7Jkoy9%2Fq3wU%3D&X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Date=20181125T144249Z&X-Amz-SignedHeaders=host&X-Amz-Expires=900&X-Amz-Credential=ASIATSXCHOUANBYXN2MZ%2F20181125%2Fus-east-2%2Fs3%2Faws4_request&X-Amz-Signature=4c5ce23cedc55ac2aea72e5f915f097cd0583436802b08c7c358ceaa795ff77c"
        },
        "CreationTime": "2018-11-25T14:18:33.371Z",
        "CompletionTime": "2018-11-25T14:23:34.931Z",
        "Settings": {
            "ChannelIdentification": false
        }
    }
}

5. Download trasnsciption

6. 
