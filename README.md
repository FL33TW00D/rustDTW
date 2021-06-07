<h1 align="center">
  <img src="https://miro.medium.com/max/907/1*Bzubc5uGFXd_-Sj7W_QFjg.png" width="200px"/><br/>
  RustDTW
</h1>
<p align="center">Python extension backed by a multi-threaded Rust implementation of Dynamic Time Warping (DTW).</p>
<div align="center">
<img src="https://img.shields.io/pypi/v/rust-dtw?style=flat-square"/><br/>  
</div>


## ⚡️ Quick Installation

To install rustDTW, simply:

```shell
pip install rust-dtw
```

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
  
## Developing

### Built With
- PyO3
- Maturin
- rust-numpy
- Rayon


### Setting up Dev

Here's a brief intro about what a developer must do in order to start developing
the project further:

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
How well does the Rust implementation stack up against some other Python/C versions.

## ⚠️ License

`rustDTW` is free and open-source software licensed under the [MIT License](https://github.com/FLE33TW00D/rustDTW/blob/master/LICENSE).

