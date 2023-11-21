package cicd_test

import (
	"testing"
	"fmt"
)

func TestMain(m *testing.M) {
	fmt.Println("begin")
	m.Run()
	fmt.Println("end")
}
