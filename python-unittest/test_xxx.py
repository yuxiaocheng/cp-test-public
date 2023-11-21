import example


class TestClass():
    def test_success_one(self):
        res = example.add(1, 2)
        assert res == 3

    def test_success_two(self):
        res = example.add(1, 2)
        assert res == 3

    def test_success_three(self):
        res = example.add(3, 5)
        assert res == 8

    def test_failed_one(self):
        res = example.add(1, 2)
        assert res == 5

    def test_failed_two(self):
        res = example.add(1, 2)
        assert res == 8

    def test_failed_three(self):
        res = example.add(4,5)
        assert res == 10