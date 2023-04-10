//GOsequential.go
package main

import(
	"fmt"
	"time"
	"strconv"
	"rand"
	"os"
)

func main(){
	
	if len(os.Args) != 4 {
		fmt.Println("Usage: make sequential [matrizN] [seed]")
	}
	args := os.Args
	N, err1 := strconv.Atoi(args[3])
	if err1 == nil {
		fmt.Println(err1)
	}
	seed, err2 := strconv.Atoi(args[4])
	if err2 == nil{
		fmt.Println(err2)
	}
	
	// A * X  = B
	var B,X [N]float64
	var A [N][N]float64
	
	fmt.Printf("Matriz dimension size: %d.\n Seed: %d.\n", N, seed)
	
	// Initialize A, B and X
	for i := 0; i < N; i++ {
		for j := 0; j < N; j++ {
			A[j][i] = float64(rand.Seed(int64(seed))) /32768.0
		}
		B[i] =  float64(rand.Seed(int64(seed))) /32768.0
		X[i] = 0.0
	}
	
	// Print inputs
	printIn(N,A,B,X)

}

func printIn(N int, A [][]float64, B,X []float64){
	
	fmt.Println("--A--")
	for i := 0; i < N; i++{
		for j := 0; j < N; j++{
			fmt.Printf("[ %v ]", A[i][j])			
		}
		fmt.Println(\n)
	}
	fmt.Println("--B--")
	for i := 0; i < N; i++{
		fmt.Printf("[ %v ]", B[i])
	}
	fmt.Println("--X--")
	for i := 0; i < N; i++{
		fmt.Printf("[ %v ]", X[i])
	}

}

func printOut(N int, X []float64){
	fmt.Printf("--Answer--")
	for i := 0; i < N; i++{
		fmt.Pintln("[ %v ]", X[i])
	}
}

func gauss(N int, X,B []float64, A [][]float64){
	var multiplier float64
	var norm, col, row int
	
	for norm = 0; norm < N-1 ; norm ++ {
		for row = norm+1; row <  N; row++ {
			multiplier = A[row][norm] / A[norm][norm]
			for col = norm; col < N; col++ {
				A[row][col] -= A[norm][col] * multiplier
			}
			B[row] -= B[norm] * multiplier
		}
	}
	
	for row = N1; row >= 0; row--{
		X[row] = B[row]
		for col = N-1; col > row; col-- {
			X[row] -= A[row][col] * X[col]
		}
		X[row] /= A[row][row]
	}
}