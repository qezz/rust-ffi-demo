#include <stdbool.h>

#include <stdio.h>
#include <omp.h>

extern void transpose(double * A, double * B, int n) {
    int i,j;
    for(i=0; i<n; i++) {
	for(j=0; j<n; j++) {
	    B[j*n+i] = A[i*n+j];
	}
    }
}

// Multiply A by B into C.
// size n by n
extern void multiply(double * A, double * B, double * C, int n) {
    double * B2 = (double *) malloc(sizeof(double) * n * n);
    transpose(B, B2, n);
#pragma omp parallel
    {
#pragma omp for
	for (int i = 0; i < n; i++) {
	    for (int j = 0; j < n; j++) {
		double dot  = 0;
		for (int k = 0; k < n; k++) {
		    dot += A[i * n + k] * B2[j * n + k];
		}
		C[i * n + j] = dot;
	    }
	}

    }
    free(B2);
}
// Linear multiply A by B into C.
// size n by n
extern void multiply_l(double * A, double * B, double * C, int n) {
    double * B2 = (double *) malloc(sizeof(double) * n * n);
    transpose(B, B2, n);

    for (int i = 0; i < n; i++) {
	for (int j = 0; j < n; j++) {
	    double dot  = 0;
	    for (int k = 0; k < n; k++) {
		dot += A[i * n + k] * B2[j * n + k];
	    }
	    C[i * n + j] = dot;
	}
    }

    free(B2);
}

extern void multiply_pf(double * A, double * B, double * C, int n) {
    double * B2 = (double *) malloc(sizeof(double) * n * n);
    transpose(B, B2, n);

#pragma omp parallel for
    for (int i = 0; i < n; i++) {
	for (int j = 0; j < n; j++) {
	    double dot  = 0;
	    for (int k = 0; k < n; k++) {
		dot += A[i * n + k] * B2[j * n + k];
	    }
	    C[i * n + j] = dot;
	}
    }

    free(B2);
}


bool isEqual(double * mx1, double * mx2, int n) {
    for (int row = 0; row < n; ++row) {
	for (int col = 0; col < n; ++col) {
	    if (mx1[row * n + col] != mx2[row * n + col]) {
		return false;
	    }
	}
    }

    return true;
}
