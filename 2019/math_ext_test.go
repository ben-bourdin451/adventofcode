package adventofcode

import (
	"testing"
)

func TestMath_lcmm(t *testing.T) {
	tests := []struct {
		args []int
		want int
	}{
		{[]int{3, 4, 6}, 12},
		{[]int{21, 7, 6}, 42},
		{[]int{12, 18, 30}, 180},
	}
	for _, tt := range tests {
		t.Run("", func(t *testing.T) {
			if got := lcmm(tt.args); got != tt.want {
				t.Errorf("lcmm(%v) = %v, want %v", tt.args, got, tt.want)
			}
		})
	}
}

func TestMath_lcm(t *testing.T) {
	tests := []struct {
		a, b int
		want int
	}{
		{21, 6, 42},
		{12, 30, 60},
		{300, 24, 600},
	}
	for _, tt := range tests {
		t.Run("", func(t *testing.T) {
			if got := lcm(tt.a, tt.b); got != tt.want {
				t.Errorf("leastCommonMultiple() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestMath_gcd(t *testing.T) {
	tests := []struct {
		a    int
		b    int
		want int
	}{
		{18, 27, 9},
	}
	for _, tt := range tests {
		t.Run("", func(t *testing.T) {
			if got := gcd(tt.a, tt.b); got != tt.want {
				t.Errorf("gcd() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestMath_gcdm(t *testing.T) {
	tests := []struct {
		arr  []int
		want int
	}{
		{[]int{18, 27}, 9},
		{[]int{20, 50, 120}, 10},
		{[]int{182664, 154875, 137688}, 3},
	}
	for _, tt := range tests {
		t.Run("", func(t *testing.T) {
			if got := gcdm(tt.arr); got != tt.want {
				t.Errorf("gcdm() = %v, want %v", got, tt.want)
			}
		})
	}
}
