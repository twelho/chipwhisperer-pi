// SPDX-License-Identifier: GPL-3.0-or-later
// Iterative summation workload for simpleserial-glitch
// (c) Dennis Marttinen 2023

int glitch() {
    volatile int i, j, k;
    k = 0;

    // Loop ranges adjusted to account for the performance of the Raspberry Pi,
    // see https://www.youtube.com/watch?v=dVkCNiM0PL8
    for (i = 0; i < 10000; i++) {
        for (j = 0; j < 10000; j++) {
            k++;
        }
    }

    return k;
}
