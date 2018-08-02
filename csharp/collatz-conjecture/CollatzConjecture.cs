﻿using System;

public static class CollatzConjecture {
	public static int Steps(int number) {
		if (number < 1) {
			throw new ArgumentException("Value does not fall within the expected range.");
		}

		int step = 0;

		while (number != 1) {
			if (number % 2 == 0) {
				number /= 2;
			}
			else {
				number = number * 3 + 1;
			}
			step++;
		}
		return step;
	}
}