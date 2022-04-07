# place rust 2022

Simple Interpreter for r/place 2022 data.
Basically recreates a timelapse from the data.
It is currently based on 40000 pixels per frame, rather than taking the actual timestamps into account.
It also does not include the rectangle edits by mods.
As a side note; At this time the reddit provided data has the rectangles logged incorrectly.

## how to use

The program currently looks for data/2022_place_canvas_history.csv as provided by reddit.
This data needs to be sorted beforehand though:

```
head -n 1 2022_place_canvas_history.csv > sorted.csv
tail -n +2 2022_place_canvas_history.csv | sort >> sorted.csv
mv sorted.csv 2022_place_canvas_history.csv
```

Make sure to run in release version for best performance:

```
cargo run --release
```

The pace of the reconstruction can be changed by adjusting the rate variable in the Place struct.
