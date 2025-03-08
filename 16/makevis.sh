#!/bin/bash

ffmpeg -hide_banner -loglevel error -r 30 -y -i out_a.gif \
    -filter:v scale=1000:-1:flags=neighbor \
    out_a.mp4
echo "Finished creating part 1 with code ${?}"

ffmpeg -hide_banner -loglevel error -r 30 -y -i out_b.gif \
    -filter:v scale=1000:-1:flags=neighbor \
    out_b.mp4
echo "Finished creating part 2 with code ${?}"

ffmpeg -hide_banner -loglevel error -r 30 -y -i out_c.gif \
    -filter:v scale=1000:-1:flags=neighbor \
    out_c.mp4
echo "Finished creating part 3 with code ${?}"

echo -e "file 'out_a.mp4'\nfile 'out_b.mp4'\nfile 'out_c.mp4'" >> .makevis.list.txt

ffmpeg -hide_banner -loglevel error -f concat -y -i .makevis.list.txt -c copy final.mp4
echo "Finished concatenating video with code ${?}"

rm .makevis.list.txt
