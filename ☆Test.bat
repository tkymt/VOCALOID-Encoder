setlocal

SET "PFM_IMAGE=ame_no_hi_ni_toshokan_he.pfm"
SET "PCM_AUDIO=ame_no_hi_ni_toshokan_he.wav"
SET "OUTPUT_NAME=test_output.mov"

del output.mov
call Encode.bat %PFM_IMAGE% %PCM_AUDIO% 2> log.txt

endlocal
