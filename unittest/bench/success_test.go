package success

import "testing"

// 1个性能
func BenchmarkDivision(b *testing.B) {
	b.StopTimer()
	b.StartTimer()
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		Division(7, 3)
	}
}
