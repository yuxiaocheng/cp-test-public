package success

import "testing"

// 五个成功
func TestDivision(t *testing.T) {
	Division(1, 2)
}

func TestDivision2(t *testing.T) {
	Division(1, 2)
}

func TestDivision3(t *testing.T) {
	Division(1, 2)
}

func TestDivision4(t *testing.T) {
	Division(1, 2)
}

func TestDivision5(t *testing.T) {
	Division(1, 2)
}

// 五个失败
func TestDivision6(t *testing.T) {
	t.Fail()
}

func TestDivision7(t *testing.T) {
	t.Fail()
}

func TestDivision8(t *testing.T) {
	t.Fail()
}

func TestDivision9(t *testing.T) {
	t.Fail()
}

func TestDivision10(t *testing.T) {
	t.Fail()
}

// 1个跳过
func TestDivision11(t *testing.T) {
	t.SkipNow()
}

// 1个example
func ExampleDivision() {
	Division(2, 5)
}
