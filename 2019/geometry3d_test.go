package adventofcode

import (
	"reflect"
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_point3D_add(t *testing.T) {
	tests := []struct {
		name  string
		p     *point3D
		other *point3D
		want  *point3D
	}{
		{
			"0 gives 0",
			&point3D{0, 0, 0},
			&point3D{0, 0, 0},
			&point3D{0, 0, 0},
		},
		{
			"positive values",
			&point3D{0, 0, 0},
			&point3D{1, 0, 1},
			&point3D{1, 0, 1},
		},
		{
			"negative values",
			&point3D{1, 2, 3},
			&point3D{-2, 0, 3},
			&point3D{-1, 2, 6},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.p.add(tt.other); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("point3D.add() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Test_newPoint3DFromString(t *testing.T) {
	tests := []struct {
		s    string
		want *point3D
	}{
		{"<x=14, y=4, z=5>", &point3D{14, 4, 5}},
		{"<x=12, y=10, z=8>", &point3D{12, 10, 8}},
		{"<x=1, y=7, z=-10>", &point3D{1, 7, -10}},
		{"<x=16, y=-5, z=3>", &point3D{16, -5, 3}},
	}
	for _, tt := range tests {
		t.Run(tt.s, func(t *testing.T) {
			got := newPoint3DFromString(tt.s)
			require.Equal(t, tt.want, got)
		})
	}
}

func Test_point3D_String(t *testing.T) {
	tests := []struct {
		p    *point3D
		want string
	}{
		{&point3D{1, 2, 3}, "<x=  1, y=  2, z=  3>"},
		{&point3D{-10, -2, 13}, "<x=-10, y= -2, z= 13>"},
	}
	for _, tt := range tests {
		t.Run(tt.want, func(t *testing.T) {
			if got := tt.p.String(); got != tt.want {
				t.Errorf("point3D.String() = %v, want %v", got, tt.want)
			}
		})
	}
}
