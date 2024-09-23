import unittest
import numpy as np
from ml_model.model import ArbitragePredictor

class TestArbitragePredictor(unittest.TestCase):
    def setUp(self):
        self.predictor = ArbitragePredictor()

    def test_model_prediction(self):
        X = np.random.rand(10, 20)  # 10 samples, 20 features
        predictions = self.predictor.predict(X)
        self.assertEqual(predictions.shape, (10, 3))  # 3 outputs: profit, optimal amount, route

    def test_model_training(self):
        X = np.random.rand(100, 20)
        y = np.random.rand(100, 3)
        initial_weights = self.predictor.model.get_weights()
        self.predictor.train(X, y)
        final_weights = self.predictor.model.get_weights()
        self.assertFalse(np.allclose(initial_weights, final_weights))

    # Add more tests for other methods and edge cases

if __name__ == '__main__':
    unittest.main()
