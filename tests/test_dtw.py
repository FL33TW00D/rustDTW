import numpy as np
import os
import rust_dtw

def test_dtw_euclid():
    """
    Toy test, hardcoded result computed using DTAIDistance
    """
    result = rust_dtw.dtw(
        s=np.array([0., 1., 2.]),
        t=np.array([3., 4., 5.]),
        window=50,
        distance_mode="euclidean"
    )
    assert result == 5.0990195135927845


def test_dtw_manhattan():
    """
    Toy test, result computed by hand
    """
    result = rust_dtw.dtw(
        s=np.array([0., 0., 1., 2., 1., 0., 1., 0., 0.]),
        t=np.array([0., 1., 2., 0., 0., 0., 0., 0., 0.]),
        window=50,
        distance_mode="manhattan"
    )
    assert result == 2.0


def test_dtw_connectome():
    """
    Toy test, hardcoded result computed using DTAIDistance
    """
    timeseries = np.array([[0., 2., 4.], [1., 3., 5.]])
    result = rust_dtw.dtw_connectome(
        connectome=timeseries, window=50, distance_mode="euclidean")
    np.testing.assert_array_almost_equal(result, np.array(
        [0., 2.82842712, 0., 5.65685425, 2.82842712, 0.]))


def test_dtw_connectome_fmri():
    """
    Function that uses real data from nilearns datatsets module datasets.fetch_development_fmri(n_subjects=1)
    Masked as per the example using the MSDL atlas, should result in 39 x 39 connectome
    """
    timeseries = np.load(os.getcwd() + "/tests/resources/fmri_ex.npy")
    ground_truth = np.load(os.getcwd() + "/tests/resources/fmri_ground.npy")
    #Bumped up window due to length of timeseries (169 time points)
    result = rust_dtw.dtw_connectome(
        connectome=timeseries, window=100, distance_mode="euclidean")
    np.testing.assert_array_almost_equal(result, ground_truth)

def test_dtw_connectomes():
    """
    TODO: use a real subject list and real matricies
    """
    timeseries = np.random.rand(3,3,3)
    result = rust_dtw.dtw_connectomes(connectomes=timeseries, window=100, vectorize=False, distance_mode="euclidean")

def test_dtw_connectomes_vectorize():
    """
    TODO: use a real subject list and real matricies
    """
    timeseries = np.random.rand(3,3,3)
    result = rust_dtw.dtw_connectomes(connectomes=timeseries, window=100, vectorize=True, distance_mode="euclidean")