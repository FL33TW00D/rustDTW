import numpy as np
import rust_dtw

# Tests
def test_date():
    result = rust_dtw.dtw(
        s=np.array([0., 0., 1., 2., 1., 0., 1., 0., 0.]), 
        t=np.array([0., 1., 2., 0., 0., 0., 0., 0., 0.]), 
        window=50, 
        distance_mode="euclidean"
    );
    assert result == 1.4142135623730951