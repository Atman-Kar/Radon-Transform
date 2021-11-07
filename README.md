# Radon Transform

A rust and python implementation of the discretized version of the [radon transform](https://en.wikipedia.org/wiki/Radon_transform#:~:text=In%20mathematics%2C%20the%20Radon%20transform,the%20function%20over%20that%20line.) that I did out of boredom. The main mathematical transformation is written in Rust. The Rust functions are then wrapped up as Python functions using [PyO3](https://github.com/PyO3/pyo3), which is later used along with other libs like numpy and PIL. 


## Building the project

To the run the script, you will first have to build the project. You can build this rust project using **cargo**:

```
cargo build --release
```

After building this, navigate to `` target/release ``, where you will find the extension `` randon_lib.dll `` (if you are on windows) or `` radon_lib.so `` (on OSX). Copy this file into the `` python `` folder and rename the extension to `` radon_lib.pyd ``

## Running the project 

To call the underlying rust functions, import the `` radon_lib.pyd `` that you created earlier in your python file, `` radon_trial.py `` for example.

```
import radon_lib
```

You **must** make sure that `` radon_lib.pyd `` is present in the same directory as `` randon_trial.py ``. 

Once this step is done, you can load the image of choice and visualize the Radon and Inverse Radon Transform. 

```
python3 radon_trial.py
```

## Why is there a struct named "Tissue" in the Rust implementation?

Radon Transform is the heart of the [Computed Tomography (CT Scan)](https://en.wikipedia.org/wiki/CT_scan), which is used for non-invasive diagnosis. The CT Scan is capable of reconstructing the 3-D tissue (along with their attenuation coefficient, meaning how dense is the tissue) by taking multiple 2-D X-Ray slices and stiching them up. The 3-D reconstruction of the original tissue is done by a process called "backprojection" aka the Inverse Radon Transform. I found this interesting and thought I would try it, therefore calling the image struct as "Tissue". Feel free to change that name if you want to use this code for some reason.  
