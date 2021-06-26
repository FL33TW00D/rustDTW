<h1 align="center">
  RustDTW
</h1>
<p align="center">Python extension backed by a multi-threaded Rust implementation of Dynamic Time Warping (DTW).</p>
<div align="center">
<img src="https://img.shields.io/pypi/v/rust-dtw?style=flat-square"/><br/>  
</div>

## ❓ Why rustDTW?
**rustDTW** is a fast and user friendly implementation of Dynamic Time Warping. In constrast to most popularly used DTW libraries, it requires no C compilation (with frustrating build errors) with little performance loss. It can be installed on all operating systems and supports Python3.6 and above.

## ⚡️ Quick Install
To install rustDTW, simply:

```shell
pip install rust-dtw
```

## ❓ What is Dynamic Time Warping?
In time series analysis, dynamic time warping (DTW) is one of the algorithms for measuring similarity between two temporal sequences, which may vary in speed. [1](https://en.wikipedia.org/wiki/Dynamic_time_warping). This has applications in speech recognition, time series classification and neuroscience.

## Example Usage
  
rustDTW was designed for usage with timeseries data from functional brain regions. However any data represented as a numpy matrix can be provided.
```python
import numpy as np
import rust_dtw
  
rust_dtw.dtw(
  s=np.array([0., 1., 2.]), 
  t=np.array([3., 4., 5.]), 
  window=50, 
  distance_mode="euclidean"
  )
>>> 5.0990195
```
For more examples please see `examples/`
  
## Developing

### Built With
- PyO3
- Maturin
- rust-numpy
- Rayon

### Setting up Dev
To get started with development, simply clone the repository and edit the main library code in `src/`. Once done, simply build and test the code with `./build.sh`.

```shell
git clone https://github.com/FL33TW00D/rustDTW.git
cd rust-dtw/
./build.sh
```

## Tests

All tests are implemented using pytest.
```shell
poetry run pytest
```

## 📈 Performance
<div align="center">
<img src="https://raw.githubusercontent.com/FL33TW00D/rustDTW/master/examples/speed/time.png" width=60%/><br/>  
</div>
The above shows the performance of the rustdtw implementation vs the DTAIDistance OpenMP Python version (more benchmarks vs C implementation coming soon).

## ⚠️ License

`rustDTW` is free and open-source software licensed under the [MIT License](https://github.com/FLE33TW00D/rustDTW/blob/master/LICENSE).

